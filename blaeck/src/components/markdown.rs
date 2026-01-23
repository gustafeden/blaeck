//! Markdown component - Render CommonMark-formatted text with terminal styling.
//!
//! Parses and renders markdown using pulldown-cmark. Supports:
//! headers, bold, italic, code spans, code blocks, lists,
//! blockquotes, links (OSC 8), and horizontal rules.
//!
//! ## When to use Markdown
//!
//! - README or help text from markdown files
//! - User-provided formatted content
//! - Documentation display
//!
//! ## See also
//!
//! - [`Text`](super::Text) — Plain text without parsing
//! - [`SyntaxHighlight`](super::SyntaxHighlight) — Code blocks with highlighting

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};
use pulldown_cmark::{Event, Options, Parser, Tag};

/// Properties for the Markdown component.
#[derive(Debug, Clone)]
pub struct MarkdownProps {
    /// The markdown content to render.
    pub content: String,
    /// Color for headers.
    pub header_color: Color,
    /// Color for inline code spans.
    pub code_color: Color,
    /// Color for links.
    pub link_color: Color,
    /// Color for blockquotes.
    pub quote_color: Color,
    /// Whether to enable OSC 8 terminal hyperlinks.
    pub enable_hyperlinks: bool,
}

impl Default for MarkdownProps {
    fn default() -> Self {
        Self {
            content: String::new(),
            header_color: Color::Cyan,
            code_color: Color::Yellow,
            link_color: Color::Blue,
            quote_color: Color::DarkGray,
            enable_hyperlinks: true,
        }
    }
}

impl MarkdownProps {
    /// Create new MarkdownProps with content.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    /// Set the header color.
    #[must_use]
    pub fn header_color(mut self, color: Color) -> Self {
        self.header_color = color;
        self
    }

    /// Set the code color.
    #[must_use]
    pub fn code_color(mut self, color: Color) -> Self {
        self.code_color = color;
        self
    }

    /// Set the link color.
    #[must_use]
    pub fn link_color(mut self, color: Color) -> Self {
        self.link_color = color;
        self
    }

    /// Set the blockquote color.
    #[must_use]
    pub fn quote_color(mut self, color: Color) -> Self {
        self.quote_color = color;
        self
    }

    /// Enable or disable OSC 8 terminal hyperlinks.
    #[must_use]
    pub fn enable_hyperlinks(mut self, enable: bool) -> Self {
        self.enable_hyperlinks = enable;
        self
    }
}

/// Style state for tracking nested formatting.
#[derive(Debug, Clone, Default)]
struct StyleState {
    bold: bool,
    italic: bool,
    strikethrough: bool,
    code: bool,
    heading_level: Option<u8>,
    in_blockquote: bool,
    link_url: Option<String>,
}

impl StyleState {
    fn to_style(&self, props: &MarkdownProps) -> Style {
        let mut style = Style::new();
        let mut modifiers = Modifier::empty();

        if self.bold || self.heading_level.is_some() {
            modifiers |= Modifier::BOLD;
        }
        if self.italic {
            modifiers |= Modifier::ITALIC;
        }
        if self.strikethrough {
            modifiers |= Modifier::CROSSED_OUT;
        }

        if self.code {
            style = style.fg(props.code_color);
        } else if self.heading_level.is_some() {
            style = style.fg(props.header_color);
        } else if self.in_blockquote {
            style = style.fg(props.quote_color);
            modifiers |= Modifier::DIM;
        } else if self.link_url.is_some() {
            style = style.fg(props.link_color);
            modifiers |= Modifier::UNDERLINED;
        }

        style.add_modifier(modifiers)
    }
}

/// List tracking state.
#[derive(Debug, Clone)]
enum ListKind {
    Bullet,
    Ordered(u64),
}

/// A component that renders markdown content.
///
/// # Examples
///
/// ```ignore
/// // Simple markdown
/// Element::node::<Markdown>(
///     MarkdownProps::new("# Hello\n\nThis is **bold** and *italic*."),
///     vec![]
/// )
///
/// // With custom colors
/// Element::node::<Markdown>(
///     MarkdownProps::new("## Header\n\n`code` and [link](url)")
///         .header_color(Color::Green)
///         .code_color(Color::Magenta),
///     vec![]
/// )
/// ```
pub struct Markdown;

impl Component for Markdown {
    type Props = MarkdownProps;

    fn render(props: &Self::Props) -> Element {
        if props.content.is_empty() {
            return Element::text("");
        }

        let options = Options::ENABLE_STRIKETHROUGH;
        let parser = Parser::new_ext(&props.content, options);

        let mut lines: Vec<Element> = Vec::new();
        let mut current_line: Vec<(String, Style)> = Vec::new();
        let mut style_state = StyleState::default();
        let mut list_stack: Vec<ListKind> = Vec::new();
        let mut in_code_block = false;

        for event in parser {
            match event {
                Event::Start(tag) => {
                    match tag {
                        Tag::Heading(level, _, _) => {
                            style_state.heading_level = Some(level as u8);
                        }
                        Tag::Strong => {
                            style_state.bold = true;
                        }
                        Tag::Emphasis => {
                            style_state.italic = true;
                        }
                        Tag::Strikethrough => {
                            style_state.strikethrough = true;
                        }
                        Tag::CodeBlock(_) => {
                            in_code_block = true;
                            style_state.code = true;
                        }
                        Tag::BlockQuote => {
                            style_state.in_blockquote = true;
                        }
                        Tag::List(start) => {
                            if let Some(n) = start {
                                list_stack.push(ListKind::Ordered(n));
                            } else {
                                list_stack.push(ListKind::Bullet);
                            }
                        }
                        Tag::Item => {
                            // Add list marker
                            let indent = "  ".repeat(list_stack.len().saturating_sub(1));
                            let marker = match list_stack.last_mut() {
                                Some(ListKind::Bullet) => format!("{}• ", indent),
                                Some(ListKind::Ordered(n)) => {
                                    let marker = format!("{}{}. ", indent, n);
                                    *n += 1;
                                    marker
                                }
                                None => String::new(),
                            };
                            if !marker.is_empty() {
                                current_line.push((marker, Style::new()));
                            }
                        }
                        Tag::Link(_, dest_url, _) => {
                            style_state.link_url = Some(dest_url.to_string());
                        }
                        _ => {}
                    }
                }
                Event::End(tag) => {
                    match tag {
                        Tag::Heading(_, _, _) => {
                            // Flush current line
                            if !current_line.is_empty() {
                                lines.push(line_to_element(&current_line));
                                current_line.clear();
                            }
                            style_state.heading_level = None;
                        }
                        Tag::Strong => {
                            style_state.bold = false;
                        }
                        Tag::Emphasis => {
                            style_state.italic = false;
                        }
                        Tag::Strikethrough => {
                            style_state.strikethrough = false;
                        }
                        Tag::CodeBlock(_) => {
                            in_code_block = false;
                            style_state.code = false;
                        }
                        Tag::BlockQuote => {
                            style_state.in_blockquote = false;
                        }
                        Tag::List(_) => {
                            list_stack.pop();
                        }
                        Tag::Item => {
                            // Flush current line
                            if !current_line.is_empty() {
                                lines.push(line_to_element(&current_line));
                                current_line.clear();
                            }
                        }
                        Tag::Paragraph => {
                            // Flush current line
                            if !current_line.is_empty() {
                                lines.push(line_to_element(&current_line));
                                current_line.clear();
                            }
                        }
                        Tag::Link(_, _, _) => {
                            style_state.link_url = None;
                        }
                        _ => {}
                    }
                }
                Event::Text(text) => {
                    let style = style_state.to_style(props);
                    let text_str = text.to_string();

                    // Handle OSC 8 hyperlinks for links
                    if let Some(ref url) = style_state.link_url {
                        if props.enable_hyperlinks {
                            // OSC 8 format: \x1b]8;;URL\x1b\\TEXT\x1b]8;;\x1b\\
                            let hyperlink = format!(
                                "\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\",
                                url, text_str
                            );
                            current_line.push((hyperlink, style));
                        } else {
                            // Fallback: show as [text](url)
                            current_line.push((text_str, style));
                        }
                    } else if in_code_block {
                        // Code blocks may have multiple lines
                        for (i, line) in text_str.lines().enumerate() {
                            if i > 0 {
                                // Flush previous line
                                if !current_line.is_empty() {
                                    lines.push(line_to_element(&current_line));
                                    current_line.clear();
                                }
                            }
                            current_line.push((format!("  {}", line), style));
                        }
                    } else if style_state.in_blockquote {
                        // Blockquote prefix
                        for (i, line) in text_str.lines().enumerate() {
                            if i > 0 {
                                if !current_line.is_empty() {
                                    lines.push(line_to_element(&current_line));
                                    current_line.clear();
                                }
                            }
                            current_line.push((format!("> {}", line), style));
                        }
                    } else {
                        current_line.push((text_str, style));
                    }
                }
                Event::Code(code) => {
                    let style = Style::new().fg(props.code_color);
                    current_line.push((format!("`{}`", code), style));
                }
                Event::SoftBreak => {
                    if style_state.in_blockquote {
                        // In blockquotes, soft breaks should create new lines
                        if !current_line.is_empty() {
                            lines.push(line_to_element(&current_line));
                            current_line.clear();
                        }
                    } else {
                        current_line.push((" ".to_string(), Style::new()));
                    }
                }
                Event::HardBreak => {
                    if !current_line.is_empty() {
                        lines.push(line_to_element(&current_line));
                        current_line.clear();
                    }
                }
                Event::Rule => {
                    // Horizontal rule
                    if !current_line.is_empty() {
                        lines.push(line_to_element(&current_line));
                        current_line.clear();
                    }
                    let hr_style = Style::new().fg(Color::DarkGray);
                    lines.push(Element::styled_text("────────────────────────────────", hr_style));
                }
                _ => {}
            }
        }

        // Flush any remaining content
        if !current_line.is_empty() {
            lines.push(line_to_element(&current_line));
        }

        if lines.is_empty() {
            Element::text("")
        } else {
            // Always return Fragment - the renderer handles vertical layout
            // Even single lines must be wrapped so the renderer knows this is Markdown
            Element::Fragment(lines)
        }
    }
}

/// Convert a line of styled segments to a single Element.
fn line_to_element(segments: &[(String, Style)]) -> Element {
    if segments.is_empty() {
        return Element::text("");
    }
    if segments.len() == 1 {
        return Element::styled_text(&segments[0].0, segments[0].1);
    }
    // Multiple segments - create a Fragment for horizontal layout
    let children: Vec<Element> = segments
        .iter()
        .map(|(text, style)| Element::styled_text(text, *style))
        .collect();
    Element::Fragment(children)
}

/// Helper function to create a markdown block.
pub fn markdown_block(content: impl Into<String>) -> MarkdownProps {
    MarkdownProps::new(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_props_new() {
        let props = MarkdownProps::new("# Hello");
        assert_eq!(props.content, "# Hello");
        assert_eq!(props.header_color, Color::Cyan);
    }

    #[test]
    fn test_markdown_props_builder() {
        let props = MarkdownProps::new("test")
            .header_color(Color::Green)
            .code_color(Color::Magenta)
            .link_color(Color::Red)
            .quote_color(Color::Gray)
            .enable_hyperlinks(false);

        assert_eq!(props.header_color, Color::Green);
        assert_eq!(props.code_color, Color::Magenta);
        assert_eq!(props.link_color, Color::Red);
        assert_eq!(props.quote_color, Color::Gray);
        assert!(!props.enable_hyperlinks);
    }

    #[test]
    fn test_markdown_render_empty() {
        let props = MarkdownProps::new("");
        let elem = Markdown::render(&props);
        assert!(elem.is_text());
    }

    #[test]
    fn test_markdown_render_plain_text() {
        let props = MarkdownProps::new("Hello world");
        let elem = Markdown::render(&props);
        // Plain text becomes a single text element
        match elem {
            Element::Text { content, .. } => {
                assert!(content.contains("Hello world"));
            }
            Element::Fragment(children) => {
                // Could also be a fragment with text segments
                assert!(!children.is_empty());
            }
            _ => panic!("Expected Text or Fragment element"),
        }
    }

    #[test]
    fn test_markdown_render_bold() {
        let props = MarkdownProps::new("**bold**");
        let elem = Markdown::render(&props);
        match elem {
            Element::Text { style, content } => {
                assert!(style.modifiers.contains(Modifier::BOLD));
                assert!(content.contains("bold"));
            }
            Element::Fragment(children) => {
                // Check at least one child has bold
                let has_bold = children.iter().any(|c| {
                    if let Element::Text { style, .. } = c {
                        style.modifiers.contains(Modifier::BOLD)
                    } else {
                        false
                    }
                });
                assert!(has_bold);
            }
            _ => panic!("Expected styled element"),
        }
    }

    #[test]
    fn test_markdown_render_italic() {
        let props = MarkdownProps::new("*italic*");
        let elem = Markdown::render(&props);
        match elem {
            Element::Text { style, content } => {
                assert!(style.modifiers.contains(Modifier::ITALIC));
                assert!(content.contains("italic"));
            }
            Element::Fragment(children) => {
                let has_italic = children.iter().any(|c| {
                    if let Element::Text { style, .. } = c {
                        style.modifiers.contains(Modifier::ITALIC)
                    } else {
                        false
                    }
                });
                assert!(has_italic);
            }
            _ => panic!("Expected styled element"),
        }
    }

    #[test]
    fn test_markdown_render_header() {
        let props = MarkdownProps::new("# Header");
        let elem = Markdown::render(&props);
        match elem {
            Element::Text { style, content } => {
                assert!(style.modifiers.contains(Modifier::BOLD));
                assert_eq!(style.fg, Color::Cyan);
                assert!(content.contains("Header"));
            }
            Element::Fragment(children) => {
                let has_header_style = children.iter().any(|c| {
                    if let Element::Text { style, .. } = c {
                        style.fg == Color::Cyan && style.modifiers.contains(Modifier::BOLD)
                    } else {
                        false
                    }
                });
                assert!(has_header_style);
            }
            _ => panic!("Expected styled header element"),
        }
    }

    #[test]
    fn test_markdown_render_code_span() {
        let props = MarkdownProps::new("Use `code` here");
        let elem = Markdown::render(&props);
        // Result is Fragment of lines, where each line may be Text or Fragment
        fn find_code_in_element(elem: &Element) -> bool {
            match elem {
                Element::Text { style, content } => {
                    style.fg == Color::Yellow && content.contains("code")
                }
                Element::Fragment(children) => {
                    children.iter().any(|c| find_code_in_element(c))
                }
                _ => false,
            }
        }
        assert!(find_code_in_element(&elem));
    }

    #[test]
    fn test_markdown_render_list() {
        let props = MarkdownProps::new("- item1\n- item2");
        let elem = Markdown::render(&props);
        match elem {
            Element::Fragment(children) => {
                assert!(children.len() >= 2);
            }
            _ => panic!("Expected Fragment for list"),
        }
    }

    #[test]
    fn test_markdown_render_multiline() {
        let props = MarkdownProps::new("# Header\n\nParagraph");
        let elem = Markdown::render(&props);
        assert!(elem.is_fragment());
    }

    #[test]
    fn test_markdown_block_helper() {
        let props = markdown_block("test content");
        assert_eq!(props.content, "test content");
    }

    #[test]
    fn test_style_state_default() {
        let state = StyleState::default();
        assert!(!state.bold);
        assert!(!state.italic);
        assert!(!state.code);
    }

    #[test]
    fn test_style_state_to_style() {
        let mut state = StyleState::default();
        let props = MarkdownProps::default();

        // Bold
        state.bold = true;
        let style = state.to_style(&props);
        assert!(style.modifiers.contains(Modifier::BOLD));

        // Code
        state.bold = false;
        state.code = true;
        let style = state.to_style(&props);
        assert_eq!(style.fg, Color::Yellow);
    }
}
