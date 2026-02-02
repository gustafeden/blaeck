use blaeck::prelude::*;

pub fn build_ui() -> Element {
    let languages = vec![
        BarData::new("Rust", 92.0).color(Color::Yellow),
        BarData::new("Python", 85.0).color(Color::Blue),
        BarData::new("JavaScript", 78.0).color(Color::Green),
        BarData::new("Go", 72.0).color(Color::Cyan),
        BarData::new("Java", 65.0).color(Color::Red),
    ];

    let disk_usage = vec![
        BarData::new("/dev/sda1", 45.2).color(Color::Green),
        BarData::new("/dev/sda2", 78.5).color(Color::Yellow),
        BarData::new("/dev/sda3", 92.1).color(Color::Red),
    ];

    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            gap: 1.0,
            ..Default::default()
        },
        vec![
            // Title
            Element::node::<Text>(
                TextProps {
                    content: "Bar Chart Demo".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            // Basic bar chart
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single,
                    padding: 1.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Language Satisfaction (%)".into(),
                            bold: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<BarChart>(
                        BarChartProps::new(languages.clone())
                            .max_value(100.0)
                            .bar_width(25)
                            .show_percent()
                            .min_label_width(12),
                        vec![],
                    ),
                ],
            ),
            // With brackets and values
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single,
                    padding: 1.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Disk Usage".into(),
                            bold: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<BarChart>(
                        BarChartProps::new(disk_usage.clone())
                            .max_value(100.0)
                            .bar_width(20)
                            .brackets(true)
                            .value_format(ValueFormat::PercentDecimal)
                            .min_label_width(10),
                        vec![],
                    ),
                ],
            ),
            // Different styles
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single,
                    padding: 1.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Bar Styles".into(),
                            bold: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Box>(
                        BoxProps {
                            flex_direction: FlexDirection::Column,
                            gap: 0.0,
                            ..Default::default()
                        },
                        vec![
                            Element::node::<BarChart>(
                                BarChartProps::new(vec![
                                    BarData::new("Block", 70.0).color(Color::Cyan)
                                ])
                                .max_value(100.0)
                                .bar_width(15)
                                .style(BarStyle::Block)
                                .min_label_width(10),
                                vec![],
                            ),
                            Element::node::<BarChart>(
                                BarChartProps::new(vec![
                                    BarData::new("Hash", 70.0).color(Color::Yellow)
                                ])
                                .max_value(100.0)
                                .bar_width(15)
                                .style(BarStyle::Hash)
                                .min_label_width(10),
                                vec![],
                            ),
                            Element::node::<BarChart>(
                                BarChartProps::new(vec![
                                    BarData::new("Equals", 70.0).color(Color::Green)
                                ])
                                .max_value(100.0)
                                .bar_width(15)
                                .style(BarStyle::Equals)
                                .min_label_width(10),
                                vec![],
                            ),
                            Element::node::<BarChart>(
                                BarChartProps::new(vec![
                                    BarData::new("Dot", 70.0).color(Color::Magenta)
                                ])
                                .max_value(100.0)
                                .bar_width(15)
                                .style(BarStyle::Dot)
                                .min_label_width(10),
                                vec![],
                            ),
                            Element::node::<BarChart>(
                                BarChartProps::new(vec![
                                    BarData::new("Gradient", 70.0).color(Color::Blue)
                                ])
                                .max_value(100.0)
                                .bar_width(15)
                                .style(BarStyle::Gradient)
                                .min_label_width(10),
                                vec![],
                            ),
                        ],
                    ),
                ],
            ),
        ],
    )
}
