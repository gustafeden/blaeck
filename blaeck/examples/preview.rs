//! Preview example - showcases Blaeck's layout and component capabilities.
//!
//! Run with: cargo run --example preview

#[path = "previews/mod.rs"]
mod previews;

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.set_max_fps(15);

    let timer = AnimationTimer::new();

    // Run for 6 seconds
    while timer.elapsed_ms() < 6000 {
        blaeck.render(previews::preview::build_ui_with_timer(&timer))?;
        std::thread::sleep(Duration::from_millis(50));
    }

    blaeck.unmount()?;
    Ok(())
}
