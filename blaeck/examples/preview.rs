//! Preview example - showcases Blaeck's layout and component capabilities.
//!
//! Run with: cargo run --example preview

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;
use std::time::{Duration, Instant};

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    blaeck.set_max_fps(15);

    let start = Instant::now();
    let spinners = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

    // Run for 6 seconds
    while start.elapsed() < Duration::from_secs(6) {
        let elapsed = start.elapsed().as_millis() as usize;
        let spinner = spinners[(elapsed / 80) % spinners.len()];

        // Animate CPU values (fluctuate realistically)
        let cpu1 = 2.0 + ((elapsed as f32 / 200.0).sin() * 1.5);
        let cpu2 = 5.0 + ((elapsed as f32 / 150.0).cos() * 2.0);
        let mem1 = 128 + ((elapsed / 500) % 20) as i32 - 10;
        let mem2 = 512 + ((elapsed / 400) % 30) as i32 - 15;

        // Animate progress from 60% to 100%
        let progress = 60 + ((elapsed / 100) % 41).min(40);
        let filled = progress / 10;
        let empty = 10 - filled;
        let progress_bar = format!(
            "[{}{}] {}%",
            "█".repeat(filled),
            "░".repeat(empty),
            progress
        );

        let ui = element! {
            Box(padding: 1.0, flex_direction: FlexDirection::Column, gap: 1.0) {
                // System Status section
                Box(border_style: BorderStyle::Round, flex_direction: FlexDirection::Column, padding: 1.0) {
                    Box(flex_direction: FlexDirection::Row, margin_top: -2.0, margin_left: 1.0) {
                        Text(content: " System Status ", bold: true)
                    }
                    Box(flex_direction: FlexDirection::Row, gap: 2.0) {
                        Text(content: "●", color: Color::Green)
                        Text(content: "API Server")
                        Text(content: "running", color: Color::Green)
                        Text(content: format!("cpu {:.1}%", cpu1), dim: true)
                        Text(content: format!("mem {}MB", mem1), dim: true)
                    }
                    Box(flex_direction: FlexDirection::Row, gap: 2.0) {
                        Text(content: "●", color: Color::Green)
                        Text(content: "Database")
                        Text(content: "running", color: Color::Green)
                        Text(content: format!("cpu {:.1}%", cpu2), dim: true)
                        Text(content: format!("mem {}MB", mem2), dim: true)
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
                            Text(content: spinner, color: Color::Cyan)
                            Text(content: " Running tests...")
                        }
                    }
                    Box(flex_direction: FlexDirection::Column) {
                        Text(content: "Progress", bold: true)
                        Text(content: progress_bar.clone(), color: Color::Yellow)
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
        std::thread::sleep(Duration::from_millis(50));
    }

    blaeck.unmount()?;
    Ok(())
}
