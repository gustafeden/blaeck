//! Countdown - Animated countdown timer with big digits
//!
//! Run with: cargo run --example countdown

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::time::{Duration, Instant};
use std::{io, thread};

/// ASCII art digits (3 lines tall)
const DIGITS: &[&[&str]] = &[
    // 0
    &[" ██████╗ ", "██╔═══██╗", "╚██████╔╝"],
    // 1
    &["   ██╗   ", "   ██║   ", "   ██║   "],
    // 2
    &["██████╗  ", "╚════██╗ ", "██████╔╝ "],
    // 3
    &["██████╗  ", " █████╗  ", "██████╔╝ "],
    // 4
    &["██╗  ██╗ ", "╚████╔╝  ", "   ██║   "],
    // 5
    &["███████╗ ", "██████╗  ", "╚█████╔╝ "],
    // 6
    &["██████╗  ", "██████╗  ", "╚█████╔╝ "],
    // 7
    &["███████╗ ", "    ██╔╝ ", "   ██╔╝  "],
    // 8
    &[" █████╗  ", "██████╗  ", "╚█████╔╝ "],
    // 9
    &[" █████╗  ", "╚█████╗  ", " █████╔╝ "],
];

const COLON: &[&str] = &["   ", " ● ", " ● "];

fn render_big_number(seconds: u32) -> Vec<Element> {
    let mins = seconds / 60;
    let secs = seconds % 60;

    let d1 = (mins / 10) as usize % 10;
    let d2 = (mins % 10) as usize;
    let d3 = (secs / 10) as usize;
    let d4 = (secs % 10) as usize;

    let mut lines: Vec<Element> = Vec::new();

    #[allow(clippy::needless_range_loop)]
    for row in 0..3 {
        let line = format!(
            "{}{}{}{}{}",
            DIGITS[d1][row], DIGITS[d2][row], COLON[row], DIGITS[d3][row], DIGITS[d4][row]
        );
        lines.push(element! {
            Text(content: line, color: Color::Cyan, bold: true)
        });
    }

    lines
}

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let countdown_from: u32 = 10; // 10 seconds
    let start = Instant::now();

    loop {
        let elapsed = start.elapsed().as_secs() as u32;
        let remaining = countdown_from.saturating_sub(elapsed);

        // Build digit display
        let digit_lines = render_big_number(remaining);

        // Progress bar
        let progress = ((countdown_from - remaining) as f32 / countdown_from as f32 * 100.0) as u32;
        let bar_width = 40;
        let filled = (bar_width * progress as usize) / 100;
        let empty = bar_width - filled;
        let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));

        let color = if remaining <= 3 {
            Color::Red
        } else if remaining <= 5 {
            Color::Yellow
        } else {
            Color::Green
        };

        let mut children: Vec<Element> = vec![
            element! {
                Text(content: "Countdown Timer", bold: true, color: Color::White)
            },
            Element::text(""),
        ];

        // Add big digits
        children.extend(digit_lines);

        children.push(Element::text(""));
        children.push(element! {
            Text(content: bar, color: color)
        });
        children.push(Element::text(""));

        // Status message
        let status = if remaining == 0 {
            element! {
                Box(border_style: BorderStyle::Double, padding: 1.0) {
                    Text(content: "🎉 TIME'S UP! 🎉", color: Color::Green, bold: true)
                }
            }
        } else {
            element! {
                Text(content: format!("{} seconds remaining", remaining),
                    color: color, dim: true)
            }
        };
        children.push(status);

        let ui = Element::node::<Box>(
            BoxProps {
                flex_direction: FlexDirection::Column,
                padding: 1.0,
                border_style: BorderStyle::Round,
                ..Default::default()
            },
            children,
        );

        blaeck.render(ui)?;

        if remaining == 0 {
            thread::sleep(Duration::from_secs(2));
            break;
        }

        thread::sleep(Duration::from_millis(100));
    }

    blaeck.unmount()?;
    println!("Countdown complete!");
    Ok(())
}
