//! KeyHints example - Keyboard shortcut displays
//!
//! Run with: cargo run --example keyhints

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let ui = Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            border_style: BorderStyle::Round,
            ..Default::default()
        },
        vec![
            // Title
            Element::node::<Text>(
                TextProps {
                    content: "KeyHints Component".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            // Default style (Compact with bullet separator)
            Element::node::<Text>(
                TextProps {
                    content: "Default (Compact + Bullet):".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<KeyHints>(
                KeyHintsProps::new([
                    ("^C", "exit"),
                    ("↑↓", "navigate"),
                    ("Enter", "select"),
                ]),
                vec![],
            ),
            Element::text(""),
            // Bracketed style
            Element::node::<Text>(
                TextProps {
                    content: "Bracketed style:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<KeyHints>(
                KeyHintsProps::new([
                    ("q", "quit"),
                    ("?", "help"),
                    ("Tab", "switch"),
                ])
                .style(KeyHintStyle::Bracketed),
                vec![],
            ),
            Element::text(""),
            // Colon style with pipe separator
            Element::node::<Text>(
                TextProps {
                    content: "Colon style + Pipe separator:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<KeyHints>(
                KeyHintsProps::new([
                    ("Space", "toggle"),
                    ("a", "select all"),
                    ("Esc", "cancel"),
                ])
                .style(KeyHintStyle::Colon)
                .separator(KeyHintSeparator::Pipe),
                vec![],
            ),
            Element::text(""),
            // Action first style
            Element::node::<Text>(
                TextProps {
                    content: "Action first style:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<KeyHints>(
                KeyHintsProps::new([
                    ("^S", "save"),
                    ("^Z", "undo"),
                    ("^Y", "redo"),
                ])
                .style(KeyHintStyle::ActionFirst)
                .separator(KeyHintSeparator::Slash),
                vec![],
            ),
            Element::text(""),
            // With colors
            Element::node::<Text>(
                TextProps {
                    content: "With custom color:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<KeyHints>(
                KeyHintsProps::new([
                    ("F1", "docs"),
                    ("F5", "refresh"),
                    ("F12", "debug"),
                ])
                .key_color(Color::Yellow)
                .separator(KeyHintSeparator::DoubleSpace),
                vec![],
            ),
            Element::text(""),
            // Builder pattern
            Element::node::<Text>(
                TextProps {
                    content: "Builder pattern:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<KeyHints>(
                KeyHintsProps::default()
                    .hint("j/k", "up/down")
                    .hint("g", "top")
                    .hint("G", "bottom")
                    .hint("/", "search")
                    .style(KeyHintStyle::Compact)
                    .key_color(Color::Green),
                vec![],
            ),
            Element::text(""),
            // Helper function demo
            Element::node::<Divider>(
                DividerProps::new()
                    .width(40)
                    .line_style(DividerStyle::Dashed)
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: "Helper function (returns string):".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: key_hints([("^C", "exit"), ("q", "quit")]),
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
        ],
    );

    blaeck.render(ui)?;
    blaeck.unmount()?;

    Ok(())
}
