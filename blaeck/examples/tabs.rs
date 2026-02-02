//! Tabs example - Interactive tab navigation
//!
//! Run with: cargo run --example tabs

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

    let tabs = previews::tabs::TABS;
    let contents = previews::tabs::CONTENTS;
    let mut state = TabsState::new(tabs.len());

    enable_raw_mode()?;

    loop {
        let ui = previews::tabs::build_ui_with_state(tabs, contents, &state);
        blaeck.render(ui)?;

        if let Some(key) = poll_key(Duration::from_millis(50))? {
            if key.is_ctrl_c() {
                break;
            }
            match key.code {
                crossterm::event::KeyCode::Char('q') | crossterm::event::KeyCode::Esc => break,
                crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Char('h') => {
                    state.prev();
                }
                crossterm::event::KeyCode::Right | crossterm::event::KeyCode::Char('l') => {
                    state.next();
                }
                crossterm::event::KeyCode::Char('1') => state.select(0),
                crossterm::event::KeyCode::Char('2') => state.select(1),
                crossterm::event::KeyCode::Char('3') => state.select(2),
                crossterm::event::KeyCode::Char('4') => state.select(3),
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    blaeck.unmount()?;
    println!("Goodbye!");
    Ok(())
}
