//! MultiSelect example - Checkbox list with multiple selections
//!
//! Run with: cargo run --example multiselect

use blaeck::input::poll_key;
use blaeck::prelude::*;
use blaeck::Blaeck;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let items = vec![
        "Rust",
        "Python",
        "JavaScript",
        "TypeScript",
        "Go",
        "Java",
        "C++",
        "Ruby",
    ];

    let mut state = MultiSelectState::new(items.len());

    enable_raw_mode()?;

    loop {
        let props = MultiSelectProps::new(items.clone())
            .cursor(state.cursor)
            .selected(state.selected.clone())
            .scroll_offset(state.scroll_offset)
            .cursor_color(Color::Cyan)
            .selected_color(Color::Green)
            .style(MultiSelectStyle::Bracket);

        let selected_count = state.selected_count();
        let total = items.len();

        let ui = Element::node::<Box>(
            BoxProps {
                flex_direction: FlexDirection::Column,
                padding: 1.0,
                border_style: BorderStyle::Round,
                ..Default::default()
            },
            vec![
                // Title
                Element::node::<Text>(
                    TextProps {
                        content: "MultiSelect Demo".into(),
                        bold: true,
                        color: Some(Color::Cyan),
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::text(""),
                Element::node::<Text>(
                    TextProps {
                        content: "Select your favorite languages:".into(),
                        dim: true,
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::text(""),
                // MultiSelect
                Element::node::<MultiSelect>(props, vec![]),
                Element::text(""),
                // Selection count
                Element::node::<Text>(
                    TextProps {
                        content: format!("{}/{} selected", selected_count, total),
                        color: Some(Color::DarkGray),
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::text(""),
                // Instructions
                Element::node::<Divider>(
                    DividerProps::new()
                        .width(40)
                        .line_style(DividerStyle::Dashed)
                        .color(Color::DarkGray),
                    vec![],
                ),
                Element::node::<Text>(
                    TextProps {
                        content: "↑/↓ move • Space toggle • a all • Enter done".into(),
                        dim: true,
                        ..Default::default()
                    },
                    vec![],
                ),
            ],
        );

        blaeck.render(ui)?;

        if let Some(key) = poll_key(Duration::from_millis(50))? {
            if key.is_ctrl_c() {
                break;
            }
            match key.code {
                crossterm::event::KeyCode::Esc => break,
                crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                    state.up();
                }
                crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                    state.down();
                }
                crossterm::event::KeyCode::Char(' ') => {
                    state.toggle();
                }
                crossterm::event::KeyCode::Char('a') => {
                    if state.all_selected() {
                        state.deselect_all();
                    } else {
                        state.select_all();
                    }
                }
                crossterm::event::KeyCode::Char('i') => {
                    state.toggle_all();
                }
                crossterm::event::KeyCode::Home => {
                    state.first();
                }
                crossterm::event::KeyCode::End => {
                    state.last();
                }
                crossterm::event::KeyCode::Enter => {
                    break;
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    blaeck.unmount()?;

    // Show results
    if state.has_selection() {
        let selected: Vec<_> = state.selected_indices().iter().map(|&i| items[i]).collect();
        println!("Selected: {}", selected.join(", "));
    } else {
        println!("No languages selected.");
    }

    Ok(())
}
