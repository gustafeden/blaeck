//! Syntax Highlighting component for code display.
//!
//! Uses `syntect` for syntax highlighting with 7 built-in themes
//! (Monokai, Dracula, Solarized, etc.) and optional line numbers.
//!
//! ## When to use SyntaxHighlight
//!
//! - Displaying source code with highlighting
//! - Code snippets in documentation
//! - Diff views showing code context
//!
//! ## See also
//!
//! - [`Diff`](super::Diff) — Code changes with +/- markers
//! - [`Markdown`](super::Markdown) — Fenced code blocks (simpler)
//!
//! # Example
//!
//! ```ignore
//! use blaeck::prelude::*;
//!
//! let code = r#"fn main() {
//!     println!("Hello, world!");
//! }"#;
//!
//! Element::node::<SyntaxHighlight>(
//!     SyntaxHighlightProps::new(code)
//!         .language("rust")
//!         .theme(SyntaxTheme::Monokai)
//!         .line_numbers(true),
//!     vec![],
//! )
//! ```

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};
use syntect::easy::HighlightLines;
use syntect::highlighting::{self, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

/// Built-in syntax themes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SyntaxTheme {
    /// Monokai theme (dark, colorful) - may have low contrast on some terminals.
    Monokai,
    /// Base16 Ocean Dark - good contrast on dark terminals.
    #[default]
    /// Base16 Ocean Dark.
    OceanDark,
    /// Base16 Ocean Light.
    OceanLight,
    /// Base16 Eighties Dark.
    EightiesDark,
    /// Solarized Dark.
    SolarizedDark,
    /// Solarized Light.
    SolarizedLight,
    /// InspiredGitHub (light theme).
    InspiredGitHub,
}

impl SyntaxTheme {
    /// Get the syntect theme name.
    fn theme_name(&self) -> &'static str {
        match self {
            SyntaxTheme::Monokai => "base16-monokai.dark",
            SyntaxTheme::OceanDark => "base16-ocean.dark",
            SyntaxTheme::OceanLight => "base16-ocean.light",
            SyntaxTheme::EightiesDark => "base16-eighties.dark",
            SyntaxTheme::SolarizedDark => "Solarized (dark)",
            SyntaxTheme::SolarizedLight => "Solarized (light)",
            SyntaxTheme::InspiredGitHub => "InspiredGitHub",
        }
    }

    /// Get the theme, falling back to a default if not found.
    fn get_theme<'a>(&self, ts: &'a ThemeSet) -> &'a highlighting::Theme {
        ts.themes
            .get(self.theme_name())
            .or_else(|| ts.themes.values().next())
            .expect("No themes available")
    }
}

/// Line number style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineNumberStyle {
    /// No line numbers.
    #[default]
    None,
    /// Simple numbers: "1 ", "2 ", etc.
    Simple,
    /// Padded numbers: " 1 ", " 2 ", etc.
    Padded,
    /// With separator: " 1 │ ", " 2 │ "
    WithSeparator,
}

/// Properties for the SyntaxHighlight component.
#[derive(Debug, Clone)]
pub struct SyntaxHighlightProps {
    /// Source code to highlight.
    pub code: String,
    /// Language for syntax highlighting.
    pub language: Option<String>,
    /// Theme to use.
    pub theme: SyntaxTheme,
    /// Line number style.
    pub line_numbers: LineNumberStyle,
    /// Starting line number.
    pub start_line: usize,
    /// Line number color.
    pub line_number_color: Option<Color>,
    /// Whether to trim trailing whitespace.
    pub trim_trailing: bool,
    /// Maximum width (for wrapping, 0 = no limit).
    pub max_width: usize,
}

impl Default for SyntaxHighlightProps {
    fn default() -> Self {
        Self {
            code: String::new(),
            language: None,
            theme: SyntaxTheme::Monokai,
            line_numbers: LineNumberStyle::None,
            start_line: 1,
            line_number_color: Some(Color::DarkGray),
            trim_trailing: true,
            max_width: 0,
        }
    }
}

impl SyntaxHighlightProps {
    /// Create new props with code.
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            ..Default::default()
        }
    }

    /// Set the language.
    #[must_use]
    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.language = Some(lang.into());
        self
    }

    /// Set the theme.
    #[must_use]
    pub fn theme(mut self, theme: SyntaxTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Enable line numbers.
    #[must_use]
    pub fn line_numbers(mut self, style: LineNumberStyle) -> Self {
        self.line_numbers = style;
        self
    }

    /// Enable simple line numbers.
    #[must_use]
    pub fn with_line_numbers(mut self) -> Self {
        self.line_numbers = LineNumberStyle::WithSeparator;
        self
    }

    /// Set starting line number.
    #[must_use]
    pub fn start_line(mut self, line: usize) -> Self {
        self.start_line = line.max(1);
        self
    }

    /// Set line number color.
    #[must_use]
    pub fn line_number_color(mut self, color: Color) -> Self {
        self.line_number_color = Some(color);
        self
    }

    /// Set maximum width.
    #[must_use]
    pub fn max_width(mut self, width: usize) -> Self {
        self.max_width = width;
        self
    }
}

/// Convert syntect color to blaeck color.
fn syntect_to_blaeck_color(c: highlighting::Color) -> Color {
    Color::Rgb(c.r, c.g, c.b)
}

/// Convert syntect style to blaeck style.
fn syntect_to_blaeck_style(style: highlighting::Style) -> Style {
    let mut s = Style::new().fg(syntect_to_blaeck_color(style.foreground));

    if style.font_style.contains(highlighting::FontStyle::BOLD) {
        s = s.add_modifier(Modifier::BOLD);
    }
    if style.font_style.contains(highlighting::FontStyle::ITALIC) {
        s = s.add_modifier(Modifier::ITALIC);
    }
    if style
        .font_style
        .contains(highlighting::FontStyle::UNDERLINE)
    {
        s = s.add_modifier(Modifier::UNDERLINED);
    }

    s
}

/// A component that displays syntax-highlighted code.
pub struct SyntaxHighlight;

impl Component for SyntaxHighlight {
    type Props = SyntaxHighlightProps;

    fn render(props: &Self::Props) -> Element {
        if props.code.is_empty() {
            return Element::Empty;
        }

        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();

        // Find syntax for language
        let syntax = if let Some(ref lang) = props.language {
            ps.find_syntax_by_token(lang)
                .or_else(|| ps.find_syntax_by_extension(lang))
        } else {
            // Try to detect from first line
            ps.find_syntax_by_first_line(&props.code)
        }
        .unwrap_or_else(|| ps.find_syntax_plain_text());

        // Get theme
        let theme = props.theme.get_theme(&ts);

        let mut highlighter = HighlightLines::new(syntax, theme);
        let mut lines: Vec<Element> = Vec::new();

        // Calculate line number width
        let total_lines = props.code.lines().count();
        let end_line = props.start_line + total_lines;
        let line_num_width = end_line.to_string().len();

        for (i, line) in LinesWithEndings::from(&props.code).enumerate() {
            let line_num = props.start_line + i;
            let mut line_segments: Vec<Element> = Vec::new();

            // Add line number if enabled
            match props.line_numbers {
                LineNumberStyle::None => {}
                LineNumberStyle::Simple => {
                    let num_str = format!("{} ", line_num);
                    let style = if let Some(color) = props.line_number_color {
                        Style::new().fg(color)
                    } else {
                        Style::new().add_modifier(Modifier::DIM)
                    };
                    line_segments.push(Element::styled_text(&num_str, style));
                }
                LineNumberStyle::Padded => {
                    let num_str = format!("{:>width$} ", line_num, width = line_num_width);
                    let style = if let Some(color) = props.line_number_color {
                        Style::new().fg(color)
                    } else {
                        Style::new().add_modifier(Modifier::DIM)
                    };
                    line_segments.push(Element::styled_text(&num_str, style));
                }
                LineNumberStyle::WithSeparator => {
                    let num_str = format!("{:>width$} │ ", line_num, width = line_num_width);
                    let style = if let Some(color) = props.line_number_color {
                        Style::new().fg(color)
                    } else {
                        Style::new().add_modifier(Modifier::DIM)
                    };
                    line_segments.push(Element::styled_text(&num_str, style));
                }
            }

            // Highlight the line
            let highlighted = highlighter.highlight_line(line, &ps);
            match highlighted {
                Ok(ranges) => {
                    let ranges: Vec<_> = ranges.into_iter().collect();
                    let range_count = ranges.len();
                    for (idx, (style, text)) in ranges.into_iter().enumerate() {
                        let mut text = text.to_string();
                        // Remove trailing newline/carriage return for display
                        if text.ends_with('\n') {
                            text.pop();
                        }
                        if text.ends_with('\r') {
                            text.pop();
                        }
                        // Only trim the last segment's trailing whitespace
                        if props.trim_trailing && idx == range_count - 1 {
                            text = text.trim_end().to_string();
                        }
                        // Keep whitespace segments - they're important for spacing
                        if !text.is_empty() {
                            let blaeck_style = syntect_to_blaeck_style(style);
                            line_segments.push(Element::styled_text(&text, blaeck_style));
                        }
                    }
                }
                Err(_) => {
                    // Fallback to plain text
                    let mut text = line.to_string();
                    if text.ends_with('\n') {
                        text.pop();
                    }
                    if props.trim_trailing {
                        text = text.trim_end().to_string();
                    }
                    line_segments.push(Element::text(&text));
                }
            }

            // Combine segments into a line
            if line_segments.is_empty() {
                lines.push(Element::text(""));
            } else if line_segments.len() == 1 {
                lines.push(line_segments.remove(0));
            } else {
                lines.push(Element::Fragment(line_segments));
            }
        }

        if lines.is_empty() {
            Element::Empty
        } else if lines.len() == 1 {
            lines.remove(0)
        } else {
            Element::Fragment(lines)
        }
    }
}

/// Helper to create syntax-highlighted code.
pub fn syntax_highlight(code: &str, language: &str) -> Element {
    SyntaxHighlight::render(&SyntaxHighlightProps::new(code).language(language))
}

/// Helper to create syntax-highlighted code with line numbers.
pub fn syntax_highlight_with_lines(code: &str, language: &str) -> Element {
    SyntaxHighlight::render(
        &SyntaxHighlightProps::new(code)
            .language(language)
            .with_line_numbers(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_theme_names() {
        assert_eq!(SyntaxTheme::Monokai.theme_name(), "base16-monokai.dark");
        assert_eq!(SyntaxTheme::OceanDark.theme_name(), "base16-ocean.dark");
    }

    #[test]
    fn test_syntax_props_new() {
        let props = SyntaxHighlightProps::new("let x = 1;");
        assert_eq!(props.code, "let x = 1;");
        assert!(props.language.is_none());
    }

    #[test]
    fn test_syntax_props_builder() {
        let props = SyntaxHighlightProps::new("code")
            .language("rust")
            .theme(SyntaxTheme::OceanDark)
            .with_line_numbers()
            .start_line(10);

        assert_eq!(props.language, Some("rust".to_string()));
        assert_eq!(props.theme, SyntaxTheme::OceanDark);
        assert_eq!(props.line_numbers, LineNumberStyle::WithSeparator);
        assert_eq!(props.start_line, 10);
    }

    #[test]
    fn test_syntax_render_empty() {
        let props = SyntaxHighlightProps::new("");
        let elem = SyntaxHighlight::render(&props);
        assert!(elem.is_empty());
    }

    #[test]
    fn test_syntax_render_rust() {
        let code = "fn main() {}";
        let props = SyntaxHighlightProps::new(code).language("rust");
        let elem = SyntaxHighlight::render(&props);
        assert!(elem.is_fragment() || elem.is_text());
    }

    #[test]
    fn test_syntax_render_with_line_numbers() {
        let code = "line 1\nline 2\nline 3";
        let props = SyntaxHighlightProps::new(code).with_line_numbers();
        let elem = SyntaxHighlight::render(&props);
        assert!(elem.is_fragment());
    }

    #[test]
    fn test_syntax_helper() {
        let elem = syntax_highlight("let x = 1;", "rust");
        assert!(elem.is_fragment() || elem.is_text());
    }

    #[test]
    fn test_syntax_helper_with_lines() {
        let elem = syntax_highlight_with_lines("let x = 1;", "rust");
        assert!(elem.is_fragment() || elem.is_text());
    }

    #[test]
    fn test_syntect_to_blaeck_color() {
        let c = highlighting::Color {
            r: 255,
            g: 128,
            b: 64,
            a: 255,
        };
        let color = syntect_to_blaeck_color(c);
        assert_eq!(color, Color::Rgb(255, 128, 64));
    }
}
