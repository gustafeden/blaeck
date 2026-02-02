//! Stagger Animation Demo
//!
//! Demonstrates staggered animations where multiple items animate
//! with a delay between each one, creating a wave or cascade effect.
//!
//! Controls:
//! - 1-5: Switch stagger order (Forward, Reverse, CenterOut, EdgesIn, Random)
//! - Space: Pause/resume
//! - r: Restart
//! - q/Esc: Quit

#[path = "previews/mod.rs"]
mod previews;

use blaeck::reactive::*;

fn main() -> std::io::Result<()> {
    ReactiveApp::run(previews::stagger_demo::component)?;
    Ok(())
}
