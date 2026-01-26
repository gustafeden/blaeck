//! Reactive Timeline Example
//!
//! Demonstrates the `use_timeline` hook for declarative animations
//! in the reactive system.
//!
//! Controls:
//! - Space: Pause/resume
//! - r: Restart
//! - Left/Right: Seek backward/forward
//! - Up/Down: Speed up/slow down
//! - q/Esc: Quit

use blaeck::animation::Easing;
use blaeck::prelude::*;
use blaeck::reactive::*;

fn animated_dashboard(cx: Scope) -> Element {
    // Define the animation sequence
    let timeline_def = Timeline::new()
        // Phase 1: Fade in
        .act(
            Act::new("fade_in")
                .duration(1.0)
                .animate("opacity", 0.0f64, 1.0, Easing::EaseOutCubic)
                .animate("bar_width", 0.0f64, 100.0, Easing::EaseOutCubic),
        )
        // Phase 2: Hold
        .act(
            Act::new("hold")
                .duration(2.0)
                .animate("opacity", 1.0f64, 1.0, Easing::Linear)
                .animate("bar_width", 100.0f64, 100.0, Easing::Linear),
        )
        // Phase 3: Pulse animation using keyframes
        .act(
            Act::new("pulse")
                .duration(1.5)
                .track(
                    "opacity",
                    Track::new()
                        .keyframe(0.0, 1.0f64, Easing::Linear)
                        .keyframe(0.25, 0.5, Easing::EaseInOutCubic)
                        .keyframe(0.5, 1.0, Easing::EaseInOutCubic)
                        .keyframe(0.75, 0.5, Easing::EaseInOutCubic)
                        .keyframe(1.0, 1.0, Easing::EaseInOutCubic),
                )
                .animate("bar_width", 100.0f64, 80.0, Easing::EaseInOutCubic),
        )
        // Phase 4: Fade out
        .act(
            Act::new("fade_out")
                .duration(1.0)
                .animate("opacity", 1.0f64, 0.3, Easing::EaseInCubic)
                .animate("bar_width", 80.0f64, 20.0, Easing::EaseInCubic),
        )
        // Loop back to fade_in
        .loop_from("fade_in");

    // Create the timeline using the hook
    let timeline = use_timeline(cx.clone(), timeline_def);

    // Clone for input handler
    let tl = timeline.clone();
    use_input(cx, move |key| {
        use crossterm::event::KeyCode;
        match key.code {
            KeyCode::Char(' ') => tl.toggle_pause(),
            KeyCode::Char('r') => tl.restart(),
            KeyCode::Left => tl.seek((tl.elapsed() - 0.5).max(0.0)),
            KeyCode::Right => tl.seek(tl.elapsed() + 0.5),
            KeyCode::Up => tl.set_speed((tl.speed() + 0.25).min(4.0)),
            KeyCode::Down => tl.set_speed((tl.speed() - 0.25).max(0.25)),
            _ => {}
        }
    });

    // Get animated values
    let opacity = timeline.get_or("opacity", 1.0f64);
    let bar_width = timeline.get_or("bar_width", 0.0f64);

    // Timeline state for display
    let act_name = timeline.current_act();
    let elapsed = timeline.elapsed();
    let speed = timeline.speed();
    let paused = timeline.is_paused();

    // Build the UI
    let brightness = (opacity * 255.0) as u8;
    let text_color = Color::Rgb(brightness, brightness, brightness);

    let bar_len = (bar_width / 100.0 * 40.0) as usize;
    let bar = format!("[{}{}]", "█".repeat(bar_len), "░".repeat(40 - bar_len));

    let status = if paused { "⏸ PAUSED" } else { "▶ PLAYING" };
    let status_color = if paused { Color::Yellow } else { Color::Green };

    element! {
        Box(flex_direction: FlexDirection::Column, padding: 1.0, border_style: BorderStyle::Round) {
            // Title
            Text(content: "Reactive Timeline Demo", bold: true, color: Color::Cyan)
            Newline

            // Timeline state
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Act: ", dim: true)
                Text(content: act_name, color: Color::Magenta, bold: true)
                Text(content: format!("  Time: {:.2}s  Speed: {:.2}x", elapsed, speed), dim: true)
            }
            Newline

            // Animated progress bar
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Progress: ", dim: true)
                Text(content: bar, color: text_color)
            }

            // Opacity indicator
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Opacity:  ", dim: true)
                Text(content: format!("{:.0}%", opacity * 100.0), color: text_color, bold: true)
            }
            Newline

            // Animated text sample
            Text(content: "This text brightness follows opacity", color: text_color)
            Newline

            // Status and controls
            Text(content: status, color: status_color, bold: true)
            Newline
            Text(content: "Space: pause | r: restart | ←/→: seek | ↑/↓: speed | q: quit", dim: true)
        }
    }
}

fn main() -> std::io::Result<()> {
    ReactiveApp::run(animated_dashboard)?;
    Ok(())
}
