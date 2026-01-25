//! Demo: Inline rendering - blaeck doesn't take over your terminal.
//!
//! This shows the killer feature: output stays in scrollback,
//! println!() works before and after, terminal keeps working.

use blaeck::prelude::*;
use std::{thread, time::Duration};

fn main() -> std::io::Result<()> {
    println!("Starting build...\n");

    let mut blaeck = Blaeck::new(std::io::stdout())?;

    // Simulate a build process with progress
    for i in 0..=10 {
        let progress = i as f32 / 10.0;
        let filled = (progress * 20.0) as usize;
        let empty = 20 - filled;
        let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));

        let status = match i {
            0..=3 => "Compiling dependencies...",
            4..=6 => "Building project...",
            7..=9 => "Linking...",
            _ => "Done!",
        };

        blaeck.render(element! {
            Box(border_style: BorderStyle::Round, padding: 1.0) {
                Text(content: status, bold: true)
                Box(flex_direction: FlexDirection::Row, gap: 1.0) {
                    Text(content: bar, color: Color::Green)
                    Text(content: format!("{}%", i * 10), dim: true)
                }
            }
        })?;

        thread::sleep(Duration::from_millis(200));
    }

    blaeck.unmount()?;

    // Terminal keeps working normally!
    println!("\nBuild complete! Output written to ./dist");
    println!("You can keep using your terminal normally.");

    Ok(())
}
