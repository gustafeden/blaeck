//! LogBox command example - Real-time command output viewer with blinking indicator
//!
//! Run with: cargo run --example logbox_command

#[path = "previews/mod.rs"]
mod previews;

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let mut child = Command::new("sh")
        .arg("-c")
        .arg("for i in 1 2 3 4 5 6 7 8; do echo \"Processing item $i...\"; sleep 0.3; done && echo \"Done!\"")
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().expect("Failed to capture stdout");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            let _ = tx.send(line);
        }
    });

    let mut lines: Vec<LogLine> = Vec::new();
    let timer = AnimationTimer::new();
    let mut running = true;

    while running {
        loop {
            match rx.try_recv() {
                Ok(line) => {
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

        let ui = Element::node::<Box>(
            BoxProps {
                flex_direction: FlexDirection::Column,
                padding: 1.0,
                border_style: BorderStyle::Round,
                ..Default::default()
            },
            vec![
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
        thread::sleep(Duration::from_millis(100));
    }

    child.wait()?;

    // Final state uses shared completed UI
    blaeck.render(previews::logbox_command::build_completed_ui())?;
    blaeck.unmount()?;

    Ok(())
}
