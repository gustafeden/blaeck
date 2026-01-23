//! Autocomplete example - Text input with filtered suggestions
//!
//! Run with: cargo run --example autocomplete

use blaeck::input::poll_key;
use blaeck::prelude::*;
use blaeck::Blaeck;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    // Sample suggestions - programming languages
    let suggestions = vec![
        "Rust",
        "Python",
        "JavaScript",
        "TypeScript",
        "Go",
        "Java",
        "C++",
        "C#",
        "Ruby",
        "Swift",
        "Kotlin",
        "Scala",
        "Haskell",
        "Elixir",
        "Clojure",
    ];

    let mut state = AutocompleteState::new();

    enable_raw_mode()?;

    loop {
        // Build props with current state
        let props = AutocompleteProps::new(suggestions.clone())
            .input(&state.input)
            .cursor(state.cursor)
            .selected(state.selected)
            .placeholder("Search languages...")
            .filter_mode(FilterMode::Contains)
            .max_suggestions(6)
            .selected_color(Color::Cyan);

        // Update filtered count for state navigation
        let filtered_count = props.filtered_suggestions().len();
        state.set_filtered_count(filtered_count);

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
                        content: "Autocomplete Demo".into(),
                        bold: true,
                        color: Some(Color::Cyan),
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::text(""),
                Element::node::<Text>(
                    TextProps {
                        content: "Type to filter programming languages:".into(),
                        dim: true,
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::text(""),
                // Autocomplete input
                Element::node::<Autocomplete>(props, vec![]),
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
                        content: "↑/↓ navigate • Enter/Tab select • Esc quit".into(),
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
                crossterm::event::KeyCode::Char(c) => {
                    state.insert(c);
                }
                crossterm::event::KeyCode::Backspace => {
                    state.backspace();
                }
                crossterm::event::KeyCode::Delete => {
                    state.delete();
                }
                crossterm::event::KeyCode::Left => {
                    state.move_left();
                }
                crossterm::event::KeyCode::Right => {
                    state.move_right();
                }
                crossterm::event::KeyCode::Up => {
                    state.prev();
                }
                crossterm::event::KeyCode::Down => {
                    state.next();
                }
                crossterm::event::KeyCode::Home => {
                    state.move_home();
                }
                crossterm::event::KeyCode::End => {
                    state.move_end();
                }
                crossterm::event::KeyCode::Enter | crossterm::event::KeyCode::Tab => {
                    // Select current suggestion
                    let props = AutocompleteProps::new(suggestions.clone())
                        .input(&state.input)
                        .selected(state.selected)
                        .filter_mode(FilterMode::Contains);

                    if let Some(value) = props.selected_value() {
                        state.set_input(value);
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    blaeck.unmount()?;

    if !state.input.is_empty() {
        println!("Selected: {}", state.input);
    } else {
        println!("No selection made.");
    }

    Ok(())
}
