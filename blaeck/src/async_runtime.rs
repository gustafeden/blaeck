//! Async runtime support for Blaeck.
//!
//! This module provides async/await-compatible versions of the event loop
//! and input handling, built on tokio.
//!
//! Enable with the `async` feature:
//! ```toml
//! blaeck = { version = "0.1", features = ["async"] }
//! ```
//!
//! # Example
//!
//! See the `async_demo` example for a complete working example.
//!
//! The basic pattern is:
//! 1. Create an `AsyncApp` or use manual `tokio::select!` loop
//! 2. Use `app.sender()` to get a channel for background tasks
//! 3. Spawn async tasks that send messages back via the channel
//! 4. Handle messages alongside keyboard events in the event loop

use crate::element::Element;
use crate::input::Key;
use crate::renderer::Blaeck;
use crossterm::event::{Event, EventStream};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use futures::StreamExt;
use std::io::{self, Write};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::{interval, Interval};

/// Result type for async operations.
pub type Result<T> = std::io::Result<T>;

/// Messages that can be sent to the async app.
#[derive(Debug)]
pub enum AppEvent<M> {
    /// A keyboard input event
    Key(Key),
    /// A user-defined message from a background task
    Message(M),
    /// A tick event for periodic updates
    Tick,
    /// Request to exit the app
    Exit,
}

/// Sender for sending messages to the app from background tasks.
pub type Sender<M> = mpsc::Sender<M>;

/// Receiver for messages from background tasks.
pub type Receiver<M> = mpsc::Receiver<M>;

/// Creates a new message channel.
///
/// The returned sender can be cloned and sent to background tasks.
/// The receiver should be passed to `AsyncApp::run_with_receiver`.
pub fn channel<M>(buffer: usize) -> (Sender<M>, Receiver<M>) {
    mpsc::channel(buffer)
}

/// Configuration for the async app.
#[derive(Clone)]
pub struct AsyncAppConfig {
    /// Interval for tick events (None to disable)
    pub tick_interval: Option<Duration>,
    /// Whether to exit on Ctrl+C
    pub exit_on_ctrl_c: bool,
    /// Buffer size for the message channel
    pub message_buffer: usize,
}

impl Default for AsyncAppConfig {
    fn default() -> Self {
        Self {
            tick_interval: Some(Duration::from_millis(100)),
            exit_on_ctrl_c: true,
            message_buffer: 32,
        }
    }
}

/// Async application runtime.
///
/// Provides an event loop that can receive:
/// - Keyboard input
/// - Messages from background async tasks
/// - Periodic tick events
pub struct AsyncApp<W: Write, M: Send + 'static = ()> {
    blaeck: Blaeck<W>,
    config: AsyncAppConfig,
    tx: Sender<M>,
    rx: Receiver<M>,
    should_exit: bool,
}

impl<M: Send + 'static> AsyncApp<io::Stdout, M> {
    /// Create a new async app with stdout.
    pub fn new() -> Result<Self> {
        Self::with_config(AsyncAppConfig::default())
    }

    /// Create a new async app with custom config.
    pub fn with_config(config: AsyncAppConfig) -> Result<Self> {
        let stdout = io::stdout();
        let blaeck = Blaeck::new(stdout)?;
        let (tx, rx) = mpsc::channel(config.message_buffer);
        Ok(Self {
            blaeck,
            config,
            tx,
            rx,
            should_exit: false,
        })
    }
}

impl<W: Write, M: Send + 'static> AsyncApp<W, M> {
    /// Create an async app with a custom writer.
    pub fn with_writer(writer: W, config: AsyncAppConfig) -> Result<Self> {
        let blaeck = Blaeck::new(writer)?;
        let (tx, rx) = mpsc::channel(config.message_buffer);
        Ok(Self {
            blaeck,
            config,
            tx,
            rx,
            should_exit: false,
        })
    }

    /// Get a sender for sending messages from background tasks.
    ///
    /// The sender can be cloned and moved into async tasks.
    pub fn sender(&self) -> Sender<M> {
        self.tx.clone()
    }

    /// Request the app to exit.
    pub fn exit(&mut self) {
        self.should_exit = true;
    }

    /// Check if the app should exit.
    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    /// Get the underlying Blaeck renderer.
    pub fn blaeck(&self) -> &Blaeck<W> {
        &self.blaeck
    }

    /// Get a mutable reference to the Blaeck renderer.
    pub fn blaeck_mut(&mut self) -> &mut Blaeck<W> {
        &mut self.blaeck
    }

    /// Run the async event loop.
    ///
    /// - `render`: Function that returns the UI element tree
    /// - `handle`: Function that handles events (keys, messages, ticks)
    pub async fn run<R, H>(mut self, mut render: R, mut handle: H) -> Result<()>
    where
        R: FnMut(&mut Self) -> Element,
        H: FnMut(&mut Self, AppEvent<M>),
    {
        enable_raw_mode()?;

        // Initial render
        let ui = render(&mut self);
        self.blaeck.render(ui)?;

        // Create event stream for keyboard input
        let mut event_stream = EventStream::new();

        // Create tick interval if configured
        let mut tick_interval: Option<Interval> = self.config.tick_interval.map(|d| interval(d));

        // Main event loop
        loop {
            if self.should_exit {
                break;
            }

            // Select between keyboard events, messages, and ticks
            let event = tokio::select! {
                // Keyboard events
                maybe_event = event_stream.next() => {
                    match maybe_event {
                        Some(Ok(Event::Key(key_event))) => {
                            let key = Key::from(key_event);
                            if self.config.exit_on_ctrl_c && key.is_ctrl_c() {
                                self.should_exit = true;
                                break;
                            }
                            Some(AppEvent::Key(key))
                        }
                        Some(Ok(_)) => None, // Ignore other events (mouse, resize, etc.)
                        Some(Err(_)) => None,
                        None => {
                            self.should_exit = true;
                            break;
                        }
                    }
                }
                // Messages from background tasks
                maybe_msg = self.rx.recv() => {
                    match maybe_msg {
                        Some(msg) => Some(AppEvent::Message(msg)),
                        None => None, // All senders dropped
                    }
                }
                // Tick events
                _ = async {
                    if let Some(ref mut interval) = tick_interval {
                        interval.tick().await
                    } else {
                        // If no tick interval, this future never completes
                        std::future::pending::<tokio::time::Instant>().await
                    }
                } => {
                    Some(AppEvent::Tick)
                }
            };

            // Handle the event if there is one
            if let Some(evt) = event {
                handle(&mut self, evt);

                // Re-render after handling event
                let ui = render(&mut self);
                self.blaeck.render(ui)?;
            }
        }

        // Cleanup
        disable_raw_mode()?;
        self.blaeck.unmount()?;

        Ok(())
    }

    /// Run with only keyboard input (no messages or ticks).
    pub async fn run_simple<R, H>(mut self, mut render: R, mut handle: H) -> Result<()>
    where
        R: FnMut(&mut Self) -> Element,
        H: FnMut(&mut Self, Key),
    {
        enable_raw_mode()?;

        let ui = render(&mut self);
        self.blaeck.render(ui)?;

        let mut event_stream = EventStream::new();

        loop {
            if self.should_exit {
                break;
            }

            match event_stream.next().await {
                Some(Ok(Event::Key(key_event))) => {
                    let key = Key::from(key_event);
                    if self.config.exit_on_ctrl_c && key.is_ctrl_c() {
                        self.should_exit = true;
                        break;
                    }
                    handle(&mut self, key);
                    let ui = render(&mut self);
                    self.blaeck.render(ui)?;
                }
                Some(Ok(_)) => {} // Ignore other events
                Some(Err(_)) => {}
                None => break,
            }
        }

        disable_raw_mode()?;
        self.blaeck.unmount()?;
        Ok(())
    }
}

/// Async key polling - reads a key with timeout.
pub async fn poll_key_async(timeout: Duration) -> Result<Option<Key>> {
    let mut event_stream = EventStream::new();

    tokio::select! {
        maybe_event = event_stream.next() => {
            match maybe_event {
                Some(Ok(Event::Key(key_event))) => Ok(Some(Key::from(key_event))),
                Some(Ok(_)) => Ok(None),
                Some(Err(e)) => Err(e),
                None => Ok(None),
            }
        }
        _ = tokio::time::sleep(timeout) => {
            Ok(None)
        }
    }
}

/// Async key reading - blocks until a key is pressed.
pub async fn read_key_async() -> Result<Key> {
    let mut event_stream = EventStream::new();

    loop {
        match event_stream.next().await {
            Some(Ok(Event::Key(key_event))) => return Ok(Key::from(key_event)),
            Some(Ok(_)) => continue,
            Some(Err(e)) => return Err(e),
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Event stream ended",
                ))
            }
        }
    }
}

/// Helper for running a simple async UI update loop.
///
/// This is useful when you just need to periodically re-render
/// based on some async data source.
pub async fn run_with_updates<S, R, U>(
    mut blaeck: Blaeck<impl Write>,
    mut state: S,
    mut render: R,
    mut update: U,
) -> Result<()>
where
    R: FnMut(&S) -> Element,
    U: FnMut(&mut S) -> std::future::Ready<bool>,
{
    enable_raw_mode()?;

    let mut event_stream = EventStream::new();
    let mut tick = interval(Duration::from_millis(50));

    loop {
        // Initial render
        let ui = render(&state);
        blaeck.render(ui)?;

        tokio::select! {
            // Check for exit key
            maybe_event = event_stream.next() => {
                if let Some(Ok(Event::Key(key_event))) = maybe_event {
                    let key = Key::from(key_event);
                    if key.is_ctrl_c() {
                        break;
                    }
                }
            }
            // Periodic update
            _ = tick.tick() => {
                if !update(&mut state).await {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    blaeck.unmount()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_app_config_default() {
        let config = AsyncAppConfig::default();
        assert!(config.tick_interval.is_some());
        assert!(config.exit_on_ctrl_c);
        assert_eq!(config.message_buffer, 32);
    }

    #[test]
    fn test_channel_creation() {
        let (tx, mut rx) = channel::<i32>(10);

        // Use tokio runtime for async test
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            tx.send(42).await.unwrap();
            let msg = rx.recv().await.unwrap();
            assert_eq!(msg, 42);
        });
    }

    #[test]
    fn test_app_event_debug() {
        let event: AppEvent<String> = AppEvent::Tick;
        let debug_str = format!("{:?}", event);
        assert!(debug_str.contains("Tick"));
    }
}
