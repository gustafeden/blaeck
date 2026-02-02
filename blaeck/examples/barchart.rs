//! Bar Chart example - Horizontal bar visualization
//!
//! Run with: cargo run --example barchart

#[path = "previews/mod.rs"]
mod previews;

use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.render(previews::barchart::build_ui())?;
    blaeck.unmount()?;
    Ok(())
}
