use blaeck::prelude::*;

/// Animated preview — cycles through the build progress like the real example.
pub fn build_ui_with_timer(timer: &AnimationTimer) -> Element {
    // Each step is 200ms, 11 steps total, then loop
    let cycle_ms = 11 * 200;
    let elapsed = timer.elapsed_ms() as usize;
    let i = ((elapsed % cycle_ms) / 200).min(10);

    build_ui_frame(i)
}

/// Render a single frame at a given step (0..=10).
pub fn build_ui_frame(i: usize) -> Element {
    let progress = i as f32 / 10.0;
    let filled = (progress * 20.0) as usize;
    let empty = 20 - filled;
    let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));

    let status = match i {
        0..=3 => "Compiling dependencies...",
        4..=6 => "Building project...",
        7..=9 => "Linking...",
        _ => "Done!",
    };

    element! {
        Box(border_style: BorderStyle::Round, padding: 1.0) {
            Text(content: status, bold: true)
            Box(flex_direction: FlexDirection::Row, gap: 1.0) {
                Text(content: bar, color: Color::Green)
                Text(content: format!("{}%", i * 10), dim: true)
            }
        }
    }
}
