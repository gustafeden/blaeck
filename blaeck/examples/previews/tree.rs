use blaeck::prelude::*;

pub fn build_ui() -> Element {
    let tree = TreeNode::new("blaeck")
        .icon("\u{1f4c1}")
        .child(
            TreeNode::new("src")
                .icon("\u{1f4c1}")
                .child(TreeNode::leaf("main.rs").icon("\u{1f4c4}").color(Color::Yellow))
                .child(TreeNode::leaf("lib.rs").icon("\u{1f4c4}").color(Color::Yellow))
                .child(
                    TreeNode::new("components")
                        .icon("\u{1f4c1}")
                        .child(TreeNode::leaf("mod.rs").icon("\u{1f4c4}"))
                        .child(TreeNode::leaf("box.rs").icon("\u{1f4c4}"))
                        .child(TreeNode::leaf("text.rs").icon("\u{1f4c4}"))
                        .child(TreeNode::leaf("tree.rs").icon("\u{1f4c4}").color(Color::Green)),
                ),
        )
        .child(
            TreeNode::new("examples")
                .icon("\u{1f4c1}")
                .child(TreeNode::leaf("tree.rs").icon("\u{1f4c4}").color(Color::Cyan))
                .child(TreeNode::leaf("timer.rs").icon("\u{1f4c4}")),
        )
        .child(TreeNode::leaf("Cargo.toml").icon("\u{1f4e6}").color(Color::Blue))
        .child(TreeNode::leaf("README.md").icon("\u{1f4dd}").color(Color::Magenta));

    let state = TreeState::new()
        .expand("blaeck")
        .expand("src")
        .expand("components")
        .select("tree.rs");

    Element::node::<Box>(
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
    )
}
