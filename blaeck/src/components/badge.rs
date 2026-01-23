//! Badge component - colored label/tag.
//!
//! The Badge component displays a small colored label, useful for
//! status indicators, categories, counts, etc.
//!
//! ## When to use Badge
//!
//! - Status labels (SUCCESS, FAILED, PENDING)
//! - Category tags
//! - Count indicators (e.g., notification count)
//!
//! ## See also
//!
//! - [`StatusBar`](super::StatusBar) — Multiple status segments together
//! - [`Text`](super::Text) — Plain styled text without badge styling

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// Visual style for the badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeStyle {
    /// Simple text with color: `label`
    #[default]
    Simple,
    /// Bracketed: `[label]`
    Bracket,
    /// Rounded: `(label)`
    Round,
    /// Pill/tag style: `‹label›`
    Pill,
    /// Filled style with background: ` label ` (inverted colors)
    Filled,
}

/// Properties for the Badge component.
#[derive(Debug, Clone)]
pub struct BadgeProps {
    /// The text to display.
    pub text: String,
    /// Text/foreground color.
    pub color: Option<Color>,
    /// Background color (for filled style).
    pub bg_color: Option<Color>,
    /// Visual style.
    pub style: BadgeStyle,
    /// Whether to make the text bold.
    pub bold: bool,
    /// Whether to dim the badge.
    pub dim: bool,
}

impl Default for BadgeProps {
    fn default() -> Self {
        Self {
            text: String::new(),
            color: None,
            bg_color: None,
            style: BadgeStyle::Simple,
            bold: false,
            dim: false,
        }
    }
}

impl BadgeProps {
    /// Create a new badge with the given text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
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

    /// Set the visual style.
    #[must_use]
    pub fn badge_style(mut self, style: BadgeStyle) -> Self {
        self.style = style;
        self
    }

    /// Make the badge bold.
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Make the badge dimmed.
    #[must_use]
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Build the display string.
    pub fn render_string(&self) -> String {
        match self.style {
            BadgeStyle::Simple => self.text.clone(),
            BadgeStyle::Bracket => format!("[{}]", self.text),
            BadgeStyle::Round => format!("({})", self.text),
            BadgeStyle::Pill => format!("‹{}›", self.text),
            BadgeStyle::Filled => format!(" {} ", self.text),
        }
    }
}

/// A component that displays a colored badge/tag.
///
/// # Examples
///
/// ```ignore
/// // Simple colored badge
/// Element::node::<Badge>(
///     BadgeProps::new("NEW").color(Color::Green),
///     vec![]
/// )
///
/// // Status badge with background
/// Element::node::<Badge>(
///     BadgeProps::new("ERROR")
///         .color(Color::White)
///         .bg_color(Color::Red)
///         .badge_style(BadgeStyle::Filled),
///     vec![]
/// )
/// ```
pub struct Badge;

impl Component for Badge {
    type Props = BadgeProps;

    fn render(props: &Self::Props) -> Element {
        let content = props.render_string();

        let mut style = Style::new();
        if let Some(color) = props.color {
            style = style.fg(color);
        }
        if let Some(bg) = props.bg_color {
            style = style.bg(bg);
        }
        if props.bold {
            style = style.add_modifier(Modifier::BOLD);
        }
        if props.dim {
            style = style.add_modifier(Modifier::DIM);
        }

        Element::styled_text(&content, style)
    }
}

/// Helper to create a simple badge string.
pub fn badge(text: &str) -> String {
    BadgeProps::new(text).render_string()
}

/// Helper to create a bracketed badge string.
pub fn badge_bracket(text: &str) -> String {
    BadgeProps::new(text)
        .badge_style(BadgeStyle::Bracket)
        .render_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_simple() {
        let props = BadgeProps::new("test");
        assert_eq!(props.render_string(), "test");
    }

    #[test]
    fn test_badge_bracket() {
        let props = BadgeProps::new("INFO").badge_style(BadgeStyle::Bracket);
        assert_eq!(props.render_string(), "[INFO]");
    }

    #[test]
    fn test_badge_round() {
        let props = BadgeProps::new("1").badge_style(BadgeStyle::Round);
        assert_eq!(props.render_string(), "(1)");
    }

    #[test]
    fn test_badge_pill() {
        let props = BadgeProps::new("tag").badge_style(BadgeStyle::Pill);
        assert_eq!(props.render_string(), "‹tag›");
    }

    #[test]
    fn test_badge_filled() {
        let props = BadgeProps::new("OK").badge_style(BadgeStyle::Filled);
        assert_eq!(props.render_string(), " OK ");
    }

    #[test]
    fn test_badge_helper() {
        assert_eq!(badge("test"), "test");
        assert_eq!(badge_bracket("test"), "[test]");
    }

    #[test]
    fn test_badge_component_render() {
        let props = BadgeProps::new("Test").color(Color::Green);
        let elem = Badge::render(&props);
        match elem {
            Element::Text { content, style } => {
                assert_eq!(content, "Test");
                assert_eq!(style.fg, Color::Green);
            }
            _ => panic!("Expected Text element"),
        }
    }
}
