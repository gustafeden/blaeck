//! Table example - Display data in rows and columns
//!
//! Run with: cargo run --example table

#[path = "previews/mod.rs"]
mod previews;

use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.render(previews::table::build_ui())?;
    blaeck.unmount()?;
    println!();
    Ok(())
}
