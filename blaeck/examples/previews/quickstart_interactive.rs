use blaeck::prelude::*;
use blaeck::reactive::*;
use crossterm::event::KeyCode;

/// The reactive component — used by both the standalone example and the viewer.
pub fn task_selector(cx: Scope) -> Element {
    let tasks = vec!["Build project", "Run tests", "Deploy", "Rollback"];
    let selected = use_state(cx.clone(), || 0usize);
    let status = use_state(cx.clone(), || "Select a task...");

    let selected_handler = selected.clone();
    let status_handler = status.clone();
    let task_count = tasks.len();

    use_input(cx, move |key| match key.code {
        KeyCode::Up => {
            let current = selected_handler.get();
            if current > 0 {
                selected_handler.set(current - 1);
                status_handler.set("Select a task...");
            }
        }
        KeyCode::Down => {
            let current = selected_handler.get();
            if current < task_count - 1 {
                selected_handler.set(current + 1);
                status_handler.set("Select a task...");
            }
        }
        KeyCode::Enter => {
            status_handler.set("Running...");
        }
        _ => {}
    });

    let current = selected.get();
    let current_status = status.get();

    render_task_selector(&tasks, current, current_status)
}

/// Render the task selector UI given tasks, selection, and status.
pub fn render_task_selector(tasks: &[&str], selected: usize, status: &str) -> Element {
    let task_list = Element::column(
        tasks.iter().enumerate().map(|(i, task)| {
            let is_selected = i == selected;
            let prefix = if is_selected { "> " } else { "  " };
            let color = if is_selected { Color::Green } else { Color::White };
            element! { Text(content: format!("{}{}", prefix, task), color: color, bold: is_selected) }
        }).collect()
    );

    element! {
        Box(flex_direction: FlexDirection::Column, border_style: BorderStyle::Round, padding: 1.0) {
            Text(content: "Task Runner", bold: true, color: Color::Cyan)
            Newline
            #(task_list)
            Newline
            Text(content: status, dim: true)
            Text(content: "Up/Down: navigate | Enter: run | Ctrl+C: exit", dim: true)
        }
    }
}

/// Static snapshot for the example viewer preview panel.
pub fn build_ui() -> Element {
    let tasks = vec!["Build project", "Run tests", "Deploy", "Rollback"];
    render_task_selector(&tasks, 0, "Select a task...")
}
