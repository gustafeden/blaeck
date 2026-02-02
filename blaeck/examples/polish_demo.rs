//! Polish Demo - Showcases Divider, Badge, Link, and button-style Confirm
//!
//! Run with: cargo run --example polish_demo

#[path = "previews/mod.rs"]
mod previews;

use blaeck::{input, Blaeck};
use crossterm::terminal;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    terminal::enable_raw_mode()?;

    let mut confirm = blaeck::ConfirmProps::new("Delete this file?")
        .labels("Delete", "Cancel")
        .button_style()
        .default_value(false);

    loop {
        let ui = previews::polish_demo::build_ui_with_confirm(&confirm);
        blaeck.render(ui)?;

        if let Some(key) = input::poll_key(std::time::Duration::from_millis(100))? {
            match key.code {
                crossterm::event::KeyCode::Char('q') | crossterm::event::KeyCode::Esc => break,
                crossterm::event::KeyCode::Char('c')
                    if key
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::CONTROL) =>
                {
                    break
                }
                crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Char('y') => {
                    confirm.select_yes();
                }
                crossterm::event::KeyCode::Right | crossterm::event::KeyCode::Char('n') => {
                    confirm.select_no();
                }
                crossterm::event::KeyCode::Enter => {
                    terminal::disable_raw_mode()?;
                    blaeck.unmount()?;
                    println!(
                        "You selected: {}",
                        if confirm.answer() { "Delete" } else { "Cancel" }
                    );
                    return Ok(());
                }
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    blaeck.unmount()?;
    Ok(())
}
