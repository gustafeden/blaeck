//! Link component - hyperlink text.
//!
//! The Link component displays underlined text that can optionally be
//! a clickable hyperlink in terminals that support OSC 8.
//!
//! ## When to use Link
//!
//! - URLs that should be clickable
//! - References to documentation
//! - "Learn more" or help links
//!
//! ## See also
//!
//! - [`Text`](super::Text) — Plain text with underline style
//! - [`Markdown`](super::Markdown) — Auto-converts [text](url) to links

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// Properties for the Link component.
#[derive(Debug, Clone)]
pub struct LinkProps {
    /// The text to display.
    pub text: String,
    /// The URL to link to (optional).
    pub url: Option<String>,
    /// Text color (defaults to Cyan).
    pub color: Option<Color>,
    /// Whether to underline (defaults to true).
    pub underline: bool,
    /// Whether to make the text bold.
    pub bold: bool,
    /// Whether to dim the text.
    pub dim: bool,
}

impl Default for LinkProps {
    fn default() -> Self {
        Self {
            text: String::new(),
            url: None,
            color: Some(Color::Cyan),
            underline: true,
            bold: false,
            dim: false,
        }
    }
}

impl LinkProps {
    /// Create a new link with the given text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    /// Create a new link with text and URL.
    pub fn with_url(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: Some(url.into()),
            ..Default::default()
        }
    }

    /// Set the URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the text color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Disable underline.
    #[must_use]
    pub fn no_underline(mut self) -> Self {
        self.underline = false;
        self
    }

    /// Make the link bold.
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Make the link dimmed.
    #[must_use]
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Build the display string with OSC 8 hyperlink if URL is set.
    ///
    /// OSC 8 format: `\x1b]8;;URL\x07TEXT\x1b]8;;\x07`
    pub fn render_string(&self) -> String {
        match &self.url {
            Some(url) => {
                // OSC 8 hyperlink format
                format!("\x1b]8;;{}\x07{}\x1b]8;;\x07", url, self.text)
            }
            None => self.text.clone(),
        }
    }

    /// Check if this link has a URL.
    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }
}

/// A component that displays a hyperlink.
///
/// # Examples
///
/// ```ignore
/// // Simple underlined link text
/// Element::node::<Link>(
///     LinkProps::new("Click here"),
///     vec![]
/// )
///
/// // Clickable hyperlink (in supporting terminals)
/// Element::node::<Link>(
///     LinkProps::with_url("Rust Homepage", "https://rust-lang.org"),
///     vec![]
/// )
/// ```
pub struct Link;

impl Component for Link {
    type Props = LinkProps;

    fn render(props: &Self::Props) -> Element {
        // Note: We render just the text without OSC 8 sequences because
        // the layout system's character grid doesn't handle escape sequences.
        // Use link_url() helper for raw OSC 8 output outside the layout system.
        let content = props.text.clone();

        let mut style = Style::new();
        if let Some(color) = props.color {
            style = style.fg(color);
        }
        if props.underline {
            style = style.add_modifier(Modifier::UNDERLINED);
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

/// Helper to create a simple link string (underlined text, no URL).
pub fn link(text: &str) -> String {
    text.to_string()
}

/// Helper to create a link with URL (OSC 8 format).
pub fn link_url(text: &str, url: &str) -> String {
    format!("\x1b]8;;{}\x07{}\x1b]8;;\x07", url, text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_props_default() {
        let props = LinkProps::default();
        assert!(props.text.is_empty());
        assert!(props.url.is_none());
        assert_eq!(props.color, Some(Color::Cyan));
        assert!(props.underline);
        assert!(!props.bold);
    }

    #[test]
    fn test_link_props_new() {
        let props = LinkProps::new("Click me");
        assert_eq!(props.text, "Click me");
        assert!(props.url.is_none());
    }

    #[test]
    fn test_link_props_with_url() {
        let props = LinkProps::with_url("Rust", "https://rust-lang.org");
        assert_eq!(props.text, "Rust");
        assert_eq!(props.url, Some("https://rust-lang.org".to_string()));
    }

    #[test]
    fn test_link_props_builder() {
        let props = LinkProps::new("Test")
            .url("https://example.com")
            .color(Color::Blue)
            .bold()
            .no_underline();
        assert_eq!(props.text, "Test");
        assert_eq!(props.url, Some("https://example.com".to_string()));
        assert_eq!(props.color, Some(Color::Blue));
        assert!(props.bold);
        assert!(!props.underline);
    }

    #[test]
    fn test_link_render_string_no_url() {
        let props = LinkProps::new("Just text");
        assert_eq!(props.render_string(), "Just text");
    }

    #[test]
    fn test_link_render_string_with_url() {
        let props = LinkProps::with_url("Link", "https://example.com");
        let output = props.render_string();
        assert!(output.contains("\x1b]8;;https://example.com\x07"));
        assert!(output.contains("Link"));
        assert!(output.ends_with("\x1b]8;;\x07"));
    }

    #[test]
    fn test_link_has_url() {
        let no_url = LinkProps::new("Text");
        assert!(!no_url.has_url());

        let with_url = LinkProps::with_url("Text", "https://example.com");
        assert!(with_url.has_url());
    }

    #[test]
    fn test_link_helper() {
        assert_eq!(link("test"), "test");
    }

    #[test]
    fn test_link_url_helper() {
        let output = link_url("Click", "https://example.com");
        assert!(output.contains("\x1b]8;;https://example.com\x07"));
        assert!(output.contains("Click"));
    }

    #[test]
    fn test_link_component_render() {
        let props = LinkProps::new("Test Link").color(Color::Blue);
        let elem = Link::render(&props);
        match elem {
            Element::Text { content, style } => {
                assert_eq!(content, "Test Link");
                assert_eq!(style.fg, Color::Blue);
                assert!(style.modifiers.contains(Modifier::UNDERLINED));
            }
            _ => panic!("Expected Text element"),
        }
    }

    #[test]
    fn test_link_component_render_no_underline() {
        let props = LinkProps::new("No underline").no_underline();
        let elem = Link::render(&props);
        match elem {
            Element::Text { style, .. } => {
                assert!(!style.modifiers.contains(Modifier::UNDERLINED));
            }
            _ => panic!("Expected Text element"),
        }
    }
}
