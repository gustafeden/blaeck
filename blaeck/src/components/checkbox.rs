//! Checkbox component - toggleable boolean input.
//!
//! The Checkbox component displays a toggleable checkbox with a label.
//! Supports 5 visual styles (Bracket, Unicode, Circle, Check, Toggle).
//!
//! ## When to use Checkbox
//!
//! - Single boolean option (on/off, enabled/disabled)
//! - Terms acceptance, feature toggles
//! - When you need just one toggle, not a list
//!
//! ## See also
//!
//! - [`MultiSelect`](super::MultiSelect) — Multiple checkboxes in a list
//! - [`Confirm`](super::Confirm) — Yes/no question (not a toggle)

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// Style for checkbox indicators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CheckboxStyle {
    /// Square brackets: [x] [ ]
    #[default]
    Bracket,
    /// Unicode checkboxes: ☑ ☐
    Unicode,
    /// Filled circles: ● ○
    Circle,
    /// Check marks: ✓ ✗
    Check,
    /// Toggle switch: [■] [ ]
    Toggle,
}

impl CheckboxStyle {
    /// Get the characters for checked and unchecked states.
    pub fn chars(&self) -> (&'static str, &'static str) {
        match self {
            CheckboxStyle::Bracket => ("[x]", "[ ]"),
            CheckboxStyle::Unicode => ("☑", "☐"),
            CheckboxStyle::Circle => ("●", "○"),
            CheckboxStyle::Check => ("✓", "✗"),
            CheckboxStyle::Toggle => ("[■]", "[ ]"),
        }
    }
}

/// Properties for the Checkbox component.
#[derive(Debug, Clone)]
pub struct CheckboxProps {
    /// Whether the checkbox is checked.
    pub checked: bool,
    /// Label text to display after the checkbox.
    pub label: Option<String>,
    /// Whether the checkbox is focused.
    pub focused: bool,
    /// Checkbox visual style.
    pub style: CheckboxStyle,
    /// Color when checked.
    pub checked_color: Option<Color>,
    /// Color when unchecked.
    pub unchecked_color: Option<Color>,
    /// Label color.
    pub label_color: Option<Color>,
    /// Whether the checkbox is disabled.
    pub disabled: bool,
    /// Focus indicator character.
    pub focus_indicator: Option<String>,
}

impl Default for CheckboxProps {
    fn default() -> Self {
        Self {
            checked: false,
            label: None,
            focused: false,
            style: CheckboxStyle::Bracket,
            checked_color: None,
            unchecked_color: None,
            label_color: None,
            disabled: false,
            focus_indicator: Some("> ".to_string()),
        }
    }
}

impl CheckboxProps {
    /// Create new CheckboxProps.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create new CheckboxProps with a label.
    pub fn with_label(label: impl Into<String>) -> Self {
        Self {
            label: Some(label.into()),
            ..Default::default()
        }
    }

    /// Set the checked state.
    #[must_use]
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Set the label text.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set whether the checkbox is focused.
    #[must_use]
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set the checkbox style.
    #[must_use]
    pub fn style(mut self, style: CheckboxStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the color when checked.
    #[must_use]
    pub fn checked_color(mut self, color: Color) -> Self {
        self.checked_color = Some(color);
        self
    }

    /// Set the color when unchecked.
    #[must_use]
    pub fn unchecked_color(mut self, color: Color) -> Self {
        self.unchecked_color = Some(color);
        self
    }

    /// Set the label color.
    #[must_use]
    pub fn label_color(mut self, color: Color) -> Self {
        self.label_color = Some(color);
        self
    }

    /// Set a single color for the checkbox.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.checked_color = Some(color);
        self.unchecked_color = Some(color);
        self.label_color = Some(color);
        self
    }

    /// Set the checkbox as disabled.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the focus indicator string.
    #[must_use]
    pub fn focus_indicator(mut self, indicator: impl Into<String>) -> Self {
        self.focus_indicator = Some(indicator.into());
        self
    }

    /// Remove the focus indicator.
    #[must_use]
    pub fn no_focus_indicator(mut self) -> Self {
        self.focus_indicator = None;
        self
    }

    /// Build the display string.
    pub fn render_string(&self) -> String {
        let (checked_str, unchecked_str) = self.style.chars();
        let indicator = if self.checked { checked_str } else { unchecked_str };

        let focus_prefix = if self.focused {
            self.focus_indicator.as_deref().unwrap_or("")
        } else {
            // Add spaces to align with focus indicator
            if let Some(ref fi) = self.focus_indicator {
                &" ".repeat(fi.len())
            } else {
                ""
            }
        };

        if let Some(ref label) = self.label {
            format!("{}{} {}", focus_prefix, indicator, label)
        } else {
            format!("{}{}", focus_prefix, indicator)
        }
    }
}

/// A component that displays a checkbox.
///
/// # Examples
///
/// ```ignore
/// let props = CheckboxProps::with_label("Enable notifications")
///     .checked(true)
///     .focused(is_focused)
///     .checked_color(Color::Green);
///
/// // Toggle in your event loop:
/// if key.code == KeyCode::Char(' ') || key.code == KeyCode::Enter {
///     checked = !checked;
/// }
/// ```
pub struct Checkbox;

impl Component for Checkbox {
    type Props = CheckboxProps;

    fn render(props: &Self::Props) -> Element {
        let content = props.render_string();

        let mut style = Style::new();

        // Apply colors based on state
        let color = if props.checked {
            props.checked_color
        } else {
            props.unchecked_color
        };

        if let Some(c) = color {
            style = style.fg(c);
        }

        if props.disabled {
            style = style.add_modifier(Modifier::DIM);
        }

        if props.focused && !props.disabled {
            style = style.add_modifier(Modifier::BOLD);
        }

        Element::styled_text(&content, style)
    }
}

/// Helper to render a simple checkbox string.
///
/// # Example
///
/// ```ignore
/// let cb = checkbox(true, "Accept terms");
/// // Returns: "[x] Accept terms"
/// ```
pub fn checkbox(checked: bool, label: &str) -> String {
    CheckboxProps::with_label(label)
        .checked(checked)
        .no_focus_indicator()
        .render_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_props_default() {
        let props = CheckboxProps::default();
        assert!(!props.checked);
        assert!(!props.focused);
        assert!(props.label.is_none());
    }

    #[test]
    fn test_checkbox_props_with_label() {
        let props = CheckboxProps::with_label("Test");
        assert_eq!(props.label, Some("Test".to_string()));
    }

    #[test]
    fn test_checkbox_props_builder() {
        let props = CheckboxProps::with_label("Option")
            .checked(true)
            .focused(true)
            .style(CheckboxStyle::Unicode)
            .checked_color(Color::Green);

        assert!(props.checked);
        assert!(props.focused);
        assert_eq!(props.style, CheckboxStyle::Unicode);
        assert_eq!(props.checked_color, Some(Color::Green));
    }

    #[test]
    fn test_checkbox_style_bracket() {
        let (checked, unchecked) = CheckboxStyle::Bracket.chars();
        assert_eq!(checked, "[x]");
        assert_eq!(unchecked, "[ ]");
    }

    #[test]
    fn test_checkbox_style_unicode() {
        let (checked, unchecked) = CheckboxStyle::Unicode.chars();
        assert_eq!(checked, "☑");
        assert_eq!(unchecked, "☐");
    }

    #[test]
    fn test_checkbox_style_circle() {
        let (checked, unchecked) = CheckboxStyle::Circle.chars();
        assert_eq!(checked, "●");
        assert_eq!(unchecked, "○");
    }

    #[test]
    fn test_checkbox_render_string_unchecked() {
        let props = CheckboxProps::with_label("Test").no_focus_indicator();
        assert_eq!(props.render_string(), "[ ] Test");
    }

    #[test]
    fn test_checkbox_render_string_checked() {
        let props = CheckboxProps::with_label("Test")
            .checked(true)
            .no_focus_indicator();
        assert_eq!(props.render_string(), "[x] Test");
    }

    #[test]
    fn test_checkbox_render_string_focused() {
        let props = CheckboxProps::with_label("Test").focused(true);
        assert_eq!(props.render_string(), "> [ ] Test");
    }

    #[test]
    fn test_checkbox_render_string_not_focused_aligned() {
        let props = CheckboxProps::with_label("Test").focused(false);
        assert_eq!(props.render_string(), "  [ ] Test");
    }

    #[test]
    fn test_checkbox_render_string_unicode_style() {
        let props = CheckboxProps::with_label("Option")
            .checked(true)
            .style(CheckboxStyle::Unicode)
            .no_focus_indicator();
        assert_eq!(props.render_string(), "☑ Option");
    }

    #[test]
    fn test_checkbox_render_string_no_label() {
        let props = CheckboxProps::new()
            .checked(true)
            .no_focus_indicator();
        assert_eq!(props.render_string(), "[x]");
    }

    #[test]
    fn test_checkbox_helper_function() {
        assert_eq!(checkbox(true, "Yes"), "[x] Yes");
        assert_eq!(checkbox(false, "No"), "[ ] No");
    }

    #[test]
    fn test_checkbox_component_render() {
        let props = CheckboxProps::with_label("Test")
            .checked(true)
            .no_focus_indicator();
        let elem = Checkbox::render(&props);
        match elem {
            Element::Text { content, .. } => {
                assert_eq!(content, "[x] Test");
            }
            _ => panic!("Expected Text element"),
        }
    }

    #[test]
    fn test_all_checkbox_styles() {
        let styles = [
            CheckboxStyle::Bracket,
            CheckboxStyle::Unicode,
            CheckboxStyle::Circle,
            CheckboxStyle::Check,
            CheckboxStyle::Toggle,
        ];

        for style in &styles {
            let (checked, unchecked) = style.chars();
            assert!(!checked.is_empty(), "Style {:?} has empty checked char", style);
            assert!(!unchecked.is_empty(), "Style {:?} has empty unchecked char", style);
        }
    }
}
