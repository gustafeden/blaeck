//! Breadcrumbs example - Navigation path displays
//!
//! Run with: cargo run --example breadcrumbs

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
                    content: "Breadcrumbs Component".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            // Basic breadcrumbs
            Element::node::<Text>(
                TextProps {
                    content: "Basic (slash separator):".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Breadcrumbs>(
                BreadcrumbsProps::new(["Home", "Projects", "Blaeck", "Examples"]),
                vec![],
            ),
            Element::text(""),
            // From path
            Element::node::<Text>(
                TextProps {
                    content: "From path string:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Breadcrumbs>(
                BreadcrumbsProps::from_path("/home/user/documents/report.pdf"),
                vec![],
            ),
            Element::text(""),
            // Chevron separator
            Element::node::<Text>(
                TextProps {
                    content: "Chevron separator:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Breadcrumbs>(
                BreadcrumbsProps::new(["Dashboard", "Settings", "Profile"])
                    .separator(BreadcrumbSeparator::Chevron),
                vec![],
            ),
            Element::text(""),
            // Arrow separator
            Element::node::<Text>(
                TextProps {
                    content: "Arrow separator:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Breadcrumbs>(
                BreadcrumbsProps::new(["Main Menu", "Options", "Audio"])
                    .separator(BreadcrumbSeparator::Arrow),
                vec![],
            ),
            Element::text(""),
            // Double chevron separator
            Element::node::<Text>(
                TextProps {
                    content: "Double chevron separator:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Breadcrumbs>(
                BreadcrumbsProps::new(["Level 1", "Level 2", "Level 3"])
                    .separator(BreadcrumbSeparator::DoubleChevron),
                vec![],
            ),
            Element::text(""),
            // With colors
            Element::node::<Text>(
                TextProps {
                    content: "With active color:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Breadcrumbs>(
                BreadcrumbsProps::new(["Root", "Branch", "Leaf"])
                    .active_color(Color::Green)
                    .separator(BreadcrumbSeparator::Bullet),
                vec![],
            ),
            Element::text(""),
            // With root indicator
            Element::node::<Text>(
                TextProps {
                    content: "With root indicator (~):".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Breadcrumbs>(
                BreadcrumbsProps::new(["home", "user", "docs"]).show_root(true),
                vec![],
            ),
            Element::text(""),
            // Truncated (long path)
            Element::node::<Text>(
                TextProps {
                    content: "Truncated (max 4 items):".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Breadcrumbs>(
                BreadcrumbsProps::new(["a", "b", "c", "d", "e", "f", "g", "h"]).max_items(4),
                vec![],
            ),
            Element::text(""),
            // Custom separator
            Element::node::<Text>(
                TextProps {
                    content: "Custom separator:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Breadcrumbs>(
                BreadcrumbsProps::new(["src", "components", "breadcrumbs.rs"])
                    .separator(BreadcrumbSeparator::Custom(" :: ")),
                vec![],
            ),
            Element::text(""),
            // Helper functions
            Element::node::<Divider>(
                DividerProps::new()
                    .width(45)
                    .line_style(DividerStyle::Dashed)
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: "Helper functions (return strings):".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: format!("breadcrumbs_path: {}", breadcrumbs_path("/usr/local/bin")),
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: format!("breadcrumbs: {}", breadcrumbs(["A", "B", "C"])),
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
