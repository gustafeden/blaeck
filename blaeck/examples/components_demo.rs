//! Components Demo - Showcases Spinner and Progress components
//!
//! Run with: cargo run --example components_demo

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::time::{Duration, Instant};
use std::{io, thread};

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.set_max_fps(30);

    let start = Instant::now();

    for _ in 0..150 {
        let elapsed = start.elapsed().as_secs_f32();

        // Calculate progress (0 to 100% over 5 seconds, then reset)
        let progress = (elapsed % 5.0) / 5.0;
        let percent = (progress * 100.0) as u32;

        // Calculate spinner frames
        let dots_frame = spinner_frame(start, SpinnerStyle::Dots);
        let arrow_frame = spinner_frame(start, SpinnerStyle::Arrow);
        let circle_frame = spinner_frame(start, SpinnerStyle::Circle);
        let moon_frame = spinner_frame(start, SpinnerStyle::Moon);

        // Build UI manually since we have dynamic content
        let ui = Element::node::<Box>(
            BoxProps {
                flex_direction: FlexDirection::Column,
                padding: 1.0,
                ..Default::default()
            },
            vec![
                // Title
                Element::node::<Text>(
                    TextProps {
                        content: "Blaeck Components Demo".into(),
                        bold: true,
                        color: Some(Color::Cyan),
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::text(""),
                // Spinners section
                Element::node::<Text>(
                    TextProps {
                        content: "Spinners".into(),
                        bold: true,
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::node::<Box>(
                    BoxProps {
                        flex_direction: FlexDirection::Column,
                        padding_left: Some(2.0),
                        ..Default::default()
                    },
                    vec![
                        // Dots spinner
                        Element::node::<Spinner>(
                            SpinnerProps::new()
                                .style(SpinnerStyle::Dots)
                                .frame(dots_frame)
                                .label("Loading data...")
                                .color(Color::Green),
                            vec![],
                        ),
                        // Arrow spinner
                        Element::node::<Spinner>(
                            SpinnerProps::new()
                                .style(SpinnerStyle::Arrow)
                                .frame(arrow_frame)
                                .label("Processing")
                                .color(Color::Yellow),
                            vec![],
                        ),
                        // Circle spinner
                        Element::node::<Spinner>(
                            SpinnerProps::new()
                                .style(SpinnerStyle::Circle)
                                .frame(circle_frame)
                                .label("Syncing")
                                .color(Color::Magenta),
                            vec![],
                        ),
                        // Moon spinner
                        Element::node::<Spinner>(
                            SpinnerProps::new()
                                .style(SpinnerStyle::Moon)
                                .frame(moon_frame)
                                .label("Night mode"),
                            vec![],
                        ),
                    ],
                ),
                Element::text(""),
                // Progress bars section
                Element::node::<Text>(
                    TextProps {
                        content: "Progress Bars".into(),
                        bold: true,
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::node::<Box>(
                    BoxProps {
                        flex_direction: FlexDirection::Column,
                        padding_left: Some(2.0),
                        ..Default::default()
                    },
                    vec![
                        // Block style
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
                                        .width(25)
                                        .style(ProgressStyle::Block)
                                        .filled_color(Color::Green)
                                        .show_percentage(),
                                    vec![],
                                ),
                            ],
                        ),
                        // ASCII style
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
                                        .width(25)
                                        .style(ProgressStyle::Ascii)
                                        .brackets()
                                        .filled_color(Color::Cyan)
                                        .show_percentage(),
                                    vec![],
                                ),
                            ],
                        ),
                        // Dots style
                        Element::node::<Box>(
                            BoxProps {
                                flex_direction: FlexDirection::Row,
                                ..Default::default()
                            },
                            vec![
                                Element::node::<Text>(
                                    TextProps {
                                        content: "Dots:   ".into(),
                                        dim: true,
                                        ..Default::default()
                                    },
                                    vec![],
                                ),
                                Element::node::<Progress>(
                                    ProgressProps::new(progress)
                                        .width(25)
                                        .style(ProgressStyle::Dots)
                                        .filled_color(Color::Yellow)
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
                                        .width(25)
                                        .style(ProgressStyle::Thick)
                                        .filled_color(Color::Magenta)
                                        .show_percentage(),
                                    vec![],
                                ),
                            ],
                        ),
                    ],
                ),
                Element::text(""),
                // Helper functions demo
                Element::node::<Text>(
                    TextProps {
                        content: "Helper Functions".into(),
                        bold: true,
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::node::<Box>(
                    BoxProps {
                        flex_direction: FlexDirection::Column,
                        padding_left: Some(2.0),
                        ..Default::default()
                    },
                    vec![
                        Element::node::<Text>(
                            TextProps {
                                content: format!(
                                    "progress_bar({}, 20):     {}",
                                    percent,
                                    progress_bar(percent, 20)
                                ),
                                ..Default::default()
                            },
                            vec![],
                        ),
                        Element::node::<Text>(
                            TextProps {
                                content: format!(
                                    "progress_bar_bracketed: {}",
                                    progress_bar_bracketed(percent, 20)
                                ),
                                ..Default::default()
                            },
                            vec![],
                        ),
                    ],
                ),
                Element::text(""),
                // Footer
                Element::node::<Text>(
                    TextProps {
                        content: format!("Elapsed: {:.1}s | Progress resets every 5s", elapsed),
                        dim: true,
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::node::<Text>(
                    TextProps {
                        content: "Press Ctrl+C to exit".into(),
                        dim: true,
                        italic: true,
                        ..Default::default()
                    },
                    vec![],
                ),
            ],
        );

        blaeck.render(ui)?;
        thread::sleep(Duration::from_millis(33));
    }

    blaeck.unmount()?;
    println!("Done!");
    Ok(())
}
