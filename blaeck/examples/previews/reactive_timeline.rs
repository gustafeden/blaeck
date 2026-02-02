use blaeck::animation::Easing;
use blaeck::prelude::*;
use blaeck::reactive::*;

/// The reactive component — used by both the standalone example and the viewer.
pub fn animated_dashboard(cx: Scope) -> Element {
    let timeline_def = Timeline::new()
        .act(
            Act::new("fade_in")
                .duration(1.0)
                .animate("opacity", 0.0f64, 1.0, Easing::EaseOutCubic)
                .animate("bar_width", 0.0f64, 100.0, Easing::EaseOutCubic),
        )
        .act(
            Act::new("hold")
                .duration(2.0)
                .animate("opacity", 1.0f64, 1.0, Easing::Linear)
                .animate("bar_width", 100.0f64, 100.0, Easing::Linear),
        )
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
        .act(
            Act::new("fade_out")
                .duration(1.0)
                .animate("opacity", 1.0f64, 0.3, Easing::EaseInCubic)
                .animate("bar_width", 80.0f64, 20.0, Easing::EaseInCubic),
        )
        .loop_from("fade_in");

    let timeline = use_timeline(cx.clone(), timeline_def);

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

    let opacity = timeline.get_or("opacity", 1.0f64);
    let bar_width = timeline.get_or("bar_width", 0.0f64);
    let act_name = timeline.current_act();
    let elapsed = timeline.elapsed();
    let speed = timeline.speed();
    let paused = timeline.is_paused();

    render_dashboard(opacity, bar_width, &act_name, elapsed, speed, paused)
}

/// Render the dashboard UI given animated values.
pub fn render_dashboard(
    opacity: f64,
    bar_width: f64,
    act_name: &str,
    elapsed: f64,
    speed: f64,
    paused: bool,
) -> Element {
    let brightness = (opacity * 255.0) as u8;
    let text_color = Color::Rgb(brightness, brightness, brightness);

    let bar_len = (bar_width / 100.0 * 40.0) as usize;
    let bar = format!("[{}{}]", "█".repeat(bar_len), "░".repeat(40 - bar_len));

    let status = if paused { "⏸ PAUSED" } else { "▶ PLAYING" };
    let status_color = if paused { Color::Yellow } else { Color::Green };

    element! {
        Box(flex_direction: FlexDirection::Column, padding: 1.0, border_style: BorderStyle::Round) {
            Text(content: "Reactive Timeline Demo", bold: true, color: Color::Cyan)
            Newline

            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Act: ", dim: true)
                Text(content: act_name.to_string(), color: Color::Magenta, bold: true)
                Text(content: format!("  Time: {:.2}s  Speed: {:.2}x", elapsed, speed), dim: true)
            }
            Newline

            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Progress: ", dim: true)
                Text(content: bar, color: text_color)
            }

            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Opacity:  ", dim: true)
                Text(content: format!("{:.0}%", opacity * 100.0), color: text_color, bold: true)
            }
            Newline

            Text(content: "This text brightness follows opacity", color: text_color)
            Newline

            Text(content: status, color: status_color, bold: true)
            Newline
            Text(content: "Space: pause | r: restart | ←/→: seek | ↑/↓: speed | q: quit", dim: true)
        }
    }
}

/// Static snapshot for the example viewer preview panel.
pub fn build_ui() -> Element {
    render_dashboard(0.8, 80.0, "hold", 2.5, 1.0, false)
}
