use blaeck::prelude::*;

const DOTS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const BREATH: &[&str] = &["·", "•", "*", "✱", "*", "•"];
const ARROWS: &[&str] = &["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"];
const BOUNCE: &[&str] = &["⠁", "⠂", "⠄", "⠂"];
const CLOCK: &[&str] = &[
    "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛",
];
const MOON: &[&str] = &["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"];

fn get_frame<'a>(frames: &'a [&'a str], elapsed_ms: u64, interval_ms: u64) -> &'a str {
    let idx = (elapsed_ms / interval_ms) as usize % frames.len();
    frames[idx]
}

/// Live spinner showcase — all spinners animate via the timer.
pub fn build_ui_with_timer(timer: &AnimationTimer) -> Element {
    let elapsed_ms = timer.elapsed_ms() as u64;
    let dots = get_frame(DOTS, elapsed_ms, 80);
    let breath = get_frame(BREATH, elapsed_ms, 150);
    let arrows = get_frame(ARROWS, elapsed_ms, 100);
    let bounce = get_frame(BOUNCE, elapsed_ms, 120);
    let clock = get_frame(CLOCK, elapsed_ms, 200);
    let moon = get_frame(MOON, elapsed_ms, 150);
    let elapsed_secs = elapsed_ms as f32 / 1000.0;

    element! {
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
            Text(content: format!("Elapsed: {:.1}s", elapsed_secs), dim: true)
            Text(content: "Press Ctrl+C to exit", dim: true, italic: true)
        }
    }
}
