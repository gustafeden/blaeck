//! Demo: Inline rendering - blaeck doesn't take over your terminal.
//!
//! This shows the killer feature: output stays in scrollback,
//! println!() works before and after, terminal keeps working.

#[path = "previews/mod.rs"]
mod previews;

use std::{thread, time::Duration};

fn main() -> std::io::Result<()> {
    println!("Starting build...\n");

    let mut blaeck = blaeck::Blaeck::new(std::io::stdout())?;

    for i in 0..=10 {
        blaeck.render(previews::demo_inline::build_ui_frame(i))?;
        thread::sleep(Duration::from_millis(200));
    }

    blaeck.unmount()?;

    println!("\nBuild complete! Output written to ./dist");
    println!("You can keep using your terminal normally.");

    Ok(())
}
