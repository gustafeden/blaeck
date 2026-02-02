//! Breadcrumbs example - Navigation path displays
//!
//! Run with: cargo run --example breadcrumbs

#[path = "previews/mod.rs"]
mod previews;

use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.render(previews::breadcrumbs::build_ui())?;
    blaeck.unmount()?;
    Ok(())
}
