//! Tabs example - Interactive tab navigation
//!
//! Run with: cargo run --example tabs

use blaeck::input::poll_key;
use blaeck::prelude::*;
use blaeck::Blaeck;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let tabs = vec!["Home", "Profile", "Settings", "Help"];
    let mut state = TabsState::new(tabs.len());

    // Content for each tab
    let contents = [
        "Welcome to the Home tab!\n\nThis is where you start.",
        "User Profile\n\nName: Alice\nEmail: alice@example.com",
        "Settings\n\n[x] Dark mode\n[ ] Notifications\n[x] Auto-save",
        "Help & Support\n\nPress ←/→ to navigate tabs\nPress q to quit",
    ];

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
                // Title
                Element::node::<Text>(
                    TextProps {
                        content: "Tabs Component Demo".into(),
                        bold: true,
                        color: Some(Color::Cyan),
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::text(""),
                // Tabs
                Element::node::<Tabs>(
                    TabsProps::new(tabs.clone())
                        .selected(state.selected)
                        .selected_color(Color::Cyan)
                        .unselected_color(Color::White)
                        .divider(TabDivider::Line)
                        .divider_color(Color::DarkGray),
                    vec![],
                ),
                Element::text(""),
                // Divider
                Element::node::<Divider>(
                    DividerProps::new().width(40).color(Color::DarkGray),
                    vec![],
                ),
                Element::text(""),
                // Content area
                Element::node::<Box>(
                    BoxProps {
                        padding: 1.0,
                        ..Default::default()
                    },
                    vec![Element::node::<Text>(
                        TextProps {
                            content: contents[state.selected].into(),
                            ..Default::default()
                        },
                        vec![],
                    )],
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
                        content: "←/→ switch tabs • q quit".into(),
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
