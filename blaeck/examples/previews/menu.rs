use blaeck::prelude::*;
use blaeck::{SelectIndicator, SelectItem, SelectProps, SelectState};

pub fn build_ui_with_state(items: &[SelectItem], state: &SelectState) -> Element {
    let props = SelectProps::new(items.to_vec())
        .indicator(SelectIndicator::Arrow)
        .selected_color(Color::Cyan)
        .selected(state.selected)
        .scroll_offset(state.scroll_offset);

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
                    content: "Main Menu".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            Element::node::<Divider>(
                DividerProps::new().width(30).color(Color::DarkGray),
                vec![],
            ),
            Element::text(""),
            Element::node::<Select>(props, vec![]),
            Element::text(""),
            Element::node::<Divider>(
                DividerProps::new().width(30).color(Color::DarkGray),
                vec![],
            ),
            Element::text(""),
            Element::node::<Text>(
                TextProps {
                    content: "↑/↓ navigate • Enter select • q quit".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
        ],
    )
}

pub fn default_items() -> Vec<SelectItem> {
    vec![
        SelectItem::new("New Project").value("new"),
        SelectItem::new("Open Recent").value("open"),
        SelectItem::new("Settings").value("settings"),
        SelectItem::new("Help").value("help"),
        SelectItem::new("Quit").value("quit"),
    ]
}

pub fn build_ui() -> Element {
    let items = default_items();
    let state = SelectState::new(items.len());
    build_ui_with_state(&items, &state)
}
