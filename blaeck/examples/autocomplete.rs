//! Autocomplete example - Text input with filtered suggestions
//!
//! Run with: cargo run --example autocomplete

#[path = "previews/mod.rs"]
mod previews;

use blaeck::input::poll_key;
use blaeck::prelude::*;
use blaeck::Blaeck;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let suggestions = previews::autocomplete::SUGGESTIONS;
    let mut state = AutocompleteState::new();

    enable_raw_mode()?;

    loop {
        // Update filtered count for state navigation
        let props = AutocompleteProps::new(suggestions.to_vec())
            .input(&state.input)
            .selected(state.selected)
            .filter_mode(FilterMode::Contains);
        let filtered_count = props.filtered_suggestions().len();
        state.set_filtered_count(filtered_count);

        let ui = previews::autocomplete::build_ui_with_state(suggestions, &state);
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
                    let props = AutocompleteProps::new(suggestions.to_vec())
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
