//! Focus Demo - Demonstrates FocusManager with focus hooks
//!
//! Run with: cargo run --example focus_demo

use blaeck::input::{poll_key, Key};
use blaeck::prelude::*;
use blaeck::{Blaeck, FocusId, FocusManager, Text, TextProps};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use std::time::Duration;

struct AppState {
    focus: FocusManager,
    last_event: String,
    buttons: Vec<&'static str>,
}

impl AppState {
    fn new() -> Self {
        let mut focus = FocusManager::new();

        // Register 4 focusable buttons
        for i in 0..4 {
            focus.register(FocusId(i));
        }

        Self {
            focus,
            last_event: "None".to_string(),
            buttons: vec!["Save", "Load", "Settings", "Quit"],
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key.code {
            crossterm::event::KeyCode::Char('q') => return true,
            crossterm::event::KeyCode::Tab => {
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::SHIFT)
                {
                    self.focus.focus_previous();
                } else {
                    self.focus.focus_next();
                }
            }
            crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Char('h') => {
                self.focus.focus_previous();
            }
            crossterm::event::KeyCode::Right | crossterm::event::KeyCode::Char('l') => {
                self.focus.focus_next();
            }
            crossterm::event::KeyCode::Char('b') => {
                self.focus.blur();
            }
            crossterm::event::KeyCode::Char('1') => self.focus.focus(FocusId(0)),
            crossterm::event::KeyCode::Char('2') => self.focus.focus(FocusId(1)),
            crossterm::event::KeyCode::Char('3') => self.focus.focus(FocusId(2)),
            crossterm::event::KeyCode::Char('4') => self.focus.focus(FocusId(3)),
            crossterm::event::KeyCode::Enter => {
                if let Some(FocusId(3)) = self.focus.focused() {
                    return true; // Quit button
                }
            }
            _ => {}
        }
        false
    }
}

fn render(state: &AppState) -> Element {
    let focused = state.focus.focused();

    // Create button elements
    let buttons: Vec<Element> = state
        .buttons
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let is_focused = focused == Some(FocusId(i));
            let border_style = if is_focused {
                BorderStyle::Double
            } else {
                BorderStyle::Single
            };
            let color = if is_focused {
                Color::Cyan
            } else {
                Color::White
            };

            Element::node::<Box>(
                BoxProps {
                    border_style,
                    padding_left: Some(2.0),
                    padding_right: Some(2.0),
                    border_color: Some(color),
                    ..Default::default()
                },
                vec![Element::node::<Text>(
                    TextProps {
                        content: format!("{} {}", if is_focused { ">" } else { " " }, label),
                        color: Some(color),
                        bold: is_focused,
                        ..Default::default()
                    },
                    vec![],
                )],
            )
        })
        .collect();

    // Build the button row
    let button_row = Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Row,
            gap: 1.0,
            ..Default::default()
        },
        buttons,
    );

    // Build event display box
    let event_box = Element::node::<Box>(
        BoxProps {
            border_style: BorderStyle::Round,
            padding: 1.0,
            ..Default::default()
        },
        vec![
            Element::node::<Text>(
                TextProps {
                    content: "Last Focus Event:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: state.last_event.clone(),
                    color: Some(Color::Yellow),
                    ..Default::default()
                },
                vec![],
            ),
        ],
    );

    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            ..Default::default()
        },
        vec![
            Element::node::<Text>(
                TextProps {
                    content: "Focus Demo".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: "Tab/Arrows to navigate, 1-4 to jump, b to blur, q to quit".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            button_row,
            Element::text(""),
            event_box,
            Element::text(""),
            Element::node::<Text>(
                TextProps {
                    content: format!("Currently focused: {:?}", focused),
                    color: Some(Color::Green),
                    ..Default::default()
                },
                vec![],
            ),
        ],
    )
}

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    let mut state = AppState::new();

    enable_raw_mode()?;

    // Initial render
    blaeck.render(render(&state))?;

    // Event loop
    loop {
        if let Some(key) = poll_key(Duration::from_millis(50))? {
            if key.is_ctrl_c() || state.handle_key(&key) {
                break;
            }

            // Update last event display (simplified - in real app you'd use the callback)
            if let Some(id) = state.focus.focused() {
                state.last_event = format!("Focused: {} (ID: {})", state.buttons[id.0], id.0);
            } else {
                state.last_event = "Blurred all".to_string();
            }

            blaeck.render(render(&state))?;
        }
    }

    disable_raw_mode()?;
    blaeck.unmount()?;

    println!("Goodbye!");
    Ok(())
}
