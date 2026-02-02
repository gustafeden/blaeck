//! Simple hello world example demonstrating Blaeck's Ink-style inline rendering.

#[path = "previews/mod.rs"]
mod previews;

use blaeck::Blaeck;

fn main() -> std::io::Result<()> {
    let stdout = std::io::stdout();
    let mut blaeck = Blaeck::new(stdout)?;
    blaeck.render(previews::hello::build_ui())?;
    blaeck.unmount()?;
    println!("\nThis text appears below the UI.");
    Ok(())
}
