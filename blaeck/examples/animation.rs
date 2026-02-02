//! Animation example - Demonstrates animation utilities
//!
//! Run with: cargo run --example animation

#[path = "previews/mod.rs"]
mod previews;

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let timer = AnimationTimer::new();

    // Run animation for 5 seconds
    while timer.elapsed_ms() < 5000 {
        blaeck.render(previews::animation::build_ui_with_timer(&timer))?;
        thread::sleep(Duration::from_millis(50)); // 20 FPS
    }

    // Final state
    let final_ui = Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            border_style: BorderStyle::Round,
            border_color: Some(Color::Green),
            ..Default::default()
        },
        vec![Element::node::<Text>(
            TextProps {
                content: "● Animation complete!".into(),
                color: Some(Color::Green),
                bold: true,
                ..Default::default()
            },
            vec![],
        )],
    );

    blaeck.render(final_ui)?;
    blaeck.unmount()?;

    Ok(())
}
