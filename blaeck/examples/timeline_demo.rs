//! Timeline Animation Demo
//!
//! Demonstrates the declarative timeline animation system.
//!
//! Controls:
//! - Space: Pause/resume
//! - r: Restart
//! - q/Esc: Quit

use blaeck::prelude::*;
use blaeck::animation::Easing;
use crossterm::event::{poll, read, Event, KeyCode};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // Build a timeline with multiple acts
    let timeline = Timeline::new()
        // Act 1: Fade in
        .act(Act::new("fade_in")
            .duration(1.5)
            .animate("opacity", 0.0f64, 1.0, Easing::EaseOutCubic)
            .animate("scale", 0.5f64, 1.0, Easing::EaseOutElastic))

        // Act 2: Hold
        .act(Act::new("hold")
            .duration(2.0)
            .animate("opacity", 1.0f64, 1.0, Easing::Linear) // Stay at 1.0
            .animate("scale", 1.0f64, 1.0, Easing::Linear))

        // Act 3: Pulse (using keyframes for more control)
        .act(Act::new("pulse")
            .duration(1.0)
            .track("opacity", Track::new()
                .keyframe(0.0, 1.0f64, Easing::Linear)
                .keyframe(0.5, 0.6, Easing::EaseInOutCubic)
                .keyframe(1.0, 1.0, Easing::EaseInOutCubic))
            .animate("scale", 1.0f64, 1.1, Easing::EaseInOutCubic))

        // Act 4: Fade out
        .act(Act::new("fade_out")
            .duration(1.5)
            .animate("opacity", 1.0f64, 0.0, Easing::EaseInCubic)
            .animate("scale", 1.0f64, 0.8, Easing::EaseInCubic))

        // Loop from fade_in
        .loop_from("fade_in");

    let mut playing = timeline.start();
    let mut blaeck = Blaeck::new(std::io::stdout())?;

    crossterm::terminal::enable_raw_mode()?;

    loop {
        // Handle input
        if poll(Duration::from_millis(16))? {
            match read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => break,
                    KeyCode::Char(' ') => playing.toggle_pause(),
                    KeyCode::Char('r') => playing.restart(),
                    _ => {}
                },
                _ => {}
            }
        }

        // Get animated values
        let opacity = playing.get_or("opacity", 1.0f64);
        let scale = playing.get_or("scale", 1.0f64);
        let act_name = playing.current_act();
        let elapsed = playing.elapsed();

        // Build the UI
        let ui = build_ui(opacity, scale, &act_name, elapsed, playing.is_paused());
        blaeck.render(ui)?;
    }

    crossterm::terminal::disable_raw_mode()?;
    blaeck.unmount()?;

    Ok(())
}

fn build_ui(opacity: f64, scale: f64, act_name: &str, elapsed: f64, paused: bool) -> Element {
    // Convert opacity to a visual bar
    let bar_width = 30;
    let filled = (opacity * bar_width as f64) as usize;
    let bar = format!(
        "[{}{}]",
        "█".repeat(filled),
        "░".repeat(bar_width - filled)
    );

    // Scale visualization
    let scale_width = (scale * 20.0) as usize;
    let scale_bar = "▓".repeat(scale_width);

    // Opacity affects text color
    let text_brightness = (opacity * 255.0) as u8;
    let text_color = Color::Rgb(text_brightness, text_brightness, text_brightness);

    // Status indicator
    let status = if paused { "⏸ PAUSED" } else { "▶ PLAYING" };
    let status_color = if paused { Color::Yellow } else { Color::Green };

    element! {
        Box(flex_direction: FlexDirection::Column, padding: 1.0, border_style: BorderStyle::Round) {
            // Title
            Text(content: "Timeline Animation Demo", bold: true, color: Color::Cyan)
            Newline

            // Current act
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Act: ", dim: true)
                Text(content: act_name.to_string(), color: Color::Magenta, bold: true)
                Text(content: format!("  ({:.2}s)", elapsed), dim: true)
            }
            Newline

            // Opacity bar
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Opacity: ", dim: true)
                Text(content: bar, color: text_color)
                Text(content: format!(" {:.0}%", opacity * 100.0), color: text_color)
            }

            // Scale bar
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Scale:   ", dim: true)
                Text(content: format!("[{:<20}]", scale_bar), color: Color::Blue)
                Text(content: format!(" {:.2}x", scale), color: Color::Blue)
            }
            Newline

            // Animated text sample
            Text(content: "Animated Text", color: text_color, bold: true)
            Newline

            // Status and controls
            Box(flex_direction: FlexDirection::Row) {
                Text(content: status, color: status_color)
            }
            Text(content: "Space: pause | r: restart | q: quit", dim: true)
        }
    }
}
