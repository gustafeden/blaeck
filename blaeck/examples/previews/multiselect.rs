use blaeck::prelude::*;

pub const ITEMS: &[&str] = &[
    "Rust",
    "Python",
    "JavaScript",
    "TypeScript",
    "Go",
    "Java",
    "C++",
    "Ruby",
];

pub fn build_ui_with_state(items: &[&str], state: &MultiSelectState) -> Element {
    let props = MultiSelectProps::new(items.to_vec())
        .cursor(state.cursor)
        .selected(state.selected.clone())
        .scroll_offset(state.scroll_offset)
        .cursor_color(Color::Cyan)
        .selected_color(Color::Green)
        .style(MultiSelectStyle::Bracket);

    let selected_count = state.selected_count();
    let total = items.len();

    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            border_style: BorderStyle::Round,
            ..Default::default()
        },
        vec![
            Element::node::<Text>(
                TextProps {
                    content: "MultiSelect Demo".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            Element::node::<Text>(
                TextProps {
                    content: "Select your favorite languages:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            Element::node::<MultiSelect>(props, vec![]),
            Element::text(""),
            Element::node::<Text>(
                TextProps {
                    content: format!("{}/{} selected", selected_count, total),
                    color: Some(Color::DarkGray),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            Element::node::<Divider>(
                DividerProps::new()
                    .width(40)
                    .line_style(DividerStyle::Dashed)
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: "↑/↓ move • Space toggle • a all • Enter done".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
        ],
    )
}

pub fn build_ui() -> Element {
    let state = MultiSelectState::new(ITEMS.len());
    build_ui_with_state(ITEMS, &state)
}
