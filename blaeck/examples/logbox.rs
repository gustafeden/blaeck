//! LogBox example - Auto-expanding scrolling log viewer
//!
//! Run with: cargo run --example logbox

#[path = "previews/mod.rs"]
mod previews;

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    let timer = AnimationTimer::new();

    // Streaming phase: show lines appearing one by one
    for _ in 0..40 {
        blaeck.render(previews::logbox::build_ui_with_timer(&timer))?;
        thread::sleep(Duration::from_millis(100));
    }

    // Hold on final multi-variant view
    blaeck.render(previews::logbox::build_final_ui())?;
    blaeck.unmount()?;
    Ok(())
}
