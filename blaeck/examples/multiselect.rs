//! MultiSelect example - Checkbox list with multiple selections
//!
//! Run with: cargo run --example multiselect

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

    let items = previews::multiselect::ITEMS;
    let mut state = MultiSelectState::new(items.len());

    enable_raw_mode()?;

    loop {
        let ui = previews::multiselect::build_ui_with_state(items, &state);
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

    if state.has_selection() {
        let selected: Vec<_> = state.selected_indices().iter().map(|&i| items[i]).collect();
        println!("Selected: {}", selected.join(", "));
    } else {
        println!("No languages selected.");
    }

    Ok(())
}
