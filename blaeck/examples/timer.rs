//! Timer example - Stopwatch and countdown display
//!
//! Run with: cargo run --example timer

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let timer = AnimationTimer::new();
    let countdown_duration = Duration::from_secs(10);

    // Run for 12 seconds to show countdown completing
    while timer.elapsed_ms() < 12000 {
        let elapsed = timer.elapsed();
        let remaining = countdown_duration.saturating_sub(elapsed);

        let ui = Element::node::<Box>(
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
        );

        blaeck.render(ui)?;
        thread::sleep(Duration::from_millis(100));
    }

    // Final state
    let final_ui = Element::node::<Box>(
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
    );

    blaeck.render(final_ui)?;
    blaeck.unmount()?;

    Ok(())
}
