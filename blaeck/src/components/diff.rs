//! Diff component - Git-style diff display.
//!
//! The Diff component displays file changes in unified diff format,
//! with support for line numbers, colors, and chunk headers.
//!
//! ## When to use Diff
//!
//! - Showing code changes before committing
//! - Comparing file versions
//! - Displaying patch content
//!
//! ## See also
//!
//! - [`SyntaxHighlight`](super::SyntaxHighlight) — Code without diff markers
//! - [`Markdown`](super::Markdown) — Formatted text display

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// Type of a diff line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLineType {
    /// Added line (+)
    Added,
    /// Removed line (-)
    Removed,
    /// Context/unchanged line
    Context,
    /// Chunk header (@@ ... @@)
    Header,
}

/// A single line in a diff.
#[derive(Debug, Clone)]
pub struct DiffLine {
    /// The line content.
    pub content: String,
    /// Type of change.
    pub line_type: DiffLineType,
    /// Old line number (for removed/context lines).
    pub old_line: Option<usize>,
    /// New line number (for added/context lines).
    pub new_line: Option<usize>,
}

impl DiffLine {
    /// Create a new diff line.
    pub fn new(content: impl Into<String>, line_type: DiffLineType) -> Self {
        Self {
            content: content.into(),
            line_type,
            old_line: None,
            new_line: None,
        }
    }

    /// Create an added line.
    pub fn added(content: impl Into<String>) -> Self {
        Self::new(content, DiffLineType::Added)
    }

    /// Create a removed line.
    pub fn removed(content: impl Into<String>) -> Self {
        Self::new(content, DiffLineType::Removed)
    }

    /// Create a context line.
    pub fn context(content: impl Into<String>) -> Self {
        Self::new(content, DiffLineType::Context)
    }

    /// Create a chunk header.
    pub fn header(content: impl Into<String>) -> Self {
        Self::new(content, DiffLineType::Header)
    }

    /// Set the old line number.
    #[must_use]
    pub fn old_num(mut self, num: usize) -> Self {
        self.old_line = Some(num);
        self
    }

    /// Set the new line number.
    #[must_use]
    pub fn new_num(mut self, num: usize) -> Self {
        self.new_line = Some(num);
        self
    }

    /// Set both line numbers.
    #[must_use]
    pub fn line_nums(mut self, old: usize, new: usize) -> Self {
        self.old_line = Some(old);
        self.new_line = Some(new);
        self
    }

    /// Get the prefix character for this line type.
    pub fn prefix(&self) -> &str {
        match self.line_type {
            DiffLineType::Added => "+",
            DiffLineType::Removed => "-",
            DiffLineType::Context => " ",
            DiffLineType::Header => "",
        }
    }
}

/// Display style for diffs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiffStyle {
    /// Unified diff format (default)
    #[default]
    Unified,
    /// Minimal - just +/- with colors, no line numbers
    Minimal,
    /// With line numbers
    LineNumbers,
}

/// Properties for the Diff component.
#[derive(Debug, Clone)]
pub struct DiffProps {
    /// The diff lines to display.
    pub lines: Vec<DiffLine>,
    /// Display style.
    pub style: DiffStyle,
    /// Color for added lines.
    pub added_color: Color,
    /// Color for removed lines.
    pub removed_color: Color,
    /// Color for context lines.
    pub context_color: Color,
    /// Color for chunk headers.
    pub header_color: Color,
    /// Color for line numbers.
    pub line_num_color: Color,
    /// Whether to show +/- prefixes.
    pub show_prefix: bool,
    /// Whether to dim context lines.
    pub dim_context: bool,
    /// Width for line number column (0 to auto-calculate).
    pub line_num_width: usize,
}

impl Default for DiffProps {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            style: DiffStyle::Unified,
            added_color: Color::Green,
            removed_color: Color::Red,
            context_color: Color::Reset,
            header_color: Color::Cyan,
            line_num_color: Color::DarkGray,
            show_prefix: true,
            dim_context: true,
            line_num_width: 0,
        }
    }
}

impl DiffProps {
    /// Create new DiffProps.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create from a list of diff lines.
    pub fn with_lines(lines: Vec<DiffLine>) -> Self {
        Self {
            lines,
            ..Default::default()
        }
    }

    /// Parse a unified diff string.
    ///
    /// Recognizes lines starting with:
    /// - `+` as added
    /// - `-` as removed
    /// - `@@` as chunk headers
    /// - Everything else as context
    pub fn from_unified(diff_text: &str) -> Self {
        let mut lines = Vec::new();
        let mut old_line = 1usize;
        let mut new_line = 1usize;

        for line in diff_text.lines() {
            if line.starts_with("@@") {
                // Parse chunk header: @@ -start,count +start,count @@
                lines.push(DiffLine::header(line));
                // Try to parse line numbers from header
                if let Some((old_start, new_start)) = parse_chunk_header(line) {
                    old_line = old_start;
                    new_line = new_start;
                }
            } else if line.starts_with('+') && !line.starts_with("+++") {
                let content = if line.len() > 1 { &line[1..] } else { "" };
                lines.push(DiffLine::added(content).new_num(new_line));
                new_line += 1;
            } else if line.starts_with('-') && !line.starts_with("---") {
                let content = if line.len() > 1 { &line[1..] } else { "" };
                lines.push(DiffLine::removed(content).old_num(old_line));
                old_line += 1;
            } else if line.starts_with("---") || line.starts_with("+++") {
                // File headers - treat as headers
                lines.push(DiffLine::header(line));
            } else {
                // Context line (may start with space)
                let content = if line.starts_with(' ') && line.len() > 1 {
                    &line[1..]
                } else {
                    line
                };
                lines.push(DiffLine::context(content).line_nums(old_line, new_line));
                old_line += 1;
                new_line += 1;
            }
        }

        Self::with_lines(lines)
    }

    /// Add an added line.
    #[must_use]
    pub fn added(mut self, content: impl Into<String>) -> Self {
        self.lines.push(DiffLine::added(content));
        self
    }

    /// Add a removed line.
    #[must_use]
    pub fn removed(mut self, content: impl Into<String>) -> Self {
        self.lines.push(DiffLine::removed(content));
        self
    }

    /// Add a context line.
    #[must_use]
    pub fn context(mut self, content: impl Into<String>) -> Self {
        self.lines.push(DiffLine::context(content));
        self
    }

    /// Add a chunk header.
    #[must_use]
    pub fn header(mut self, content: impl Into<String>) -> Self {
        self.lines.push(DiffLine::header(content));
        self
    }

    /// Add a diff line.
    #[must_use]
    pub fn line(mut self, line: DiffLine) -> Self {
        self.lines.push(line);
        self
    }

    /// Set the display style.
    #[must_use]
    pub fn style(mut self, style: DiffStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the color for added lines.
    #[must_use]
    pub fn added_color(mut self, color: Color) -> Self {
        self.added_color = color;
        self
    }

    /// Set the color for removed lines.
    #[must_use]
    pub fn removed_color(mut self, color: Color) -> Self {
        self.removed_color = color;
        self
    }

    /// Set the color for context lines.
    #[must_use]
    pub fn context_color(mut self, color: Color) -> Self {
        self.context_color = color;
        self
    }

    /// Set the color for headers.
    #[must_use]
    pub fn header_color(mut self, color: Color) -> Self {
        self.header_color = color;
        self
    }

    /// Enable/disable +/- prefixes.
    #[must_use]
    pub fn show_prefix(mut self, show: bool) -> Self {
        self.show_prefix = show;
        self
    }

    /// Enable/disable dimming context lines.
    #[must_use]
    pub fn dim_context(mut self, dim: bool) -> Self {
        self.dim_context = dim;
        self
    }

    /// Calculate the width needed for line numbers.
    fn calc_line_num_width(&self) -> usize {
        if self.line_num_width > 0 {
            return self.line_num_width;
        }

        let max_line = self
            .lines
            .iter()
            .filter_map(|l| l.old_line.max(l.new_line))
            .max()
            .unwrap_or(0);

        if max_line == 0 {
            0
        } else {
            max_line.to_string().len()
        }
    }

    /// Render a single line to a string.
    fn render_line(&self, line: &DiffLine, line_width: usize) -> String {
        let prefix = if self.show_prefix { line.prefix() } else { "" };

        match self.style {
            DiffStyle::Minimal => {
                format!("{}{}", prefix, line.content)
            }
            DiffStyle::Unified => {
                format!("{}{}", prefix, line.content)
            }
            DiffStyle::LineNumbers => {
                let old_num = line
                    .old_line
                    .map(|n| format!("{:>width$}", n, width = line_width))
                    .unwrap_or_else(|| " ".repeat(line_width));
                let new_num = line
                    .new_line
                    .map(|n| format!("{:>width$}", n, width = line_width))
                    .unwrap_or_else(|| " ".repeat(line_width));

                if line.line_type == DiffLineType::Header {
                    line.content.clone()
                } else {
                    format!("{} {} {}{}", old_num, new_num, prefix, line.content)
                }
            }
        }
    }

    /// Render the diff as lines (for plain text output).
    pub fn render_lines(&self) -> Vec<String> {
        let line_width = self.calc_line_num_width();
        self.lines
            .iter()
            .map(|line| self.render_line(line, line_width))
            .collect()
    }

    /// Render the diff as a single string.
    pub fn render_string(&self) -> String {
        self.render_lines().join("\n")
    }
}

/// Parse chunk header to extract line numbers.
/// Format: @@ -old_start,old_count +new_start,new_count @@
fn parse_chunk_header(header: &str) -> Option<(usize, usize)> {
    // Simple parsing - look for -N and +N patterns
    let mut old_start = None;
    let mut new_start = None;

    for part in header.split_whitespace() {
        if part.starts_with('-') && part.len() > 1 {
            if let Some(num_str) = part[1..].split(',').next() {
                old_start = num_str.parse().ok();
            }
        } else if part.starts_with('+') && part.len() > 1 {
            if let Some(num_str) = part[1..].split(',').next() {
                new_start = num_str.parse().ok();
            }
        }
    }

    match (old_start, new_start) {
        (Some(o), Some(n)) => Some((o, n)),
        _ => None,
    }
}

/// A component that displays a diff.
///
/// # Examples
///
/// ```ignore
/// // Simple diff
/// Element::node::<Diff>(
///     DiffProps::new()
///         .removed("old line")
///         .added("new line")
///         .context("unchanged"),
///     vec![]
/// )
///
/// // From unified diff string
/// Element::node::<Diff>(
///     DiffProps::from_unified("+added\n-removed\n context"),
///     vec![]
/// )
/// ```
pub struct Diff;

impl Component for Diff {
    type Props = DiffProps;

    fn render(props: &Self::Props) -> Element {
        if props.lines.is_empty() {
            return Element::text("");
        }

        // Build a Fragment with each line as a styled Text element
        let line_width = props.calc_line_num_width();
        let mut elements = Vec::new();

        for line in props.lines.iter() {
            let rendered = props.render_line(line, line_width);

            let mut style = Style::new();
            match line.line_type {
                DiffLineType::Added => {
                    style = style.fg(props.added_color);
                }
                DiffLineType::Removed => {
                    style = style.fg(props.removed_color);
                }
                DiffLineType::Context => {
                    style = style.fg(props.context_color);
                    if props.dim_context {
                        style = style.add_modifier(Modifier::DIM);
                    }
                }
                DiffLineType::Header => {
                    style = style.fg(props.header_color).add_modifier(Modifier::BOLD);
                }
            }

            elements.push(Element::Text {
                content: rendered,
                style,
            });
        }

        Element::Fragment(elements)
    }
}

/// Helper to create a simple diff from old/new content.
pub fn diff_lines(old: &[&str], new: &[&str]) -> DiffProps {
    let mut props = DiffProps::new();

    // Simple diff: show all old as removed, all new as added
    // (A real diff algorithm would be more sophisticated)
    for line in old {
        props = props.removed(*line);
    }
    for line in new {
        props = props.added(*line);
    }

    props
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_line_new() {
        let line = DiffLine::new("test", DiffLineType::Added);
        assert_eq!(line.content, "test");
        assert_eq!(line.line_type, DiffLineType::Added);
    }

    #[test]
    fn test_diff_line_helpers() {
        let added = DiffLine::added("new");
        assert_eq!(added.line_type, DiffLineType::Added);
        assert_eq!(added.prefix(), "+");

        let removed = DiffLine::removed("old");
        assert_eq!(removed.line_type, DiffLineType::Removed);
        assert_eq!(removed.prefix(), "-");

        let context = DiffLine::context("same");
        assert_eq!(context.line_type, DiffLineType::Context);
        assert_eq!(context.prefix(), " ");
    }

    #[test]
    fn test_diff_line_with_nums() {
        let line = DiffLine::added("new").new_num(42);
        assert_eq!(line.new_line, Some(42));
        assert!(line.old_line.is_none());

        let line2 = DiffLine::context("same").line_nums(10, 12);
        assert_eq!(line2.old_line, Some(10));
        assert_eq!(line2.new_line, Some(12));
    }

    #[test]
    fn test_diff_props_new() {
        let props = DiffProps::new();
        assert!(props.lines.is_empty());
        assert_eq!(props.added_color, Color::Green);
        assert_eq!(props.removed_color, Color::Red);
    }

    #[test]
    fn test_diff_props_builder() {
        let props = DiffProps::new().removed("old").added("new").context("same");

        assert_eq!(props.lines.len(), 3);
        assert_eq!(props.lines[0].line_type, DiffLineType::Removed);
        assert_eq!(props.lines[1].line_type, DiffLineType::Added);
        assert_eq!(props.lines[2].line_type, DiffLineType::Context);
    }

    #[test]
    fn test_diff_props_from_unified() {
        let diff_text = "+added line\n-removed line\n context line";
        let props = DiffProps::from_unified(diff_text);

        assert_eq!(props.lines.len(), 3);
        assert_eq!(props.lines[0].line_type, DiffLineType::Added);
        assert_eq!(props.lines[0].content, "added line");
        assert_eq!(props.lines[1].line_type, DiffLineType::Removed);
        assert_eq!(props.lines[1].content, "removed line");
        assert_eq!(props.lines[2].line_type, DiffLineType::Context);
    }

    #[test]
    fn test_diff_props_from_unified_with_header() {
        let diff_text = "@@ -1,3 +1,4 @@\n context\n-removed\n+added";
        let props = DiffProps::from_unified(diff_text);

        assert_eq!(props.lines.len(), 4);
        assert_eq!(props.lines[0].line_type, DiffLineType::Header);
    }

    #[test]
    fn test_parse_chunk_header() {
        let result = parse_chunk_header("@@ -10,5 +12,7 @@");
        assert_eq!(result, Some((10, 12)));

        let result2 = parse_chunk_header("@@ -1 +1 @@");
        assert_eq!(result2, Some((1, 1)));
    }

    #[test]
    fn test_diff_render_string() {
        let props = DiffProps::new()
            .removed("old")
            .added("new")
            .style(DiffStyle::Minimal);

        let result = props.render_string();
        assert!(result.contains("-old"));
        assert!(result.contains("+new"));
    }

    #[test]
    fn test_diff_render_no_prefix() {
        let props = DiffProps::new()
            .removed("old")
            .added("new")
            .show_prefix(false);

        let result = props.render_string();
        assert!(!result.contains("-old"));
        assert!(!result.contains("+new"));
        assert!(result.contains("old"));
        assert!(result.contains("new"));
    }

    #[test]
    fn test_diff_component_render() {
        let props = DiffProps::new().added("test");
        let elem = Diff::render(&props);
        assert!(elem.is_fragment());
    }

    #[test]
    fn test_diff_component_render_empty() {
        let props = DiffProps::new();
        let elem = Diff::render(&props);
        assert!(elem.is_text());
    }

    #[test]
    fn test_diff_lines_helper() {
        let props = diff_lines(&["old1", "old2"], &["new1"]);
        assert_eq!(props.lines.len(), 3);
        assert_eq!(props.lines[0].line_type, DiffLineType::Removed);
        assert_eq!(props.lines[1].line_type, DiffLineType::Removed);
        assert_eq!(props.lines[2].line_type, DiffLineType::Added);
    }

    #[test]
    fn test_diff_line_numbers_style() {
        let props = DiffProps::new()
            .line(DiffLine::removed("old").old_num(10))
            .line(DiffLine::added("new").new_num(10))
            .style(DiffStyle::LineNumbers);

        let result = props.render_string();
        assert!(result.contains("10"));
    }

    #[test]
    fn test_diff_component_render_has_styles() {
        let props = DiffProps::new().removed("old line").added("new line");
        let elem = Diff::render(&props);

        if let crate::Element::Fragment(children) = elem {
            assert_eq!(children.len(), 2);
            // First child (removed) should have red color
            if let crate::Element::Text { style, .. } = &children[0] {
                assert_eq!(style.fg, Color::Red);
            } else {
                panic!("Expected Text element");
            }
            // Second child (added) should have green color
            if let crate::Element::Text { style, .. } = &children[1] {
                assert_eq!(style.fg, Color::Green);
            } else {
                panic!("Expected Text element");
            }
        } else {
            panic!("Expected Fragment element");
        }
    }
}
