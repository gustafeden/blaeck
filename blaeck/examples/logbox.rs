//! LogBox example - Auto-expanding scrolling log viewer
//!
//! Run with: cargo run --example logbox

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    // Simulate an agent/task with streaming output
    let tasks = vec![
        ("Initializing...", None),
        ("Scanning directory", None),
        ("Read(/blaeck/src/lib.rs)", Some(Color::DarkGray)),
        ("Read(/blaeck/src/main.rs)", Some(Color::DarkGray)),
        ("Error reading file", Some(Color::Red)),
        ("Glob(**/*.rs)", Some(Color::DarkGray)),
        ("Found 42 files", None),
        ("Analyzing imports", None),
        ("Building dependency graph", None),
        ("Optimization complete", Some(Color::Green)),
    ];

    let mut lines: Vec<LogLine> = Vec::new();

    for (i, (msg, color)) in tasks.iter().enumerate() {
        // Add new line
        let line = if let Some(c) = color {
            LogLine::new(*msg).color(*c)
        } else {
            LogLine::new(*msg)
        };
        lines.push(line);

        // Build UI with header and log box
        let ui = Element::node::<Box>(
            BoxProps {
                flex_direction: FlexDirection::Column,
                padding: 1.0,
                border_style: BorderStyle::Round,
                ..Default::default()
            },
            vec![
                // Header
                Element::node::<Box>(
                    BoxProps {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    vec![
                        Element::node::<Text>(
                            TextProps {
                                content: "●".into(),
                                color: Some(Color::Yellow),
                                ..Default::default()
                            },
                            vec![],
                        ),
                        Element::node::<Text>(
                            TextProps {
                                content: " Explore".into(),
                                bold: true,
                                ..Default::default()
                            },
                            vec![],
                        ),
                        Element::node::<Text>(
                            TextProps {
                                content: "(Explore /blaeck repository)".into(),
                                dim: true,
                                ..Default::default()
                            },
                            vec![],
                        ),
                    ],
                ),
                // Log box with tree connectors
                Element::node::<LogBox>(
                    LogBoxProps::with_lines(lines.clone())
                        .max_lines(5)
                        .tree_style(TreeStyle::Unicode)
                        .indent(1),
                    vec![],
                ),
                // Footer hints
                Element::text(""),
                Element::node::<Text>(
                    TextProps {
                        content: format!("Step {}/{}", i + 1, tasks.len()),
                        dim: true,
                        ..Default::default()
                    },
                    vec![],
                ),
            ],
        );

        blaeck.render(ui)?;
        thread::sleep(Duration::from_millis(400));
    }

    // Final state - show all examples in one render
    thread::sleep(Duration::from_millis(500));

    // Build all three examples in a single UI
    let final_ui = Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            gap: 1.0,
            ..Default::default()
        },
        vec![
            // Example 1: Completed explore task
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    padding: 1.0,
                    border_style: BorderStyle::Round,
                    border_color: Some(Color::Green),
                    ..Default::default()
                },
                vec![
                    Element::node::<Box>(
                        BoxProps {
                            flex_direction: FlexDirection::Row,
                            ..Default::default()
                        },
                        vec![
                            Element::node::<Text>(
                                TextProps {
                                    content: "●".into(),
                                    color: Some(Color::Green),
                                    ..Default::default()
                                },
                                vec![],
                            ),
                            Element::node::<Text>(
                                TextProps {
                                    content: " Explore".into(),
                                    bold: true,
                                    ..Default::default()
                                },
                                vec![],
                            ),
                            Element::node::<Text>(
                                TextProps {
                                    content: "(completed)".into(),
                                    dim: true,
                                    ..Default::default()
                                },
                                vec![],
                            ),
                        ],
                    ),
                    Element::node::<LogBox>(
                        LogBoxProps::with_lines(lines.clone())
                            .max_lines(5)
                            .tree_style(TreeStyle::Unicode)
                            .indent(1),
                        vec![],
                    ),
                ],
            ),
            // Example 2: Simple log without tree style
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single,
                    padding: 1.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Build Output (no tree, max 3 lines):".into(),
                            bold: true,
                            color: Some(Color::Cyan),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<LogBox>(
                        LogBoxProps::new()
                            .line("Compiling blaeck v0.1.0")
                            .line("Compiling blaeck-macros v0.1.0")
                            .line(LogLine::warning("warning: unused import"))
                            .line("Compiling example v0.1.0")
                            .line(LogLine::success("Finished dev [unoptimized] in 2.34s"))
                            .max_lines(3),
                        vec![],
                    ),
                ],
            ),
            // Example 3: Error log
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single,
                    border_color: Some(Color::Red),
                    padding: 1.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Error Log:".into(),
                            bold: true,
                            color: Some(Color::Red),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<LogBox>(
                        LogBoxProps::new()
                            .line(LogLine::error("Error: Connection refused"))
                            .line(LogLine::muted("  at src/network.rs:42"))
                            .line(LogLine::muted("  at src/main.rs:15"))
                            .line(LogLine::warning("Retrying in 5s..."))
                            .max_lines(10)
                            .tree_style(TreeStyle::Unicode),
                        vec![],
                    ),
                ],
            ),
        ],
    );

    blaeck.render(final_ui)?;
    blaeck.unmount()?;
    Ok(())
}
