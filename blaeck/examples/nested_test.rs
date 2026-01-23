//! Test nested boxes with borders and long text
use blaeck::prelude::*;
use blaeck::Blaeck;

fn main() -> std::io::Result<()> {
    let stdout = std::io::stdout();
    let mut blaeck = Blaeck::new(stdout)?;

    // Test 1: Fixed-width outer box
    println!("Test 1: Fixed-width nested boxes");
    let ui = element! {
        Box(flex_direction: FlexDirection::Column, border_style: BorderStyle::Round, padding: 1.0, width: 60.0) {
            Text(content: "Outer box (60 chars wide)", bold: true)
            Text(content: "")
            Box(flex_direction: FlexDirection::Column, border_style: BorderStyle::Single, padding: 0.5) {
                Text(content: "Inner box", color: Color::Yellow)
                Text(content: "Some content here")
            }
            Text(content: "")
            Text(content: "Back in outer box")
        }
    };

    blaeck.render(ui)?;
    blaeck.unmount()?;

    println!("\n--- Test 1 complete ---\n");

    // Test 2: Simple single box with border
    println!("Test 2: Simple bordered box");
    let mut blaeck2 = Blaeck::new(std::io::stdout())?;
    let ui2 = element! {
        Box(border_style: BorderStyle::Round, padding: 1.0, width: 40.0, height: 5.0) {
            Text(content: "Hello", color: Color::Green)
            Text(content: "World")
        }
    };
    blaeck2.render(ui2)?;
    blaeck2.unmount()?;

    println!("\n--- Test 2 complete ---");
    Ok(())
}
