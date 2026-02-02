//! Gradient example - Colorful text gradients
//!
//! Run with: cargo run --example gradient

#[path = "previews/mod.rs"]
mod previews;

use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.render(previews::gradient::build_ui())?;
    blaeck.unmount()?;
    Ok(())
}
