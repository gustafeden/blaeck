//! Borders - Showcase of enhanced border styling
//!
//! Run with: cargo run --example borders

#[path = "previews/mod.rs"]
mod previews;

use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.render(previews::borders::build_ui())?;
    blaeck.unmount()?;
    println!();
    Ok(())
}
