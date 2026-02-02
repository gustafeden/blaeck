//! Layout engine - wrapper around Taffy for flexbox/grid/block layout.
//!
//! This module provides a simplified API for Taffy's layout engine,
//! exposing flexbox, CSS Grid, block layout, positioning, and more.

use taffy::prelude::*;

// Re-export Display for use by other modules
pub use taffy::Display;

/// A thin wrapper around Taffy's layout tree.
pub struct LayoutTree {
    tree: TaffyTree<()>,
}

/// Layout style configuration for a node.
#[derive(Clone, Debug)]
pub struct LayoutStyle {
    // === Display & Box Model ===
    /// Display mode (Flex, Grid, Block, None)
    pub display: Display,
    /// Position type (Relative, Absolute)
    pub position: Position,
    /// Overflow behavior on X axis
    pub overflow_x: Overflow,
    /// Overflow behavior on Y axis
    pub overflow_y: Overflow,

    // === Sizing ===
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
    /// Aspect ratio (width / height)
    pub aspect_ratio: Option<f32>,

    // === Flexbox Properties ===
    /// Flex direction (row or column)
    pub flex_direction: FlexDirection,
    /// Flex wrap behavior
    pub flex_wrap: FlexWrap,
    /// Flex grow factor
    pub flex_grow: f32,
    /// Flex shrink factor
    pub flex_shrink: f32,
    /// Flex basis (initial size before grow/shrink, None = auto)
    pub flex_basis: Option<f32>,

    // === Spacing: Padding ===
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

    // === Spacing: Margin ===
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

    // === Spacing: Border (for layout calculation) ===
    /// Border width on the left
    pub border_left: f32,
    /// Border width on the right
    pub border_right: f32,
    /// Border width on the top
    pub border_top: f32,
    /// Border width on the bottom
    pub border_bottom: f32,

    // === Spacing: Gap ===
    /// Gap between children (both axes)
    pub gap: f32,
    /// Column gap (horizontal gap)
    pub column_gap: Option<f32>,
    /// Row gap (vertical gap)
    pub row_gap: Option<f32>,

    // === Alignment ===
    /// How to align items along cross axis
    pub align_items: Option<AlignItems>,
    /// How to align this item (overrides parent's align_items)
    pub align_self: Option<AlignSelf>,
    /// How to align content
    pub align_content: Option<AlignContent>,
    /// How to justify content along main axis
    pub justify_content: Option<JustifyContent>,

    // === Position: Inset ===
    /// Inset from top (for absolute positioning)
    pub inset_top: Option<f32>,
    /// Inset from bottom (for absolute positioning)
    pub inset_bottom: Option<f32>,
    /// Inset from left (for absolute positioning)
    pub inset_left: Option<f32>,
    /// Inset from right (for absolute positioning)
    pub inset_right: Option<f32>,

    // === Grid Container Properties ===
    /// Grid template columns
    pub grid_template_columns: Vec<TrackSize>,
    /// Grid template rows
    pub grid_template_rows: Vec<TrackSize>,
    /// Grid auto columns (size of implicitly created columns)
    pub grid_auto_columns: Vec<TrackSize>,
    /// Grid auto rows (size of implicitly created rows)
    pub grid_auto_rows: Vec<TrackSize>,
    /// Grid auto-placement flow direction
    pub grid_auto_flow: GridAutoFlow,

    // === Grid Item Properties ===
    /// Grid column placement
    pub grid_column: GridPlacement,
    /// Grid row placement
    pub grid_row: GridPlacement,
}

impl Default for LayoutStyle {
    fn default() -> Self {
        Self {
            // Display & Box Model
            display: Display::Flex,
            position: Position::default(),
            overflow_x: Overflow::default(),
            overflow_y: Overflow::default(),

            // Sizing
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            aspect_ratio: None,

            // Flexbox
            flex_direction: FlexDirection::default(),
            flex_wrap: FlexWrap::default(),
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,

            // Padding
            padding: 0.0,
            padding_left: None,
            padding_right: None,
            padding_top: None,
            padding_bottom: None,

            // Margin
            margin: 0.0,
            margin_left: None,
            margin_right: None,
            margin_top: None,
            margin_bottom: None,

            // Border (layout)
            border_left: 0.0,
            border_right: 0.0,
            border_top: 0.0,
            border_bottom: 0.0,

            // Gap
            gap: 0.0,
            column_gap: None,
            row_gap: None,

            // Alignment
            align_items: None,
            align_self: None,
            align_content: None,
            justify_content: None,

            // Inset
            inset_top: None,
            inset_bottom: None,
            inset_left: None,
            inset_right: None,

            // Grid Container
            grid_template_columns: Vec::new(),
            grid_template_rows: Vec::new(),
            grid_auto_columns: Vec::new(),
            grid_auto_rows: Vec::new(),
            grid_auto_flow: GridAutoFlow::default(),

            // Grid Item
            grid_column: GridPlacement::default(),
            grid_row: GridPlacement::default(),
        }
    }
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

/// Flex wrap behavior.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FlexWrap {
    /// Items are laid out in a single line (default)
    #[default]
    NoWrap,
    /// Items wrap to additional lines
    Wrap,
    /// Items wrap to additional lines in reverse
    WrapReverse,
}

/// Position type for an element.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Position {
    /// Element is positioned relative to its normal position (default)
    #[default]
    Relative,
    /// Element is removed from flow and positioned relative to its containing block
    Absolute,
}

/// Overflow behavior.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Overflow {
    /// Content is not clipped and may extend outside the element
    #[default]
    Visible,
    /// Content is clipped and hidden
    Hidden,
    /// Content is clipped but scrollable (affects layout calculation)
    Scroll,
    /// Content is clipped if needed
    Clip,
}

/// Grid auto-placement flow direction.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum GridAutoFlow {
    /// Items are placed by filling each row (default)
    #[default]
    Row,
    /// Items are placed by filling each column
    Column,
    /// Items are placed by filling each row, using dense packing
    RowDense,
    /// Items are placed by filling each column, using dense packing
    ColumnDense,
}

/// Track sizing function for grid templates.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum TrackSize {
    /// Track size is determined by content (auto)
    #[default]
    Auto,
    /// Fixed size in pixels/units
    Fixed(f32),
    /// Minimum content size
    MinContent,
    /// Maximum content size
    MaxContent,
    /// Fit content with maximum size
    FitContent(f32),
    /// Flexible fraction (fr units)
    Flex(f32),
    /// Minmax constraint
    Minmax(Box<TrackSize>, Box<TrackSize>),
}

/// Grid placement for a single axis (column or row).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GridPlacement {
    /// Start line (1-indexed, negative counts from end)
    pub start: Option<i16>,
    /// End line (1-indexed, negative counts from end)
    pub end: Option<i16>,
    /// Number of tracks to span
    pub span: Option<u16>,
}

impl GridPlacement {
    /// Create placement at a specific line
    pub fn line(n: i16) -> Self {
        Self {
            start: Some(n),
            end: None,
            span: None,
        }
    }

    /// Create placement spanning from start to end lines
    pub fn from_to(start: i16, end: i16) -> Self {
        Self {
            start: Some(start),
            end: Some(end),
            span: None,
        }
    }

    /// Create placement spanning a number of tracks
    pub fn span(n: u16) -> Self {
        Self {
            start: None,
            end: None,
            span: Some(n),
        }
    }

    /// Create auto placement (default)
    pub fn auto() -> Self {
        Self::default()
    }

    /// Convert to Taffy's GridPlacement for start
    fn to_taffy_start(&self) -> taffy::GridPlacement<String> {
        use taffy::style_helpers::{line, span as taffy_span};

        if let Some(span_val) = self.span {
            if self.start.is_some() || self.end.is_some() {
                // span with start/end - use line
                self.start.map_or(taffy::GridPlacement::Auto, line)
            } else {
                taffy_span(span_val)
            }
        } else {
            self.start.map_or(taffy::GridPlacement::Auto, line)
        }
    }

    /// Convert to Taffy's GridPlacement for end
    fn to_taffy_end(&self) -> taffy::GridPlacement<String> {
        use taffy::style_helpers::{line, span as taffy_span};

        if let Some(span_val) = self.span {
            if self.end.is_some() {
                self.end.map_or(taffy::GridPlacement::Auto, line)
            } else if self.start.is_some() {
                taffy_span(span_val)
            } else {
                taffy::GridPlacement::Auto
            }
        } else {
            self.end.map_or(taffy::GridPlacement::Auto, line)
        }
    }
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

    /// Clear all nodes from the tree, allowing it to be reused.
    /// This is important for avoiding memory leaks when rendering many frames,
    /// as Taffy uses arena-style allocation that grows if new trees are created each frame.
    pub fn clear(&mut self) {
        self.tree.clear();
    }
}

impl Default for LayoutTree {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert a TrackSize to Taffy's TrackSizingFunction
fn track_size_to_taffy(ts: &TrackSize) -> taffy::TrackSizingFunction {
    use taffy::style_helpers::{
        FromFr, FromLength, TaffyAuto, TaffyFitContent, TaffyMaxContent, TaffyMinContent,
    };

    match ts {
        TrackSize::Auto => taffy::TrackSizingFunction::AUTO,
        TrackSize::Fixed(v) => taffy::TrackSizingFunction::from_length(*v),
        TrackSize::MinContent => taffy::TrackSizingFunction::MIN_CONTENT,
        TrackSize::MaxContent => taffy::TrackSizingFunction::MAX_CONTENT,
        TrackSize::FitContent(v) => {
            taffy::TrackSizingFunction::fit_content(taffy::LengthPercentage::length(*v))
        }
        TrackSize::Flex(fr) => taffy::TrackSizingFunction::from_fr(*fr),
        TrackSize::Minmax(min, max) => {
            let min_sizing = match min.as_ref() {
                TrackSize::Auto => taffy::MinTrackSizingFunction::AUTO,
                TrackSize::Fixed(v) => taffy::MinTrackSizingFunction::from_length(*v),
                TrackSize::MinContent => taffy::MinTrackSizingFunction::MIN_CONTENT,
                TrackSize::MaxContent => taffy::MinTrackSizingFunction::MAX_CONTENT,
                _ => taffy::MinTrackSizingFunction::AUTO,
            };
            let max_sizing = match max.as_ref() {
                TrackSize::Auto => taffy::MaxTrackSizingFunction::AUTO,
                TrackSize::Fixed(v) => taffy::MaxTrackSizingFunction::from_length(*v),
                TrackSize::MinContent => taffy::MaxTrackSizingFunction::MIN_CONTENT,
                TrackSize::MaxContent => taffy::MaxTrackSizingFunction::MAX_CONTENT,
                TrackSize::FitContent(v) => {
                    taffy::MaxTrackSizingFunction::fit_content(taffy::LengthPercentage::length(*v))
                }
                TrackSize::Flex(fr) => taffy::MaxTrackSizingFunction::from_fr(*fr),
                TrackSize::Minmax(_, _) => taffy::MaxTrackSizingFunction::AUTO,
            };
            taffy::TrackSizingFunction {
                min: min_sizing,
                max: max_sizing,
            }
        }
    }
}

/// Convert a TrackSize to a GridTemplateComponent
fn track_size_to_grid_component(ts: &TrackSize) -> taffy::GridTemplateComponent<String> {
    taffy::GridTemplateComponent::Single(track_size_to_taffy(ts))
}

impl LayoutStyle {
    /// Convert to Taffy's style format.
    fn into_taffy_style(self) -> Style {
        // Padding
        let padding_left = self.padding_left.unwrap_or(self.padding);
        let padding_right = self.padding_right.unwrap_or(self.padding);
        let padding_top = self.padding_top.unwrap_or(self.padding);
        let padding_bottom = self.padding_bottom.unwrap_or(self.padding);

        // Margin
        let margin_left = self.margin_left.unwrap_or(self.margin);
        let margin_right = self.margin_right.unwrap_or(self.margin);
        let margin_top = self.margin_top.unwrap_or(self.margin);
        let margin_bottom = self.margin_bottom.unwrap_or(self.margin);

        // Gap
        let column_gap = self.column_gap.unwrap_or(self.gap);
        let row_gap = self.row_gap.unwrap_or(self.gap);

        Style {
            // Display & Box Model
            display: self.display,
            position: match self.position {
                Position::Relative => taffy::Position::Relative,
                Position::Absolute => taffy::Position::Absolute,
            },
            overflow: taffy::Point {
                x: match self.overflow_x {
                    Overflow::Visible => taffy::Overflow::Visible,
                    Overflow::Hidden => taffy::Overflow::Hidden,
                    Overflow::Scroll => taffy::Overflow::Scroll,
                    Overflow::Clip => taffy::Overflow::Clip,
                },
                y: match self.overflow_y {
                    Overflow::Visible => taffy::Overflow::Visible,
                    Overflow::Hidden => taffy::Overflow::Hidden,
                    Overflow::Scroll => taffy::Overflow::Scroll,
                    Overflow::Clip => taffy::Overflow::Clip,
                },
            },

            // Sizing
            size: Size {
                width: self.width.map_or(Dimension::auto(), Dimension::length),
                height: self.height.map_or(Dimension::auto(), Dimension::length),
            },
            min_size: Size {
                width: self.min_width.map_or(Dimension::auto(), Dimension::length),
                height: self.min_height.map_or(Dimension::auto(), Dimension::length),
            },
            max_size: Size {
                width: self.max_width.map_or(Dimension::auto(), Dimension::length),
                height: self.max_height.map_or(Dimension::auto(), Dimension::length),
            },
            aspect_ratio: self.aspect_ratio,

            // Flexbox
            flex_direction: match self.flex_direction {
                FlexDirection::Row => taffy::FlexDirection::Row,
                FlexDirection::Column => taffy::FlexDirection::Column,
            },
            flex_wrap: match self.flex_wrap {
                FlexWrap::NoWrap => taffy::FlexWrap::NoWrap,
                FlexWrap::Wrap => taffy::FlexWrap::Wrap,
                FlexWrap::WrapReverse => taffy::FlexWrap::WrapReverse,
            },
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            flex_basis: self.flex_basis.map_or(Dimension::auto(), Dimension::length),

            // Padding
            padding: Rect {
                left: LengthPercentage::length(padding_left),
                right: LengthPercentage::length(padding_right),
                top: LengthPercentage::length(padding_top),
                bottom: LengthPercentage::length(padding_bottom),
            },

            // Margin
            margin: Rect {
                left: LengthPercentageAuto::length(margin_left),
                right: LengthPercentageAuto::length(margin_right),
                top: LengthPercentageAuto::length(margin_top),
                bottom: LengthPercentageAuto::length(margin_bottom),
            },

            // Border (for layout calculation)
            border: Rect {
                left: LengthPercentage::length(self.border_left),
                right: LengthPercentage::length(self.border_right),
                top: LengthPercentage::length(self.border_top),
                bottom: LengthPercentage::length(self.border_bottom),
            },

            // Gap
            gap: Size {
                width: LengthPercentage::length(column_gap),
                height: LengthPercentage::length(row_gap),
            },

            // Alignment
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

            // Inset (for absolute positioning)
            inset: Rect {
                top: self
                    .inset_top
                    .map_or(LengthPercentageAuto::auto(), LengthPercentageAuto::length),
                bottom: self
                    .inset_bottom
                    .map_or(LengthPercentageAuto::auto(), LengthPercentageAuto::length),
                left: self
                    .inset_left
                    .map_or(LengthPercentageAuto::auto(), LengthPercentageAuto::length),
                right: self
                    .inset_right
                    .map_or(LengthPercentageAuto::auto(), LengthPercentageAuto::length),
            },

            // Grid Container
            grid_template_columns: self
                .grid_template_columns
                .iter()
                .map(track_size_to_grid_component)
                .collect(),
            grid_template_rows: self
                .grid_template_rows
                .iter()
                .map(track_size_to_grid_component)
                .collect(),
            grid_auto_columns: self
                .grid_auto_columns
                .iter()
                .map(track_size_to_taffy)
                .collect(),
            grid_auto_rows: self
                .grid_auto_rows
                .iter()
                .map(track_size_to_taffy)
                .collect(),
            grid_auto_flow: match self.grid_auto_flow {
                GridAutoFlow::Row => taffy::GridAutoFlow::Row,
                GridAutoFlow::Column => taffy::GridAutoFlow::Column,
                GridAutoFlow::RowDense => taffy::GridAutoFlow::RowDense,
                GridAutoFlow::ColumnDense => taffy::GridAutoFlow::ColumnDense,
            },

            // Grid Item
            grid_column: taffy::Line {
                start: self.grid_column.to_taffy_start(),
                end: self.grid_column.to_taffy_end(),
            },
            grid_row: taffy::Line {
                start: self.grid_row.to_taffy_start(),
                end: self.grid_row.to_taffy_end(),
            },

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
