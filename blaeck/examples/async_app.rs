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

#[path = "previews/mod.rs"]
mod previews;

use blaeck::{poll_key, Blaeck, Key};
use previews::async_app::{AppState, BackgroundEvent};
use std::io::stdout;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

/// Handle keyboard input
fn handle_input(state: &mut AppState, key: Key) {
    if key.is_ctrl_c() {
        state.should_exit = true;
        return;
    }

    if state.is_loading {
        return;
    }

    if key.is_enter() && !state.input.is_empty() {
        let _input = std::mem::take(&mut state.input);
        state.is_loading = true;
        state.progress = 0;
        state.text.clear();
        state.spinner_start = Instant::now();
        state.status = format!("Processing: {}", _input);
        return;
    }

    if key.is_backspace() {
        state.input.pop();
    } else if let Some(c) = key.as_char() {
        state.input.push(c);
    }
}

/// Simulate a background task that streams data
async fn background_worker(input: String, tx: mpsc::Sender<BackgroundEvent>) {
    let words: Vec<&str> = input.split_whitespace().collect();
    let total = words.len().max(1);

    for (i, word) in words.iter().enumerate() {
        tokio::time::sleep(Duration::from_millis(200)).await;

        let chunk = format!("{} ", word.to_uppercase());
        let _ = tx.send(BackgroundEvent::DataChunk(chunk)).await;

        let progress = ((i + 1) * 100 / total) as u32;
        let _ = tx.send(BackgroundEvent::Progress(progress)).await;
    }

    tokio::time::sleep(Duration::from_millis(300)).await;
    let _ = tx.send(BackgroundEvent::Done).await;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut blaeck = Blaeck::new(stdout())?;
    let mut state = AppState::new();

    let (bg_tx, mut bg_rx) = mpsc::channel::<BackgroundEvent>(100);

    let mut pending_input: Option<String> = None;

    crossterm::terminal::enable_raw_mode()?;

    loop {
        blaeck.render(previews::async_app::render(&state))?;

        if state.should_exit {
            break;
        }

        if let Some(input) = pending_input.take() {
            let tx = bg_tx.clone();
            tokio::spawn(background_worker(input, tx));
        }

        tokio::select! {
            _ = tokio::time::sleep(Duration::from_millis(50)) => {
                if let Some(key) = poll_key(Duration::ZERO)? {
                    handle_input(&mut state, key);

                    if state.is_loading && state.text.is_empty() && state.progress == 0 {
                        if let Some(input) = state.status.strip_prefix("Processing: ") {
                            pending_input = Some(input.to_string());
                        }
                    }
                }
            }

            Some(event) = bg_rx.recv() => {
                previews::async_app::handle_background_event(&mut state, event);
            }
        }
    }

    blaeck.unmount()?;
    crossterm::terminal::disable_raw_mode()?;
    println!("\nGoodbye!");

    Ok(())
}
