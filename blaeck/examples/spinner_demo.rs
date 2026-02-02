//! Spinner Demo - Various animated spinner styles
//!
//! Run with: cargo run --example spinner_demo

#[path = "previews/mod.rs"]
mod previews;

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::{io, thread, time::Duration};

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.set_max_fps(20);

    let timer = AnimationTimer::new();

    for _ in 0..100 {
        blaeck.render(previews::spinner_demo::build_ui_with_timer(&timer))?;
        thread::sleep(Duration::from_millis(50));
    }

    blaeck.unmount()?;
    println!("Done!");
    Ok(())
}
