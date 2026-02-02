//! KeyHints example - Keyboard shortcut displays
//!
//! Run with: cargo run --example keyhints

#[path = "previews/mod.rs"]
mod previews;

use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.render(previews::keyhints::build_ui())?;
    blaeck.unmount()?;
    Ok(())
}
