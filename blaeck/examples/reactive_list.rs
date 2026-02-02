//! Reactive list example demonstrating multiple signals and selection.
//!
//! Run with: cargo run --example reactive_list
//!
//! Controls:
//! - Up/Down: Navigate list
//! - Enter/Space: Toggle selection
//! - 'a': Add new item
//! - 'd': Delete selected item
//! - 'q' or Ctrl+C: Exit

#[path = "previews/mod.rs"]
mod previews;

use blaeck::reactive::*;
use std::io;

fn main() -> io::Result<()> {
    ReactiveApp::run(previews::reactive_list::list_app)?;
    Ok(())
}
