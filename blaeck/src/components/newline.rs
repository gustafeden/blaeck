//! Newline component for explicit line breaks.

use crate::element::{Component, Element};
use crate::layout::LayoutStyle;

/// Properties for the Newline component.
#[derive(Default, Clone, PartialEq)]
pub struct NewlineProps {
    /// Number of newlines to insert (default: 1).
    pub count: usize,
}

impl NewlineProps {
    pub fn new() -> Self {
        Self { count: 1 }
    }

    pub fn with_count(count: usize) -> Self {
        Self { count }
    }
}

/// Newline component that inserts explicit line breaks.
pub struct Newline;

impl Newline {
    /// Get the layout style for a Newline.
    pub fn layout_style(props: &NewlineProps) -> LayoutStyle {
        LayoutStyle {
            height: Some(props.count.max(1) as f32),
            width: Some(0.0),
            ..Default::default()
        }
    }
}

impl Component for Newline {
    type Props = NewlineProps;

    fn render(props: &Self::Props) -> Element {
        // Newline renders as empty space with height
        // For count=1, we return empty text (the layout height handles the line)
        // For count>1, we return (count-1) newlines (the extra lines beyond the first)
        if props.count <= 1 {
            Element::text("")
        } else {
            Element::text("\n".repeat(props.count - 1))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newline_props_default() {
        let props = NewlineProps::new();
        assert_eq!(props.count, 1);
    }

    #[test]
    fn test_newline_props_with_count() {
        let props = NewlineProps::with_count(3);
        assert_eq!(props.count, 3);
    }

    #[test]
    fn test_newline_layout_style() {
        let props = NewlineProps::with_count(2);
        let style = Newline::layout_style(&props);
        assert_eq!(style.height, Some(2.0));
    }

    #[test]
    fn test_newline_layout_style_default() {
        let props = NewlineProps::new();
        let style = Newline::layout_style(&props);
        assert_eq!(style.height, Some(1.0));
        assert_eq!(style.width, Some(0.0));
    }

    #[test]
    fn test_newline_layout_style_zero_count() {
        // Zero count should still result in at least 1 line
        let props = NewlineProps::with_count(0);
        let style = Newline::layout_style(&props);
        assert_eq!(style.height, Some(1.0));
    }

    #[test]
    fn test_newline_render() {
        let props = NewlineProps::new();
        let elem = Newline::render(&props);
        assert!(elem.is_text());
    }

    #[test]
    fn test_newline_default_trait() {
        let props = NewlineProps::default();
        // Default should have count = 0 due to #[derive(Default)]
        // But new() gives 1
        assert_eq!(props.count, 0);
    }
}
