//! Banner - ASCII art banners, styled headers, and component showcase
//!
//! Run with: cargo run --example banner

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let ui = Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            ..Default::default()
        },
        vec![
            // Blaeck ASCII art banner
            Element::node::<Text>(
                TextProps {
                    content: " ▗▄▄▄▄▄▄▄▖".into(),
                    color: Some(Color::Rgb(100, 200, 255)),
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: "▐█ QUILL █▌".into(),
                    color: Some(Color::Rgb(100, 200, 255)),
                    bold: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: " ▝▀▀▀▀▀▀▀▘".into(),
                    color: Some(Color::Rgb(100, 200, 255)),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            // Tagline with badge
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    gap: 2.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Ink-style terminal UI for Rust".into(),
                            italic: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Badge>(
                        BadgeProps::new("v0.1")
                            .color(Color::Green)
                            .badge_style(BadgeStyle::Bracket),
                        vec![],
                    ),
                ],
            ),
            Element::text(""),
            // Divider
            Element::node::<Divider>(
                DividerProps::new()
                    .width(70)
                    .label("Features")
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::text(""),
            // Feature boxes
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    gap: 1.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Box>(
                        BoxProps {
                            border_style: BorderStyle::Round,
                            padding: 1.0,
                            width: Some(22.0),
                            ..Default::default()
                        },
                        vec![
                            Element::node::<Text>(
                                TextProps {
                                    content: "Components".into(),
                                    bold: true,
                                    color: Some(Color::Green),
                                    ..Default::default()
                                },
                                vec![],
                            ),
                            Element::node::<Text>(
                                TextProps {
                                    content: "Box, Text, Spacer".into(),
                                    dim: true,
                                    ..Default::default()
                                },
                                vec![],
                            ),
                            Element::node::<Text>(
                                TextProps {
                                    content: "Select, Confirm".into(),
                                    dim: true,
                                    ..Default::default()
                                },
                                vec![],
                            ),
                            Element::node::<Text>(
                                TextProps {
                                    content: "Progress, Spinner".into(),
                                    dim: true,
                                    ..Default::default()
                                },
                                vec![],
                            ),
                        ],
                    ),
                    Element::node::<Box>(
                        BoxProps {
                            border_style: BorderStyle::Round,
                            padding: 1.0,
                            width: Some(22.0),
                            ..Default::default()
                        },
                        vec![
                            Element::node::<Text>(
                                TextProps {
                                    content: "Styling".into(),
                                    bold: true,
                                    color: Some(Color::Magenta),
                                    ..Default::default()
                                },
                                vec![],
                            ),
                            Element::node::<Text>(
                                TextProps {
                                    content: "16 colors + RGB".into(),
                                    dim: true,
                                    ..Default::default()
                                },
                                vec![],
                            ),
                            Element::node::<Text>(
                                TextProps {
                                    content: "Bold, dim, italic".into(),
                                    dim: true,
                                    ..Default::default()
                                },
                                vec![],
                            ),
                            Element::node::<Text>(
                                TextProps {
                                    content: "Underline".into(),
                                    dim: true,
                                    ..Default::default()
                                },
                                vec![],
                            ),
                        ],
                    ),
                    Element::node::<Box>(
                        BoxProps {
                            border_style: BorderStyle::Round,
                            padding: 1.0,
                            width: Some(22.0),
                            ..Default::default()
                        },
                        vec![
                            Element::node::<Text>(
                                TextProps {
                                    content: "Input".into(),
                                    bold: true,
                                    color: Some(Color::Yellow),
                                    ..Default::default()
                                },
                                vec![],
                            ),
                            Element::node::<Text>(
                                TextProps {
                                    content: "Keyboard handling".into(),
                                    dim: true,
                                    ..Default::default()
                                },
                                vec![],
                            ),
                            Element::node::<Text>(
                                TextProps {
                                    content: "TextInput, Select".into(),
                                    dim: true,
                                    ..Default::default()
                                },
                                vec![],
                            ),
                            Element::node::<Text>(
                                TextProps {
                                    content: "Focus management".into(),
                                    dim: true,
                                    ..Default::default()
                                },
                                vec![],
                            ),
                        ],
                    ),
                ],
            ),
            Element::text(""),
            // Divider
            Element::node::<Divider>(
                DividerProps::new()
                    .width(70)
                    .label("Badges")
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::text(""),
            // Badge showcase
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    gap: 2.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Badge>(BadgeProps::new("Simple").color(Color::White), vec![]),
                    Element::node::<Badge>(
                        BadgeProps::new("Bracket")
                            .color(Color::Cyan)
                            .badge_style(BadgeStyle::Bracket),
                        vec![],
                    ),
                    Element::node::<Badge>(
                        BadgeProps::new("5")
                            .color(Color::Yellow)
                            .badge_style(BadgeStyle::Round),
                        vec![],
                    ),
                    Element::node::<Badge>(
                        BadgeProps::new("tag")
                            .color(Color::Magenta)
                            .badge_style(BadgeStyle::Pill),
                        vec![],
                    ),
                    Element::node::<Badge>(
                        BadgeProps::new("ERROR")
                            .color(Color::White)
                            .bg_color(Color::Red)
                            .badge_style(BadgeStyle::Filled)
                            .bold(),
                        vec![],
                    ),
                    Element::node::<Badge>(
                        BadgeProps::new("OK")
                            .color(Color::White)
                            .bg_color(Color::Green)
                            .badge_style(BadgeStyle::Filled)
                            .bold(),
                        vec![],
                    ),
                ],
            ),
            Element::text(""),
            // Divider
            Element::node::<Divider>(
                DividerProps::new()
                    .width(70)
                    .label("Colors")
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::text(""),
            // Color palette
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "██".into(),
                            color: Some(Color::Red),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: "██".into(),
                            color: Some(Color::Yellow),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: "██".into(),
                            color: Some(Color::Green),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: "██".into(),
                            color: Some(Color::Cyan),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: "██".into(),
                            color: Some(Color::Blue),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: "██".into(),
                            color: Some(Color::Magenta),
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            ),
            Element::text(""),
            // Divider
            Element::node::<Divider>(
                DividerProps::new()
                    .width(70)
                    .label("Borders")
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::text(""),
            // Border styles
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    gap: 1.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Box>(
                        BoxProps {
                            border_style: BorderStyle::Single,
                            padding: 0.5,
                            ..Default::default()
                        },
                        vec![Element::node::<Text>(
                            TextProps {
                                content: "Single".into(),
                                ..Default::default()
                            },
                            vec![],
                        )],
                    ),
                    Element::node::<Box>(
                        BoxProps {
                            border_style: BorderStyle::Round,
                            padding: 0.5,
                            ..Default::default()
                        },
                        vec![Element::node::<Text>(
                            TextProps {
                                content: "Round".into(),
                                ..Default::default()
                            },
                            vec![],
                        )],
                    ),
                    Element::node::<Box>(
                        BoxProps {
                            border_style: BorderStyle::Double,
                            padding: 0.5,
                            ..Default::default()
                        },
                        vec![Element::node::<Text>(
                            TextProps {
                                content: "Double".into(),
                                ..Default::default()
                            },
                            vec![],
                        )],
                    ),
                    Element::node::<Box>(
                        BoxProps {
                            border_style: BorderStyle::Bold,
                            padding: 0.5,
                            ..Default::default()
                        },
                        vec![Element::node::<Text>(
                            TextProps {
                                content: "Bold".into(),
                                ..Default::default()
                            },
                            vec![],
                        )],
                    ),
                ],
            ),
            Element::text(""),
            // Divider styles
            Element::node::<Divider>(
                DividerProps::new()
                    .width(70)
                    .label("Divider Styles")
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::text(""),
            Element::node::<Text>(
                TextProps {
                    content: "Single:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Divider>(
                DividerProps::new()
                    .width(50)
                    .line_style(DividerStyle::Single),
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: "Double:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Divider>(
                DividerProps::new()
                    .width(50)
                    .line_style(DividerStyle::Double),
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: "Dashed:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Divider>(
                DividerProps::new()
                    .width(50)
                    .line_style(DividerStyle::Dashed),
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: "Bold:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Divider>(
                DividerProps::new()
                    .width(50)
                    .line_style(DividerStyle::Bold)
                    .color(Color::Yellow),
                vec![],
            ),
        ],
    );

    blaeck.render(ui)?;
    blaeck.unmount()?;

    println!();
    Ok(())
}
