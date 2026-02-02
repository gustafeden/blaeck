//! ReactiveApp - the entry point for reactive applications.
//!
//! ReactiveApp manages the render loop, input handling, and runtime lifecycle
//! for reactive components.

use super::runtime::RuntimeHandle;
use super::scope::Scope;
use crate::element::Element;
use crate::input::poll_key;
use crate::renderer::Blaeck;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Write};
use std::time::Duration;

/// Configuration for ReactiveApp.
#[derive(Clone)]
pub struct ReactiveAppConfig {
    /// How often to poll for input (default: 50ms).
    pub poll_interval: Duration,

    /// Whether to exit on Ctrl+C (default: true).
    pub exit_on_ctrl_c: bool,
}

impl Default for ReactiveAppConfig {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_millis(50),
            exit_on_ctrl_c: true,
        }
    }
}

/// Result returned when the reactive app exits.
pub struct ReactiveAppResult {
    /// How the app exited.
    pub exit_reason: ReactiveExitReason,
}

/// Why the reactive app exited.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReactiveExitReason {
    /// User requested exit (e.g., Ctrl+C).
    UserExit,

    /// App completed normally.
    Completed,
}

/// A reactive application runtime.
///
/// ReactiveApp provides the main entry point for reactive/signals-based UIs.
/// It manages:
/// - The reactive runtime (signals, component instances)
/// - The render loop with automatic re-rendering on state changes
/// - Input handling and dispatch to registered handlers
///
/// # Example
///
/// ```ignore
/// use blaeck::reactive::*;
/// use blaeck::prelude::*;
///
/// fn counter(cx: Scope) -> Element {
///     let count = use_state(cx, || 0);
///
///     use_input(cx, move |key| {
///         if key.is_char(' ') {
///             count.set(count.get() + 1);
///         }
///         if key.is_char('q') || key.is_ctrl_c() {
///             // Will exit via Ctrl+C handling
///         }
///     });
///
///     element! {
///         Box(border_style: BorderStyle::Round, padding: 1.0) {
///             Text(content: format!("Count: {}", count.get()))
///         }
///     }
/// }
///
/// fn main() -> std::io::Result<()> {
///     ReactiveApp::run(counter)
/// }
/// ```
pub struct ReactiveApp<W: Write> {
    /// The reactive runtime.
    runtime: RuntimeHandle,

    /// The underlying renderer.
    blaeck: Blaeck<W>,

    /// Configuration.
    config: ReactiveAppConfig,

    /// Whether the app should exit.
    should_exit: bool,

    /// Why the app is exiting.
    exit_reason: ReactiveExitReason,
}

impl ReactiveApp<io::Stdout> {
    /// Run a reactive component with default configuration.
    ///
    /// This is the main entry point for reactive applications.
    ///
    /// # Example
    ///
    /// ```ignore
    /// ReactiveApp::run(my_component)?;
    /// ```
    pub fn run<F>(component: F) -> io::Result<ReactiveAppResult>
    where
        F: Fn(Scope) -> Element,
    {
        Self::run_with_config(component, ReactiveAppConfig::default())
    }

    /// Run a reactive component with custom configuration.
    pub fn run_with_config<F>(
        component: F,
        config: ReactiveAppConfig,
    ) -> io::Result<ReactiveAppResult>
    where
        F: Fn(Scope) -> Element,
    {
        let app = Self::new(config)?;
        app.run_component(component)
    }

    /// Create a new ReactiveApp with stdout.
    pub fn new(config: ReactiveAppConfig) -> io::Result<Self> {
        Self::with_writer(io::stdout(), config)
    }
}

impl<W: Write> ReactiveApp<W> {
    /// Create a ReactiveApp with a custom writer.
    ///
    /// Useful for testing or writing to a buffer.
    pub fn with_writer(writer: W, config: ReactiveAppConfig) -> io::Result<Self> {
        let runtime = RuntimeHandle::new();
        let blaeck = Blaeck::new(writer)?;

        Ok(Self {
            runtime,
            blaeck,
            config,
            should_exit: false,
            exit_reason: ReactiveExitReason::Completed,
        })
    }

    /// Request the app to exit.
    pub fn exit(&mut self) {
        self.should_exit = true;
        self.exit_reason = ReactiveExitReason::UserExit;
    }

    /// Get a reference to the runtime.
    pub fn runtime(&self) -> &RuntimeHandle {
        &self.runtime
    }

    /// Get a reference to the underlying renderer.
    pub fn blaeck(&self) -> &Blaeck<W> {
        &self.blaeck
    }

    /// Get a mutable reference to the underlying renderer.
    pub fn blaeck_mut(&mut self) -> &mut Blaeck<W> {
        &mut self.blaeck
    }

    /// Run the component render loop.
    fn run_component<F>(mut self, component: F) -> io::Result<ReactiveAppResult>
    where
        F: Fn(Scope) -> Element,
    {
        // Create root component instance
        let root_id = self.runtime.create_instance();

        // Enable raw mode for keyboard input
        enable_raw_mode()?;

        // Initial render
        let scope = Scope::new(self.runtime.clone(), root_id);
        self.runtime.set_current_instance(Some(root_id));
        self.runtime.reset_hook_cursor(root_id);
        let element = component(scope);
        self.runtime.set_current_instance(None);
        self.blaeck.render(element)?;
        self.runtime.clear_dirty();

        // Main event loop
        while !self.should_exit {
            // Poll for input
            if let Some(key) = poll_key(self.config.poll_interval)? {
                // Handle Ctrl+C
                if self.config.exit_on_ctrl_c && key.is_ctrl_c() {
                    self.should_exit = true;
                    self.exit_reason = ReactiveExitReason::UserExit;
                    break;
                }

                // Dispatch to registered input handlers
                self.runtime.dispatch_input(&key);
            }

            // Re-render if state changed
            if self.runtime.needs_render() {
                let scope = Scope::new(self.runtime.clone(), root_id);
                self.runtime.set_current_instance(Some(root_id));
                self.runtime.reset_hook_cursor(root_id);
                let element = component(scope);
                self.runtime.set_current_instance(None);
                self.blaeck.render(element)?;
                self.runtime.clear_dirty();
            }
        }

        // Cleanup
        disable_raw_mode()?;
        self.blaeck.unmount()?;

        Ok(ReactiveAppResult {
            exit_reason: self.exit_reason,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = ReactiveAppConfig::default();
        assert_eq!(config.poll_interval, Duration::from_millis(50));
        assert!(config.exit_on_ctrl_c);
    }

    #[test]
    fn test_config_custom() {
        let config = ReactiveAppConfig {
            poll_interval: Duration::from_millis(100),
            exit_on_ctrl_c: false,
        };
        assert_eq!(config.poll_interval, Duration::from_millis(100));
        assert!(!config.exit_on_ctrl_c);
    }

    #[test]
    fn test_exit_reason_eq() {
        assert_eq!(ReactiveExitReason::UserExit, ReactiveExitReason::UserExit);
        assert_ne!(ReactiveExitReason::UserExit, ReactiveExitReason::Completed);
    }

    #[test]
    fn test_exit_reason_clone() {
        let reason = ReactiveExitReason::Completed;
        let cloned = reason.clone();
        assert_eq!(reason, cloned);
    }

    #[test]
    fn test_with_writer() {
        let buf = Vec::new();
        let config = ReactiveAppConfig::default();
        let app = ReactiveApp::with_writer(buf, config);
        assert!(app.is_ok());
    }

    #[test]
    fn test_runtime_access() {
        let buf = Vec::new();
        let config = ReactiveAppConfig::default();
        let app = ReactiveApp::with_writer(buf, config).unwrap();

        // Verify we can access the runtime
        let _rt = app.runtime();
    }

    #[test]
    fn test_blaeck_access() {
        let buf = Vec::new();
        let config = ReactiveAppConfig::default();
        let app = ReactiveApp::with_writer(buf, config).unwrap();

        // Verify we can access blaeck
        let _width = app.blaeck().width();
    }

    #[test]
    fn test_config_clone() {
        let config = ReactiveAppConfig::default();
        let cloned = config.clone();
        assert_eq!(config.poll_interval, cloned.poll_interval);
        assert_eq!(config.exit_on_ctrl_c, cloned.exit_on_ctrl_c);
    }

    #[test]
    fn test_two_runtimes_coexist() {
        // This test verifies that two ReactiveApp instances can coexist,
        // which is important for parallel tests. We don't use thread-local
        // storage, so each runtime is independent.
        let buf1 = Vec::new();
        let buf2 = Vec::new();
        let config = ReactiveAppConfig::default();

        let app1 = ReactiveApp::with_writer(buf1, config.clone()).unwrap();
        let app2 = ReactiveApp::with_writer(buf2, config).unwrap();

        // Each app has its own runtime
        let rt1 = app1.runtime();
        let rt2 = app2.runtime();

        // Create signals in each runtime
        let signal1 = rt1.create_signal(100i32);
        let signal2 = rt2.create_signal(200i32);

        // Verify they are independent
        assert_eq!(rt1.get_signal::<i32>(signal1), 100);
        assert_eq!(rt2.get_signal::<i32>(signal2), 200);

        // Modify one doesn't affect the other
        rt1.set_signal(signal1, 999);
        assert_eq!(rt1.get_signal::<i32>(signal1), 999);
        assert_eq!(rt2.get_signal::<i32>(signal2), 200); // Unchanged
    }
}
