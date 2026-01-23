//! Interactive example demonstrating input handling and focus management.
//!
//! Run with: cargo run --example interactive
//!
//! Demonstrates:
//! - App runtime with event loop
//! - Keyboard input handling via match_key
//! - Focus management with FocusManager
//! - Tab/Shift+Tab and arrow key navigation
//! - Enter to select, Ctrl+C to exit

use blaeck::prelude::*;
use blaeck::{match_key, App, Arrow, FocusId, FocusManager, Key};
use std::cell::RefCell;

struct AppState {
    focus: FocusManager,
    items: Vec<&'static str>,
    selected: Option<usize>,
    message: String,
}

impl AppState {
    fn new() -> Self {
        let mut focus = FocusManager::new();
        focus.register(FocusId(0));
        focus.register(FocusId(1));
        focus.register(FocusId(2));

        Self {
            focus,
            items: vec!["Option A", "Option B", "Option C"],
            selected: None,
            message: String::new(),
        }
    }

    fn render(&self) -> Element {
        let mut children: Vec<Element> = vec![
            element! {
                Text(content: "Interactive Demo", bold: true, color: Color::Cyan)
            },
            element! {
                Text(content: "Use Tab/Arrows to navigate, Enter to select, Ctrl+C to quit", dim: true)
            },
            Element::text(""),
        ];

        // Menu items
        for (i, item) in self.items.iter().enumerate() {
            let is_focused = self.focus.is_focused(FocusId(i));
            let is_selected = self.selected == Some(i);

            let prefix = if is_selected {
                "● "
            } else if is_focused {
                "▸ "
            } else {
                "  "
            };
            let color = if is_focused {
                Color::Yellow
            } else {
                Color::White
            };

            children.push(element! {
                Text(
                    content: format!("{}{}", prefix, item),
                    color: color,
                    bold: is_focused
                )
            });
        }

        // Message
        if !self.message.is_empty() {
            children.push(Element::text(""));
            children.push(element! {
                Box(border_style: BorderStyle::Round, padding: 1.0) {
                    Text(content: self.message.clone(), color: Color::Green)
                }
            });
        }

        Element::node::<Box>(
            BoxProps {
                flex_direction: FlexDirection::Column,
                padding: 1.0,
                ..Default::default()
            },
            children,
        )
    }

    fn handle_input(&mut self, key: &Key) {
        match_key(key, self)
            .on_tab(|s| {
                s.focus.focus_next();
            })
            .on_backtab(|s| {
                s.focus.focus_previous();
            })
            .on_enter(|s| {
                if let Some(id) = s.focus.focused() {
                    s.selected = Some(id.0);
                    s.message = format!("Selected: {}", s.items[id.0]);
                }
            })
            .on_arrow(|s, arrow| match arrow {
                Arrow::Up => s.focus.focus_previous(),
                Arrow::Down => s.focus.focus_next(),
                _ => {}
            });
    }
}

fn main() -> std::io::Result<()> {
    // Use RefCell for interior mutability - allows both closures to access state
    let state = RefCell::new(AppState::new());

    let app = App::new()?;

    app.run(
        |_app| state.borrow().render(),
        |_app, key| {
            state.borrow_mut().handle_input(&key);
        },
    )?;

    println!("Goodbye!");
    Ok(())
}
