//! Spinner Demo - Various animated spinner styles
//!
//! Run with: cargo run --example spinner_demo

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::time::{Duration, Instant};
use std::{io, thread};

/// Different spinner frame sets
const DOTS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const BREATH: &[&str] = &["·", "•", "*", "✱", "*", "•"];
const ARROWS: &[&str] = &["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"];
const BOUNCE: &[&str] = &["⠁", "⠂", "⠄", "⠂"];
const CLOCK: &[&str] = &[
    "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛",
];
const MOON: &[&str] = &["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"];

fn get_frame<'a>(frames: &'a [&'a str], start: Instant, interval_ms: u64) -> &'a str {
    let elapsed_ms = start.elapsed().as_millis() as u64;
    let idx = (elapsed_ms / interval_ms) as usize % frames.len();
    frames[idx]
}

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    // 20 FPS is plenty for spinners - saves CPU
    blaeck.set_max_fps(20);

    let start = Instant::now();

    for _ in 0..100 {
        let dots = get_frame(DOTS, start, 80);
        let breath = get_frame(BREATH, start, 150);
        let arrows = get_frame(ARROWS, start, 100);
        let bounce = get_frame(BOUNCE, start, 120);
        let clock = get_frame(CLOCK, start, 200);
        let moon = get_frame(MOON, start, 150);
        let elapsed = start.elapsed().as_secs_f32();

        let ui = element! {
            Box(flex_direction: FlexDirection::Column, padding: 1.0) {
                Text(content: "Spinner Showcase", bold: true, color: Color::Cyan)
                Text(content: "")

                Box(flex_direction: FlexDirection::Row) {
                    Text(content: format!("{} ", dots), color: Color::Green)
                    Text(content: "Dots spinner (braille)")
                }

                Box(flex_direction: FlexDirection::Row) {
                    Text(content: format!("{} ", breath), color: Color::Yellow)
                    Text(content: "Breathing animation")
                }

                Box(flex_direction: FlexDirection::Row) {
                    Text(content: format!("{} ", arrows), color: Color::Magenta)
                    Text(content: "Arrow rotation")
                }

                Box(flex_direction: FlexDirection::Row) {
                    Text(content: format!("{} ", bounce), color: Color::Blue)
                    Text(content: "Bounce dots")
                }

                Box(flex_direction: FlexDirection::Row) {
                    Text(content: format!("{} ", clock), color: Color::White)
                    Text(content: "Clock (emoji)")
                }

                Box(flex_direction: FlexDirection::Row) {
                    Text(content: format!("{} ", moon), color: Color::White)
                    Text(content: "Moon phases (emoji)")
                }

                Text(content: "")
                Text(content: format!("Elapsed: {:.1}s", elapsed), dim: true)
                Text(content: "Press Ctrl+C to exit", dim: true, italic: true)
            }
        };

        blaeck.render(ui)?;
        thread::sleep(Duration::from_millis(50));
    }

    blaeck.unmount()?;
    println!("Done!");
    Ok(())
}
