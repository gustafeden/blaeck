//! Async application example - the proper pattern for apps with async backends.
//!
//! This demonstrates how to integrate Blaeck with async code (tokio) by:
//! 1. Using `Ink` directly for rendering (not `App::run`)
//! 2. Writing your own async event loop with `tokio::select!`
//! 3. Polling both keyboard input AND async channels in the same loop
//!
//! This is the recommended pattern for:
//! - Apps that stream data from async sources (APIs, subprocesses)
//! - Apps that need background tasks
//! - Apps with animated UI that updates independently of user input
//!
//! Run with: cargo run --example async_app

use blaeck::prelude::*;
use blaeck::{poll_key, Blaeck, Key};
use std::io::stdout;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

/// Events from the background "worker" task
#[derive(Debug)]
enum BackgroundEvent {
    /// New data chunk received
    DataChunk(String),
    /// Progress update
    Progress(u32),
    /// Work complete
    Done,
}

/// Application state
struct AppState {
    /// Collected text from background task
    text: String,
    /// Current progress (0-100)
    progress: u32,
    /// Whether background work is running
    is_loading: bool,
    /// User input buffer
    input: String,
    /// Status message
    status: String,
    /// Spinner start time (for animation)
    spinner_start: Instant,
    /// Should exit
    should_exit: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            text: String::new(),
            progress: 0,
            is_loading: false,
            input: String::new(),
            status: "Type something and press Enter".to_string(),
            spinner_start: Instant::now(),
            should_exit: false,
        }
    }
}

/// Spinner frames for animation
const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

fn get_spinner_frame(start: Instant) -> &'static str {
    let elapsed_ms = start.elapsed().as_millis() as usize;
    let frame_idx = (elapsed_ms / 80) % SPINNER_FRAMES.len();
    SPINNER_FRAMES[frame_idx]
}

/// Render the UI based on current state
fn render(state: &AppState) -> Element {
    let mut children: Vec<Element> = vec![
        element! {
            Text(content: "Async App Demo", bold: true, color: Color::Cyan)
        },
        element! {
            Text(content: state.status.clone(), dim: true)
        },
        Element::text(""),
    ];

    // Show collected text if any
    if !state.text.is_empty() {
        children.push(element! {
            Text(content: state.text.clone(), color: Color::Green)
        });
        children.push(Element::text(""));
    }

    // Show loading state with spinner
    if state.is_loading {
        let spinner = get_spinner_frame(state.spinner_start);
        let progress_bar = "█".repeat((state.progress / 5) as usize);
        let empty_bar = "░".repeat((20 - state.progress / 5) as usize);

        children.push(element! {
            Box(flex_direction: FlexDirection::Row) {
                Text(content: format!("{} ", spinner), color: Color::Yellow)
                Text(content: format!("Working... {}% ", state.progress))
                Text(content: format!("[{}{}]", progress_bar, empty_bar), dim: true)
            }
        });
    } else {
        // Show input prompt
        children.push(element! {
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "> ", color: Color::Blue)
                Text(content: state.input.clone())
                Text(content: "_", dim: true)
            }
        });
    }

    children.push(Element::text(""));
    children.push(element! {
        Text(content: "Press Ctrl+C to exit", dim: true)
    });

    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        children,
    )
}

/// Handle keyboard input
fn handle_input(state: &mut AppState, key: Key) -> Option<mpsc::Sender<String>> {
    if key.is_ctrl_c() {
        state.should_exit = true;
        return None;
    }

    if state.is_loading {
        // Ignore input while loading (could add cancel support here)
        return None;
    }

    if key.is_enter() && !state.input.is_empty() {
        // Return signal to start background work
        let input = std::mem::take(&mut state.input);
        state.is_loading = true;
        state.progress = 0;
        state.text.clear();
        state.spinner_start = Instant::now();
        state.status = format!("Processing: {}", input);

        // We'll return a sender for the spawned task to signal
        // Actually, we need to communicate what to process - let's return the input
        // For this example, we'll handle it differently
        return None; // Handled below
    }

    if key.is_backspace() {
        state.input.pop();
    } else if let Some(c) = key.as_char() {
        state.input.push(c);
    }

    None
}

/// Handle events from background task
fn handle_background_event(state: &mut AppState, event: BackgroundEvent) {
    match event {
        BackgroundEvent::DataChunk(chunk) => {
            state.text.push_str(&chunk);
        }
        BackgroundEvent::Progress(p) => {
            state.progress = p;
        }
        BackgroundEvent::Done => {
            state.is_loading = false;
            state.status = "Done! Type something else".to_string();
        }
    }
}

/// Simulate a background task that streams data
async fn background_worker(input: String, tx: mpsc::Sender<BackgroundEvent>) {
    // Simulate processing with progress updates
    let words: Vec<&str> = input.split_whitespace().collect();
    let total = words.len().max(1);

    for (i, word) in words.iter().enumerate() {
        // Simulate work
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Send data chunk
        let chunk = format!("{} ", word.to_uppercase());
        let _ = tx.send(BackgroundEvent::DataChunk(chunk)).await;

        // Send progress
        let progress = ((i + 1) * 100 / total) as u32;
        let _ = tx.send(BackgroundEvent::Progress(progress)).await;
    }

    // Final delay then done
    tokio::time::sleep(Duration::from_millis(300)).await;
    let _ = tx.send(BackgroundEvent::Done).await;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut blaeck = Blaeck::new(stdout())?;
    let mut state = AppState::new();

    // Channel for background events
    let (bg_tx, mut bg_rx) = mpsc::channel::<BackgroundEvent>(100);

    // For tracking pending input to process
    let mut pending_input: Option<String> = None;

    // Enable raw mode for keyboard input
    crossterm::terminal::enable_raw_mode()?;

    loop {
        // Render current state
        blaeck.render(render(&state))?;

        if state.should_exit {
            break;
        }

        // Check if we need to spawn background work
        if let Some(input) = pending_input.take() {
            let tx = bg_tx.clone();
            tokio::spawn(background_worker(input, tx));
        }

        // The key pattern: use tokio::select! to poll multiple sources
        tokio::select! {
            // Timeout for animation updates (spinner)
            _ = tokio::time::sleep(Duration::from_millis(50)) => {
                // Check for keyboard input (non-blocking)
                if let Some(key) = poll_key(Duration::ZERO)? {
                    handle_input(&mut state, key);

                    // Check if we should start work
                    if state.is_loading && state.text.is_empty() && state.progress == 0 {
                        // Extract the input from status message (hacky but works for demo)
                        if let Some(input) = state.status.strip_prefix("Processing: ") {
                            pending_input = Some(input.to_string());
                        }
                    }
                }
            }

            // Background events from worker
            Some(event) = bg_rx.recv() => {
                handle_background_event(&mut state, event);
            }
        }
    }

    // Clean up
    blaeck.unmount()?;
    crossterm::terminal::disable_raw_mode()?;
    println!("\nGoodbye!");

    Ok(())
}
