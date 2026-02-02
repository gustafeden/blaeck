use blaeck::prelude::*;

/// Render the timeline animation UI given animated values.
pub fn render(opacity: f64, scale: f64, act_name: &str, elapsed: f64, paused: bool) -> Element {
    let bar_width = 30;
    let filled = (opacity * bar_width as f64) as usize;
    let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(bar_width - filled));

    let scale_width = (scale * 20.0) as usize;
    let scale_bar = "▓".repeat(scale_width);

    let text_brightness = (opacity * 255.0) as u8;
    let text_color = Color::Rgb(text_brightness, text_brightness, text_brightness);

    let status = if paused { "⏸ PAUSED" } else { "▶ PLAYING" };
    let status_color = if paused { Color::Yellow } else { Color::Green };

    element! {
        Box(flex_direction: FlexDirection::Column, padding: 1.0, border_style: BorderStyle::Round) {
            Text(content: "Timeline Animation Demo", bold: true, color: Color::Cyan)
            Newline

            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Act: ", dim: true)
                Text(content: act_name.to_string(), color: Color::Magenta, bold: true)
                Text(content: format!("  ({:.2}s)", elapsed), dim: true)
            }
            Newline

            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Opacity: ", dim: true)
                Text(content: bar, color: text_color)
                Text(content: format!(" {:.0}%", opacity * 100.0), color: text_color)
            }

            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Scale:   ", dim: true)
                Text(content: format!("[{:<20}]", scale_bar), color: Color::Blue)
                Text(content: format!(" {:.2}x", scale), color: Color::Blue)
            }
            Newline

            Text(content: "Animated Text", color: text_color, bold: true)
            Newline

            Box(flex_direction: FlexDirection::Row) {
                Text(content: status, color: status_color)
            }
            Text(content: "Space: pause | r: restart | q: quit", dim: true)
        }
    }
}

/// Animated preview — simulates timeline cycling through acts.
pub fn build_ui_with_timer(timer: &AnimationTimer) -> Element {
    let elapsed_ms = timer.elapsed_ms() as f64;
    // Simulate a 6s cycle: fade_in(1.5) + hold(2.0) + pulse(1.0) + fade_out(1.5)
    let cycle_s = 6.0;
    let t = (elapsed_ms / 1000.0) % cycle_s;

    let (opacity, scale, act_name, act_elapsed) = if t < 1.5 {
        // fade_in
        let p = t / 1.5;
        (p, 0.5 + p * 0.5, "fade_in", t)
    } else if t < 3.5 {
        // hold
        (1.0, 1.0, "hold", t - 1.5)
    } else if t < 4.5 {
        // pulse
        let p = (t - 3.5) / 1.0;
        let opacity = 1.0 - 0.4 * (p * std::f64::consts::PI).sin();
        (
            opacity,
            1.0 + 0.1 * (p * std::f64::consts::PI).sin(),
            "pulse",
            t - 3.5,
        )
    } else {
        // fade_out
        let p = (t - 4.5) / 1.5;
        (1.0 - p, 1.0 - 0.2 * p, "fade_out", t - 4.5)
    };

    render(opacity, scale, act_name, act_elapsed, false)
}
