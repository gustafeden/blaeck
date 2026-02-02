//! StatusBar example - Git-style status displays
//!
//! Run with: cargo run --example statusbar

#[path = "previews/mod.rs"]
mod previews;

use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.render(previews::statusbar::build_ui())?;
    blaeck.unmount()?;
    Ok(())
}
