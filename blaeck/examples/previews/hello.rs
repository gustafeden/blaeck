use blaeck::prelude::*;

pub fn build_ui() -> Element {
    element! {
        Box(padding: 1.0, border_style: BorderStyle::Single) {
            Text(content: "Hello from Blaeck!", color: Color::Green, bold: true)
            Text(content: "Inline rendering - no fullscreen!")
        }
    }
}
