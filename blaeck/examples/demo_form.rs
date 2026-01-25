//! Demo: Interactive form with reactive state.

use blaeck::prelude::*;
use blaeck::reactive::*;
use crossterm::event::KeyCode;

fn form(cx: Scope) -> Element {
    let name = use_state(cx.clone(), || String::new());
    let env = use_state(cx.clone(), || 0usize); // 0=dev, 1=staging, 2=prod
    let confirmed = use_state(cx.clone(), || false);
    let focus = use_state(cx.clone(), || 0usize); // 0=name, 1=env, 2=confirm

    let envs = vec!["Development", "Staging", "Production"];

    let name_h = name.clone();
    let env_h = env.clone();
    let confirmed_h = confirmed.clone();
    let focus_h = focus.clone();

    use_input(cx, move |key| {
        let f = focus_h.get();
        match key.code {
            KeyCode::Tab => focus_h.set((f + 1) % 3),
            KeyCode::BackTab => focus_h.set(if f == 0 { 2 } else { f - 1 }),
            KeyCode::Up if f == 1 => env_h.set(if env_h.get() == 0 { 2 } else { env_h.get() - 1 }),
            KeyCode::Down if f == 1 => env_h.set((env_h.get() + 1) % 3),
            KeyCode::Char(' ') if f == 2 => confirmed_h.set(!confirmed_h.get()),
            KeyCode::Char(c) if f == 0 => {
                let mut s = name_h.get();
                s.push(c);
                name_h.set(s);
            }
            KeyCode::Backspace if f == 0 => {
                let mut s = name_h.get();
                s.pop();
                name_h.set(s);
            }
            _ => {}
        }
    });

    let f = focus.get();
    let name_val = name.get();
    let env_val = env.get();
    let conf_val = confirmed.get();

    element! {
        Box(flex_direction: FlexDirection::Column, border_style: BorderStyle::Round, padding: 1.0) {
            Text(content: "Deploy Configuration", bold: true, color: Color::Cyan)
            Newline

            // Name field
            Text(content: "Project name:", bold: f == 0)
            Box(flex_direction: FlexDirection::Row) {
                Text(content: if f == 0 { "> " } else { "  " }, color: Color::Green)
                Text(
                    content: if name_val.is_empty() { "(type here)".to_string() } else { name_val },
                    color: if f == 0 { Color::White } else { Color::DarkGray }
                )
                Text(content: if f == 0 { "_" } else { "" }, color: Color::Green)
            }
            Newline

            // Environment selector
            Text(content: "Environment:", bold: f == 1)
            #(Element::column(
                envs.iter().enumerate().map(|(i, e)| {
                    let selected = i == env_val;
                    let focused = f == 1;
                    let prefix = if selected { "● " } else { "○ " };
                    element! {
                        Text(
                            content: format!("  {}{}", prefix, e),
                            color: if selected && focused { Color::Green } else { Color::White },
                            bold: selected
                        )
                    }
                }).collect()
            ))
            Newline

            // Confirm checkbox
            Box(flex_direction: FlexDirection::Row) {
                Text(content: if f == 2 { "> " } else { "  " }, color: Color::Green)
                Text(
                    content: format!("[{}] I confirm this deployment", if conf_val { "x" } else { " " }),
                    bold: f == 2
                )
            }
            Newline

            Text(content: "Tab: next field | Space: toggle | Ctrl+C: exit", dim: true)
        }
    }
}

fn main() -> std::io::Result<()> {
    ReactiveApp::run(form)?;
    Ok(())
}
