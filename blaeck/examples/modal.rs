//! Modal/Dialog example - Overlay prompts and alerts
//!
//! Run with: cargo run --example modal

#[path = "previews/mod.rs"]
mod previews;

use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.render(previews::modal::build_ui())?;
    blaeck.unmount()?;
    Ok(())
}
