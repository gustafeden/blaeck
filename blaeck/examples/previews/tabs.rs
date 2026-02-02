use blaeck::prelude::*;

pub const TABS: &[&str] = &["Home", "Profile", "Settings", "Help"];

pub const CONTENTS: &[&str] = &[
    "Welcome to the Home tab!\n\nThis is where you start.",
    "User Profile\n\nName: Alice\nEmail: alice@example.com",
    "Settings\n\n[x] Dark mode\n[ ] Notifications\n[x] Auto-save",
    "Help & Support\n\nPress ←/→ to navigate tabs\nPress q to quit",
];

pub fn build_ui_with_state(tabs: &[&str], contents: &[&str], state: &TabsState) -> Element {
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
                    content: "Tabs Component Demo".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            Element::node::<Tabs>(
                TabsProps::new(tabs.to_vec())
                    .selected(state.selected)
                    .selected_color(Color::Cyan)
                    .unselected_color(Color::White)
                    .divider(TabDivider::Line)
                    .divider_color(Color::DarkGray),
                vec![],
            ),
            Element::text(""),
            Element::node::<Divider>(DividerProps::new().width(40).color(Color::DarkGray), vec![]),
            Element::text(""),
            Element::node::<Box>(
                BoxProps {
                    padding: 1.0,
                    ..Default::default()
                },
                vec![Element::node::<Text>(
                    TextProps {
                        content: contents[state.selected].into(),
                        ..Default::default()
                    },
                    vec![],
                )],
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
                    content: "←/→ switch tabs • q quit".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
        ],
    )
}

pub fn build_ui() -> Element {
    let state = TabsState::new(TABS.len());
    build_ui_with_state(TABS, CONTENTS, &state)
}
