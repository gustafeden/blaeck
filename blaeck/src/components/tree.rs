//! Tree View component for hierarchical data display.
//!
//! ## When to use TreeView
//!
//! - File/directory structures
//! - Nested configuration or settings
//! - Any parent-child hierarchical data
//!
//! ## See also
//!
//! - [`Table`](super::Table) — Flat tabular data (non-hierarchical)
//! - [`Select`](super::Select) — Flat list selection
//!
//! # Example
//!
//! ```ignore
//! use blaeck::prelude::*;
//!
//! let tree = TreeNode::new("root")
//!     .child(TreeNode::new("src")
//!         .child(TreeNode::leaf("main.rs"))
//!         .child(TreeNode::leaf("lib.rs")))
//!     .child(TreeNode::leaf("Cargo.toml"));
//!
//! let state = TreeState::new().expand("root").expand("src");
//!
//! Element::node::<TreeView>(
//!     TreeViewProps::new(tree).state(state),
//!     vec![],
//! )
//! ```

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};
use std::collections::HashSet;

/// A node in the tree.
#[derive(Debug, Clone)]
pub struct TreeNode {
    /// Unique identifier for this node.
    pub id: String,
    /// Display label.
    pub label: String,
    /// Optional icon/prefix.
    pub icon: Option<String>,
    /// Child nodes.
    pub children: Vec<TreeNode>,
    /// Custom color for this node.
    pub color: Option<Color>,
    /// Whether this node is disabled.
    pub disabled: bool,
}

impl TreeNode {
    /// Create a new tree node.
    pub fn new(label: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            id: label.clone(),
            label,
            icon: None,
            children: Vec::new(),
            color: None,
            disabled: false,
        }
    }

    /// Create a new tree node with a specific ID.
    pub fn with_id(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            children: Vec::new(),
            color: None,
            disabled: false,
        }
    }

    /// Create a leaf node (no children).
    pub fn leaf(label: impl Into<String>) -> Self {
        Self::new(label)
    }

    /// Add a child node.
    #[must_use]
    pub fn child(mut self, node: TreeNode) -> Self {
        self.children.push(node);
        self
    }

    /// Add multiple children.
    #[must_use]
    pub fn children(mut self, nodes: Vec<TreeNode>) -> Self {
        self.children.extend(nodes);
        self
    }

    /// Set icon/prefix.
    #[must_use]
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set disabled state.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Check if this node has children.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Check if this is a leaf node.
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

/// State for tree view (expanded nodes, selected node).
#[derive(Debug, Clone, Default)]
pub struct TreeState {
    /// Set of expanded node IDs.
    pub expanded: HashSet<String>,
    /// Currently selected/focused node ID.
    pub selected: Option<String>,
}

impl TreeState {
    /// Create a new tree state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Expand a node.
    #[must_use]
    pub fn expand(mut self, id: impl Into<String>) -> Self {
        self.expanded.insert(id.into());
        self
    }

    /// Collapse a node.
    #[must_use]
    pub fn collapse(mut self, id: impl Into<String>) -> Self {
        self.expanded.remove(&id.into());
        self
    }

    /// Toggle a node's expanded state.
    pub fn toggle(&mut self, id: &str) {
        if self.expanded.contains(id) {
            self.expanded.remove(id);
        } else {
            self.expanded.insert(id.to_string());
        }
    }

    /// Check if a node is expanded.
    pub fn is_expanded(&self, id: &str) -> bool {
        self.expanded.contains(id)
    }

    /// Select a node.
    #[must_use]
    pub fn select(mut self, id: impl Into<String>) -> Self {
        self.selected = Some(id.into());
        self
    }

    /// Clear selection.
    #[must_use]
    pub fn clear_selection(mut self) -> Self {
        self.selected = None;
        self
    }

    /// Check if a node is selected.
    pub fn is_selected(&self, id: &str) -> bool {
        self.selected.as_ref().is_some_and(|s| s == id)
    }

    /// Expand all nodes in a tree.
    pub fn expand_all(&mut self, node: &TreeNode) {
        self.expanded.insert(node.id.clone());
        for child in &node.children {
            self.expand_all(child);
        }
    }

    /// Collapse all nodes.
    pub fn collapse_all(&mut self) {
        self.expanded.clear();
    }
}

/// Tree connector style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TreeConnectors {
    /// Unicode box drawing: ├── └── │
    #[default]
    Unicode,
    /// ASCII characters: |-- `-- |
    Ascii,
    /// Simple indent only.
    Indent,
    /// No connectors.
    None,
}

impl TreeConnectors {
    /// Get connector strings: (branch, last, vertical, space).
    pub fn chars(&self) -> (&'static str, &'static str, &'static str, &'static str) {
        match self {
            TreeConnectors::Unicode => ("├── ", "└── ", "│   ", "    "),
            TreeConnectors::Ascii => ("|-- ", "`-- ", "|   ", "    "),
            TreeConnectors::Indent => ("  ", "  ", "  ", "  "),
            TreeConnectors::None => ("", "", "", ""),
        }
    }
}

/// Properties for the TreeView component.
#[derive(Debug, Clone)]
pub struct TreeViewProps {
    /// Root node of the tree.
    pub root: TreeNode,
    /// Tree state (expanded nodes, selection).
    pub state: TreeState,
    /// Whether to show the root node.
    pub show_root: bool,
    /// Connector style.
    pub connectors: TreeConnectors,
    /// Color for branch nodes.
    pub branch_color: Option<Color>,
    /// Color for leaf nodes.
    pub leaf_color: Option<Color>,
    /// Color for selected node.
    pub selected_color: Option<Color>,
    /// Color for disabled nodes.
    pub disabled_color: Option<Color>,
    /// Expand/collapse indicators.
    pub expand_indicator: String,
    pub collapse_indicator: String,
    /// Show indicators for expandable nodes.
    pub show_indicators: bool,
    /// Indent size (in spaces, for Indent connector style).
    pub indent_size: usize,
}

impl Default for TreeViewProps {
    fn default() -> Self {
        Self {
            root: TreeNode::new("root"),
            state: TreeState::new(),
            show_root: true,
            connectors: TreeConnectors::Unicode,
            branch_color: None,
            leaf_color: None,
            selected_color: Some(Color::Cyan),
            disabled_color: Some(Color::DarkGray),
            expand_indicator: "▶".to_string(),
            collapse_indicator: "▼".to_string(),
            show_indicators: true,
            indent_size: 2,
        }
    }
}

impl TreeViewProps {
    /// Create new TreeViewProps with a root node.
    pub fn new(root: TreeNode) -> Self {
        Self {
            root,
            ..Default::default()
        }
    }

    /// Set tree state.
    #[must_use]
    pub fn state(mut self, state: TreeState) -> Self {
        self.state = state;
        self
    }

    /// Show or hide root node.
    #[must_use]
    pub fn show_root(mut self, show: bool) -> Self {
        self.show_root = show;
        self
    }

    /// Set connector style.
    #[must_use]
    pub fn connectors(mut self, style: TreeConnectors) -> Self {
        self.connectors = style;
        self
    }

    /// Set branch color.
    #[must_use]
    pub fn branch_color(mut self, color: Color) -> Self {
        self.branch_color = Some(color);
        self
    }

    /// Set leaf color.
    #[must_use]
    pub fn leaf_color(mut self, color: Color) -> Self {
        self.leaf_color = Some(color);
        self
    }

    /// Set selected color.
    #[must_use]
    pub fn selected_color(mut self, color: Color) -> Self {
        self.selected_color = Some(color);
        self
    }

    /// Set expand/collapse indicators.
    #[must_use]
    pub fn indicators(mut self, expand: impl Into<String>, collapse: impl Into<String>) -> Self {
        self.expand_indicator = expand.into();
        self.collapse_indicator = collapse.into();
        self
    }

    /// Show or hide expand/collapse indicators.
    #[must_use]
    pub fn show_indicators(mut self, show: bool) -> Self {
        self.show_indicators = show;
        self
    }
}

/// A component that displays hierarchical data as a tree.
pub struct TreeView;

impl TreeView {
    /// Render a node and its children recursively.
    fn render_node(
        node: &TreeNode,
        props: &TreeViewProps,
        prefix: &str,
        is_last: bool,
        is_root: bool,
        lines: &mut Vec<Element>,
    ) {
        let (branch, last, vertical, space) = props.connectors.chars();

        // Determine the connector for this node
        let connector = if is_root {
            "".to_string()
        } else if is_last {
            last.to_string()
        } else {
            branch.to_string()
        };

        // Build the line content
        if is_root && !props.show_root {
            // Skip rendering root, just render children
        } else {
            let mut line_content = String::new();
            line_content.push_str(prefix);
            line_content.push_str(&connector);

            // Add expand/collapse indicator
            if props.show_indicators && node.has_children() {
                if props.state.is_expanded(&node.id) {
                    line_content.push_str(&props.collapse_indicator);
                } else {
                    line_content.push_str(&props.expand_indicator);
                }
                line_content.push(' ');
            }

            // Add icon if present
            if let Some(ref icon) = node.icon {
                line_content.push_str(icon);
                line_content.push(' ');
            }

            // Add label
            line_content.push_str(&node.label);

            // Determine style
            let mut style = Style::new();

            if node.disabled {
                if let Some(color) = props.disabled_color {
                    style = style.fg(color);
                }
                style = style.add_modifier(Modifier::DIM);
            } else if props.state.is_selected(&node.id) {
                if let Some(color) = props.selected_color {
                    style = style.fg(color);
                }
                style = style.add_modifier(Modifier::BOLD);
            } else if let Some(color) = node.color {
                style = style.fg(color);
            } else if node.has_children() {
                if let Some(color) = props.branch_color {
                    style = style.fg(color);
                }
            } else if let Some(color) = props.leaf_color {
                style = style.fg(color);
            }

            lines.push(Element::styled_text(&line_content, style));
        }

        // Render children if expanded (or if root and not showing root)
        let should_render_children =
            (is_root && !props.show_root) || props.state.is_expanded(&node.id);

        if should_render_children {
            let child_count = node.children.len();
            for (i, child) in node.children.iter().enumerate() {
                let is_last_child = i == child_count - 1;

                // Calculate prefix for children
                let child_prefix = if is_root && !props.show_root {
                    prefix.to_string()
                } else if is_root {
                    "".to_string()
                } else {
                    let continuation = if is_last { space } else { vertical };
                    format!("{}{}", prefix, continuation)
                };

                Self::render_node(child, props, &child_prefix, is_last_child, false, lines);
            }
        }
    }
}

impl Component for TreeView {
    type Props = TreeViewProps;

    fn render(props: &Self::Props) -> Element {
        let mut lines: Vec<Element> = Vec::new();
        Self::render_node(&props.root, props, "", true, true, &mut lines);

        if lines.is_empty() {
            Element::Empty
        } else if lines.len() == 1 {
            lines.remove(0)
        } else {
            Element::Fragment(lines)
        }
    }
}

/// Helper to create a simple tree view.
pub fn tree_view(root: TreeNode, state: &TreeState) -> Element {
    TreeView::render(&TreeViewProps::new(root).state(state.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_new() {
        let node = TreeNode::new("test");
        assert_eq!(node.label, "test");
        assert_eq!(node.id, "test");
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_tree_node_with_id() {
        let node = TreeNode::with_id("id1", "Label");
        assert_eq!(node.id, "id1");
        assert_eq!(node.label, "Label");
    }

    #[test]
    fn test_tree_node_child() {
        let node = TreeNode::new("parent").child(TreeNode::new("child"));
        assert_eq!(node.children.len(), 1);
        assert!(node.has_children());
    }

    #[test]
    fn test_tree_node_leaf() {
        let node = TreeNode::leaf("file.txt");
        assert!(node.is_leaf());
        assert!(!node.has_children());
    }

    #[test]
    fn test_tree_state_expand() {
        let state = TreeState::new().expand("node1");
        assert!(state.is_expanded("node1"));
        assert!(!state.is_expanded("node2"));
    }

    #[test]
    fn test_tree_state_collapse() {
        let state = TreeState::new().expand("node1").collapse("node1");
        assert!(!state.is_expanded("node1"));
    }

    #[test]
    fn test_tree_state_toggle() {
        let mut state = TreeState::new();
        state.toggle("node1");
        assert!(state.is_expanded("node1"));
        state.toggle("node1");
        assert!(!state.is_expanded("node1"));
    }

    #[test]
    fn test_tree_state_select() {
        let state = TreeState::new().select("node1");
        assert!(state.is_selected("node1"));
        assert!(!state.is_selected("node2"));
    }

    #[test]
    fn test_tree_connectors() {
        let (branch, last, vert, space) = TreeConnectors::Unicode.chars();
        assert_eq!(branch, "├── ");
        assert_eq!(last, "└── ");
        assert_eq!(vert, "│   ");
        assert_eq!(space, "    ");
    }

    #[test]
    fn test_tree_view_props() {
        let root = TreeNode::new("root").child(TreeNode::leaf("child"));
        let props = TreeViewProps::new(root.clone()).show_root(false);
        assert!(!props.show_root);
    }

    #[test]
    fn test_tree_view_render_simple() {
        let root = TreeNode::new("root").child(TreeNode::leaf("child"));
        let state = TreeState::new().expand("root");
        let props = TreeViewProps::new(root).state(state);
        let elem = TreeView::render(&props);
        assert!(elem.is_fragment());
    }

    #[test]
    fn test_tree_view_render_collapsed() {
        let root = TreeNode::new("root").child(TreeNode::leaf("child"));
        let state = TreeState::new(); // Not expanded
        let props = TreeViewProps::new(root).state(state);
        let elem = TreeView::render(&props);
        // Should only render root since it's collapsed
        assert!(elem.is_text());
    }

    #[test]
    fn test_tree_view_render_no_root() {
        let root = TreeNode::new("root").child(TreeNode::leaf("child"));
        let state = TreeState::new();
        let props = TreeViewProps::new(root).state(state).show_root(false);
        let elem = TreeView::render(&props);
        // Should render child only
        assert!(elem.is_text());
    }

    #[test]
    fn test_tree_view_helper() {
        let root = TreeNode::new("root");
        let state = TreeState::new();
        let elem = tree_view(root, &state);
        assert!(elem.is_text());
    }

    #[test]
    fn test_tree_state_expand_all() {
        let root = TreeNode::new("root")
            .child(TreeNode::new("a").child(TreeNode::leaf("a1")))
            .child(TreeNode::leaf("b"));
        let mut state = TreeState::new();
        state.expand_all(&root);
        assert!(state.is_expanded("root"));
        assert!(state.is_expanded("a"));
    }

    #[test]
    fn test_tree_state_collapse_all() {
        let mut state = TreeState::new().expand("a").expand("b");
        state.collapse_all();
        assert!(!state.is_expanded("a"));
        assert!(!state.is_expanded("b"));
    }
}
