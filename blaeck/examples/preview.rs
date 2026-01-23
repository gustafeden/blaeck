//! Preview example - showcases Blaeck's layout and component capabilities.
//!
//! Run with: cargo run --example preview

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let ui = element! {
        Box(border_style: BorderStyle::Round, padding: 1.0, flex_direction: FlexDirection::Column, gap: 1.0) {
            // System Status section
            Box(border_style: BorderStyle::Round, flex_direction: FlexDirection::Column, padding_left: 1.0, padding_right: 1.0) {
                Box(flex_direction: FlexDirection::Row, margin_top: -1.0, margin_left: 1.0) {
                    Text(content: " System Status ", bold: true)
                }
                Box(flex_direction: FlexDirection::Row, gap: 2.0) {
                    Text(content: "●", color: Color::Green)
                    Text(content: "API Server")
                    Text(content: "running", color: Color::Green)
                    Text(content: "cpu 2.3%", dim: true)
                    Text(content: "mem 128MB", dim: true)
                }
                Box(flex_direction: FlexDirection::Row, gap: 2.0) {
                    Text(content: "●", color: Color::Green)
                    Text(content: "Database")
                    Text(content: "running", color: Color::Green)
                    Text(content: "cpu 5.1%", dim: true)
                    Text(content: "mem 512MB", dim: true)
                }
                Box(flex_direction: FlexDirection::Row, gap: 2.0) {
                    Text(content: "○", color: Color::Red)
                    Text(content: "Cache")
                    Text(content: "stopped", color: Color::Red)
                }
            }

            // Tasks and Progress section
            Box(flex_direction: FlexDirection::Row, gap: 4.0) {
                Box(flex_direction: FlexDirection::Column) {
                    Text(content: "Tasks", bold: true)
                    Box(flex_direction: FlexDirection::Row) {
                        Text(content: "├── ", dim: true)
                        Text(content: "Downloading assets")
                    }
                    Box(flex_direction: FlexDirection::Row) {
                        Text(content: "├── ", dim: true)
                        Text(content: "Compiling")
                    }
                    Box(flex_direction: FlexDirection::Row) {
                        Text(content: "└── ", dim: true)
                        Text(content: "⠋ ", color: Color::Cyan)
                        Text(content: "Running tests...")
                    }
                }
                Box(flex_direction: FlexDirection::Column) {
                    Text(content: "Progress", bold: true)
                    Text(content: "[████████░░] 80%", color: Color::Yellow)
                    Text(content: "[██████████] done", color: Color::Green)
                }
            }

            // File tree section
            Box(flex_direction: FlexDirection::Column) {
                Box(flex_direction: FlexDirection::Row) {
                    Text(content: "│ ", color: Color::Blue)
                    Text(content: "src/", bold: true)
                }
                Box(flex_direction: FlexDirection::Row) {
                    Text(content: "│ ", color: Color::Blue)
                    Text(content: "├── ", dim: true)
                    Text(content: "main.rs")
                }
                Box(flex_direction: FlexDirection::Row) {
                    Text(content: "│ ", color: Color::Blue)
                    Text(content: "└── ", dim: true)
                    Text(content: "components/", bold: true)
                }
                Box(flex_direction: FlexDirection::Row) {
                    Text(content: "│ ", color: Color::Blue)
                    Text(content: "    ├── ", dim: true)
                    Text(content: "mod.rs")
                }
                Box(flex_direction: FlexDirection::Row) {
                    Text(content: "│ ", color: Color::Blue)
                    Text(content: "    └── ", dim: true)
                    Text(content: "button.rs")
                }
            }

            // Key hints
            Box(flex_direction: FlexDirection::Row, gap: 2.0) {
                Text(content: "[Tab]", bold: true)
                Text(content: "Next", dim: true)
                Text(content: "[Enter]", bold: true)
                Text(content: "Select", dim: true)
                Text(content: "[q]", bold: true)
                Text(content: "Quit", dim: true)
            }
        }
    };

    blaeck.render(ui)?;
    blaeck.unmount()?;

    Ok(())
}
