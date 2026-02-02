use blaeck::prelude::*;

const SPINNER: [char; 4] = ['◐', '◓', '◑', '◒'];

const TASKS: [(&str, f32); 4] = [
    ("Analyzing codebase", 2.3),
    ("Running linter", 1.1),
    ("Type checking", 3.2),
    ("Building project", 4.0),
];

const FILES: [&str; 6] = [
    "src/main.rs",
    "src/lib.rs",
    "src/utils.rs",
    "src/config.rs",
    "src/parser.rs",
    "src/render.rs",
];

const TOTAL_MODULES: usize = 54;

pub fn format_duration(secs: f32) -> String {
    format!("{:.1}s", secs)
}

pub fn progress_bar(percent: u32, width: usize) -> String {
    let filled = (width * percent as usize) / 100;
    let empty = width - filled;
    format!(
        "[{}{}] {:>3}%",
        "█".repeat(filled),
        "░".repeat(empty),
        percent
    )
}

/// Animated preview — tasks complete one by one, then show final success state.
pub fn build_ui_with_timer(timer: &AnimationTimer) -> Element {
    let elapsed_ms = timer.elapsed_ms() as usize;
    // Total animation: sum of task durations * 100ms per frame + 2s hold on final
    // Tasks: 2.3+1.1+3.2+4.0 = 10.6s simulated = 106 frames * 100ms = 10600ms
    // + 3000ms hold on final = 13600ms cycle
    let cycle_ms = 13600;
    let phase_time = elapsed_ms % cycle_ms;

    if phase_time >= 10600 {
        // Final success state
        return build_final_ui();
    }

    // Figure out which task and frame we're at
    let frame_idx = phase_time / 100; // 100ms per frame
    let mut accumulated_frames = 0usize;
    let mut completed: Vec<(&str, f32)> = Vec::new();

    for (task_idx, (task_name, task_duration)) in TASKS.iter().enumerate() {
        let frames = (*task_duration * 10.0) as usize;

        if frame_idx < accumulated_frames + frames {
            // We're in this task
            let frame = frame_idx - accumulated_frames;
            return build_running_ui(
                &completed,
                task_idx,
                task_name,
                *task_duration,
                frame,
                frames,
            );
        }

        accumulated_frames += frames;
        completed.push((task_name, *task_duration));
    }

    build_final_ui()
}

fn build_running_ui(
    completed: &[(&str, f32)],
    task_idx: usize,
    task_name: &str,
    _task_duration: f32,
    frame: usize,
    frames: usize,
) -> Element {
    let spinner = SPINNER[frame % 4];
    let elapsed = frame as f32 * 0.1;
    let percent = ((frame as u32 * 100) / frames as u32).min(99);
    let modules_done = if task_idx == 3 {
        (frame * TOTAL_MODULES) / frames
    } else {
        0
    };
    let current_file = FILES[frame % FILES.len()];

    let mut children: Vec<Element> = Vec::new();

    for (name, dur) in completed {
        children.push(element! {
            Box(flex_direction: FlexDirection::Row) {
                Text(content: format!("✓ {}", name), color: Color::Green)
                Spacer
                Text(content: format_duration(*dur), color: Color::Green, dim: true)
            }
        });
    }

    let mut task_box_children: Vec<Element> = vec![element! {
        Box(flex_direction: FlexDirection::Row) {
            Text(content: format!("{} {}...", spinner, task_name),
                color: Color::Yellow, bold: true)
            Spacer
            Text(content: format_duration(elapsed),
                color: Color::Cyan, dim: true)
        }
    }];

    if task_idx == 3 {
        task_box_children.push(element! {
            Box(border_style: BorderStyle::Round, padding: 1.0, margin_top: 1.0) {
                Text(content: progress_bar(percent, 35))
                Text(content: "")
                Box(flex_direction: FlexDirection::Row) {
                    Text(content: "File:", color: Color::White, dim: true)
                    Text(content: format!(" {}", current_file))
                }
                Box(flex_direction: FlexDirection::Row) {
                    Text(content: "Progress:", color: Color::White, dim: true)
                    Text(content: format!(" {}/{} modules", modules_done, TOTAL_MODULES),
                        color: Color::Cyan)
                }
            }
        });
    }

    let task_box = Element::node::<Box>(
        BoxProps {
            border_style: BorderStyle::Single,
            padding: 1.0,
            margin_top: Some(1.0),
            ..Default::default()
        },
        task_box_children,
    );

    children.push(task_box);
    children.push(Element::text(""));
    children.push(element! {
        Text(content: "Press Ctrl+C to cancel", dim: true, italic: true)
    });

    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            width: Some(60.0),
            ..Default::default()
        },
        children,
    )
}

/// Final success state — all tasks completed.
pub fn build_final_ui() -> Element {
    let total_time: f32 = TASKS.iter().map(|(_, d)| d).sum();

    let mut children: Vec<Element> = Vec::new();

    for (name, dur) in &TASKS {
        children.push(element! {
            Box(flex_direction: FlexDirection::Row) {
                Text(content: format!("✓ {}", name), color: Color::Green)
                Spacer
                Text(content: format_duration(*dur), color: Color::Green, dim: true)
            }
        });
    }

    children.push(Element::text(""));

    children.push(element! {
        Box(border_style: BorderStyle::Double, padding: 1.0) {
            Text(content: "✓ All tasks completed successfully!",
                color: Color::Green, bold: true)
            Text(content: "")
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Total time:", dim: true)
                Spacer
                Text(content: format_duration(total_time), color: Color::Cyan, bold: true)
            }
        }
    });

    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            width: Some(60.0),
            ..Default::default()
        },
        children,
    )
}
