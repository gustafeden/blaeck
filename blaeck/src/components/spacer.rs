//! Spacer component - flexible space or explicit vertical spacing.
//!
//! The Spacer component serves two purposes:
//! 1. **Flex spacing**: Expands to fill available space (default, like CSS `flex-grow: 1`)
//! 2. **Fixed line spacing**: Renders a specific number of empty lines
//!
//! ## When to use Spacer
//!
//! - Push elements apart (e.g., header left, buttons right)
//! - Add vertical gaps between sections
//! - Center content by adding Spacers on both sides
//!
//! ## See also
//!
//! - [`Box`](super::Box) — Use `gap` prop for consistent spacing between children
//! - [`Newline`](super::Newline) — Single line break
//!
//! Use `SpacerProps::default()` for flex behavior, or `SpacerProps::lines(n)` for fixed spacing.

use crate::element::{Component, Element};
use crate::layout::LayoutStyle;

/// Properties for the Spacer component.
///
/// By default, Spacer acts as a flex expander. Use `SpacerProps::lines(n)` for
/// explicit vertical spacing.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SpacerProps {
    /// Number of empty lines to render. If 0 (default), uses flex behavior.
    pub lines: u16,
}

impl SpacerProps {
    /// Create a spacer that renders a fixed number of empty lines.
    ///
    /// This is useful for explicit vertical spacing instead of relying on `gap`
    /// or empty Text elements.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Add 2 empty lines between elements
    /// Element::node::<Spacer>(SpacerProps::lines(2), vec![])
    /// ```
    pub fn lines(count: u16) -> Self {
        Self { lines: count }
    }

    /// Create a spacer that expands to fill available space (default).
    pub fn flex() -> Self {
        Self { lines: 0 }
    }
}

/// A component that provides flexible or fixed spacing.
///
/// # Flex Mode (default)
///
/// With default props, Spacer expands to fill available space, useful for
/// pushing content to opposite ends of a container.
///
/// ```ignore
/// // Push items to opposite ends of a row
/// Element::node::<Box>(BoxProps::row(), vec![
///     Element::node::<Text>(TextProps::new("Left"), vec![]),
///     Element::node::<Spacer>(SpacerProps::default(), vec![]),
///     Element::node::<Text>(TextProps::new("Right"), vec![]),
/// ])
/// ```
///
/// # Fixed Line Mode
///
/// With `SpacerProps::lines(n)`, Spacer renders exactly `n` empty lines.
/// This is clearer than using empty Text elements for spacing.
///
/// ```ignore
/// // Add 1 empty line between sections
/// Element::node::<Spacer>(SpacerProps::lines(1), vec![])
/// ```
pub struct Spacer;

impl Spacer {
    /// Get the layout style for a Spacer.
    ///
    /// - Flex mode: flex_grow: 1.0 to fill available space
    /// - Lines mode: fixed height for the specified number of lines
    pub fn layout_style(props: &SpacerProps) -> LayoutStyle {
        if props.lines > 0 {
            LayoutStyle {
                height: Some(props.lines as f32),
                ..Default::default()
            }
        } else {
            LayoutStyle {
                flex_grow: 1.0,
                ..Default::default()
            }
        }
    }
}

impl Component for Spacer {
    type Props = SpacerProps;

    fn render(props: &Self::Props) -> Element {
        if props.lines > 0 {
            // Render as empty lines
            let empty_lines: Vec<Element> = (0..props.lines)
                .map(|_| Element::Text {
                    content: String::new(),
                    style: Default::default(),
                })
                .collect();
            Element::Fragment(empty_lines)
        } else {
            // Flex mode - renders as empty, effect comes from layout style
            Element::empty()
        }
    }
}

/// Convenience function to create a spacer with a fixed number of lines.
///
/// # Example
///
/// ```ignore
/// use blaeck::prelude::*;
///
/// let spacing = spacer(1); // 1 empty line
/// ```
pub fn spacer(lines: u16) -> Element {
    Element::node::<Spacer>(SpacerProps::lines(lines), vec![])
}

/// Convenience function to create a flex spacer that expands to fill space.
pub fn flex_spacer() -> Element {
    Element::node::<Spacer>(SpacerProps::flex(), vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacer_props_default() {
        let props = SpacerProps::default();
        assert_eq!(props.lines, 0);
    }

    #[test]
    fn test_spacer_props_lines() {
        let props = SpacerProps::lines(3);
        assert_eq!(props.lines, 3);
    }

    #[test]
    fn test_spacer_layout_style_flex() {
        let props = SpacerProps::default();
        let layout = Spacer::layout_style(&props);
        assert_eq!(layout.flex_grow, 1.0);
        assert!(layout.height.is_none());
    }

    #[test]
    fn test_spacer_layout_style_lines() {
        let props = SpacerProps::lines(2);
        let layout = Spacer::layout_style(&props);
        assert_eq!(layout.height, Some(2.0));
        assert_eq!(layout.flex_grow, 0.0);
    }

    #[test]
    fn test_spacer_render_flex() {
        let elem = Spacer::render(&SpacerProps::default());
        assert!(elem.is_empty());
    }

    #[test]
    fn test_spacer_render_lines() {
        let elem = Spacer::render(&SpacerProps::lines(2));
        if let Element::Fragment(children) = elem {
            assert_eq!(children.len(), 2);
        } else {
            panic!("Expected Fragment");
        }
    }

    #[test]
    fn test_spacer_helper() {
        let elem = spacer(1);
        assert!(elem.is_node());
    }

    #[test]
    fn test_flex_spacer_helper() {
        let elem = flex_spacer();
        assert!(elem.is_node());
    }
}
