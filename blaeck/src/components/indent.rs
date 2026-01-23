//! Indent component for adding horizontal indentation.

use crate::element::{Component, Element};
use crate::layout::LayoutStyle;

/// Properties for the Indent component.
#[derive(Default, Clone, PartialEq)]
pub struct IndentProps {
    /// Number of spaces to indent (default: 2).
    pub size: usize,
}

impl IndentProps {
    pub fn new() -> Self {
        Self { size: 2 }
    }

    pub fn with_size(size: usize) -> Self {
        Self { size }
    }
}

/// Indent component that adds horizontal spacing.
pub struct Indent;

impl Indent {
    /// Get the layout style for an Indent.
    pub fn layout_style(props: &IndentProps) -> LayoutStyle {
        LayoutStyle {
            width: Some(props.size as f32),
            height: Some(1.0),
            ..Default::default()
        }
    }
}

impl Component for Indent {
    type Props = IndentProps;

    fn render(props: &Self::Props) -> Element {
        Element::text(" ".repeat(props.size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent_props_default() {
        let props = IndentProps::new();
        assert_eq!(props.size, 2);
    }

    #[test]
    fn test_indent_props_with_size() {
        let props = IndentProps::with_size(4);
        assert_eq!(props.size, 4);
    }

    #[test]
    fn test_indent_layout_style() {
        let props = IndentProps::with_size(4);
        let style = Indent::layout_style(&props);
        assert_eq!(style.width, Some(4.0));
    }

    #[test]
    fn test_indent_layout_style_default() {
        let props = IndentProps::new();
        let style = Indent::layout_style(&props);
        assert_eq!(style.width, Some(2.0));
        assert_eq!(style.height, Some(1.0));
    }

    #[test]
    fn test_indent_layout_style_zero() {
        let props = IndentProps::with_size(0);
        let style = Indent::layout_style(&props);
        assert_eq!(style.width, Some(0.0));
    }

    #[test]
    fn test_indent_render() {
        let props = IndentProps::new();
        let elem = Indent::render(&props);
        assert!(elem.is_text());
    }

    #[test]
    fn test_indent_render_content() {
        let props = IndentProps::with_size(4);
        let elem = Indent::render(&props);
        match elem {
            Element::Text { content, .. } => {
                assert_eq!(content, "    ");
                assert_eq!(content.len(), 4);
            }
            _ => panic!("Expected Text element"),
        }
    }

    #[test]
    fn test_indent_default_trait() {
        let props = IndentProps::default();
        // Default should have size = 0 due to #[derive(Default)]
        // But new() gives 2
        assert_eq!(props.size, 0);
    }
}
