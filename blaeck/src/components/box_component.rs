//! Box component - a container with optional border and layout properties.
//!
//! **Study this component first** when learning how Blaeck components work.
//! It demonstrates all the key patterns: props struct, builder methods, Component trait.
//!
//! The Box component is the primary container component in Blaeck, similar to
//! the Box component in Ink. It supports flexbox layout and optional borders.
//!
//! ## When to use Box
//!
//! - Grouping related elements together
//! - Adding borders or padding around content
//! - Creating row/column layouts with flexbox
//! - Controlling spacing with gap, margin, padding
//!
//! ## See also
//!
//! - [`Spacer`](super::Spacer) — Fills available space (use inside Box)
//! - [`Text`](super::Text) — Text content (common Box child)
//! - [`Modal`](super::Modal) — Pre-styled dialog boxes (built on Box)
//!
//! # Stable Layout Patterns
//!
//! When building multi-phase UIs (welcome → loading → results), maintain a **stable layout
//! structure** across states. This prevents jarring visual shifts when content changes.
//!
//! ## Bad: Different layouts per phase
//!
//! ```ignore
//! // DON'T DO THIS - causes layout shifts
//! match state.phase {
//!     Phase::Welcome => build_welcome_ui(),    // has banner
//!     Phase::Loading => build_loading_ui(),    // no banner, different structure!
//! }
//! ```
//!
//! ## Good: Stable structure, dynamic content
//!
//! ```ignore
//! // DO THIS - stable layout, content updates in place
//! Element::node::<Box>(BoxProps::column(), vec![
//!     build_header(&state),           // always present, text changes
//!     build_content(&state),          // always present, content changes
//!     build_progress(&state),         // always present, hidden when not needed
//!     build_footer(&state),           // always present, instructions change
//! ])
//! ```
//!
//! ## Using visibility to reserve space
//!
//! Use `visible: false` to hide content while preserving its layout space:
//!
//! ```ignore
//! Element::node::<Box>(
//!     BoxProps::column()
//!         .with_visible(state.phase == Phase::Loading),  // invisible until loading
//!     vec![progress_bar]
//! )
//! ```
//!
//! This is similar to CSS `visibility: hidden` - the element takes up space but
//! renders nothing, preventing layout shifts when it appears.

use crate::element::{Component, Element};
use crate::layout::{
    AlignContent, AlignItems, AlignSelf, FlexDirection, JustifyContent, LayoutStyle,
};
use crate::style::Color;

/// Border character set for drawing box borders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorderChars {
    /// Top-left corner character
    pub top_left: char,
    /// Top-right corner character
    pub top_right: char,
    /// Bottom-left corner character
    pub bottom_left: char,
    /// Bottom-right corner character
    pub bottom_right: char,
    /// Horizontal border character
    pub horizontal: char,
    /// Vertical border character
    pub vertical: char,
}

impl Default for BorderChars {
    fn default() -> Self {
        BorderStyle::Single.chars()
    }
}

/// Border style for the Box component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderStyle {
    /// No border
    #[default]
    None,
    /// Single line border: ┌─┐│└─┘
    Single,
    /// Double line border: ╔═╗║╚═╝
    Double,
    /// Rounded corners: ╭─╮│╰─╯
    Round,
    /// Bold line border: ┏━┓┃┗━┛
    Bold,
    /// Classic ASCII border: +-+|+-+
    Classic,
    /// Custom border characters
    Custom(BorderChars),
}

impl BorderStyle {
    /// Get the border characters for this style.
    pub fn chars(self) -> BorderChars {
        match self {
            BorderStyle::None => BorderChars {
                top_left: ' ',
                top_right: ' ',
                bottom_left: ' ',
                bottom_right: ' ',
                horizontal: ' ',
                vertical: ' ',
            },
            BorderStyle::Single => BorderChars {
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
                horizontal: '─',
                vertical: '│',
            },
            BorderStyle::Double => BorderChars {
                top_left: '╔',
                top_right: '╗',
                bottom_left: '╚',
                bottom_right: '╝',
                horizontal: '═',
                vertical: '║',
            },
            BorderStyle::Round => BorderChars {
                top_left: '╭',
                top_right: '╮',
                bottom_left: '╰',
                bottom_right: '╯',
                horizontal: '─',
                vertical: '│',
            },
            BorderStyle::Bold => BorderChars {
                top_left: '┏',
                top_right: '┓',
                bottom_left: '┗',
                bottom_right: '┛',
                horizontal: '━',
                vertical: '┃',
            },
            BorderStyle::Classic => BorderChars {
                top_left: '+',
                top_right: '+',
                bottom_left: '+',
                bottom_right: '+',
                horizontal: '-',
                vertical: '|',
            },
            BorderStyle::Custom(chars) => chars,
        }
    }

    /// Returns true if this border style has a visible border.
    pub fn has_border(self) -> bool {
        !matches!(self, BorderStyle::None)
    }
}

/// Per-side border visibility configuration.
#[derive(Debug, Clone, Copy, Default)]
pub struct BorderSides {
    /// Show top border
    pub top: bool,
    /// Show bottom border
    pub bottom: bool,
    /// Show left border
    pub left: bool,
    /// Show right border
    pub right: bool,
}

impl BorderSides {
    /// All sides visible (default when border_style is set).
    pub fn all() -> Self {
        Self {
            top: true,
            bottom: true,
            left: true,
            right: true,
        }
    }

    /// No sides visible.
    pub fn none() -> Self {
        Self {
            top: false,
            bottom: false,
            left: false,
            right: false,
        }
    }

    /// Only horizontal borders (top and bottom).
    pub fn horizontal() -> Self {
        Self {
            top: true,
            bottom: true,
            left: false,
            right: false,
        }
    }

    /// Only vertical borders (left and right).
    pub fn vertical() -> Self {
        Self {
            top: false,
            bottom: false,
            left: true,
            right: true,
        }
    }

    /// Only top border.
    pub fn top_only() -> Self {
        Self {
            top: true,
            bottom: false,
            left: false,
            right: false,
        }
    }

    /// Only bottom border.
    pub fn bottom_only() -> Self {
        Self {
            top: false,
            bottom: true,
            left: false,
            right: false,
        }
    }
}

/// Per-side border colors.
#[derive(Debug, Clone, Copy, Default)]
pub struct BorderColors {
    /// Color for top border
    pub top: Option<Color>,
    /// Color for bottom border
    pub bottom: Option<Color>,
    /// Color for left border
    pub left: Option<Color>,
    /// Color for right border
    pub right: Option<Color>,
}

impl BorderColors {
    /// Create border colors with the same color on all sides.
    pub fn all(color: Color) -> Self {
        Self {
            top: Some(color),
            bottom: Some(color),
            left: Some(color),
            right: Some(color),
        }
    }

    /// Get the color for the top border, falling back to the provided default.
    pub fn top_or(&self, default: Option<Color>) -> Option<Color> {
        self.top.or(default)
    }

    /// Get the color for the bottom border, falling back to the provided default.
    pub fn bottom_or(&self, default: Option<Color>) -> Option<Color> {
        self.bottom.or(default)
    }

    /// Get the color for the left border, falling back to the provided default.
    pub fn left_or(&self, default: Option<Color>) -> Option<Color> {
        self.left.or(default)
    }

    /// Get the color for the right border, falling back to the provided default.
    pub fn right_or(&self, default: Option<Color>) -> Option<Color> {
        self.right.or(default)
    }
}

/// Properties for the Box component.
///
/// # Units
///
/// All spacing values (width, height, padding, margin, gap) use terminal units:
/// - **Horizontal values** (width, padding_left/right, margin_left/right): characters
/// - **Vertical values** (height, padding_top/bottom, margin_top/bottom, gap in Column): lines
///
/// For example, `gap: 1.0` in a `FlexDirection::Column` layout adds 1 empty line between children.
///
/// # Quick Start
///
/// ```ignore
/// // Compact column layout (no spacing)
/// BoxProps::column()
///
/// // Row with gap
/// BoxProps::row().with_gap(1.0)
///
/// // Column with border
/// BoxProps::column().with_border(BorderStyle::Round)
/// ```
#[derive(Debug, Clone)]
pub struct BoxProps {
    // Layout properties
    /// Width of the box in terminal characters.
    pub width: Option<f32>,
    /// Height of the box in terminal lines.
    pub height: Option<f32>,
    /// Minimum width constraint (characters)
    pub min_width: Option<f32>,
    /// Minimum height constraint (lines)
    pub min_height: Option<f32>,
    /// Maximum width constraint (characters)
    pub max_width: Option<f32>,
    /// Maximum height constraint (lines)
    pub max_height: Option<f32>,
    /// Flex direction for child layout
    pub flex_direction: FlexDirection,
    /// How much this box should grow relative to siblings
    pub flex_grow: f32,
    /// How much this box should shrink relative to siblings
    pub flex_shrink: f32,
    /// Padding on all sides (characters horizontally, lines vertically)
    pub padding: f32,
    /// Padding on the left side (characters)
    pub padding_left: Option<f32>,
    /// Padding on the right side (characters)
    pub padding_right: Option<f32>,
    /// Padding on the top side (lines)
    pub padding_top: Option<f32>,
    /// Padding on the bottom side (lines)
    pub padding_bottom: Option<f32>,
    /// Margin on all sides (characters horizontally, lines vertically)
    pub margin: f32,
    /// Margin on the left side (characters)
    pub margin_left: Option<f32>,
    /// Margin on the right side (characters)
    pub margin_right: Option<f32>,
    /// Margin on the top side (lines)
    pub margin_top: Option<f32>,
    /// Margin on the bottom side (lines)
    pub margin_bottom: Option<f32>,
    /// Gap between children in terminal units.
    ///
    /// - In `FlexDirection::Column`: gap is in **lines** (1.0 = 1 empty line between children)
    /// - In `FlexDirection::Row`: gap is in **characters** (1.0 = 1 space between children)
    ///
    /// Default is `0.0` for compact layouts. Add gap explicitly when spacing is needed.
    pub gap: f32,
    /// How to align items along cross axis
    pub align_items: Option<AlignItems>,
    /// How to align this box (overrides parent's align_items)
    pub align_self: Option<AlignSelf>,
    /// How to align content
    pub align_content: Option<AlignContent>,
    /// How to justify content along main axis
    pub justify_content: Option<JustifyContent>,

    // Border properties
    /// Border style
    pub border_style: BorderStyle,
    /// Border color for all sides (can be overridden by per-side colors)
    pub border_color: Option<Color>,
    /// Per-side border colors (overrides border_color for specific sides)
    pub border_colors: BorderColors,
    /// Which sides to show borders on (None = all sides when border_style is set)
    pub border_sides: Option<BorderSides>,
    /// Dim the border color (renders border with dim style)
    pub border_dim: bool,

    // Background
    /// Background color (optional)
    pub background_color: Option<Color>,

    // Visibility
    /// Whether this box is visible.
    ///
    /// When `false`, the box still takes up space in the layout but renders nothing.
    /// This is useful for reserving space for elements that appear/disappear,
    /// preventing layout shifts. Similar to CSS `visibility: hidden`.
    ///
    /// Default is `true`.
    pub visible: bool,
}

impl Default for BoxProps {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            flex_direction: FlexDirection::default(),
            flex_grow: 0.0,
            flex_shrink: 0.0,
            padding: 0.0,
            padding_left: None,
            padding_right: None,
            padding_top: None,
            padding_bottom: None,
            margin: 0.0,
            margin_left: None,
            margin_right: None,
            margin_top: None,
            margin_bottom: None,
            gap: 0.0,
            align_items: None,
            align_self: None,
            align_content: None,
            justify_content: None,
            border_style: BorderStyle::default(),
            border_color: None,
            border_colors: BorderColors::default(),
            border_sides: None,
            border_dim: false,
            background_color: None,
            visible: true, // Default to visible
        }
    }
}

impl BoxProps {
    /// Create a new BoxProps with default values.
    pub fn new() -> Self {
        Self::default()
    }

    // ============ Layout Presets ============

    /// Create a column layout (children stacked vertically).
    ///
    /// This is the most common layout for CLI apps. Equivalent to:
    /// ```ignore
    /// BoxProps { flex_direction: FlexDirection::Column, ..Default::default() }
    /// ```
    pub fn column() -> Self {
        Self {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        }
    }

    /// Create a row layout (children arranged horizontally).
    ///
    /// Equivalent to:
    /// ```ignore
    /// BoxProps { flex_direction: FlexDirection::Row, ..Default::default() }
    /// ```
    pub fn row() -> Self {
        Self {
            flex_direction: FlexDirection::Row,
            ..Default::default()
        }
    }

    // ============ Builder Methods ============

    /// Set the gap between children.
    ///
    /// - In column layout: gap is in lines (1.0 = 1 empty line)
    /// - In row layout: gap is in characters (1.0 = 1 space)
    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Set padding on all sides.
    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the border style.
    pub fn with_border(mut self, style: BorderStyle) -> Self {
        self.border_style = style;
        self
    }

    /// Set the border style and color.
    pub fn with_border_color(mut self, style: BorderStyle, color: Color) -> Self {
        self.border_style = style;
        self.border_color = Some(color);
        self
    }

    /// Set the width.
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the height.
    pub fn with_height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set visibility. When false, the box takes up space but renders nothing.
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Hide this box (still takes up space in layout).
    pub fn hidden(mut self) -> Self {
        self.visible = false;
        self
    }

    // ============ Query Methods ============

    /// Get the effective border sides (which sides should show a border).
    pub fn effective_border_sides(&self) -> BorderSides {
        if !self.border_style.has_border() {
            return BorderSides::none();
        }
        self.border_sides.unwrap_or_else(BorderSides::all)
    }

    /// Get the color for the top border.
    pub fn top_border_color(&self) -> Option<Color> {
        self.border_colors.top_or(self.border_color)
    }

    /// Get the color for the bottom border.
    pub fn bottom_border_color(&self) -> Option<Color> {
        self.border_colors.bottom_or(self.border_color)
    }

    /// Get the color for the left border.
    pub fn left_border_color(&self) -> Option<Color> {
        self.border_colors.left_or(self.border_color)
    }

    /// Get the color for the right border.
    pub fn right_border_color(&self) -> Option<Color> {
        self.border_colors.right_or(self.border_color)
    }

    /// Convert these props to a LayoutStyle.
    pub fn to_layout_style(&self) -> LayoutStyle {
        let sides = self.effective_border_sides();

        // Calculate per-side border sizes based on visibility
        let border_top: f32 = if sides.top { 1.0 } else { 0.0 };
        let border_bottom: f32 = if sides.bottom { 1.0 } else { 0.0 };
        let border_left: f32 = if sides.left { 1.0 } else { 0.0 };
        let border_right: f32 = if sides.right { 1.0 } else { 0.0 };

        // For the base padding, we use the maximum border size if no per-side padding is set
        let max_border = border_top
            .max(border_bottom)
            .max(border_left)
            .max(border_right);

        LayoutStyle {
            width: self.width,
            height: self.height,
            min_width: self.min_width,
            min_height: self.min_height,
            max_width: self.max_width,
            max_height: self.max_height,
            flex_direction: self.flex_direction,
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            // Add border to padding based on which sides have borders
            padding: self.padding + max_border,
            padding_left: Some(self.padding_left.unwrap_or(self.padding) + border_left),
            padding_right: Some(self.padding_right.unwrap_or(self.padding) + border_right),
            padding_top: Some(self.padding_top.unwrap_or(self.padding) + border_top),
            padding_bottom: Some(self.padding_bottom.unwrap_or(self.padding) + border_bottom),
            margin: self.margin,
            margin_left: self.margin_left,
            margin_right: self.margin_right,
            margin_top: self.margin_top,
            margin_bottom: self.margin_bottom,
            gap: self.gap,
            align_items: self.align_items,
            align_self: self.align_self,
            align_content: self.align_content,
            justify_content: self.justify_content,
        }
    }
}

/// A container component with flexbox layout and optional border.
///
/// Box is the primary building block for layouts in Blaeck. It wraps its
/// children and can optionally display a border around them.
///
/// # Examples
///
/// ```ignore
/// // Create a box with a border
/// Element::node::<Box>(BoxProps {
///     border_style: BorderStyle::Single,
///     padding: 1.0,
///     ..Default::default()
/// }, children)
/// ```
pub struct Box;

impl Component for Box {
    type Props = BoxProps;

    fn render(_props: &Self::Props) -> Element {
        // Box doesn't render its own content directly - it just provides
        // layout and border info. The actual rendering happens in the
        // Ink runtime which uses the layout_style and border_style from props.
        // For now, we return an empty element since children are handled separately.
        Element::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_chars_default() {
        let chars = BorderChars::default();
        assert_eq!(chars.top_left, '┌');
    }

    #[test]
    fn test_border_style_has_border() {
        assert!(!BorderStyle::None.has_border());
        assert!(BorderStyle::Single.has_border());
        assert!(BorderStyle::Double.has_border());
        assert!(BorderStyle::Round.has_border());
        assert!(BorderStyle::Bold.has_border());
        assert!(BorderStyle::Classic.has_border());
    }

    #[test]
    fn test_box_props_to_layout_style() {
        let props = BoxProps {
            width: Some(80.0),
            height: Some(24.0),
            flex_direction: FlexDirection::Row,
            padding: 2.0,
            ..Default::default()
        };
        let layout = props.to_layout_style();
        assert_eq!(layout.width, Some(80.0));
        assert_eq!(layout.height, Some(24.0));
        assert_eq!(layout.flex_direction, FlexDirection::Row);
        assert_eq!(layout.padding, 2.0);
    }

    #[test]
    fn test_box_props_with_border_adds_to_padding() {
        let props = BoxProps {
            border_style: BorderStyle::Single,
            padding: 1.0,
            ..Default::default()
        };
        let layout = props.to_layout_style();
        // Padding should be 1.0 (user padding) + 1.0 (border) = 2.0
        assert_eq!(layout.padding, 2.0);
    }

    #[test]
    fn test_box_props_without_border() {
        let props = BoxProps {
            border_style: BorderStyle::None,
            padding: 1.0,
            ..Default::default()
        };
        let layout = props.to_layout_style();
        // Padding should just be 1.0 (no border)
        assert_eq!(layout.padding, 1.0);
    }

    #[test]
    fn test_border_sides_all() {
        let sides = BorderSides::all();
        assert!(sides.top);
        assert!(sides.bottom);
        assert!(sides.left);
        assert!(sides.right);
    }

    #[test]
    fn test_border_sides_none() {
        let sides = BorderSides::none();
        assert!(!sides.top);
        assert!(!sides.bottom);
        assert!(!sides.left);
        assert!(!sides.right);
    }

    #[test]
    fn test_border_sides_horizontal() {
        let sides = BorderSides::horizontal();
        assert!(sides.top);
        assert!(sides.bottom);
        assert!(!sides.left);
        assert!(!sides.right);
    }

    #[test]
    fn test_border_sides_vertical() {
        let sides = BorderSides::vertical();
        assert!(!sides.top);
        assert!(!sides.bottom);
        assert!(sides.left);
        assert!(sides.right);
    }

    #[test]
    fn test_border_sides_top_only() {
        let sides = BorderSides::top_only();
        assert!(sides.top);
        assert!(!sides.bottom);
        assert!(!sides.left);
        assert!(!sides.right);
    }

    #[test]
    fn test_border_sides_bottom_only() {
        let sides = BorderSides::bottom_only();
        assert!(!sides.top);
        assert!(sides.bottom);
        assert!(!sides.left);
        assert!(!sides.right);
    }

    #[test]
    fn test_border_colors_all() {
        let colors = BorderColors::all(Color::Red);
        assert_eq!(colors.top, Some(Color::Red));
        assert_eq!(colors.bottom, Some(Color::Red));
        assert_eq!(colors.left, Some(Color::Red));
        assert_eq!(colors.right, Some(Color::Red));
    }

    #[test]
    fn test_border_colors_default() {
        let colors = BorderColors::default();
        assert!(colors.top.is_none());
        assert!(colors.bottom.is_none());
        assert!(colors.left.is_none());
        assert!(colors.right.is_none());
    }

    #[test]
    fn test_border_colors_fallback() {
        let colors = BorderColors {
            top: Some(Color::Red),
            ..Default::default()
        };
        assert_eq!(colors.top_or(Some(Color::Blue)), Some(Color::Red));
        assert_eq!(colors.bottom_or(Some(Color::Blue)), Some(Color::Blue));
    }

    #[test]
    fn test_box_props_effective_border_sides_default() {
        let props = BoxProps {
            border_style: BorderStyle::Single,
            ..Default::default()
        };
        let sides = props.effective_border_sides();
        assert!(sides.top);
        assert!(sides.bottom);
        assert!(sides.left);
        assert!(sides.right);
    }

    #[test]
    fn test_box_props_effective_border_sides_custom() {
        let props = BoxProps {
            border_style: BorderStyle::Single,
            border_sides: Some(BorderSides::horizontal()),
            ..Default::default()
        };
        let sides = props.effective_border_sides();
        assert!(sides.top);
        assert!(sides.bottom);
        assert!(!sides.left);
        assert!(!sides.right);
    }

    #[test]
    fn test_box_props_effective_border_sides_no_border() {
        let props = BoxProps {
            border_style: BorderStyle::None,
            border_sides: Some(BorderSides::all()),
            ..Default::default()
        };
        let sides = props.effective_border_sides();
        // Should return none because border_style is None
        assert!(!sides.top);
        assert!(!sides.bottom);
        assert!(!sides.left);
        assert!(!sides.right);
    }

    #[test]
    fn test_box_props_per_side_colors() {
        let props = BoxProps {
            border_color: Some(Color::White),
            border_colors: BorderColors {
                top: Some(Color::Red),
                bottom: Some(Color::Blue),
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(props.top_border_color(), Some(Color::Red));
        assert_eq!(props.bottom_border_color(), Some(Color::Blue));
        assert_eq!(props.left_border_color(), Some(Color::White));
        assert_eq!(props.right_border_color(), Some(Color::White));
    }

    #[test]
    fn test_box_props_partial_border_padding() {
        let props = BoxProps {
            border_style: BorderStyle::Single,
            border_sides: Some(BorderSides::horizontal()),
            padding: 1.0,
            ..Default::default()
        };
        let layout = props.to_layout_style();
        // Top and bottom have border (1.0), left and right don't (0.0)
        assert_eq!(layout.padding_top, Some(2.0)); // 1.0 + 1.0
        assert_eq!(layout.padding_bottom, Some(2.0)); // 1.0 + 1.0
        assert_eq!(layout.padding_left, Some(1.0)); // 1.0 + 0.0
        assert_eq!(layout.padding_right, Some(1.0)); // 1.0 + 0.0
    }
}
