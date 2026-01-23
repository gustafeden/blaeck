//! KeyHints component - keyboard shortcut display.
//!
//! The KeyHints component displays keyboard shortcuts in a clean,
//! formatted style commonly seen at the bottom of CLI applications.
//!
//! ## When to use KeyHints
//!
//! - Footer showing available keyboard shortcuts
//! - Context-sensitive help (changes based on mode)
//! - Onboarding users to keyboard controls
//!
//! ## See also
//!
//! - [`StatusBar`](super::StatusBar) — Status indicators (often paired with KeyHints)

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// A single key hint (key + description).
#[derive(Debug, Clone)]
pub struct KeyHint {
    /// The key or key combination (e.g., "^C", "↑↓", "Enter").
    pub key: String,
    /// Description of what the key does (e.g., "exit", "navigate").
    pub action: String,
}

impl KeyHint {
    /// Create a new key hint.
    pub fn new(key: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            action: action.into(),
        }
    }
}

impl<K: Into<String>, A: Into<String>> From<(K, A)> for KeyHint {
    fn from((key, action): (K, A)) -> Self {
        KeyHint::new(key, action)
    }
}

/// Style for the separator between hints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyHintSeparator {
    /// Bullet: •
    #[default]
    Bullet,
    /// Pipe: |
    Pipe,
    /// Slash: /
    Slash,
    /// Space only
    Space,
    /// Double space
    DoubleSpace,
}

impl KeyHintSeparator {
    /// Get the separator string.
    pub fn as_str(&self) -> &'static str {
        match self {
            KeyHintSeparator::Bullet => " • ",
            KeyHintSeparator::Pipe => " | ",
            KeyHintSeparator::Slash => " / ",
            KeyHintSeparator::Space => " ",
            KeyHintSeparator::DoubleSpace => "  ",
        }
    }
}

/// Style for displaying key hints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyHintStyle {
    /// Key and action together: "^C exit"
    #[default]
    Compact,
    /// Key in brackets: "[^C] exit"
    Bracketed,
    /// Key with colon: "^C: exit"
    Colon,
    /// Action first: "exit ^C"
    ActionFirst,
}

/// Properties for the KeyHints component.
#[derive(Debug, Clone)]
pub struct KeyHintsProps {
    /// List of key hints to display.
    pub hints: Vec<KeyHint>,
    /// Separator between hints.
    pub separator: KeyHintSeparator,
    /// Display style.
    pub style: KeyHintStyle,
    /// Color for the key portion.
    pub key_color: Option<Color>,
    /// Color for the action/description portion.
    pub action_color: Option<Color>,
    /// Color for the separator.
    pub separator_color: Option<Color>,
    /// Whether to bold the keys.
    pub bold_keys: bool,
    /// Whether to dim the actions.
    pub dim_actions: bool,
}

impl Default for KeyHintsProps {
    fn default() -> Self {
        Self {
            hints: Vec::new(),
            separator: KeyHintSeparator::Bullet,
            style: KeyHintStyle::Compact,
            key_color: None,
            action_color: None,
            separator_color: Some(Color::DarkGray),
            bold_keys: true,
            dim_actions: true,
        }
    }
}

impl KeyHintsProps {
    /// Create new KeyHintsProps with hints.
    pub fn new<I, T>(hints: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<KeyHint>,
    {
        Self {
            hints: hints.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    /// Set the separator style.
    #[must_use]
    pub fn separator(mut self, separator: KeyHintSeparator) -> Self {
        self.separator = separator;
        self
    }

    /// Set the display style.
    #[must_use]
    pub fn style(mut self, style: KeyHintStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the key color.
    #[must_use]
    pub fn key_color(mut self, color: Color) -> Self {
        self.key_color = Some(color);
        self
    }

    /// Set the action color.
    #[must_use]
    pub fn action_color(mut self, color: Color) -> Self {
        self.action_color = Some(color);
        self
    }

    /// Set the separator color.
    #[must_use]
    pub fn separator_color(mut self, color: Color) -> Self {
        self.separator_color = Some(color);
        self
    }

    /// Enable/disable bold keys.
    #[must_use]
    pub fn bold_keys(mut self, bold: bool) -> Self {
        self.bold_keys = bold;
        self
    }

    /// Enable/disable dim actions.
    #[must_use]
    pub fn dim_actions(mut self, dim: bool) -> Self {
        self.dim_actions = dim;
        self
    }

    /// Add a hint.
    #[must_use]
    pub fn hint(mut self, key: impl Into<String>, action: impl Into<String>) -> Self {
        self.hints.push(KeyHint::new(key, action));
        self
    }

    /// Format a single hint according to style.
    fn format_hint(&self, hint: &KeyHint) -> String {
        match self.style {
            KeyHintStyle::Compact => format!("{} {}", hint.key, hint.action),
            KeyHintStyle::Bracketed => format!("[{}] {}", hint.key, hint.action),
            KeyHintStyle::Colon => format!("{}: {}", hint.key, hint.action),
            KeyHintStyle::ActionFirst => format!("{} {}", hint.action, hint.key),
        }
    }

    /// Render the hints as a string.
    pub fn render_string(&self) -> String {
        if self.hints.is_empty() {
            return String::new();
        }

        let separator = self.separator.as_str();
        self.hints
            .iter()
            .map(|h| self.format_hint(h))
            .collect::<Vec<_>>()
            .join(separator)
    }
}

/// A component that displays keyboard shortcuts.
///
/// # Examples
///
/// ```ignore
/// // Simple usage
/// Element::node::<KeyHints>(
///     KeyHintsProps::new([
///         ("^C", "exit"),
///         ("↑↓", "navigate"),
///         ("Enter", "select"),
///     ]),
///     vec![]
/// )
///
/// // With builder
/// Element::node::<KeyHints>(
///     KeyHintsProps::new([])
///         .hint("q", "quit")
///         .hint("?", "help")
///         .hint("↑↓", "move")
///         .separator(KeyHintSeparator::Pipe)
///         .key_color(Color::Cyan),
///     vec![]
/// )
/// ```
pub struct KeyHints;

impl Component for KeyHints {
    type Props = KeyHintsProps;

    fn render(props: &Self::Props) -> Element {
        let content = props.render_string();

        let mut style = Style::new();

        // Apply dim if configured (most common for hint bars)
        if props.dim_actions {
            style = style.add_modifier(Modifier::DIM);
        }

        // Apply key color as the overall color if set
        if let Some(color) = props.key_color {
            style = style.fg(color);
        }

        Element::styled_text(&content, style)
    }
}

/// Helper function to create a key hints string.
pub fn key_hints<I, T>(hints: I) -> String
where
    I: IntoIterator<Item = T>,
    T: Into<KeyHint>,
{
    KeyHintsProps::new(hints).render_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyhint_new() {
        let hint = KeyHint::new("^C", "exit");
        assert_eq!(hint.key, "^C");
        assert_eq!(hint.action, "exit");
    }

    #[test]
    fn test_keyhint_from_tuple() {
        let hint: KeyHint = ("Enter", "select").into();
        assert_eq!(hint.key, "Enter");
        assert_eq!(hint.action, "select");
    }

    #[test]
    fn test_keyhints_props_new() {
        let props = KeyHintsProps::new([("a", "action1"), ("b", "action2")]);
        assert_eq!(props.hints.len(), 2);
    }

    #[test]
    fn test_keyhints_props_builder() {
        let props = KeyHintsProps::new(Vec::<KeyHint>::new())
            .hint("^C", "exit")
            .hint("q", "quit")
            .separator(KeyHintSeparator::Pipe)
            .key_color(Color::Cyan);

        assert_eq!(props.hints.len(), 2);
        assert_eq!(props.separator, KeyHintSeparator::Pipe);
        assert_eq!(props.key_color, Some(Color::Cyan));
    }

    #[test]
    fn test_keyhints_render_empty() {
        let props = KeyHintsProps::new(Vec::<KeyHint>::new());
        assert_eq!(props.render_string(), "");
    }

    #[test]
    fn test_keyhints_render_compact() {
        let props = KeyHintsProps::new([("^C", "exit"), ("q", "quit")])
            .style(KeyHintStyle::Compact);
        assert_eq!(props.render_string(), "^C exit • q quit");
    }

    #[test]
    fn test_keyhints_render_bracketed() {
        let props = KeyHintsProps::new([("^C", "exit")])
            .style(KeyHintStyle::Bracketed);
        assert_eq!(props.render_string(), "[^C] exit");
    }

    #[test]
    fn test_keyhints_render_colon() {
        let props = KeyHintsProps::new([("^C", "exit")])
            .style(KeyHintStyle::Colon);
        assert_eq!(props.render_string(), "^C: exit");
    }

    #[test]
    fn test_keyhints_render_action_first() {
        let props = KeyHintsProps::new([("^C", "exit")])
            .style(KeyHintStyle::ActionFirst);
        assert_eq!(props.render_string(), "exit ^C");
    }

    #[test]
    fn test_keyhints_separator_pipe() {
        let props = KeyHintsProps::new([("a", "one"), ("b", "two")])
            .separator(KeyHintSeparator::Pipe);
        assert_eq!(props.render_string(), "a one | b two");
    }

    #[test]
    fn test_keyhints_separator_slash() {
        let props = KeyHintsProps::new([("a", "one"), ("b", "two")])
            .separator(KeyHintSeparator::Slash);
        assert_eq!(props.render_string(), "a one / b two");
    }

    #[test]
    fn test_keyhints_helper() {
        let result = key_hints([("^C", "exit"), ("q", "quit")]);
        assert!(result.contains("^C exit"));
        assert!(result.contains("q quit"));
    }

    #[test]
    fn test_keyhints_component_render() {
        let props = KeyHintsProps::new([("^C", "exit")]);
        let elem = KeyHints::render(&props);
        assert!(elem.is_text());
    }
}
