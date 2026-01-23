//! Task Runner - Comprehensive Blaeck showcase
//!
//! Demonstrates:
//! - Live updating UI
//! - Nested boxes with borders (Single, Round, Double)
//! - All colors (Green, Yellow, Red, Cyan, White)
//! - Text modifiers (bold, dim, italic)
//! - Flexbox (Row for alignment, Column for stacking)
//! - Spacer for pushing content
//! - Spinner animation
//! - Progress bar

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::{thread, time::Duration};

const SPINNER: [char; 4] = ['◐', '◓', '◑', '◒'];

fn progress_bar(percent: u32, width: usize) -> String {
    let filled = (width * percent as usize) / 100;
    let empty = width - filled;
    format!("[{}{}] {:>3}%", "█".repeat(filled), "░".repeat(empty), percent)
}

fn format_duration(secs: f32) -> String {
    format!("{:.1}s", secs)
}

fn main() -> std::io::Result<()> {
    let stdout = std::io::stdout();
    let mut blaeck = Blaeck::new(stdout)?;

    // Task definitions
    let tasks = [
        ("Analyzing codebase", 2.3),
        ("Running linter", 1.1),
        ("Type checking", 3.2),
        ("Building project", 4.0),
    ];

    let files = [
        "src/main.rs", "src/lib.rs", "src/utils.rs",
        "src/config.rs", "src/parser.rs", "src/render.rs"
    ];
    let total_modules = 54;

    // Simulate tasks completing one by one
    let mut completed: Vec<(&str, f32)> = Vec::new();

    for (task_idx, (task_name, task_duration)) in tasks.iter().enumerate() {
        let frames = (*task_duration * 10.0) as usize;

        for frame in 0..frames {
            let spinner = SPINNER[frame % 4];
            let elapsed = frame as f32 * 0.1;
            let percent = ((frame as u32 * 100) / frames as u32).min(99);
            let modules_done = if task_idx == 3 {
                (frame * total_modules) / frames
            } else { 0 };
            let current_file = files[frame % files.len()];

            // Build completed task rows
            let mut children: Vec<Element> = Vec::new();

            for (name, dur) in &completed {
                children.push(element! {
                    Box(flex_direction: FlexDirection::Row) {
                        Text(content: format!("✓ {}", name), color: Color::Green)
                        Spacer
                        Text(content: format_duration(*dur), color: Color::Green, dim: true)
                    }
                });
            }

            // Build current task box children
            let mut task_box_children: Vec<Element> = vec![
                // Header row
                element! {
                    Box(flex_direction: FlexDirection::Row) {
                        Text(content: format!("{} {}...", spinner, task_name),
                            color: Color::Yellow, bold: true)
                        Spacer
                        Text(content: format_duration(elapsed),
                            color: Color::Cyan, dim: true)
                    }
                },
            ];

            // Add progress box for build task
            if task_idx == 3 {
                task_box_children.push(element! {
                    Box(border_style: BorderStyle::Round, padding: 1.0, margin_top: 1.0) {
                        Text(content: progress_bar(percent, 35))
                        Text(content: "")
                        Box(flex_direction: FlexDirection::Row) {
                            Text(content: "File:", color: Color::White, dim: true)
                            Text(content: format!(" {}", current_file))
                        }
                        Box(flex_direction: FlexDirection::Row) {
                            Text(content: "Progress:", color: Color::White, dim: true)
                            Text(content: format!(" {}/{} modules", modules_done, total_modules),
                                color: Color::Cyan)
                        }
                    }
                });
            }

            // Current task box
            let task_box = Element::node::<Box>(
                BoxProps {
                    border_style: BorderStyle::Single,
                    padding: 1.0,
                    margin_top: Some(1.0),
                    ..Default::default()
                },
                task_box_children,
            );

            children.push(task_box);

            // Footer
            children.push(Element::text(""));
            children.push(element! {
                Text(content: "Press Ctrl+C to cancel", dim: true, italic: true)
            });

            // Root container
            let ui = Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    width: Some(60.0),
                    ..Default::default()
                },
                children,
            );

            blaeck.render(ui)?;
            thread::sleep(Duration::from_millis(100));
        }

        // Task completed - add to completed list
        completed.push((task_name, *task_duration));
    }

    // === Final Success State ===
    let total_time: f32 = tasks.iter().map(|(_, d)| d).sum();

    let mut final_children: Vec<Element> = Vec::new();

    // All completed tasks
    for (name, dur) in &completed {
        final_children.push(element! {
            Box(flex_direction: FlexDirection::Row) {
                Text(content: format!("✓ {}", name), color: Color::Green)
                Spacer
                Text(content: format_duration(*dur), color: Color::Green, dim: true)
            }
        });
    }

    final_children.push(Element::text(""));

    // Success box with double border
    final_children.push(element! {
        Box(border_style: BorderStyle::Double, padding: 1.0) {
            Text(content: "✓ All tasks completed successfully!",
                color: Color::Green, bold: true)
            Text(content: "")
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Total time:", dim: true)
                Spacer
                Text(content: format_duration(total_time), color: Color::Cyan, bold: true)
            }
        }
    });

    let ui = Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            width: Some(60.0),
            ..Default::default()
        },
        final_children,
    );

    blaeck.render(ui)?;
    blaeck.unmount()?;

    println!("\n📦 Build artifacts written to ./target/release/");
    Ok(())
}
