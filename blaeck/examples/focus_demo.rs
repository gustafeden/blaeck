//! Focus Demo - Demonstrates FocusManager with focus hooks
//!
//! Run with: cargo run --example focus_demo

#[path = "previews/mod.rs"]
mod previews;

use blaeck::input::poll_key;
use blaeck::Blaeck;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    let mut state = previews::focus_demo::AppState::new();

    enable_raw_mode()?;

    // Initial render
    blaeck.render(previews::focus_demo::render(&state))?;

    // Event loop
    loop {
        if let Some(key) = poll_key(Duration::from_millis(50))? {
            if key.is_ctrl_c() || state.handle_key(&key) {
                break;
            }

            // Update last event display
            if let Some(id) = state.focus.focused() {
                state.last_event = format!("Focused: {} (ID: {})", state.buttons[id.0], id.0);
            } else {
                state.last_event = "Blurred all".to_string();
            }

            blaeck.render(previews::focus_demo::render(&state))?;
        }
    }

    disable_raw_mode()?;
    blaeck.unmount()?;

    println!("Goodbye!");
    Ok(())
}
