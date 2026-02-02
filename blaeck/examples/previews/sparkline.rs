use blaeck::prelude::*;

pub fn build_ui_with_data(
    cpu_data: &[f64],
    cpu_current: f64,
    mem_data: &[f64],
    mem_current: f64,
    net_data: &[f64],
    net_current: f64,
    audio_data: &[f64],
) -> Element {
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
                    content: "Live Sparklines".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            // CPU
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "CPU:  ".into(),
                            color: Some(Color::DarkGray),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Sparkline>(
                        SparklineProps::new(cpu_data.to_vec())
                            .range(0.0, 100.0)
                            .color(Color::Green),
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: format!(" {:>3.0}%", cpu_current),
                            color: Some(Color::Green),
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            ),
            Element::text(""),
            // Memory
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "MEM:  ".into(),
                            color: Some(Color::DarkGray),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Sparkline>(
                        SparklineProps::new(mem_data.to_vec())
                            .range(0.0, 100.0)
                            .color(Color::Yellow),
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: format!(" {:>3.0}%", mem_current),
                            color: Some(Color::Yellow),
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            ),
            Element::text(""),
            // Network
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "NET:  ".into(),
                            color: Some(Color::DarkGray),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Sparkline>(
                        SparklineProps::new(net_data.to_vec())
                            .range(0.0, 100.0)
                            .color(Color::Cyan),
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: format!(" {:>3.0} MB/s", net_current),
                            color: Some(Color::Cyan),
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            ),
            Element::text(""),
            // Audio
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "AUDIO:".into(),
                            color: Some(Color::DarkGray),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Sparkline>(
                        SparklineProps::new(audio_data.to_vec())
                            .range(0.0, 100.0)
                            .color(Color::Magenta),
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: " ♪♫".into(),
                            color: Some(Color::Magenta),
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            ),
            Element::text(""),
            // Styles demo
            Element::node::<Divider>(
                DividerProps::new()
                    .width(35)
                    .line_style(DividerStyle::Dashed)
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: "Styles:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            // Block style
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Block: ".into(),
                            color: Some(Color::DarkGray),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Sparkline>(
                        SparklineProps::new(vec![1, 2, 3, 5, 8, 5, 3, 2, 1, 2, 3, 5])
                            .style(SparklineStyle::Block)
                            .color(Color::Magenta),
                        vec![],
                    ),
                ],
            ),
            // Dot style
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Dot:   ".into(),
                            color: Some(Color::DarkGray),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Sparkline>(
                        SparklineProps::new(vec![1, 2, 3, 5, 8, 5, 3, 2, 1, 2, 3, 5])
                            .style(SparklineStyle::Dot)
                            .color(Color::Blue),
                        vec![],
                    ),
                ],
            ),
            Element::text(""),
            Element::node::<Text>(
                TextProps {
                    content: "Press q to quit".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
        ],
    )
}

pub fn build_ui() -> Element {
    // Static sample data for the viewer preview
    let cpu_data = vec![
        30.0, 35.0, 40.0, 38.0, 42.0, 45.0, 50.0, 48.0, 52.0, 55.0, 53.0, 50.0, 47.0, 44.0, 48.0,
        52.0, 56.0, 60.0, 58.0, 55.0,
    ];
    let mem_data = vec![
        45.0, 46.0, 47.0, 48.0, 49.0, 50.0, 51.0, 52.0, 53.0, 54.0, 55.0, 56.0, 57.0, 58.0, 59.0,
        60.0, 61.0, 62.0, 63.0, 64.0,
    ];
    let net_data = vec![
        10.0, 15.0, 8.0, 20.0, 12.0, 25.0, 18.0, 30.0, 22.0, 35.0, 28.0, 40.0, 32.0, 45.0, 38.0,
        50.0, 42.0, 55.0, 48.0, 60.0,
    ];
    let audio_data = vec![
        80.0, 45.0, 60.0, 30.0, 90.0, 50.0, 70.0, 40.0, 85.0, 55.0, 65.0, 35.0, 75.0, 48.0, 88.0,
        42.0, 72.0, 58.0, 82.0, 38.0, 68.0, 52.0, 78.0, 46.0,
    ];
    build_ui_with_data(
        &cpu_data,
        55.0,
        &mem_data,
        64.0,
        &net_data,
        60.0,
        &audio_data,
    )
}
