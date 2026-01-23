//! Layout engine - thin wrapper around Taffy for flexbox layout.
//!
//! This module provides a simplified API for Taffy's flexbox layout engine,
//! similar to how Iocraft wraps Taffy in their render.rs.

use taffy::prelude::*;

/// A thin wrapper around Taffy's layout tree.
pub struct LayoutTree {
    tree: TaffyTree<()>,
}

/// Layout style configuration for a node.
#[derive(Default, Clone, Debug)]
pub struct LayoutStyle {
    /// Fixed width (if set)
    pub width: Option<f32>,
    /// Fixed height (if set)
    pub height: Option<f32>,
    /// Minimum width constraint
    pub min_width: Option<f32>,
    /// Minimum height constraint
    pub min_height: Option<f32>,
    /// Maximum width constraint
    pub max_width: Option<f32>,
    /// Maximum height constraint
    pub max_height: Option<f32>,
    /// Flex direction (row or column)
    pub flex_direction: FlexDirection,
    /// Flex grow factor
    pub flex_grow: f32,
    /// Flex shrink factor
    pub flex_shrink: f32,
    /// Padding on all sides
    pub padding: f32,
    /// Padding on the left
    pub padding_left: Option<f32>,
    /// Padding on the right
    pub padding_right: Option<f32>,
    /// Padding on the top
    pub padding_top: Option<f32>,
    /// Padding on the bottom
    pub padding_bottom: Option<f32>,
    /// Margin on all sides
    pub margin: f32,
    /// Margin on the left
    pub margin_left: Option<f32>,
    /// Margin on the right
    pub margin_right: Option<f32>,
    /// Margin on the top
    pub margin_top: Option<f32>,
    /// Margin on the bottom
    pub margin_bottom: Option<f32>,
    /// Gap between children
    pub gap: f32,
    /// How to align items along cross axis
    pub align_items: Option<AlignItems>,
    /// How to align this item (overrides parent's align_items)
    pub align_self: Option<AlignSelf>,
    /// How to align content
    pub align_content: Option<AlignContent>,
    /// How to justify content along main axis
    pub justify_content: Option<JustifyContent>,
}

/// Flex direction (row or column)
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexDirection {
    /// Items laid out in a row (horizontal)
    Row,
    /// Items laid out in a column (vertical) - default for terminals
    #[default]
    Column,
}

/// How to align items along the cross axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignItems {
    /// Align items to the start of the cross axis
    Start,
    /// Align items to the end of the cross axis
    End,
    /// Center items on the cross axis
    Center,
    /// Stretch items to fill the cross axis
    Stretch,
}

/// How to align this item along the cross axis (overrides parent's align_items).
/// Use `None` for auto (inherit from parent).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignSelf {
    /// Align to the start of the cross axis
    Start,
    /// Align to the end of the cross axis
    End,
    /// Center on the cross axis
    Center,
    /// Stretch to fill the cross axis
    Stretch,
}

/// How to distribute content along the cross axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignContent {
    /// Pack content at the start
    Start,
    /// Pack content at the end
    End,
    /// Center content
    Center,
    /// Stretch to fill
    Stretch,
    /// Distribute with space between
    SpaceBetween,
    /// Distribute with space around
    SpaceAround,
    /// Distribute with even space
    SpaceEvenly,
}

/// How to justify content along the main axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JustifyContent {
    /// Pack content at the start
    Start,
    /// Pack content at the end
    End,
    /// Center content
    Center,
    /// Distribute with space between
    SpaceBetween,
    /// Distribute with space around
    SpaceAround,
    /// Distribute with even space
    SpaceEvenly,
}

/// Computed layout result for a node.
#[derive(Default, Clone, Copy, Debug)]
pub struct LayoutResult {
    /// X position relative to parent
    pub x: f32,
    /// Y position relative to parent
    pub y: f32,
    /// Computed width
    pub width: f32,
    /// Computed height
    pub height: f32,
}

impl LayoutTree {
    /// Create a new layout tree.
    pub fn new() -> Self {
        Self {
            tree: TaffyTree::new(),
        }
    }

    /// Create a new leaf node with the given style.
    pub fn new_leaf(&mut self, style: LayoutStyle) -> Result<NodeId, taffy::TaffyError> {
        let taffy_style = style.into_taffy_style();
        self.tree.new_leaf(taffy_style)
    }

    /// Create a new node with children.
    pub fn new_with_children(
        &mut self,
        style: LayoutStyle,
        children: &[NodeId],
    ) -> Result<NodeId, taffy::TaffyError> {
        let taffy_style = style.into_taffy_style();
        self.tree.new_with_children(taffy_style, children)
    }

    /// Compute layout for the tree starting at the given root node.
    pub fn compute(&mut self, root: NodeId, available_width: f32, available_height: f32) {
        self.tree
            .compute_layout(
                root,
                Size {
                    width: AvailableSpace::Definite(available_width),
                    height: AvailableSpace::Definite(available_height),
                },
            )
            .expect("layout computation should succeed");
    }

    /// Get the computed layout for a node.
    pub fn get_layout(&self, node: NodeId) -> LayoutResult {
        let layout = self.tree.layout(node).expect("node should exist");
        LayoutResult {
            x: layout.location.x,
            y: layout.location.y,
            width: layout.size.width,
            height: layout.size.height,
        }
    }

    /// Set the style of a node.
    pub fn set_style(&mut self, node: NodeId, style: LayoutStyle) -> Result<(), taffy::TaffyError> {
        self.tree.set_style(node, style.into_taffy_style())
    }

    /// Set the children of a node.
    pub fn set_children(
        &mut self,
        node: NodeId,
        children: &[NodeId],
    ) -> Result<(), taffy::TaffyError> {
        self.tree.set_children(node, children)
    }

    /// Remove a node from the tree.
    pub fn remove(&mut self, node: NodeId) -> Result<NodeId, taffy::TaffyError> {
        self.tree.remove(node)
    }

    /// Get the children of a node.
    pub fn children(&self, node: NodeId) -> Vec<NodeId> {
        self.tree.children(node).unwrap_or_default()
    }
}

impl Default for LayoutTree {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutStyle {
    /// Convert to Taffy's style format.
    fn into_taffy_style(self) -> Style {
        let padding_left = self.padding_left.unwrap_or(self.padding);
        let padding_right = self.padding_right.unwrap_or(self.padding);
        let padding_top = self.padding_top.unwrap_or(self.padding);
        let padding_bottom = self.padding_bottom.unwrap_or(self.padding);

        let margin_left = self.margin_left.unwrap_or(self.margin);
        let margin_right = self.margin_right.unwrap_or(self.margin);
        let margin_top = self.margin_top.unwrap_or(self.margin);
        let margin_bottom = self.margin_bottom.unwrap_or(self.margin);

        Style {
            size: Size {
                width: self.width.map_or(Dimension::Auto, Dimension::Length),
                height: self.height.map_or(Dimension::Auto, Dimension::Length),
            },
            min_size: Size {
                width: self.min_width.map_or(Dimension::Auto, Dimension::Length),
                height: self.min_height.map_or(Dimension::Auto, Dimension::Length),
            },
            max_size: Size {
                width: self.max_width.map_or(Dimension::Auto, Dimension::Length),
                height: self.max_height.map_or(Dimension::Auto, Dimension::Length),
            },
            flex_direction: match self.flex_direction {
                FlexDirection::Row => taffy::FlexDirection::Row,
                FlexDirection::Column => taffy::FlexDirection::Column,
            },
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            padding: Rect {
                left: LengthPercentage::Length(padding_left),
                right: LengthPercentage::Length(padding_right),
                top: LengthPercentage::Length(padding_top),
                bottom: LengthPercentage::Length(padding_bottom),
            },
            margin: Rect {
                left: LengthPercentageAuto::Length(margin_left),
                right: LengthPercentageAuto::Length(margin_right),
                top: LengthPercentageAuto::Length(margin_top),
                bottom: LengthPercentageAuto::Length(margin_bottom),
            },
            gap: Size {
                width: LengthPercentage::Length(self.gap),
                height: LengthPercentage::Length(self.gap),
            },
            align_items: self.align_items.map(|a| match a {
                AlignItems::Start => taffy::AlignItems::Start,
                AlignItems::End => taffy::AlignItems::End,
                AlignItems::Center => taffy::AlignItems::Center,
                AlignItems::Stretch => taffy::AlignItems::Stretch,
            }),
            align_self: self.align_self.map(|a| match a {
                AlignSelf::Start => taffy::AlignSelf::Start,
                AlignSelf::End => taffy::AlignSelf::End,
                AlignSelf::Center => taffy::AlignSelf::Center,
                AlignSelf::Stretch => taffy::AlignSelf::Stretch,
            }),
            align_content: self.align_content.map(|a| match a {
                AlignContent::Start => taffy::AlignContent::Start,
                AlignContent::End => taffy::AlignContent::End,
                AlignContent::Center => taffy::AlignContent::Center,
                AlignContent::Stretch => taffy::AlignContent::Stretch,
                AlignContent::SpaceBetween => taffy::AlignContent::SpaceBetween,
                AlignContent::SpaceAround => taffy::AlignContent::SpaceAround,
                AlignContent::SpaceEvenly => taffy::AlignContent::SpaceEvenly,
            }),
            justify_content: self.justify_content.map(|j| match j {
                JustifyContent::Start => taffy::JustifyContent::Start,
                JustifyContent::End => taffy::JustifyContent::End,
                JustifyContent::Center => taffy::JustifyContent::Center,
                JustifyContent::SpaceBetween => taffy::JustifyContent::SpaceBetween,
                JustifyContent::SpaceAround => taffy::JustifyContent::SpaceAround,
                JustifyContent::SpaceEvenly => taffy::JustifyContent::SpaceEvenly,
            }),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_single_node() {
        let mut tree = LayoutTree::new();
        let root = tree.new_leaf(LayoutStyle::default()).unwrap();
        tree.compute(root, 80.0, 24.0);
        let layout = tree.get_layout(root);
        // A leaf node with no size constraints will be auto-sized
        // When computed with available space, it takes up available space
        // Actually, for a leaf with no content, Taffy may give it zero size
        assert!(layout.width >= 0.0);
        assert!(layout.height >= 0.0);
    }

    #[test]
    fn test_layout_fixed_size() {
        let mut tree = LayoutTree::new();
        let root = tree
            .new_leaf(LayoutStyle {
                width: Some(40.0),
                height: Some(10.0),
                ..Default::default()
            })
            .unwrap();
        tree.compute(root, 80.0, 24.0);
        let layout = tree.get_layout(root);
        assert_eq!(layout.width, 40.0);
        assert_eq!(layout.height, 10.0);
    }

    #[test]
    fn test_layout_row_flex() {
        let mut tree = LayoutTree::new();
        let c1 = tree
            .new_leaf(LayoutStyle {
                flex_grow: 1.0,
                ..Default::default()
            })
            .unwrap();
        let c2 = tree
            .new_leaf(LayoutStyle {
                flex_grow: 1.0,
                ..Default::default()
            })
            .unwrap();
        let root = tree
            .new_with_children(
                LayoutStyle {
                    flex_direction: FlexDirection::Row,
                    width: Some(80.0),
                    height: Some(24.0),
                    ..Default::default()
                },
                &[c1, c2],
            )
            .unwrap();

        tree.compute(root, 80.0, 24.0);

        assert_eq!(tree.get_layout(c1).width, 40.0);
        assert_eq!(tree.get_layout(c2).width, 40.0);
        assert_eq!(tree.get_layout(c2).x, 40.0);
    }

    #[test]
    fn test_layout_column() {
        let mut tree = LayoutTree::new();
        let c1 = tree
            .new_leaf(LayoutStyle {
                flex_grow: 1.0,
                ..Default::default()
            })
            .unwrap();
        let c2 = tree
            .new_leaf(LayoutStyle {
                flex_grow: 1.0,
                ..Default::default()
            })
            .unwrap();
        let root = tree
            .new_with_children(
                LayoutStyle {
                    flex_direction: FlexDirection::Column,
                    width: Some(80.0),
                    height: Some(24.0),
                    ..Default::default()
                },
                &[c1, c2],
            )
            .unwrap();

        tree.compute(root, 80.0, 24.0);

        assert_eq!(tree.get_layout(c1).height, 12.0);
        assert_eq!(tree.get_layout(c2).height, 12.0);
        assert_eq!(tree.get_layout(c2).y, 12.0);
    }

    #[test]
    fn test_layout_padding() {
        let mut tree = LayoutTree::new();
        let child = tree
            .new_leaf(LayoutStyle {
                flex_grow: 1.0,
                ..Default::default()
            })
            .unwrap();
        let root = tree
            .new_with_children(
                LayoutStyle {
                    width: Some(80.0),
                    height: Some(24.0),
                    padding: 2.0,
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        tree.compute(root, 80.0, 24.0);

        let cl = tree.get_layout(child);
        assert_eq!(cl.x, 2.0);
        assert_eq!(cl.y, 2.0);
        assert_eq!(cl.width, 76.0); // 80 - 2 - 2
        assert_eq!(cl.height, 20.0); // 24 - 2 - 2
    }

    #[test]
    fn test_layout_margin() {
        let mut tree = LayoutTree::new();
        let child = tree
            .new_leaf(LayoutStyle {
                width: Some(20.0),
                height: Some(10.0),
                margin: 5.0,
                ..Default::default()
            })
            .unwrap();
        let root = tree
            .new_with_children(
                LayoutStyle {
                    width: Some(80.0),
                    height: Some(24.0),
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        tree.compute(root, 80.0, 24.0);

        let cl = tree.get_layout(child);
        // Child should be offset by margin
        assert_eq!(cl.x, 5.0);
        assert_eq!(cl.y, 5.0);
        assert_eq!(cl.width, 20.0);
        assert_eq!(cl.height, 10.0);
    }

    #[test]
    fn test_layout_gap() {
        let mut tree = LayoutTree::new();
        let c1 = tree
            .new_leaf(LayoutStyle {
                width: Some(20.0),
                height: Some(10.0),
                ..Default::default()
            })
            .unwrap();
        let c2 = tree
            .new_leaf(LayoutStyle {
                width: Some(20.0),
                height: Some(10.0),
                ..Default::default()
            })
            .unwrap();
        let root = tree
            .new_with_children(
                LayoutStyle {
                    flex_direction: FlexDirection::Row,
                    width: Some(80.0),
                    height: Some(24.0),
                    gap: 10.0,
                    ..Default::default()
                },
                &[c1, c2],
            )
            .unwrap();

        tree.compute(root, 80.0, 24.0);

        assert_eq!(tree.get_layout(c1).x, 0.0);
        // Second child should be at x = 20 (c1 width) + 10 (gap) = 30
        assert_eq!(tree.get_layout(c2).x, 30.0);
    }

    #[test]
    fn test_layout_justify_content_center() {
        let mut tree = LayoutTree::new();
        let child = tree
            .new_leaf(LayoutStyle {
                width: Some(20.0),
                height: Some(10.0),
                ..Default::default()
            })
            .unwrap();
        let root = tree
            .new_with_children(
                LayoutStyle {
                    flex_direction: FlexDirection::Row,
                    width: Some(80.0),
                    height: Some(24.0),
                    justify_content: Some(JustifyContent::Center),
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        tree.compute(root, 80.0, 24.0);

        let cl = tree.get_layout(child);
        // Child should be centered: (80 - 20) / 2 = 30
        assert_eq!(cl.x, 30.0);
    }

    #[test]
    fn test_layout_align_items_center() {
        let mut tree = LayoutTree::new();
        let child = tree
            .new_leaf(LayoutStyle {
                width: Some(20.0),
                height: Some(10.0),
                ..Default::default()
            })
            .unwrap();
        let root = tree
            .new_with_children(
                LayoutStyle {
                    flex_direction: FlexDirection::Row,
                    width: Some(80.0),
                    height: Some(24.0),
                    align_items: Some(AlignItems::Center),
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        tree.compute(root, 80.0, 24.0);

        let cl = tree.get_layout(child);
        // Child should be vertically centered: (24 - 10) / 2 = 7
        assert_eq!(cl.y, 7.0);
    }

    #[test]
    fn test_layout_nested() {
        let mut tree = LayoutTree::new();

        // Create a nested structure: root -> container -> [item1, item2]
        let item1 = tree
            .new_leaf(LayoutStyle {
                flex_grow: 1.0,
                ..Default::default()
            })
            .unwrap();
        let item2 = tree
            .new_leaf(LayoutStyle {
                flex_grow: 1.0,
                ..Default::default()
            })
            .unwrap();
        let container = tree
            .new_with_children(
                LayoutStyle {
                    flex_direction: FlexDirection::Row,
                    flex_grow: 1.0,
                    ..Default::default()
                },
                &[item1, item2],
            )
            .unwrap();
        let root = tree
            .new_with_children(
                LayoutStyle {
                    flex_direction: FlexDirection::Column,
                    width: Some(80.0),
                    height: Some(24.0),
                    padding: 2.0,
                    ..Default::default()
                },
                &[container],
            )
            .unwrap();

        tree.compute(root, 80.0, 24.0);

        let container_layout = tree.get_layout(container);
        assert_eq!(container_layout.x, 2.0);
        assert_eq!(container_layout.y, 2.0);
        assert_eq!(container_layout.width, 76.0);
        assert_eq!(container_layout.height, 20.0);

        // Items should each take half of container width
        assert_eq!(tree.get_layout(item1).width, 38.0);
        assert_eq!(tree.get_layout(item2).width, 38.0);
        assert_eq!(tree.get_layout(item2).x, 38.0);
    }

    #[test]
    fn test_flex_direction_default_is_column() {
        assert_eq!(FlexDirection::default(), FlexDirection::Column);
    }

    #[test]
    fn test_layout_style_default() {
        let style = LayoutStyle::default();
        assert!(style.width.is_none());
        assert!(style.height.is_none());
        assert_eq!(style.flex_direction, FlexDirection::Column);
        assert_eq!(style.flex_grow, 0.0);
        assert_eq!(style.padding, 0.0);
    }

    #[test]
    fn test_layout_min_max_constraints() {
        let mut tree = LayoutTree::new();
        let child = tree
            .new_leaf(LayoutStyle {
                flex_grow: 1.0,
                max_width: Some(50.0),
                max_height: Some(15.0),
                ..Default::default()
            })
            .unwrap();
        let root = tree
            .new_with_children(
                LayoutStyle {
                    width: Some(80.0),
                    height: Some(24.0),
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        tree.compute(root, 80.0, 24.0);

        let cl = tree.get_layout(child);
        // Child width should be constrained to max_width
        assert!(cl.width <= 50.0);
        // Child height should be constrained to max_height
        assert!(cl.height <= 15.0);
    }
}
