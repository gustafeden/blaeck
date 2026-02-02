use blaeck::prelude::*;

const TASKS: &[(&str, Option<Color>)] = &[
    ("Initializing...", None),
    ("Scanning directory", None),
    ("Read(/blaeck/src/lib.rs)", Some(Color::DarkGray)),
    ("Read(/blaeck/src/main.rs)", Some(Color::DarkGray)),
    ("Error reading file", Some(Color::Red)),
    ("Glob(**/*.rs)", Some(Color::DarkGray)),
    ("Found 42 files", None),
    ("Analyzing imports", None),
    ("Building dependency graph", None),
    ("Optimization complete", Some(Color::Green)),
];

fn make_lines(count: usize) -> Vec<LogLine> {
    TASKS.iter().take(count).map(|(msg, color)| {
        if let Some(c) = color {
            LogLine::new(*msg).color(*c)
        } else {
            LogLine::new(*msg)
        }
    }).collect()
}

/// Animated preview — lines appear one by one, then shows multi-variant final state.
pub fn build_ui_with_timer(timer: &AnimationTimer) -> Element {
    let elapsed = timer.elapsed_ms() as usize;
    // Phase 1: streaming (400ms per line, 10 lines = 4000ms)
    // Phase 2: hold final state for 1000ms
    // Phase 3: multi-variant view for 3000ms
    // Then loop (cycle = 8000ms)
    let cycle_ms = 8000;
    let phase_time = elapsed % cycle_ms;

    if phase_time < 4000 {
        // Streaming phase
        let line_count = (phase_time / 400 + 1).min(TASKS.len());
        let lines = make_lines(line_count);
        build_streaming_ui(&lines, line_count, TASKS.len())
    } else {
        // Final multi-variant view
        build_final_ui()
    }
}

fn build_streaming_ui(lines: &[LogLine], step: usize, total: usize) -> Element {
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
                    Element::node::<Text>(
                        TextProps {
                            content: "●".into(),
                            color: Some(Color::Yellow),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: " Explore".into(),
                            bold: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: "(Explore /blaeck repository)".into(),
                            dim: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            ),
            Element::node::<LogBox>(
                LogBoxProps::with_lines(lines.to_vec())
                    .max_lines(5)
                    .tree_style(TreeStyle::Unicode)
                    .indent(1),
                vec![],
            ),
            Element::text(""),
            Element::node::<Text>(
                TextProps {
                    content: format!("Step {}/{}", step, total),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
        ],
    )
}

/// Final state showing three LogBox variants.
pub fn build_final_ui() -> Element {
    let lines = make_lines(TASKS.len());

    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            gap: 1.0,
            ..Default::default()
        },
        vec![
            // Example 1: Completed explore task
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
                                    content: " Explore".into(),
                                    bold: true,
                                    ..Default::default()
                                },
                                vec![],
                            ),
                            Element::node::<Text>(
                                TextProps {
                                    content: "(completed)".into(),
                                    dim: true,
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
            ),
            // Example 2: Simple log without tree style
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
                            content: "Build Output (no tree, max 3 lines):".into(),
                            bold: true,
                            color: Some(Color::Cyan),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<LogBox>(
                        LogBoxProps::new()
                            .line("Compiling blaeck v0.1.0")
                            .line("Compiling blaeck-macros v0.1.0")
                            .line(LogLine::warning("warning: unused import"))
                            .line("Compiling example v0.1.0")
                            .line(LogLine::success("Finished dev [unoptimized] in 2.34s"))
                            .max_lines(3),
                        vec![],
                    ),
                ],
            ),
            // Example 3: Error log
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single,
                    border_color: Some(Color::Red),
                    padding: 1.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Error Log:".into(),
                            bold: true,
                            color: Some(Color::Red),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<LogBox>(
                        LogBoxProps::new()
                            .line(LogLine::error("Error: Connection refused"))
                            .line(LogLine::muted("  at src/network.rs:42"))
                            .line(LogLine::muted("  at src/main.rs:15"))
                            .line(LogLine::warning("Retrying in 5s..."))
                            .max_lines(10)
                            .tree_style(TreeStyle::Unicode),
                        vec![],
                    ),
                ],
            ),
        ],
    )
}
