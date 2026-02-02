use blaeck::prelude::*;
use std::time::Duration;

const COUNTDOWN_DURATION: Duration = Duration::from_secs(10);

/// Live timer demo — stopwatch formats, countdown with thresholds, styled timers.
pub fn build_ui_with_timer(timer: &AnimationTimer) -> Element {
    let elapsed = timer.elapsed();
    let remaining = COUNTDOWN_DURATION.saturating_sub(elapsed);

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
                    content: "Timer Demo".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            // Stopwatch examples
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Stopwatch formats:".into(),
                            dim: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Box>(
                        BoxProps {
                            flex_direction: FlexDirection::Row,
                            gap: 2.0,
                            ..Default::default()
                        },
                        vec![
                            timer_display(elapsed, TimeFormat::MinSec),
                            timer_display(elapsed, TimeFormat::MinSecTenths),
                            timer_display(elapsed, TimeFormat::Human),
                        ],
                    ),
                ],
            ),
            // Countdown with thresholds
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Countdown (warn at 5s, danger at 3s):".into(),
                            dim: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Timer>(
                        TimerProps::countdown(remaining)
                            .format(TimeFormat::MinSecTenths)
                            .warn_at(Duration::from_secs(5))
                            .danger_at(Duration::from_secs(3))
                            .blink_on_danger(true)
                            .blink_visible(timer.blink(250))
                            .bold(),
                        vec![],
                    ),
                ],
            ),
            // Styled timers
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    gap: 2.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Timer>(
                        TimerProps::stopwatch(elapsed)
                            .format(TimeFormat::HourMinSecPadded)
                            .prefix("⏱ ")
                            .color(Color::Blue),
                        vec![],
                    ),
                    Element::node::<Timer>(
                        TimerProps::stopwatch(elapsed)
                            .format(TimeFormat::Seconds)
                            .suffix("s")
                            .color(Color::Magenta),
                        vec![],
                    ),
                ],
            ),
        ],
    )
}

/// Final "complete" state.
pub fn build_final_ui() -> Element {
    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            border_style: BorderStyle::Round,
            border_color: Some(Color::Green),
            ..Default::default()
        },
        vec![
            Element::node::<Text>(
                TextProps {
                    content: "● Timer complete!".into(),
                    color: Some(Color::Green),
                    bold: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Timer>(
                TimerProps::countdown(Duration::ZERO)
                    .format(TimeFormat::MinSec)
                    .complete_color(Color::Green)
                    .prefix("Final: "),
                vec![],
            ),
        ],
    )
}
