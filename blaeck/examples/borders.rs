//! Borders - Showcase of enhanced border styling
//!
//! Run with: cargo run --example borders

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let ui = element! {
        Box(flex_direction: FlexDirection::Column, padding: 1.0) {
            Text(content: "Enhanced Border Styling", bold: true, color: Color::Cyan)
            Text(content: "")

            // Per-side colors
            Text(content: "Per-Side Border Colors:", bold: true)
            Box(flex_direction: FlexDirection::Row, gap: 2.0) {
                Box(
                    border_style: BorderStyle::Single,
                    border_colors: BorderColors {
                        top: Some(Color::Red),
                        bottom: Some(Color::Blue),
                        left: Some(Color::Green),
                        right: Some(Color::Yellow),
                    },
                    padding: 1.0,
                    width: 20.0,
                    height: 5.0
                ) {
                    Text(content: "4 colors!")
                }
                Box(
                    border_style: BorderStyle::Double,
                    border_colors: BorderColors {
                        top: Some(Color::Magenta),
                        bottom: Some(Color::Magenta),
                        left: Some(Color::Cyan),
                        right: Some(Color::Cyan),
                    },
                    padding: 1.0,
                    width: 20.0,
                    height: 5.0
                ) {
                    Text(content: "H/V colors")
                }
            }
            Text(content: "")

            // Partial borders
            Text(content: "Partial Borders:", bold: true)
            Box(flex_direction: FlexDirection::Row, gap: 2.0) {
                Box(
                    border_style: BorderStyle::Single,
                    border_sides: Some(BorderSides::horizontal()),
                    border_color: Some(Color::Green),
                    padding: 1.0,
                    width: 18.0,
                    height: 4.0
                ) {
                    Text(content: "Top & Bottom")
                }
                Box(
                    border_style: BorderStyle::Single,
                    border_sides: Some(BorderSides::vertical()),
                    border_color: Some(Color::Blue),
                    padding: 1.0,
                    width: 18.0,
                    height: 4.0
                ) {
                    Text(content: "Left & Right")
                }
                Box(
                    border_style: BorderStyle::Bold,
                    border_sides: Some(BorderSides::top_only()),
                    border_color: Some(Color::Yellow),
                    padding: 1.0,
                    width: 18.0,
                    height: 4.0
                ) {
                    Text(content: "Top Only")
                }
                Box(
                    border_style: BorderStyle::Bold,
                    border_sides: Some(BorderSides::bottom_only()),
                    border_color: Some(Color::Magenta),
                    padding: 1.0,
                    width: 18.0,
                    height: 4.0
                ) {
                    Text(content: "Bottom Only")
                }
            }
            Text(content: "")

            // Dim borders
            Text(content: "Dim Borders:", bold: true)
            Box(flex_direction: FlexDirection::Row, gap: 2.0) {
                Box(
                    border_style: BorderStyle::Round,
                    border_color: Some(Color::White),
                    padding: 1.0,
                    width: 20.0
                ) {
                    Text(content: "Normal border")
                }
                Box(
                    border_style: BorderStyle::Round,
                    border_color: Some(Color::White),
                    border_dim: true,
                    padding: 1.0,
                    width: 20.0
                ) {
                    Text(content: "Dim border")
                }
            }
            Text(content: "")

            // Combined features
            Text(content: "Combined Features:", bold: true)
            Box(
                border_style: BorderStyle::Double,
                border_sides: Some(BorderSides { top: true, bottom: true, left: false, right: false }),
                border_colors: BorderColors {
                    top: Some(Color::Cyan),
                    bottom: Some(Color::Magenta),
                    ..Default::default()
                },
                padding: 1.0,
                width: 40.0
            ) {
                Text(content: "Horizontal borders with gradient effect")
            }
        }
    };

    blaeck.render(ui)?;
    blaeck.unmount()?;

    println!();
    Ok(())
}
