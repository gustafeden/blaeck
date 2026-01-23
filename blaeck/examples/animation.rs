//! Animation example - Demonstrates animation utilities
//!
//! Run with: cargo run --example animation

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let timer = AnimationTimer::new();

    // Run animation for 5 seconds
    while timer.elapsed_ms() < 5000 {
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
                        blinking_dot(&timer, 500, Color::Green),
                        Element::node::<Text>(
                            TextProps {
                                content: "Pulse:".into(),
                                ..Default::default()
                            },
                            vec![],
                        ),
                        pulsing_dot(&timer, 500, Color::Yellow),
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
                        animated_indicator_colored(IndicatorStyle::SpinnerDots, &timer, Color::Cyan),
                        animated_indicator_colored(IndicatorStyle::SpinnerLine, &timer, Color::Magenta),
                        Element::node::<Text>(
                            TextProps {
                                content: format!("Dots{}", IndicatorStyle::GrowingDots.render(&timer)),
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
                        blink_pattern(BlinkPattern::Fast, &timer, "●", "○"),
                        blink_pattern(BlinkPattern::Slow, &timer, "◆", "◇"),
                        blink_pattern(BlinkPattern::Heartbeat, &timer, "❤", " "),
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
        );

        blaeck.render(ui)?;
        thread::sleep(Duration::from_millis(50)); // 20 FPS
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
                    content: "● Animation complete!".into(),
                    color: Some(Color::Green),
                    bold: true,
                    ..Default::default()
                },
                vec![],
            ),
        ],
    );

    blaeck.render(final_ui)?;
    blaeck.unmount()?;

    Ok(())
}
