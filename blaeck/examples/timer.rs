//! Timer example - Stopwatch and countdown display
//!
//! Run with: cargo run --example timer

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

    // Run for 12 seconds to show countdown completing
    while timer.elapsed_ms() < 12000 {
        blaeck.render(previews::timer::build_ui_with_timer(&timer))?;
        thread::sleep(Duration::from_millis(100));
    }

    blaeck.render(previews::timer::build_final_ui())?;
    blaeck.unmount()?;

    Ok(())
}
