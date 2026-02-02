//! Banner - ASCII art banners, styled headers, and component showcase
//!
//! Run with: cargo run --example banner

#[path = "previews/mod.rs"]
mod previews;

use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.render(previews::banner::build_ui())?;
    blaeck.unmount()?;
    println!();
    Ok(())
}
