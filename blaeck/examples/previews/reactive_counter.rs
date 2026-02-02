use blaeck::element;
use blaeck::prelude::*;
use blaeck::reactive::*;

/// The reactive component — used by both the standalone example and the viewer.
pub fn counter(cx: Scope) -> Element {
    let count = use_state(cx.clone(), || 0);

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

/// Static snapshot for the example viewer preview panel.
pub fn build_ui() -> Element {
    element! {
        Box(flex_direction: FlexDirection::Column, padding: 1.0, border_style: BorderStyle::Round) {
            Text(content: "Reactive Counter", bold: true, color: Color::Cyan)
            Spacer
            Text(content: "Count: 0", color: Color::Green)
            Spacer
            Text(content: "Press SPACE to increment, 'r' to reset, 'q' to quit", dim: true)
        }
    }
}
