//! Reactive counter example demonstrating use_state and use_input.
//!
//! Run with: cargo run --example reactive_counter
//!
//! Controls:
//! - Space: Increment counter
//! - 'r': Reset counter
//! - 'q' or Ctrl+C: Exit

use blaeck::element;
use blaeck::prelude::*;
use blaeck::reactive::*;
use std::io;

fn counter(cx: Scope) -> Element {
    let count = use_state(cx.clone(), || 0);

    // Clone the signal for use in the input handler
    let count_handler = count.clone();
    use_input(cx, move |key| {
        if key.is_char(' ') {
            count_handler.set(count_handler.get() + 1);
        } else if key.is_char('r') {
            count_handler.set(0);
        }
    });

    element! {
        Box(flex_direction: FlexDirection::Column, padding: 1.0, border_style: BorderStyle::Round) {
            Text(content: "Reactive Counter", bold: true, color: Color::Cyan)
            Spacer
            Text(content: format!("Count: {}", count.get()), color: Color::Green)
            Spacer
            Text(content: "Press SPACE to increment, 'r' to reset, 'q' to quit", dim: true)
        }
    }
}

fn main() -> io::Result<()> {
    ReactiveApp::run(counter)?;
    Ok(())
}
