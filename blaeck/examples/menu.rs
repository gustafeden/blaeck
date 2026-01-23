//! Menu - Interactive menu using the Select component
//!
//! Run with: cargo run --example menu

use blaeck::input::poll_key;
use blaeck::prelude::*;
use blaeck::{Blaeck, SelectIndicator, SelectItem, SelectProps, SelectState};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let items = vec![
        SelectItem::new("New Project").value("new"),
        SelectItem::new("Open Recent").value("open"),
        SelectItem::new("Settings").value("settings"),
        SelectItem::new("Help").value("help"),
        SelectItem::new("Quit").value("quit"),
    ];

    let props = SelectProps::new(items.clone())
        .indicator(SelectIndicator::Arrow)
        .selected_color(Color::Cyan);

    let mut state = SelectState::new(items.len());

    enable_raw_mode()?;

    loop {
        let ui = Element::node::<Box>(
            BoxProps {
                flex_direction: FlexDirection::Column,
                padding: 1.0,
                border_style: BorderStyle::Round,
                ..Default::default()
            },
            vec![
                Element::node::<Text>(
                    TextProps {
                        content: "Main Menu".into(),
                        bold: true,
                        color: Some(Color::Cyan),
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::text(""),
                Element::node::<Divider>(
                    DividerProps::new().width(30).color(Color::DarkGray),
                    vec![],
                ),
                Element::text(""),
                Element::node::<Select>(
                    props
                        .clone()
                        .selected(state.selected)
                        .scroll_offset(state.scroll_offset),
                    vec![],
                ),
                Element::text(""),
                Element::node::<Divider>(
                    DividerProps::new().width(30).color(Color::DarkGray),
                    vec![],
                ),
                Element::text(""),
                Element::node::<Text>(
                    TextProps {
                        content: "↑/↓ navigate • Enter select • q quit".into(),
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
