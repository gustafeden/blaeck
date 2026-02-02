//! Interactive example demonstrating input handling and focus management.
//!
//! Run with: cargo run --example interactive
//!
//! Demonstrates:
//! - App runtime with event loop
//! - Keyboard input handling via match_key
//! - Focus management with FocusManager
//! - Tab/Shift+Tab and arrow key navigation
//! - Enter to select, Ctrl+C to exit

#[path = "previews/mod.rs"]
mod previews;

use blaeck::App;
use std::cell::RefCell;

fn main() -> std::io::Result<()> {
    // Use RefCell for interior mutability - allows both closures to access state
    let state = RefCell::new(previews::interactive::AppState::new());

    let app = App::new()?;

    app.run(
        |_app| state.borrow().render(),
        |_app, key| {
            state.borrow_mut().handle_input(&key);
        },
    )?;

    println!("Goodbye!");
    Ok(())
}
