//! StatusBar example - Git-style status displays
//!
//! Run with: cargo run --example statusbar

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

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
                    content: "StatusBar Component".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            // Simple status bar
            Element::node::<Text>(
                TextProps {
                    content: "Simple status bar:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<StatusBar>(
                StatusBarProps::new(["ready", "3 tasks", "0 errors"]),
                vec![],
            ),
            Element::text(""),
            // With pipe separator
            Element::node::<Text>(
                TextProps {
                    content: "Pipe separator:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<StatusBar>(
                StatusBarProps::new(["online", "connected", "synced"])
                    .separator(StatusSeparator::Pipe),
                vec![],
            ),
            Element::text(""),
            // Git-style status
            Element::node::<Text>(
                TextProps {
                    content: "Git-style status:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<StatusBar>(
                StatusBarProps::new(Vec::<StatusSegment>::new())
                    .segment(git_branch("main", Color::Green))
                    .segment(StatusSegment::with_icon(icons::PLUS, "5").color(Color::Green))
                    .segment(StatusSegment::with_icon(icons::MINUS, "2").color(Color::Red))
                    .segment(StatusSegment::with_icon(icons::MODIFIED, "3").color(Color::Yellow))
                    .separator(StatusSeparator::Pipe),
                vec![],
            ),
            Element::text(""),
            // With square brackets
            Element::node::<Text>(
                TextProps {
                    content: "With brackets:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<StatusBar>(
                StatusBarProps::new(["main", "clean"])
                    .square_brackets()
                    .separator(StatusSeparator::Bullet),
                vec![],
            ),
            Element::text(""),
            // Status icons
            Element::node::<Text>(
                TextProps {
                    content: "Status helper functions:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<StatusBar>(
                StatusBarProps::new(Vec::<StatusSegment>::new())
                    .segment(status_ok("build"))
                    .segment(status_warning("lint"))
                    .segment(status_error("test"))
                    .separator(StatusSeparator::Pipe),
                vec![],
            ),
            Element::text(""),
            // Colored segments
            Element::node::<Text>(
                TextProps {
                    content: "Colored segments:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<StatusBar>(
                StatusBarProps::new(Vec::<StatusSegment>::new())
                    .text("CPU: 45%", Color::Green)
                    .text("MEM: 72%", Color::Yellow)
                    .text("DISK: 89%", Color::Red)
                    .separator(StatusSeparator::DoubleSpace),
                vec![],
            ),
            Element::text(""),
            // With icons
            Element::node::<Text>(
                TextProps {
                    content: "With icons:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<StatusBar>(
                StatusBarProps::new(Vec::<StatusSegment>::new())
                    .with_icon(icons::USER, "admin", Color::Cyan)
                    .with_icon(icons::CLOCK, "12:34", Color::White)
                    .with_icon(icons::SYNC, "syncing", Color::Yellow)
                    .separator(StatusSeparator::Bullet),
                vec![],
            ),
            Element::text(""),
            // Build status example
            Element::node::<Text>(
                TextProps {
                    content: "Build status:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<StatusBar>(
                StatusBarProps::new(Vec::<StatusSegment>::new())
                    .segment(StatusSegment::with_icon(icons::CHECK, "compiled").color(Color::Green))
                    .segment(StatusSegment::new("42 tests").color(Color::Blue))
                    .segment(StatusSegment::new("0 warnings").dim())
                    .parens()
                    .separator(StatusSeparator::Slash),
                vec![],
            ),
            Element::text(""),
            // Custom segments
            Element::node::<Divider>(
                DividerProps::new()
                    .width(45)
                    .line_style(DividerStyle::Dashed)
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: "Available icons:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: format!(
                        "BRANCH: {} CHECK: {} CROSS: {} WARNING: {} INFO: {}",
                        icons::BRANCH,
                        icons::CHECK,
                        icons::CROSS,
                        icons::WARNING,
                        icons::INFO
                    ),
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: format!(
                        "CLOCK: {} USER: {} SYNC: {} PLUS: {} MINUS: {} MODIFIED: {}",
                        icons::CLOCK,
                        icons::USER,
                        icons::SYNC,
                        icons::PLUS,
                        icons::MINUS,
                        icons::MODIFIED
                    ),
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: format!(
                        "ARROW_UP: {} ARROW_DOWN: {} FOLDER: {} FILE: {} LOCK: {} UNLOCK: {}",
                        icons::ARROW_UP,
                        icons::ARROW_DOWN,
                        icons::FOLDER,
                        icons::FILE,
                        icons::LOCK,
                        icons::UNLOCK
                    ),
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
        ],
    );

    blaeck.render(ui)?;
    blaeck.unmount()?;

    Ok(())
}
