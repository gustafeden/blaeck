//! Divider component - horizontal separator line.
//!
//! The Divider component displays a horizontal line to visually separate content.
//!
//! ## When to use Divider
//!
//! - Separating sections of content
//! - Visual break between groups of items
//! - Emphasizing boundaries in a layout
//!
//! ## See also
//!
//! - [`Spacer`](super::Spacer) — Empty space (no visible line)
//! - [`Box`](super::Box) — Containers with borders

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// Style for the divider line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DividerStyle {
    /// Single line: ─
    #[default]
    Single,
    /// Double line: ═
    Double,
    /// Dashed: ┄
    Dashed,
    /// Dotted: ···
    Dotted,
    /// Bold: ━
    Bold,
    /// ASCII: -
    Ascii,
}

impl DividerStyle {
    /// Get the character for this style.
    pub fn char(&self) -> char {
        match self {
            DividerStyle::Single => '─',
            DividerStyle::Double => '═',
            DividerStyle::Dashed => '┄',
            DividerStyle::Dotted => '·',
            DividerStyle::Bold => '━',
            DividerStyle::Ascii => '-',
        }
    }
}

/// Properties for the Divider component.
#[derive(Debug, Clone)]
pub struct DividerProps {
    /// Width of the divider (in characters). None = 20 default.
    pub width: Option<usize>,
    /// Style of the line.
    pub style: DividerStyle,
    /// Color of the line.
    pub color: Option<Color>,
    /// Whether to dim the line.
    pub dim: bool,
    /// Optional label in the middle of the divider.
    pub label: Option<String>,
    /// Label color (defaults to line color).
    pub label_color: Option<Color>,
}

impl Default for DividerProps {
    fn default() -> Self {
        Self {
            width: None,
            style: DividerStyle::Single,
            color: None,
            dim: false,
            label: None,
            label_color: None,
        }
    }
}

impl DividerProps {
    /// Create a new divider with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the line style.
    #[must_use]
    pub fn line_style(mut self, style: DividerStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Make the divider dimmed.
    #[must_use]
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Add a label in the middle.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the label color.
    #[must_use]
    pub fn label_color(mut self, color: Color) -> Self {
        self.label_color = Some(color);
        self
    }

    /// Build the display string.
    pub fn render_string(&self) -> String {
        let width = self.width.unwrap_or(20);
        let ch = self.style.char();

        if let Some(ref label) = self.label {
            // Label in the middle: ── Label ──
            let label_len = label.chars().count();
            let remaining = width.saturating_sub(label_len + 2); // +2 for spaces
            let left = remaining / 2;
            let right = remaining - left;

            format!(
                "{} {} {}",
                ch.to_string().repeat(left),
                label,
                ch.to_string().repeat(right)
            )
        } else {
            ch.to_string().repeat(width)
        }
    }
}

/// A component that displays a horizontal divider line.
///
/// # Examples
///
/// ```ignore
/// // Simple divider
/// Element::node::<Divider>(DividerProps::new().width(40), vec![])
///
/// // Divider with label
/// Element::node::<Divider>(
///     DividerProps::new()
///         .width(40)
///         .label("Section")
///         .color(Color::DarkGray),
///     vec![]
/// )
/// ```
pub struct Divider;

impl Component for Divider {
    type Props = DividerProps;

    fn render(props: &Self::Props) -> Element {
        let content = props.render_string();

        let mut style = Style::new();
        if let Some(color) = props.color {
            style = style.fg(color);
        }
        if props.dim {
            style = style.add_modifier(Modifier::DIM);
        }

        Element::styled_text(&content, style)
    }
}

/// Helper to create a simple divider string.
pub fn divider(width: usize) -> String {
    DividerProps::new().width(width).render_string()
}

/// Helper to create a divider string with a label.
pub fn divider_with_label(width: usize, label: &str) -> String {
    DividerProps::new()
        .width(width)
        .label(label)
        .render_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divider_default() {
        let props = DividerProps::default();
        let s = props.render_string();
        assert_eq!(s.chars().count(), 20);
        assert!(s.chars().all(|c| c == '─'));
    }

    #[test]
    fn test_divider_width() {
        let props = DividerProps::new().width(10);
        let s = props.render_string();
        assert_eq!(s, "──────────");
    }

    #[test]
    fn test_divider_double() {
        let props = DividerProps::new()
            .width(5)
            .line_style(DividerStyle::Double);
        let s = props.render_string();
        assert_eq!(s, "═════");
    }

    #[test]
    fn test_divider_with_label() {
        let props = DividerProps::new().width(20).label("Test");
        let s = props.render_string();
        assert!(s.contains("Test"));
        assert!(s.contains("─"));
    }

    #[test]
    fn test_divider_helper() {
        let s = divider(10);
        assert_eq!(s.chars().count(), 10);
    }

    #[test]
    fn test_divider_with_label_helper() {
        let s = divider_with_label(20, "Hello");
        assert!(s.contains("Hello"));
    }
}
