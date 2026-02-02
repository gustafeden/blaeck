//! Menu - Interactive menu using the Select component
//!
//! Run with: cargo run --example menu

#[path = "previews/mod.rs"]
mod previews;

use blaeck::input::poll_key;
use blaeck::Blaeck;
use blaeck::SelectState;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let items = previews::menu::default_items();
    let mut state = SelectState::new(items.len());

    enable_raw_mode()?;

    loop {
        let ui = previews::menu::build_ui_with_state(&items, &state);
        blaeck.render(ui)?;

        if let Some(key) = poll_key(Duration::from_millis(50))? {
            if key.is_ctrl_c() {
                break;
            }
            match key.code {
                crossterm::event::KeyCode::Char('q') | crossterm::event::KeyCode::Esc => break,
                crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                    state.up();
                }
                crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                    state.down();
                }
                crossterm::event::KeyCode::Enter => {
                    let selected_value = items[state.selected].get_value();
                    disable_raw_mode()?;
                    blaeck.unmount()?;

                    if selected_value == "quit" {
                        println!("Goodbye!");
                    } else {
                        println!("Selected: {}", selected_value);
                    }
                    return Ok(());
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    blaeck.unmount()?;
    println!("Goodbye!");
    Ok(())
}
