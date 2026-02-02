use blaeck::prelude::*;

pub fn build_ui() -> Element {
    build_ui_with_timer(&AnimationTimer::new())
}

pub fn build_ui_with_timer(timer: &AnimationTimer) -> Element {
    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            border_style: BorderStyle::Round,
            gap: 1.0,
            ..Default::default()
        },
        vec![
            // Title
            Element::node::<Text>(
                TextProps {
                    content: "Animation Demo".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            // Blinking indicators
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    gap: 2.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Blink:".into(),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    blinking_dot(timer, 500, Color::Green),
                    Element::node::<Text>(
                        TextProps {
                            content: "Pulse:".into(),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    pulsing_dot(timer, 500, Color::Yellow),
                ],
            ),
            // Animated indicators
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    gap: 2.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Spinners:".into(),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    animated_indicator_colored(
                        IndicatorStyle::SpinnerDots,
                        timer,
                        Color::Cyan,
                    ),
                    animated_indicator_colored(
                        IndicatorStyle::SpinnerLine,
                        timer,
                        Color::Magenta,
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: format!(
                                "Dots{}",
                                IndicatorStyle::GrowingDots.render(timer)
                            ),
                            color: Some(Color::Blue),
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            ),
            // Blink patterns
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    gap: 2.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Patterns:".into(),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    blink_pattern(BlinkPattern::Fast, timer, "\u{25cf}", "\u{25cb}"),
                    blink_pattern(BlinkPattern::Slow, timer, "\u{25c6}", "\u{25c7}"),
                    blink_pattern(BlinkPattern::Heartbeat, timer, "\u{2764}", " "),
                ],
            ),
            // Progress with easing
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Progress (ping-pong with easing):".into(),
                            dim: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Progress>(
                        ProgressProps {
                            progress: timer.progress_pingpong(1500, Easing::EaseInOut) as f32,
                            width: 30,
                            style: ProgressStyle::Block,
                            filled_color: Some(Color::Green),
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            ),
            // Elapsed time
            Element::node::<Text>(
                TextProps {
                    content: format!("Elapsed: {:.1}s", timer.elapsed_ms() as f64 / 1000.0),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
        ],
    )
}
