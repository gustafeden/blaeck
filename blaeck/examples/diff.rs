//! Diff example - Git-style diff display
//!
//! Run with: cargo run --example diff

#[path = "previews/mod.rs"]
mod previews;

use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.render(previews::diff::build_ui())?;
    blaeck.unmount()?;
    Ok(())
}
