use blaeck::prelude::*;

const ITEMS: usize = 8;

fn make_lines(count: usize) -> Vec<LogLine> {
    let mut lines = Vec::new();
    for i in 1..=count.min(ITEMS) {
        lines.push(LogLine::new(format!("Processing item {}...", i)).color(Color::DarkGray));
    }
    if count > ITEMS {
        lines.push(LogLine::success("Done!"));
    }
    if count > ITEMS + 1 {
        lines.push(LogLine::success("✓ Command completed successfully"));
    }
    lines
}

/// Animated preview — lines stream in with blinking dot, then completed state.
pub fn build_ui_with_timer(timer: &AnimationTimer) -> Element {
    let elapsed = timer.elapsed_ms() as usize;
    // Phase 1: streaming (300ms per item, 8 items + "Done!" = 2700ms)
    // Phase 2: completed state held for 3000ms
    // Cycle = 5700ms
    let cycle_ms = 5700;
    let phase_time = elapsed % cycle_ms;

    if phase_time < 2700 {
        // Streaming phase with blinking dot
        let line_count = (phase_time / 300 + 1).min(ITEMS);
        let lines = make_lines(line_count);

        Element::node::<Box>(
            BoxProps {
                flex_direction: FlexDirection::Column,
                padding: 1.0,
                border_style: BorderStyle::Round,
                ..Default::default()
            },
            vec![
                Element::node::<Box>(
                    BoxProps {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    vec![
                        blinking_dot(timer, 500, Color::DarkGray),
                        Element::node::<Text>(
                            TextProps {
                                content: " Running command".into(),
                                bold: true,
                                ..Default::default()
                            },
                            vec![],
                        ),
                    ],
                ),
                Element::node::<LogBox>(
                    LogBoxProps::with_lines(lines)
                        .max_lines(5)
                        .tree_style(TreeStyle::Unicode)
                        .indent(1),
                    vec![],
                ),
            ],
        )
    } else {
        // Completed state
        build_completed_ui()
    }
}

/// Completed state — command finished with green indicator.
pub fn build_completed_ui() -> Element {
    let lines = make_lines(ITEMS + 2); // all items + Done! + success

    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            border_style: BorderStyle::Round,
            border_color: Some(Color::Green),
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
                            content: "●".into(),
                            color: Some(Color::Green),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: " Command finished".into(),
                            bold: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            ),
            Element::node::<LogBox>(
                LogBoxProps::with_lines(lines)
                    .max_lines(5)
                    .tree_style(TreeStyle::Unicode)
                    .indent(1),
                vec![],
            ),
        ],
    )
}
