//! Reactive Timeline Example
//!
//! Demonstrates the `use_timeline` hook for declarative animations
//! in the reactive system.
//!
//! Controls:
//! - Space: Pause/resume
//! - r: Restart
//! - Left/Right: Seek backward/forward
//! - Up/Down: Speed up/slow down
//! - q/Esc: Quit

#[path = "previews/mod.rs"]
mod previews;

use blaeck::reactive::*;

fn main() -> std::io::Result<()> {
    ReactiveApp::run(previews::reactive_timeline::animated_dashboard)?;
    Ok(())
}
