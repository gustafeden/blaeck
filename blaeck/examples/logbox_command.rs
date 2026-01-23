//! LogBox command example - Real-time command output viewer with blinking indicator
//!
//! Run with: cargo run --example logbox_command

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    // Spawn a command that produces output over time
    let mut child = Command::new("sh")
        .arg("-c")
        .arg("for i in 1 2 3 4 5 6 7 8; do echo \"Processing item $i...\"; sleep 0.3; done && echo \"Done!\"")
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().expect("Failed to capture stdout");

    // Channel for sending lines from reader thread to main thread
    let (tx, rx) = mpsc::channel::<String>();

    // Spawn thread to read command output
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                let _ = tx.send(line);
            }
        }
    });

    let mut lines: Vec<LogLine> = Vec::new();
    let timer = AnimationTimer::new();
    let mut running = true;

    // Main render loop
    while running {
        // Check for new lines (non-blocking)
        loop {
            match rx.try_recv() {
                Ok(line) => {
                    // Color the line based on content
                    let log_line = if line.contains("Done") {
                        LogLine::success(&line)
                    } else if line.contains("Error") {
                        LogLine::error(&line)
                    } else {
                        LogLine::new(&line).color(Color::DarkGray)
                    };
                    lines.push(log_line);
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    running = false;
                    break;
                }
            }
        }

        // Build UI with blinking indicator using animation helpers
        let ui = Element::node::<Box>(
            BoxProps {
                flex_direction: FlexDirection::Column,
                padding: 1.0,
                border_style: BorderStyle::Round,
                ..Default::default()
            },
            vec![
                // Header with blinking indicator
                Element::node::<Box>(
                    BoxProps {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    vec![
                        blinking_dot(&timer, 500, Color::DarkGray),
                        Element::node::<Text>(
                            TextProps {
                                content: " Running command".into(),
                                bold: true,
                                ..Default::default()
                            },
                            vec![],
                        ),
                    ],
                ),
                // Log box showing command output
                Element::node::<LogBox>(
                    LogBoxProps::with_lines(lines.clone())
                        .max_lines(5)
                        .tree_style(TreeStyle::Unicode)
                        .indent(1),
                    vec![],
                ),
            ],
        );

        blaeck.render(ui)?;

        // Sleep for render tick (100ms for smooth blinking)
        thread::sleep(Duration::from_millis(100));
    }

    // Wait for command to finish
    let status = child.wait()?;

    // Final render with status
    let status_line = if status.success() {
        LogLine::success("✓ Command completed successfully")
    } else {
        LogLine::error(format!(
            "✗ Command failed with code {}",
            status.code().unwrap_or(-1)
        ))
    };
    lines.push(status_line);

    let final_ui = Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            border_style: BorderStyle::Round,
            border_color: if status.success() {
                Some(Color::Green)
            } else {
                Some(Color::Red)
            },
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
                            color: Some(if status.success() {
                                Color::Green
                            } else {
                                Color::Red
                            }),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: " Command finished".into(),
                            bold: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            ),
            Element::node::<LogBox>(
                LogBoxProps::with_lines(lines)
                    .max_lines(5)
                    .tree_style(TreeStyle::Unicode)
                    .indent(1),
                vec![],
            ),
        ],
    );

    blaeck.render(final_ui)?;
    blaeck.unmount()?;

    Ok(())
}
