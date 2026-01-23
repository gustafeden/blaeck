//! Modal/Dialog component for displaying overlays and prompts.
//!
//! ## When to use Modal
//!
//! - Confirmation dialogs ("Are you sure?")
//! - Error/warning/success messages
//! - Information that needs acknowledgment before continuing
//!
//! ## See also
//!
//! - [`Confirm`](super::Confirm) — Inline yes/no without a modal box
//! - [`Box`](super::Box) — Build custom containers (Modal is built on Box)
//!
//! # Example
//!
//! ```ignore
//! use blaeck::prelude::*;
//!
//! let modal = Element::node::<Modal>(
//!     ModalProps::new("Confirm Action")
//!         .body("Are you sure you want to proceed?")
//!         .style(ModalStyle::Warning)
//!         .buttons(vec![
//!             ModalButton::cancel(),
//!             ModalButton::ok(),
//!         ]),
//!     vec![],
//! );
//! ```

use crate::components::box_component::BorderStyle;
use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// Modal visual style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModalStyle {
    /// Default style (no special coloring).
    #[default]
    Default,
    /// Info style (blue/cyan).
    Info,
    /// Success style (green).
    Success,
    /// Warning style (yellow).
    Warning,
    /// Error/danger style (red).
    Error,
}

impl ModalStyle {
    /// Get the title color for this style.
    pub fn title_color(&self) -> Option<Color> {
        match self {
            ModalStyle::Default => None,
            ModalStyle::Info => Some(Color::Cyan),
            ModalStyle::Success => Some(Color::Green),
            ModalStyle::Warning => Some(Color::Yellow),
            ModalStyle::Error => Some(Color::Red),
        }
    }

    /// Get the border color for this style.
    pub fn border_color(&self) -> Option<Color> {
        match self {
            ModalStyle::Default => Some(Color::White),
            ModalStyle::Info => Some(Color::Cyan),
            ModalStyle::Success => Some(Color::Green),
            ModalStyle::Warning => Some(Color::Yellow),
            ModalStyle::Error => Some(Color::Red),
        }
    }

    /// Get the icon for this style.
    pub fn icon(&self) -> Option<&'static str> {
        match self {
            ModalStyle::Default => None,
            ModalStyle::Info => Some("ℹ"),
            ModalStyle::Success => Some("✓"),
            ModalStyle::Warning => Some("⚠"),
            ModalStyle::Error => Some("✗"),
        }
    }
}

/// A button in the modal footer.
#[derive(Debug, Clone)]
pub struct ModalButton {
    /// Button label.
    pub label: String,
    /// Button color.
    pub color: Option<Color>,
    /// Whether this button is highlighted/primary.
    pub primary: bool,
}

impl ModalButton {
    /// Create a new button.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            color: None,
            primary: false,
        }
    }

    /// Set the button color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Mark as primary button.
    #[must_use]
    pub fn primary(mut self) -> Self {
        self.primary = true;
        self
    }

    /// Create an "OK" button.
    pub fn ok() -> Self {
        Self::new("OK").color(Color::Green).primary()
    }

    /// Create a "Cancel" button.
    pub fn cancel() -> Self {
        Self::new("Cancel").color(Color::DarkGray)
    }

    /// Create a "Yes" button.
    pub fn yes() -> Self {
        Self::new("Yes").color(Color::Green).primary()
    }

    /// Create a "No" button.
    pub fn no() -> Self {
        Self::new("No").color(Color::Red)
    }

    /// Create a "Confirm" button.
    pub fn confirm() -> Self {
        Self::new("Confirm").color(Color::Green).primary()
    }

    /// Create a "Delete" button.
    pub fn delete() -> Self {
        Self::new("Delete").color(Color::Red).primary()
    }

    /// Create a "Close" button.
    pub fn close() -> Self {
        Self::new("Close").color(Color::DarkGray)
    }
}

/// Properties for the Modal component.
#[derive(Debug, Clone)]
pub struct ModalProps {
    /// Modal title.
    pub title: String,
    /// Modal body text.
    pub body: Option<String>,
    /// Modal style.
    pub style: ModalStyle,
    /// Border style.
    pub border_style: BorderStyle,
    /// Buttons in footer.
    pub buttons: Vec<ModalButton>,
    /// Minimum width.
    pub min_width: usize,
    /// Maximum width (0 = no limit).
    pub max_width: usize,
    /// Padding inside the modal.
    pub padding: usize,
    /// Show icon based on style.
    pub show_icon: bool,
    /// Center the title.
    pub center_title: bool,
    /// Dim the border.
    pub dim_border: bool,
}

impl Default for ModalProps {
    fn default() -> Self {
        Self {
            title: String::new(),
            body: None,
            style: ModalStyle::Default,
            border_style: BorderStyle::Round,
            buttons: Vec::new(),
            min_width: 30,
            max_width: 60,
            padding: 1,
            show_icon: true,
            center_title: false,
            dim_border: false,
        }
    }
}

impl ModalProps {
    /// Create new modal props with a title.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Set the body text.
    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Set the modal style.
    #[must_use]
    pub fn style(mut self, style: ModalStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the border style.
    #[must_use]
    pub fn border_style(mut self, style: BorderStyle) -> Self {
        self.border_style = style;
        self
    }

    /// Set the buttons.
    #[must_use]
    pub fn buttons(mut self, buttons: Vec<ModalButton>) -> Self {
        self.buttons = buttons;
        self
    }

    /// Add a single button.
    #[must_use]
    pub fn button(mut self, button: ModalButton) -> Self {
        self.buttons.push(button);
        self
    }

    /// Set minimum width.
    #[must_use]
    pub fn min_width(mut self, width: usize) -> Self {
        self.min_width = width;
        self
    }

    /// Set maximum width.
    #[must_use]
    pub fn max_width(mut self, width: usize) -> Self {
        self.max_width = width;
        self
    }

    /// Set padding.
    #[must_use]
    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    /// Show or hide the style icon.
    #[must_use]
    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    /// Center the title.
    #[must_use]
    pub fn center_title(mut self, center: bool) -> Self {
        self.center_title = center;
        self
    }

    /// Dim the border.
    #[must_use]
    pub fn dim_border(mut self, dim: bool) -> Self {
        self.dim_border = dim;
        self
    }
}

/// A modal/dialog component.
pub struct Modal;

impl Component for Modal {
    type Props = ModalProps;

    fn render(props: &Self::Props) -> Element {
        let chars = props.border_style.chars();
        let border_color = props.style.border_color();
        let title_color = props.style.title_color();

        // Build title with optional icon
        let title = if props.show_icon {
            if let Some(icon) = props.style.icon() {
                format!("{} {}", icon, props.title)
            } else {
                props.title.clone()
            }
        } else {
            props.title.clone()
        };

        // Calculate content width
        let title_width = unicode_width::UnicodeWidthStr::width(title.as_str());
        let body_width = props.body.as_ref().map_or(0, |b| {
            b.lines()
                .map(unicode_width::UnicodeWidthStr::width)
                .max()
                .unwrap_or(0)
        });
        let buttons_width: usize = props
            .buttons
            .iter()
            .map(|b| unicode_width::UnicodeWidthStr::width(b.label.as_str()) + 4) // [ label ]
            .sum::<usize>()
            + props.buttons.len().saturating_sub(1) * 2; // spacing between buttons

        let content_width = title_width.max(body_width).max(buttons_width);
        let inner_width = content_width + props.padding * 2;
        let inner_width = inner_width.max(props.min_width);
        let inner_width = if props.max_width > 0 {
            inner_width.min(props.max_width)
        } else {
            inner_width
        };
        let _total_width = inner_width + 2; // +2 for borders

        let mut lines: Vec<Element> = Vec::new();

        // Border style
        let border_style = if props.dim_border {
            border_color
                .map(|c| Style::new().fg(c).add_modifier(Modifier::DIM))
                .unwrap_or_else(|| Style::new().add_modifier(Modifier::DIM))
        } else {
            border_color.map(|c| Style::new().fg(c)).unwrap_or_default()
        };

        // Top border line
        let top_line = format!(
            "{}{}{}",
            chars.top_left,
            chars.horizontal.to_string().repeat(inner_width),
            chars.top_right
        );
        lines.push(Element::styled_text(&top_line, border_style));

        // Title line
        let mut title_segments = vec![Element::styled_text(
            chars.vertical.to_string(),
            border_style,
        )];

        let title_style = title_color
            .map(|c| Style::new().fg(c).add_modifier(Modifier::BOLD))
            .unwrap_or_else(|| Style::new().add_modifier(Modifier::BOLD));

        if props.center_title {
            let padding_total = inner_width.saturating_sub(title_width);
            let left_pad = padding_total / 2;
            let right_pad = padding_total - left_pad;
            title_segments.push(Element::text(" ".repeat(left_pad)));
            title_segments.push(Element::styled_text(&title, title_style));
            title_segments.push(Element::text(" ".repeat(right_pad)));
        } else {
            let left_pad = " ".repeat(props.padding);
            let right_pad = " ".repeat(inner_width.saturating_sub(title_width + props.padding));
            title_segments.push(Element::text(&left_pad));
            title_segments.push(Element::styled_text(&title, title_style));
            title_segments.push(Element::text(&right_pad));
        }
        title_segments.push(Element::styled_text(
            chars.vertical.to_string(),
            border_style,
        ));
        lines.push(Element::Fragment(title_segments));

        // Separator line under title
        let sep_line = format!(
            "{}{}{}",
            chars.vertical,
            chars.horizontal.to_string().repeat(inner_width),
            chars.vertical
        );
        lines.push(Element::styled_text(&sep_line, border_style));

        // Body lines
        if let Some(ref body) = props.body {
            // Empty line for padding
            if props.padding > 0 {
                let empty_line = format!(
                    "{}{}{}",
                    chars.vertical,
                    " ".repeat(inner_width),
                    chars.vertical
                );
                lines.push(Element::styled_text(&empty_line, border_style));
            }

            for line in body.lines() {
                let line_width = unicode_width::UnicodeWidthStr::width(line);
                let content = format!(
                    "{}{}{}",
                    " ".repeat(props.padding),
                    line,
                    " ".repeat(inner_width.saturating_sub(line_width + props.padding))
                );
                let line_segments = vec![
                    Element::styled_text(chars.vertical.to_string(), border_style),
                    Element::text(&content),
                    Element::styled_text(chars.vertical.to_string(), border_style),
                ];
                lines.push(Element::Fragment(line_segments));
            }

            // Empty line for padding
            if props.padding > 0 {
                let empty_line = format!(
                    "{}{}{}",
                    chars.vertical,
                    " ".repeat(inner_width),
                    chars.vertical
                );
                lines.push(Element::styled_text(&empty_line, border_style));
            }
        }

        // Buttons
        if !props.buttons.is_empty() {
            // Separator before buttons
            let sep_line = format!(
                "{}{}{}",
                chars.vertical,
                chars.horizontal.to_string().repeat(inner_width),
                chars.vertical
            );
            lines.push(Element::styled_text(&sep_line, border_style));

            // Button line
            let mut button_segments: Vec<Element> = vec![Element::styled_text(
                chars.vertical.to_string(),
                border_style,
            )];

            // Calculate button string
            let mut button_parts: Vec<Element> = Vec::new();
            for (i, btn) in props.buttons.iter().enumerate() {
                if i > 0 {
                    button_parts.push(Element::text("  "));
                }
                let btn_text = format!("[ {} ]", btn.label);
                let btn_style = if btn.primary {
                    btn.color
                        .map(|c| Style::new().fg(c).add_modifier(Modifier::BOLD))
                        .unwrap_or_else(|| Style::new().add_modifier(Modifier::BOLD))
                } else {
                    btn.color.map(|c| Style::new().fg(c)).unwrap_or_default()
                };
                button_parts.push(Element::styled_text(&btn_text, btn_style));
            }

            // Center buttons
            let buttons_total_width: usize = props
                .buttons
                .iter()
                .map(|b| unicode_width::UnicodeWidthStr::width(b.label.as_str()) + 4)
                .sum::<usize>()
                + props.buttons.len().saturating_sub(1) * 2;
            let padding_total = inner_width.saturating_sub(buttons_total_width);
            let left_pad = padding_total / 2;
            let right_pad = padding_total - left_pad;

            button_segments.push(Element::text(" ".repeat(left_pad)));
            button_segments.extend(button_parts);
            button_segments.push(Element::text(" ".repeat(right_pad)));
            button_segments.push(Element::styled_text(
                chars.vertical.to_string(),
                border_style,
            ));
            lines.push(Element::Fragment(button_segments));
        }

        // Bottom border
        let bottom_line = format!(
            "{}{}{}",
            chars.bottom_left,
            chars.horizontal.to_string().repeat(inner_width),
            chars.bottom_right
        );
        lines.push(Element::styled_text(&bottom_line, border_style));

        Element::Fragment(lines)
    }
}

/// Create a simple alert modal.
pub fn alert(title: &str, message: &str) -> Element {
    Element::node::<Modal>(
        ModalProps::new(title)
            .body(message)
            .style(ModalStyle::Info)
            .button(ModalButton::ok()),
        vec![],
    )
}

/// Create a confirm modal with Yes/No buttons.
pub fn confirm_modal(title: &str, message: &str) -> Element {
    Element::node::<Modal>(
        ModalProps::new(title)
            .body(message)
            .style(ModalStyle::Warning)
            .buttons(vec![ModalButton::no(), ModalButton::yes()]),
        vec![],
    )
}

/// Create an error modal.
pub fn error_modal(title: &str, message: &str) -> Element {
    Element::node::<Modal>(
        ModalProps::new(title)
            .body(message)
            .style(ModalStyle::Error)
            .button(ModalButton::close()),
        vec![],
    )
}

/// Create a success modal.
pub fn success_modal(title: &str, message: &str) -> Element {
    Element::node::<Modal>(
        ModalProps::new(title)
            .body(message)
            .style(ModalStyle::Success)
            .button(ModalButton::ok()),
        vec![],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_style_colors() {
        assert!(ModalStyle::Default.title_color().is_none());
        assert_eq!(ModalStyle::Info.title_color(), Some(Color::Cyan));
        assert_eq!(ModalStyle::Success.title_color(), Some(Color::Green));
        assert_eq!(ModalStyle::Warning.title_color(), Some(Color::Yellow));
        assert_eq!(ModalStyle::Error.title_color(), Some(Color::Red));
    }

    #[test]
    fn test_modal_style_icons() {
        assert!(ModalStyle::Default.icon().is_none());
        assert_eq!(ModalStyle::Info.icon(), Some("ℹ"));
        assert_eq!(ModalStyle::Success.icon(), Some("✓"));
        assert_eq!(ModalStyle::Warning.icon(), Some("⚠"));
        assert_eq!(ModalStyle::Error.icon(), Some("✗"));
    }

    #[test]
    fn test_modal_button_new() {
        let btn = ModalButton::new("Test");
        assert_eq!(btn.label, "Test");
        assert!(!btn.primary);
        assert!(btn.color.is_none());
    }

    #[test]
    fn test_modal_button_presets() {
        let ok = ModalButton::ok();
        assert_eq!(ok.label, "OK");
        assert!(ok.primary);
        assert_eq!(ok.color, Some(Color::Green));

        let cancel = ModalButton::cancel();
        assert_eq!(cancel.label, "Cancel");
        assert!(!cancel.primary);
    }

    #[test]
    fn test_modal_props_new() {
        let props = ModalProps::new("Test Title");
        assert_eq!(props.title, "Test Title");
        assert!(props.body.is_none());
        assert_eq!(props.style, ModalStyle::Default);
    }

    #[test]
    fn test_modal_props_builder() {
        let props = ModalProps::new("Title")
            .body("Body text")
            .style(ModalStyle::Warning)
            .buttons(vec![ModalButton::cancel(), ModalButton::ok()])
            .min_width(40)
            .max_width(80);

        assert_eq!(props.title, "Title");
        assert_eq!(props.body, Some("Body text".to_string()));
        assert_eq!(props.style, ModalStyle::Warning);
        assert_eq!(props.buttons.len(), 2);
        assert_eq!(props.min_width, 40);
        assert_eq!(props.max_width, 80);
    }

    #[test]
    fn test_modal_render() {
        let props = ModalProps::new("Test").body("Test body");
        let elem = Modal::render(&props);
        assert!(elem.is_fragment());
    }

    #[test]
    fn test_alert_helper() {
        let elem = alert("Alert", "This is an alert");
        assert!(elem.is_node());
    }

    #[test]
    fn test_confirm_modal_helper() {
        let elem = confirm_modal("Confirm", "Are you sure?");
        assert!(elem.is_node());
    }

    #[test]
    fn test_error_modal_helper() {
        let elem = error_modal("Error", "Something went wrong");
        assert!(elem.is_node());
    }

    #[test]
    fn test_success_modal_helper() {
        let elem = success_modal("Success", "Operation completed");
        assert!(elem.is_node());
    }
}
