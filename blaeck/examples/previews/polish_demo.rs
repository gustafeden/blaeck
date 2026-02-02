use blaeck::prelude::*;

pub fn build_ui_with_confirm(confirm: &blaeck::ConfirmProps) -> Element {
    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            border_style: BorderStyle::Round,
            ..Default::default()
        },
        vec![
            // Title
            Element::node::<Text>(
                TextProps {
                    content: "Blaeck Polish Features Demo".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            // Dividers Section
            Element::node::<Divider>(
                DividerProps::new()
                    .label("Divider Styles")
                    .color(Color::Yellow),
                vec![],
            ),
            Element::text(""),
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    padding_left: Some(2.0),
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Single:".into(),
                            dim: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Divider>(
                        DividerProps::new()
                            .width(30)
                            .line_style(DividerStyle::Single),
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: "Double:".into(),
                            dim: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Divider>(
                        DividerProps::new()
                            .width(30)
                            .line_style(DividerStyle::Double),
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: "Dashed:".into(),
                            dim: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Divider>(
                        DividerProps::new()
                            .width(30)
                            .line_style(DividerStyle::Dashed),
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: "Bold with label:".into(),
                            dim: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Divider>(
                        DividerProps::new()
                            .width(30)
                            .line_style(DividerStyle::Bold)
                            .label("Section")
                            .color(Color::Magenta),
                        vec![],
                    ),
                ],
            ),
            Element::text(""),
            // Badges Section
            Element::node::<Divider>(
                DividerProps::new()
                    .label("Badge Styles")
                    .color(Color::Yellow),
                vec![],
            ),
            Element::text(""),
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    padding_left: Some(2.0),
                    gap: 2.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Badge>(
                        BadgeProps::new("NEW").color(Color::Green).bold(),
                        vec![],
                    ),
                    Element::node::<Badge>(
                        BadgeProps::new("INFO")
                            .color(Color::Cyan)
                            .badge_style(BadgeStyle::Bracket),
                        vec![],
                    ),
                    Element::node::<Badge>(
                        BadgeProps::new("3")
                            .color(Color::Yellow)
                            .badge_style(BadgeStyle::Round),
                        vec![],
                    ),
                    Element::node::<Badge>(
                        BadgeProps::new("tag")
                            .color(Color::Magenta)
                            .badge_style(BadgeStyle::Pill),
                        vec![],
                    ),
                    Element::node::<Badge>(
                        BadgeProps::new("ERROR")
                            .color(Color::White)
                            .bg_color(Color::Red)
                            .badge_style(BadgeStyle::Filled)
                            .bold(),
                        vec![],
                    ),
                ],
            ),
            Element::text(""),
            // Links Section
            Element::node::<Divider>(
                DividerProps::new().label("Links").color(Color::Yellow),
                vec![],
            ),
            Element::text(""),
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    padding_left: Some(2.0),
                    ..Default::default()
                },
                vec![
                    Element::node::<Box>(
                        BoxProps {
                            flex_direction: FlexDirection::Row,
                            ..Default::default()
                        },
                        vec![
                            Element::node::<Text>(
                                TextProps {
                                    content: "Simple link: ".into(),
                                    dim: true,
                                    ..Default::default()
                                },
                                vec![],
                            ),
                            Element::node::<Link>(LinkProps::new("Click me"), vec![]),
                        ],
                    ),
                    Element::node::<Box>(
                        BoxProps {
                            flex_direction: FlexDirection::Row,
                            ..Default::default()
                        },
                        vec![
                            Element::node::<Text>(
                                TextProps {
                                    content: "With URL:    ".into(),
                                    dim: true,
                                    ..Default::default()
                                },
                                vec![],
                            ),
                            Element::node::<Link>(
                                LinkProps::with_url("Rust Lang", "https://rust-lang.org"),
                                vec![],
                            ),
                        ],
                    ),
                    Element::node::<Box>(
                        BoxProps {
                            flex_direction: FlexDirection::Row,
                            ..Default::default()
                        },
                        vec![
                            Element::node::<Text>(
                                TextProps {
                                    content: "Styled:      ".into(),
                                    dim: true,
                                    ..Default::default()
                                },
                                vec![],
                            ),
                            Element::node::<Link>(
                                LinkProps::with_url("GitHub", "https://github.com")
                                    .color(Color::Green)
                                    .bold(),
                                vec![],
                            ),
                        ],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: "(Links are clickable in supporting terminals)".into(),
                            dim: true,
                            italic: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            ),
            Element::text(""),
            // Confirm Button Style Section
            Element::node::<Divider>(
                DividerProps::new()
                    .label("Button-Style Confirm")
                    .color(Color::Yellow),
                vec![],
            ),
            Element::text(""),
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    padding_left: Some(2.0),
                    ..Default::default()
                },
                vec![Element::node::<Confirm>(confirm.clone(), vec![])],
            ),
            Element::text(""),
            // Instructions
            Element::node::<Divider>(DividerProps::new().line_style(DividerStyle::Dotted), vec![]),
            Element::node::<Text>(
                TextProps {
                    content: "Left/Right or Y/N to toggle confirm | Enter to submit | Q to quit"
                        .into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
        ],
    )
}

pub fn build_ui() -> Element {
    let confirm = blaeck::ConfirmProps::new("Delete this file?")
        .labels("Delete", "Cancel")
        .button_style()
        .default_value(false);
    build_ui_with_confirm(&confirm)
}
