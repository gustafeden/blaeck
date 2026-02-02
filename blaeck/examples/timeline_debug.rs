//! Timeline Debug Visualization Demo
//!
//! Demonstrates the TimelineDebugInfo feature for visualizing
//! timeline state during development.
//!
//! Controls:
//! - Space: Pause/resume
//! - r: Restart
//! - Left/Right: Seek backward/forward
//! - q/Esc: Quit

#[path = "previews/mod.rs"]
mod previews;

use blaeck::reactive::*;

fn main() -> std::io::Result<()> {
    ReactiveApp::run(previews::timeline_debug::debug_dashboard)?;
    Ok(())
}
