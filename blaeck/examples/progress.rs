//! Progress bar example using the Progress component
//!
//! Run with: cargo run --example progress

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;
use std::{thread, time::Duration};

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    // Animate progress bars with different styles
    for i in 0..=20 {
        let progress = i as f32 / 20.0;

        let ui = Element::node::<Box>(
            BoxProps {
                flex_direction: FlexDirection::Column,
                padding: 1.0,
                border_style: BorderStyle::Round,
                ..Default::default()
            },
            vec![
                Element::node::<Text>(
                    TextProps {
                        content: "Progress Component Demo".into(),
                        bold: true,
                        color: Some(Color::Cyan),
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::text(""),
                // Block style (default)
                Element::node::<Box>(
                    BoxProps {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    vec![
                        Element::node::<Text>(
                            TextProps {
                                content: "Block:  ".into(),
                                dim: true,
                                ..Default::default()
                            },
                            vec![],
                        ),
                        Element::node::<Progress>(
                            ProgressProps::new(progress)
                                .width(20)
                                .style(ProgressStyle::Block)
                                .filled_color(Color::Green)
                                .show_percentage(),
                            vec![],
                        ),
                    ],
                ),
                // ASCII style with brackets
                Element::node::<Box>(
                    BoxProps {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    vec![
                        Element::node::<Text>(
                            TextProps {
                                content: "ASCII:  ".into(),
                                dim: true,
                                ..Default::default()
                            },
                            vec![],
                        ),
                        Element::node::<Progress>(
                            ProgressProps::new(progress)
                                .width(20)
                                .style(ProgressStyle::Ascii)
                                .brackets()
                                .filled_color(Color::Cyan)
                                .show_percentage(),
                            vec![],
                        ),
                    ],
                ),
                // Thick style
                Element::node::<Box>(
                    BoxProps {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    vec![
                        Element::node::<Text>(
                            TextProps {
                                content: "Thick:  ".into(),
                                dim: true,
                                ..Default::default()
                            },
                            vec![],
                        ),
                        Element::node::<Progress>(
                            ProgressProps::new(progress)
                                .width(20)
                                .style(ProgressStyle::Thick)
                                .filled_color(Color::Magenta)
                                .show_percentage(),
                            vec![],
                        ),
                    ],
                ),
                Element::text(""),
                // Status message
                if progress >= 1.0 {
                    Element::node::<Text>(
                        TextProps {
                            content: "Complete!".into(),
                            bold: true,
                            color: Some(Color::Green),
                            ..Default::default()
                        },
                        vec![],
                    )
                } else {
                    Element::node::<Text>(
                        TextProps {
                            content: format!("Loading... {:.0}%", progress * 100.0),
                            dim: true,
                            ..Default::default()
                        },
                        vec![],
                    )
                },
            ],
        );

        blaeck.render(ui)?;
        thread::sleep(Duration::from_millis(100));
    }

    // Keep the final state visible briefly
    thread::sleep(Duration::from_millis(500));
    blaeck.unmount()?;

    println!("\nDone!");
    Ok(())
}
