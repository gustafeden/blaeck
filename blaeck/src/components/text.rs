//! Text component - displays styled text content.
//!
//! The Text component is used to display text with styling options like
//! color, bold, italic, dim, underline, and strikethrough.
//!
//! ## When to use Text
//!
//! - Any text content that needs styling
//! - Labels, messages, status text
//! - Content inside Box containers
//!
//! ## See also
//!
//! - [`Gradient`](super::Gradient) — Text with color gradients
//! - [`Markdown`](super::Markdown) — Render markdown-formatted text
//! - [`SyntaxHighlight`](super::SyntaxHighlight) — Code with syntax highlighting

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// How text should wrap when it exceeds the available width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextWrap {
    /// Wrap text at word boundaries
    #[default]
    Wrap,
    /// Do not wrap text
    NoWrap,
    /// Truncate text with ellipsis
    Truncate,
    /// Truncate at the start with ellipsis
    TruncateStart,
    /// Truncate in the middle with ellipsis
    TruncateMiddle,
}

/// Properties for the Text component.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextProps {
    /// The text content to display
    pub content: String,
    /// Text color (foreground)
    pub color: Option<Color>,
    /// Background color
    pub bg_color: Option<Color>,
    /// Whether the text should be bold
    pub bold: bool,
    /// Whether the text should be dimmed
    pub dim: bool,
    /// Whether the text should be italic
    pub italic: bool,
    /// Whether the text should be underlined
    pub underline: bool,
    /// Whether the text should be struck through
    pub strikethrough: bool,
    /// Whether to swap foreground and background colors
    pub inverse: bool,
    /// How to handle text wrapping
    pub wrap: TextWrap,
}

impl TextProps {
    /// Create new TextProps with the given content.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    /// Set the text color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the background color.
    #[must_use]
    pub fn bg_color(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set bold styling.
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set dim styling.
    #[must_use]
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Set italic styling.
    #[must_use]
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Set underline styling.
    #[must_use]
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Set strikethrough styling.
    #[must_use]
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    /// Set inverse styling (swap fg/bg).
    #[must_use]
    pub fn inverse(mut self) -> Self {
        self.inverse = true;
        self
    }

    /// Set the text wrap mode.
    #[must_use]
    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Convert these props to a Style.
    pub fn to_style(&self) -> Style {
        let mut style = Style::new();

        // Apply colors
        if let Some(color) = self.color {
            style = style.fg(color);
        }
        if let Some(bg) = self.bg_color {
            style = style.bg(bg);
        }

        // Apply modifiers
        if self.bold {
            style = style.add_modifier(Modifier::BOLD);
        }
        if self.dim {
            style = style.add_modifier(Modifier::DIM);
        }
        if self.italic {
            style = style.add_modifier(Modifier::ITALIC);
        }
        if self.underline {
            style = style.add_modifier(Modifier::UNDERLINED);
        }
        if self.strikethrough {
            style = style.add_modifier(Modifier::CROSSED_OUT);
        }
        if self.inverse {
            style = style.add_modifier(Modifier::REVERSED);
        }

        style
    }
}

/// A component that displays styled text.
///
/// # Examples
///
/// ```ignore
/// // Create styled text
/// let props = TextProps::new("Hello, World!")
///     .color(Color::Green)
///     .bold();
/// let elem = Text::render(&props);
/// ```
pub struct Text;

impl Component for Text {
    type Props = TextProps;

    fn render(props: &Self::Props) -> Element {
        Element::styled_text(&props.content, props.to_style())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_props_new() {
        let props = TextProps::new("Hello");
        assert_eq!(props.content, "Hello");
        assert!(props.color.is_none());
        assert!(!props.bold);
    }

    #[test]
    fn test_text_props_builder() {
        let props = TextProps::new("Test")
            .color(Color::Red)
            .bg_color(Color::Blue)
            .bold()
            .italic()
            .underline();

        assert_eq!(props.content, "Test");
        assert_eq!(props.color, Some(Color::Red));
        assert_eq!(props.bg_color, Some(Color::Blue));
        assert!(props.bold);
        assert!(props.italic);
        assert!(props.underline);
    }

    #[test]
    fn test_text_props_to_style() {
        let props = TextProps::new("Test")
            .color(Color::Green)
            .bold()
            .italic();

        let style = props.to_style();
        assert_eq!(style.fg, Color::Green);
        assert!(style.modifiers.contains(Modifier::BOLD));
        assert!(style.modifiers.contains(Modifier::ITALIC));
    }

    #[test]
    fn test_text_props_to_style_with_all_modifiers() {
        let props = TextProps {
            content: "Test".into(),
            bold: true,
            dim: true,
            italic: true,
            underline: true,
            strikethrough: true,
            inverse: true,
            ..Default::default()
        };

        let style = props.to_style();
        assert!(style.modifiers.contains(Modifier::BOLD));
        assert!(style.modifiers.contains(Modifier::DIM));
        assert!(style.modifiers.contains(Modifier::ITALIC));
        assert!(style.modifiers.contains(Modifier::UNDERLINED));
        assert!(style.modifiers.contains(Modifier::CROSSED_OUT));
        assert!(style.modifiers.contains(Modifier::REVERSED));
    }

    #[test]
    fn test_text_render() {
        let props = TextProps::new("Hello").color(Color::Red);
        let elem = Text::render(&props);

        match elem {
            Element::Text { content, style } => {
                assert_eq!(content, "Hello");
                assert_eq!(style.fg, Color::Red);
            }
            _ => panic!("Expected Text element"),
        }
    }

    #[test]
    fn test_text_wrap_default() {
        assert_eq!(TextWrap::default(), TextWrap::Wrap);
    }
}
