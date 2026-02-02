//! Syntax Highlighting example - Code display with colors
//!
//! Run with: cargo run --example syntax

#[path = "previews/mod.rs"]
mod previews;

use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.render(previews::syntax::build_ui())?;
    blaeck.unmount()?;
    Ok(())
}
