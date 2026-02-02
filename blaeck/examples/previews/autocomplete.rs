use blaeck::prelude::*;

pub const SUGGESTIONS: &[&str] = &[
    "Rust",
    "Python",
    "JavaScript",
    "TypeScript",
    "Go",
    "Java",
    "C++",
    "C#",
    "Ruby",
    "Swift",
    "Kotlin",
    "Scala",
    "Haskell",
    "Elixir",
    "Clojure",
];

pub fn build_ui_with_state(suggestions: &[&str], state: &AutocompleteState) -> Element {
    let props = AutocompleteProps::new(suggestions.to_vec())
        .input(&state.input)
        .cursor(state.cursor)
        .selected(state.selected)
        .placeholder("Search languages...")
        .filter_mode(FilterMode::Contains)
        .max_suggestions(6)
        .selected_color(Color::Cyan);

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
                    content: "Autocomplete Demo".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            Element::node::<Text>(
                TextProps {
                    content: "Type to filter programming languages:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            Element::node::<Autocomplete>(props, vec![]),
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
                    content: "↑/↓ navigate • Enter/Tab select • Esc quit".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
        ],
    )
}

pub fn build_ui() -> Element {
    let state = AutocompleteState::new();
    build_ui_with_state(SUGGESTIONS, &state)
}
