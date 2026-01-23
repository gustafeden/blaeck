//! Confirm component - yes/no prompt.
//!
//! The Confirm component displays a yes/no question that users can
//! answer using arrow keys or y/n keys.
//!
//! ## When to use Confirm
//!
//! - Binary yes/no questions
//! - Destructive action confirmation
//! - Simple approval prompts
//!
//! ## See also
//!
//! - [`Modal`](super::Modal) — Confirmation in a dialog box
//! - [`Checkbox`](super::Checkbox) — Toggleable state (not a one-time choice)
//! - [`Select`](super::Select) — More than 2 options

use crate::element::{Component, Element};
use crate::style::{Color, Style};

/// Visual style for the confirm prompt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConfirmStyle {
    /// Inline text style: `Question? [Yes] / No`
    #[default]
    Inline,
    /// Button-like style: `Question?  [ Yes ]  [ No ]`
    Button,
}

/// Properties for the Confirm component.
#[derive(Debug, Clone)]
pub struct ConfirmProps {
    /// The question to ask.
    pub message: String,
    /// Current selection (true = yes, false = no).
    pub selected: bool,
    /// Default value (affects initial selection).
    pub default: bool,
    /// Label for the "yes" option.
    pub yes_label: String,
    /// Label for the "no" option.
    pub no_label: String,
    /// Color for the selected option.
    pub selected_color: Option<Color>,
    /// Color for the unselected option.
    pub unselected_color: Option<Color>,
    /// Whether to show the question inline with options (deprecated, use style).
    pub inline: bool,
    /// Separator between yes and no (for inline style).
    pub separator: String,
    /// Visual style of the prompt.
    pub style: ConfirmStyle,
}

impl Default for ConfirmProps {
    fn default() -> Self {
        Self {
            message: String::new(),
            selected: false,
            default: false,
            yes_label: "Yes".to_string(),
            no_label: "No".to_string(),
            selected_color: Some(Color::Cyan),
            unselected_color: Some(Color::DarkGray),
            inline: true,
            separator: " / ".to_string(),
            style: ConfirmStyle::Inline,
        }
    }
}

impl ConfirmProps {
    /// Create new ConfirmProps with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            ..Default::default()
        }
    }

    /// Set the current selection.
    #[must_use]
    pub fn selected(mut self, yes: bool) -> Self {
        self.selected = yes;
        self
    }

    /// Set the default value.
    #[must_use]
    pub fn default_value(mut self, yes: bool) -> Self {
        self.default = yes;
        self.selected = yes;
        self
    }

    /// Set custom labels for yes/no.
    #[must_use]
    pub fn labels(mut self, yes: impl Into<String>, no: impl Into<String>) -> Self {
        self.yes_label = yes.into();
        self.no_label = no.into();
        self
    }

    /// Set the selected option color.
    #[must_use]
    pub fn selected_color(mut self, color: Color) -> Self {
        self.selected_color = Some(color);
        self
    }

    /// Set the unselected option color.
    #[must_use]
    pub fn unselected_color(mut self, color: Color) -> Self {
        self.unselected_color = Some(color);
        self
    }

    /// Display options on a separate line.
    #[must_use]
    pub fn multiline(mut self) -> Self {
        self.inline = false;
        self
    }

    /// Set the separator between options.
    #[must_use]
    pub fn separator(mut self, sep: impl Into<String>) -> Self {
        self.separator = sep.into();
        self
    }

    /// Use button-style rendering.
    #[must_use]
    pub fn button_style(mut self) -> Self {
        self.style = ConfirmStyle::Button;
        self
    }

    /// Toggle the selection.
    pub fn toggle(&mut self) {
        self.selected = !self.selected;
    }

    /// Select yes.
    pub fn select_yes(&mut self) {
        self.selected = true;
    }

    /// Select no.
    pub fn select_no(&mut self) {
        self.selected = false;
    }

    /// Get the current answer.
    pub fn answer(&self) -> bool {
        self.selected
    }

    /// Build the display string.
    pub fn render_string(&self) -> String {
        match self.style {
            ConfirmStyle::Inline => self.render_inline(),
            ConfirmStyle::Button => self.render_button(),
        }
    }

    fn render_inline(&self) -> String {
        let yes_display = if self.selected {
            format!("[{}]", self.yes_label)
        } else {
            self.yes_label.clone()
        };

        let no_display = if !self.selected {
            format!("[{}]", self.no_label)
        } else {
            self.no_label.clone()
        };

        if self.inline {
            format!(
                "{} {}{}{}",
                self.message, yes_display, self.separator, no_display
            )
        } else {
            format!(
                "{}\n  {}{}{}",
                self.message, yes_display, self.separator, no_display
            )
        }
    }

    fn render_button(&self) -> String {
        // Button style: [ Yes ]  [ No ] with selected one highlighted
        let yes_btn = if self.selected {
            format!("▸ {} ◂", self.yes_label)
        } else {
            format!("  {}  ", self.yes_label)
        };

        let no_btn = if !self.selected {
            format!("▸ {} ◂", self.no_label)
        } else {
            format!("  {}  ", self.no_label)
        };

        if self.message.is_empty() {
            format!("[{}]  [{}]", yes_btn, no_btn)
        } else if self.inline {
            format!("{}\n[{}]  [{}]", self.message, yes_btn, no_btn)
        } else {
            format!("{}\n\n[{}]  [{}]", self.message, yes_btn, no_btn)
        }
    }
}

/// A component that displays a yes/no confirmation prompt.
///
/// # Examples
///
/// ```ignore
/// let props = ConfirmProps::new("Are you sure you want to delete?")
///     .default_value(false)
///     .selected_color(Color::Yellow);
///
/// // Handle input in your event loop:
/// match key.code {
///     KeyCode::Left | KeyCode::Char('y') => confirm.select_yes(),
///     KeyCode::Right | KeyCode::Char('n') => confirm.select_no(),
///     KeyCode::Enter => return Some(confirm.answer()),
///     _ => {}
/// }
/// ```
pub struct Confirm;

impl Component for Confirm {
    type Props = ConfirmProps;

    fn render(props: &Self::Props) -> Element {
        let content = props.render_string();

        let mut style = Style::new();
        if let Some(color) = props.selected_color {
            style = style.fg(color);
        }

        Element::styled_text(&content, style)
    }
}

/// Helper to create a simple confirm prompt string.
///
/// # Example
///
/// ```ignore
/// let prompt = confirm_prompt("Delete file?", true);
/// // Returns: "Delete file? [Yes] / No"
/// ```
pub fn confirm_prompt(message: &str, selected_yes: bool) -> String {
    ConfirmProps::new(message)
        .selected(selected_yes)
        .render_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confirm_props_default() {
        let props = ConfirmProps::default();
        assert!(props.message.is_empty());
        assert!(!props.selected);
        assert!(!props.default);
    }

    #[test]
    fn test_confirm_props_new() {
        let props = ConfirmProps::new("Delete?");
        assert_eq!(props.message, "Delete?");
    }

    #[test]
    fn test_confirm_props_default_value() {
        let props = ConfirmProps::new("Continue?").default_value(true);
        assert!(props.selected);
        assert!(props.default);
    }

    #[test]
    fn test_confirm_props_labels() {
        let props = ConfirmProps::new("Save?").labels("Save", "Discard");
        assert_eq!(props.yes_label, "Save");
        assert_eq!(props.no_label, "Discard");
    }

    #[test]
    fn test_confirm_props_toggle() {
        let mut props = ConfirmProps::new("Test").selected(false);
        assert!(!props.selected);

        props.toggle();
        assert!(props.selected);

        props.toggle();
        assert!(!props.selected);
    }

    #[test]
    fn test_confirm_props_select() {
        let mut props = ConfirmProps::new("Test");

        props.select_yes();
        assert!(props.answer());

        props.select_no();
        assert!(!props.answer());
    }

    #[test]
    fn test_confirm_render_string_yes_selected() {
        let props = ConfirmProps::new("Continue?").selected(true);
        let output = props.render_string();
        assert!(output.contains("[Yes]"));
        assert!(output.contains("No"));
        assert!(!output.contains("[No]"));
    }

    #[test]
    fn test_confirm_render_string_no_selected() {
        let props = ConfirmProps::new("Continue?").selected(false);
        let output = props.render_string();
        assert!(output.contains("Yes"));
        assert!(output.contains("[No]"));
        assert!(!output.contains("[Yes]"));
    }

    #[test]
    fn test_confirm_render_string_custom_labels() {
        let props = ConfirmProps::new("Save changes?")
            .labels("Save", "Discard")
            .selected(true);
        let output = props.render_string();
        assert!(output.contains("[Save]"));
        assert!(output.contains("Discard"));
    }

    #[test]
    fn test_confirm_render_string_multiline() {
        let props = ConfirmProps::new("Question?").multiline();
        let output = props.render_string();
        assert!(output.contains('\n'));
    }

    #[test]
    fn test_confirm_prompt_helper() {
        let output = confirm_prompt("Delete?", true);
        assert!(output.contains("Delete?"));
        assert!(output.contains("[Yes]"));
    }

    #[test]
    fn test_confirm_component_render() {
        let props = ConfirmProps::new("Test?").selected(true);
        let elem = Confirm::render(&props);
        match elem {
            Element::Text { content, .. } => {
                assert!(content.contains("Test?"));
                assert!(content.contains("[Yes]"));
            }
            _ => panic!("Expected Text element"),
        }
    }
}
