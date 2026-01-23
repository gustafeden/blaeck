//! Select Demo - Interactive select list and confirm prompt
//!
//! Run with: cargo run --example select_demo

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use blaeck::input::{poll_key, Key};
use blaeck::prelude::*;
use blaeck::{Confirm, ConfirmProps, Select, SelectIndicator, SelectProps, SelectState};
use blaeck::Blaeck;
use std::io;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Screen {
    Select,
    Confirm,
    Result,
}

struct AppState {
    screen: Screen,
    select_state: SelectState,
    items: Vec<String>,
    selected_item: Option<String>,
    confirm_selected: bool,
    confirmed: Option<bool>,
}

impl AppState {
    fn new() -> Self {
        let items = vec![
            "Create new project".to_string(),
            "Open existing project".to_string(),
            "Import from template".to_string(),
            "View recent projects".to_string(),
            "Settings".to_string(),
            "Help & Documentation".to_string(),
            "Exit".to_string(),
        ];
        let count = items.len();
        Self {
            screen: Screen::Select,
            select_state: SelectState::new(count).max_visible(5),
            items,
            selected_item: None,
            confirm_selected: false,
            confirmed: None,
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key.code {
            crossterm::event::KeyCode::Esc => {
                match self.screen {
                    Screen::Select => return true,
                    Screen::Confirm => {
                        self.screen = Screen::Select;
                    }
                    Screen::Result => {
                        self.screen = Screen::Select;
                        self.selected_item = None;
                        self.confirmed = None;
                    }
                }
            }
            crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                match self.screen {
                    Screen::Select => self.select_state.up(),
                    Screen::Confirm => self.confirm_selected = true,
                    Screen::Result => {}
                }
            }
            crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                match self.screen {
                    Screen::Select => self.select_state.down(),
                    Screen::Confirm => self.confirm_selected = false,
                    Screen::Result => {}
                }
            }
            crossterm::event::KeyCode::Left => {
                if self.screen == Screen::Confirm {
                    self.confirm_selected = true;
                }
            }
            crossterm::event::KeyCode::Right => {
                if self.screen == Screen::Confirm {
                    self.confirm_selected = false;
                }
            }
            crossterm::event::KeyCode::Char('y') => {
                if self.screen == Screen::Confirm {
                    self.confirm_selected = true;
                }
            }
            crossterm::event::KeyCode::Char('n') => {
                if self.screen == Screen::Confirm {
                    self.confirm_selected = false;
                }
            }
            crossterm::event::KeyCode::Home => {
                if self.screen == Screen::Select {
                    self.select_state.first();
                }
            }
            crossterm::event::KeyCode::End => {
                if self.screen == Screen::Select {
                    self.select_state.last();
                }
            }
            crossterm::event::KeyCode::PageUp => {
                if self.screen == Screen::Select {
                    self.select_state.page_up();
                }
            }
            crossterm::event::KeyCode::PageDown => {
                if self.screen == Screen::Select {
                    self.select_state.page_down();
                }
            }
            crossterm::event::KeyCode::Char(c) if self.screen == Screen::Select && c.is_alphabetic() => {
                // Type-to-jump: find next item starting with this letter
                let props = SelectProps::new(self.items.clone());
                if let Some(idx) = props.find_by_char(c, self.select_state.selected) {
                    self.select_state.jump_to(idx);
                }
            }
            crossterm::event::KeyCode::Enter => {
                match self.screen {
                    Screen::Select => {
                        self.selected_item = Some(self.items[self.select_state.selected].clone());
                        self.confirm_selected = true; // Default to yes
                        self.screen = Screen::Confirm;
                    }
                    Screen::Confirm => {
                        self.confirmed = Some(self.confirm_selected);
                        self.screen = Screen::Result;
                    }
                    Screen::Result => {
                        self.screen = Screen::Select;
                        self.selected_item = None;
                        self.confirmed = None;
                    }
                }
            }
            _ => {}
        }
        false
    }
}

fn render(state: &AppState) -> Element {
    match state.screen {
        Screen::Select => render_select(state),
        Screen::Confirm => render_confirm(state),
        Screen::Result => render_result(state),
    }
}

fn render_select(state: &AppState) -> Element {
    let props = SelectProps::new(state.items.clone())
        .selected(state.select_state.selected)
        .scroll_offset(state.select_state.scroll_offset)
        .max_visible(5)
        .indicator(SelectIndicator::Arrow)
        .selected_color(Color::Cyan);

    // align_self: Start prevents horizontal stretching (shrink to fit content)
    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            border_style: BorderStyle::Round,
            padding: 1.0,
            align_self: Some(AlignSelf::Start),
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
            Element::node::<Text>(
                TextProps {
                    content: "↑/↓ navigate, type to jump, Enter select".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            Element::node::<Select>(props, vec![]),
            Element::text(""),
            Element::node::<Text>(
                TextProps {
                    content: format!("{}/{}", state.select_state.selected + 1, state.items.len()),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: "Esc to exit".into(),
                    dim: true,
                    italic: true,
                    ..Default::default()
                },
                vec![],
            ),
        ],
    )
}

fn render_confirm(state: &AppState) -> Element {
    let selected_item = state.selected_item.as_deref().unwrap_or("Unknown");

    let props = ConfirmProps::new(format!("Proceed with \"{}\"?", selected_item))
        .selected(state.confirm_selected)
        .labels("Yes", "No")
        .selected_color(Color::Yellow);

    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            border_style: BorderStyle::Round,
            padding: 1.0,
            border_color: Some(Color::Yellow),
            align_self: Some(AlignSelf::Start),
            ..Default::default()
        },
        vec![
            Element::node::<Text>(
                TextProps {
                    content: "Confirm".into(),
                    bold: true,
                    color: Some(Color::Yellow),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            Element::node::<Confirm>(props, vec![]),
            Element::text(""),
            Element::node::<Text>(
                TextProps {
                    content: "Esc to go back".into(),
                    dim: true,
                    italic: true,
                    ..Default::default()
                },
                vec![],
            ),
        ],
    )
}

fn render_result(state: &AppState) -> Element {
    let selected_item = state.selected_item.as_deref().unwrap_or("Unknown");
    let confirmed = state.confirmed.unwrap_or(false);

    let (title, color) = if confirmed {
        ("Confirmed!", Color::Green)
    } else {
        ("Cancelled", Color::Red)
    };

    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            border_style: BorderStyle::Round,
            padding: 1.0,
            border_color: Some(color),
            align_self: Some(AlignSelf::Start),
            ..Default::default()
        },
        vec![
            Element::node::<Text>(
                TextProps {
                    content: title.into(),
                    bold: true,
                    color: Some(color),
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: format!("Selected: {}", selected_item),
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: format!("Confirmed: {}", if confirmed { "Yes" } else { "No" }),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            Element::node::<Text>(
                TextProps {
                    content: "Enter/Esc to return".into(),
                    dim: true,
                    italic: true,
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
            blaeck.render(render(&state))?;
        }
    }

    disable_raw_mode()?;
    blaeck.unmount()?;

    println!("Goodbye!");
    Ok(())
}
