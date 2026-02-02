//! Markdown example - Render CommonMark-formatted text
//!
//! Run with: cargo run --example markdown

#[path = "previews/mod.rs"]
mod previews;

use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.render(previews::markdown::build_ui())?;
    blaeck.unmount()?;
    Ok(())
}
