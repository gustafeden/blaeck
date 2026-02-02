//! Reactive counter example demonstrating use_state and use_input.
//!
//! Run with: cargo run --example reactive_counter
//!
//! Controls:
//! - Space: Increment counter
//! - 'r': Reset counter
//! - 'q' or Ctrl+C: Exit

#[path = "previews/mod.rs"]
mod previews;

use blaeck::reactive::*;
use std::io;

fn main() -> io::Result<()> {
    ReactiveApp::run(previews::reactive_counter::counter)?;
    Ok(())
}
