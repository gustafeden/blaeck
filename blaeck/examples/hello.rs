//! Simple hello world example demonstrating Blaeck's Ink-style inline rendering.

use blaeck::prelude::*;
use blaeck::Blaeck;

fn main() -> std::io::Result<()> {
    let stdout = std::io::stdout();
    let mut blaeck = Blaeck::new(stdout)?;

    // Create a simple UI with a box and styled text
    let ui = element! {
        Box(padding: 1.0, border_style: BorderStyle::Single) {
            Text(content: "Hello from Blaeck!", color: Color::Green, bold: true)
            Text(content: "Inline rendering - no fullscreen!")
        }
    };

    blaeck.render(ui)?;
    blaeck.unmount()?;

    // This prints below the UI!
    println!("\nThis text appears below the UI.");

    Ok(())
}
