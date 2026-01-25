//! Static quickstart example - simple one-shot rendering.

use blaeck::prelude::*;

fn main() -> std::io::Result<()> {
    blaeck::print(element! {
        Box(border_style: BorderStyle::Round, padding: 1.0) {
            Box(flex_direction: FlexDirection::Row, gap: 2.0) {
                Text(content: "Status:", bold: true)
                Text(content: "Ready", color: Color::Green)
            }
            Box(flex_direction: FlexDirection::Row, gap: 1.0) {
                Text(content: "Progress:", dim: true)
                Text(content: "[████████░░] 80%", color: Color::Cyan)
            }
        }
    })
}
