//! Application runtime with event loop.
//!
//! Provides an `App` struct that manages the render-input loop for interactive UIs.
//!
//! The event loop pattern:
//! ```ignore
//! App::new()?.run(
//!     |app| { /* build UI */ element! { ... } },
//!     |app, key| { /* handle input */ },
//! )
//! ```
//!
//! The App handles:
//! - Raw mode setup/teardown (so arrow keys work)
//! - Render throttling (configurable FPS)
//! - Ctrl+C handling (optional, enabled by default)
//! - Clean exit (restores terminal state)
//!
//! For async apps with background tasks, see `async_runtime.rs` instead.

use crate::element::Element;
use crate::renderer::Blaeck;
use crate::input::{poll_key, Key};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Write};
use std::time::Duration;

/// Configuration for the App runtime.
#[derive(Clone)]
pub struct AppConfig {
    /// How often to poll for input (default: 50ms)
    pub poll_interval: Duration,
    /// Whether to exit on Ctrl+C (default: true)
    pub exit_on_ctrl_c: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_millis(50),
            exit_on_ctrl_c: true,
        }
    }
}

/// Result of running the app.
pub struct AppResult {
    /// How the app exited
    pub exit_reason: ExitReason,
}

/// Why the app exited.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExitReason {
    /// User requested exit (e.g., Ctrl+C or explicit exit call)
    UserExit,
    /// App completed normally
    Completed,
}

/// Main application runtime.
pub struct App<W: Write> {
    blaeck: Blaeck<W>,
    config: AppConfig,
    should_exit: bool,
    exit_reason: ExitReason,
}

impl App<io::Stdout> {
    /// Create a new App with stdout.
    pub fn new() -> io::Result<Self> {
        Self::with_config(AppConfig::default())
    }

    /// Create a new App with custom config.
    pub fn with_config(config: AppConfig) -> io::Result<Self> {
        let stdout = io::stdout();
        let blaeck = Blaeck::new(stdout)?;
        Ok(Self {
            blaeck,
            config,
            should_exit: false,
            exit_reason: ExitReason::Completed,
        })
    }
}

impl<W: Write> App<W> {
    /// Create an App with a custom writer.
    pub fn with_writer(writer: W, config: AppConfig) -> io::Result<Self> {
        let blaeck = Blaeck::new(writer)?;
        Ok(Self {
            blaeck,
            config,
            should_exit: false,
            exit_reason: ExitReason::Completed,
        })
    }

    /// Request the app to exit.
    pub fn exit(&mut self) {
        self.should_exit = true;
        self.exit_reason = ExitReason::UserExit;
    }

    /// Check if the app should exit.
    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    /// Get a reference to the underlying Blaeck renderer.
    pub fn blaeck(&self) -> &Blaeck<W> {
        &self.blaeck
    }

    /// Get a mutable reference to the underlying Blaeck renderer.
    pub fn blaeck_mut(&mut self) -> &mut Blaeck<W> {
        &mut self.blaeck
    }

    /// Run the app with a render function and input handler.
    ///
    /// The render function is called to get the UI element tree.
    /// The input handler is called for each key press.
    pub fn run<R, I>(mut self, mut render: R, mut on_input: I) -> io::Result<AppResult>
    where
        R: FnMut(&mut Self) -> Element,
        I: FnMut(&mut Self, Key),
    {
        // Enable raw mode for input handling
        enable_raw_mode()?;

        // Initial render
        let ui = render(&mut self);
        self.blaeck.render(ui)?;

        // Main event loop
        while !self.should_exit {
            // Poll for input
            if let Some(key) = poll_key(self.config.poll_interval)? {
                // Handle Ctrl+C
                if self.config.exit_on_ctrl_c && key.is_ctrl_c() {
                    self.should_exit = true;
                    self.exit_reason = ExitReason::UserExit;
                    break;
                }

                // Call user's input handler
                on_input(&mut self, key);

                // Re-render after input
                let ui = render(&mut self);
                self.blaeck.render(ui)?;
            }
        }

        // Cleanup
        disable_raw_mode()?;
        self.blaeck.unmount()?;

        Ok(AppResult {
            exit_reason: self.exit_reason,
        })
    }

    /// Run with just a render function (no input handling).
    /// Exits on Ctrl+C.
    pub fn run_simple<R>(self, render: R) -> io::Result<AppResult>
    where
        R: FnMut(&mut Self) -> Element,
    {
        self.run(render, |_, _| {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.poll_interval, Duration::from_millis(50));
        assert!(config.exit_on_ctrl_c);
    }

    #[test]
    fn test_app_config_custom() {
        let config = AppConfig {
            poll_interval: Duration::from_millis(100),
            exit_on_ctrl_c: false,
        };
        assert_eq!(config.poll_interval, Duration::from_millis(100));
        assert!(!config.exit_on_ctrl_c);
    }

    #[test]
    fn test_exit_reason_eq() {
        assert_eq!(ExitReason::UserExit, ExitReason::UserExit);
        assert_ne!(ExitReason::UserExit, ExitReason::Completed);
    }

    #[test]
    fn test_exit_reason_clone() {
        let reason = ExitReason::Completed;
        let cloned = reason.clone();
        assert_eq!(reason, cloned);
    }

    #[test]
    fn test_app_with_writer() {
        let buf = Vec::new();
        let config = AppConfig::default();
        let app = App::with_writer(buf, config);
        assert!(app.is_ok());
    }

    #[test]
    fn test_app_should_exit_default() {
        let buf = Vec::new();
        let config = AppConfig::default();
        let app = App::with_writer(buf, config).unwrap();
        assert!(!app.should_exit());
    }

    #[test]
    fn test_app_exit() {
        let buf = Vec::new();
        let config = AppConfig::default();
        let mut app = App::with_writer(buf, config).unwrap();
        assert!(!app.should_exit());
        app.exit();
        assert!(app.should_exit());
    }

    #[test]
    fn test_app_blaeck_access() {
        let buf = Vec::new();
        let config = AppConfig::default();
        let app = App::with_writer(buf, config).unwrap();
        // Verify we can access blaeck
        let _width = app.blaeck().width();
    }

    #[test]
    fn test_app_blaeck_mut_access() {
        let buf = Vec::new();
        let config = AppConfig::default();
        let mut app = App::with_writer(buf, config).unwrap();
        // Verify we can get mutable access to blaeck
        let _blaeck = app.blaeck_mut();
    }

    #[test]
    fn test_app_result_exit_reason() {
        let result = AppResult {
            exit_reason: ExitReason::UserExit,
        };
        assert_eq!(result.exit_reason, ExitReason::UserExit);
    }

    #[test]
    fn test_app_config_clone() {
        let config = AppConfig::default();
        let cloned = config.clone();
        assert_eq!(config.poll_interval, cloned.poll_interval);
        assert_eq!(config.exit_on_ctrl_c, cloned.exit_on_ctrl_c);
    }
}
