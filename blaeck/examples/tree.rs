//! Tree View example - Hierarchical data display
//!
//! Run with: cargo run --example tree

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    // Build a file tree structure
    let tree = TreeNode::new("blaeck")
        .icon("📁")
        .child(
            TreeNode::new("src")
                .icon("📁")
                .child(TreeNode::leaf("main.rs").icon("📄").color(Color::Yellow))
                .child(TreeNode::leaf("lib.rs").icon("📄").color(Color::Yellow))
                .child(
                    TreeNode::new("components")
                        .icon("📁")
                        .child(TreeNode::leaf("mod.rs").icon("📄"))
                        .child(TreeNode::leaf("box.rs").icon("📄"))
                        .child(TreeNode::leaf("text.rs").icon("📄"))
                        .child(TreeNode::leaf("tree.rs").icon("📄").color(Color::Green)),
                ),
        )
        .child(
            TreeNode::new("examples")
                .icon("📁")
                .child(TreeNode::leaf("tree.rs").icon("📄").color(Color::Cyan))
                .child(TreeNode::leaf("timer.rs").icon("📄")),
        )
        .child(TreeNode::leaf("Cargo.toml").icon("📦").color(Color::Blue))
        .child(TreeNode::leaf("README.md").icon("📝").color(Color::Magenta));

    // Create state with some nodes expanded
    let state = TreeState::new()
        .expand("blaeck")
        .expand("src")
        .expand("components")
        .select("tree.rs");

    // Render tree views with different styles
    let ui = Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            gap: 1.0,
            ..Default::default()
        },
        vec![
            // Title
            Element::node::<Text>(
                TextProps {
                    content: "Tree View Demo".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            // Unicode connectors (default)
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single,
                    padding: 1.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Unicode connectors:".into(),
                            dim: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<TreeView>(
                        TreeViewProps::new(tree.clone())
                            .state(state.clone())
                            .branch_color(Color::Blue)
                            .leaf_color(Color::White),
                        vec![],
                    ),
                ],
            ),
            // ASCII connectors
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single,
                    padding: 1.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "ASCII connectors:".into(),
                            dim: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<TreeView>(
                        TreeViewProps::new(tree.clone())
                            .state(state.clone())
                            .connectors(TreeConnectors::Ascii)
                            .indicators("+", "-")
                            .branch_color(Color::Yellow),
                        vec![],
                    ),
                ],
            ),
            // Collapsed tree
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single,
                    padding: 1.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Collapsed (only root expanded):".into(),
                            dim: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<TreeView>(
                        TreeViewProps::new(tree.clone())
                            .state(TreeState::new().expand("blaeck"))
                            .show_indicators(true),
                        vec![],
                    ),
                ],
            ),
        ],
    );

    blaeck.render(ui)?;
    blaeck.unmount()?;

    Ok(())
}
