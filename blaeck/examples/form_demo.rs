//! Form Demo - Interactive form with TextInput and Checkbox
//!
//! Run with: cargo run --example form_demo

#[path = "previews/mod.rs"]
mod previews;

use blaeck::input::poll_key;
use blaeck::Blaeck;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    let mut state = previews::form_demo::FormState::new();

    enable_raw_mode()?;

    // Initial render
    blaeck.render(previews::form_demo::render(&state))?;

    // Event loop
    loop {
        if let Some(key) = poll_key(Duration::from_millis(50))? {
            if key.is_ctrl_c() || state.handle_key(&key) {
                break;
            }
            blaeck.render(previews::form_demo::render(&state))?;
        }
    }

    disable_raw_mode()?;
    blaeck.unmount()?;

    println!("Goodbye!");
    Ok(())
}
