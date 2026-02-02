//! Task Runner - Comprehensive Blaeck showcase
//!
//! Demonstrates:
//! - Live updating UI
//! - Nested boxes with borders (Single, Round, Double)
//! - All colors (Green, Yellow, Red, Cyan, White)
//! - Text modifiers (bold, dim, italic)
//! - Flexbox (Row for alignment, Column for stacking)
//! - Spacer for pushing content
//! - Spinner animation
//! - Progress bar

#[path = "previews/mod.rs"]
mod previews;

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::{thread, time::Duration};

fn main() -> std::io::Result<()> {
    let mut blaeck = Blaeck::new(std::io::stdout())?;
    let timer = AnimationTimer::new();

    // Animate through all tasks (~10.6s), then show final state
    while timer.elapsed_ms() < 12000 {
        blaeck.render(previews::task_runner::build_ui_with_timer(&timer))?;
        thread::sleep(Duration::from_millis(100));
    }

    blaeck.render(previews::task_runner::build_final_ui())?;
    blaeck.unmount()?;

    println!("\n📦 Build artifacts written to ./target/release/");
    Ok(())
}
