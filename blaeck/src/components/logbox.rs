//! LogBox component - auto-expanding scrolling log viewer.
//!
//! The LogBox component displays a list of log lines that:
//! - Starts at 0 height when empty
//! - Expands as lines are added, up to max_lines
//! - Scrolls to show most recent lines when exceeding max
//!
//! ## When to use LogBox
//!
//! - Command output or build logs
//! - Event streams or activity feeds
//! - Any growing list where you want to show recent items
//!
//! ## See also
//!
//! - [`Text`](super::Text) — Static text (not a scrolling list)
//! - [`Table`](super::Table) — Structured data in columns
//! - Optionally shows "+N more" indicator for hidden lines

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// A single line in the log box.
#[derive(Debug, Clone)]
pub struct LogLine {
    /// The text content.
    pub content: String,
    /// Optional style for this line.
    pub style: Style,
    /// Optional prefix (e.g., "└", "├", "│").
    pub prefix: Option<String>,
}

impl LogLine {
    /// Create a new log line with content.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: Style::default(),
            prefix: None,
        }
    }

    /// Create a log line with a style.
    pub fn styled(content: impl Into<String>, style: Style) -> Self {
        Self {
            content: content.into(),
            style,
            prefix: None,
        }
    }

    /// Set the style.
    #[must_use]
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.style = self.style.fg(color);
        self
    }

    /// Make the line dim.
    #[must_use]
    pub fn dim(mut self) -> Self {
        self.style = self.style.add_modifier(Modifier::DIM);
        self
    }

    /// Make the line bold.
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.style = self.style.add_modifier(Modifier::BOLD);
        self
    }

    /// Set a prefix (tree connector like └, ├, │).
    #[must_use]
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Create an error line (red).
    pub fn error(content: impl Into<String>) -> Self {
        Self::new(content).color(Color::Red)
    }

    /// Create a success line (green).
    pub fn success(content: impl Into<String>) -> Self {
        Self::new(content).color(Color::Green)
    }

    /// Create a warning line (yellow).
    pub fn warning(content: impl Into<String>) -> Self {
        Self::new(content).color(Color::Yellow)
    }

    /// Create a dimmed/muted line.
    pub fn muted(content: impl Into<String>) -> Self {
        Self::new(content).dim()
    }
}

impl<S: Into<String>> From<S> for LogLine {
    fn from(s: S) -> Self {
        LogLine::new(s)
    }
}

/// Properties for the LogBox component.
#[derive(Debug, Clone)]
pub struct LogBoxProps {
    /// The log lines to display.
    pub lines: Vec<LogLine>,
    /// Maximum visible lines before scrolling (default: 5).
    pub max_lines: usize,
    /// Whether to show "+N more" when lines are hidden (default: true).
    pub show_overflow_count: bool,
    /// Color for the overflow count text.
    pub overflow_color: Option<Color>,
    /// Whether to show from bottom (most recent) or top.
    pub show_from_bottom: bool,
    /// Optional indent for all lines.
    pub indent: usize,
    /// Tree connector style for indented content.
    pub tree_style: TreeStyle,
}

/// Style for tree connectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TreeStyle {
    /// No tree connectors.
    #[default]
    None,
    /// Unicode box drawing: └ ├ │
    Unicode,
    /// ASCII: L | |
    Ascii,
}

impl TreeStyle {
    /// Get the characters for (last_item, middle_item, continuation).
    pub fn chars(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            TreeStyle::None => ("", "", ""),
            TreeStyle::Unicode => ("└ ", "├ ", "│ "),
            TreeStyle::Ascii => ("L ", "| ", "| "),
        }
    }
}

impl Default for LogBoxProps {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            max_lines: 5,
            show_overflow_count: true,
            overflow_color: Some(Color::DarkGray),
            show_from_bottom: true,
            indent: 0,
            tree_style: TreeStyle::None,
        }
    }
}

impl LogBoxProps {
    /// Create new LogBoxProps.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create LogBoxProps with lines.
    pub fn with_lines<I, T>(lines: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<LogLine>,
    {
        Self {
            lines: lines.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    /// Add a line.
    #[must_use]
    pub fn line(mut self, line: impl Into<LogLine>) -> Self {
        self.lines.push(line.into());
        self
    }

    /// Add multiple lines.
    #[must_use]
    pub fn lines<I, T>(mut self, lines: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<LogLine>,
    {
        self.lines.extend(lines.into_iter().map(Into::into));
        self
    }

    /// Set maximum visible lines.
    #[must_use]
    pub fn max_lines(mut self, max: usize) -> Self {
        self.max_lines = max.max(1);
        self
    }

    /// Show or hide the overflow count.
    #[must_use]
    pub fn show_overflow_count(mut self, show: bool) -> Self {
        self.show_overflow_count = show;
        self
    }

    /// Set the overflow count color.
    #[must_use]
    pub fn overflow_color(mut self, color: Color) -> Self {
        self.overflow_color = Some(color);
        self
    }

    /// Show lines from top instead of bottom.
    #[must_use]
    pub fn show_from_top(mut self) -> Self {
        self.show_from_bottom = false;
        self
    }

    /// Set indent for all lines.
    #[must_use]
    pub fn indent(mut self, spaces: usize) -> Self {
        self.indent = spaces;
        self
    }

    /// Set tree connector style.
    #[must_use]
    pub fn tree_style(mut self, style: TreeStyle) -> Self {
        self.tree_style = style;
        self
    }

    /// Get visible lines and overflow count.
    fn visible_lines(&self) -> (Vec<&LogLine>, usize) {
        let total = self.lines.len();
        if total <= self.max_lines {
            (self.lines.iter().collect(), 0)
        } else if self.show_from_bottom {
            // Show most recent (last N lines)
            let skip = total - self.max_lines;
            (self.lines.iter().skip(skip).collect(), skip)
        } else {
            // Show oldest (first N lines)
            let overflow = total - self.max_lines;
            (self.lines.iter().take(self.max_lines).collect(), overflow)
        }
    }
}

/// A component that displays a scrolling log box.
///
/// # Examples
///
/// ```ignore
/// // Basic log box
/// let props = LogBoxProps::new()
///     .line("Initializing...")
///     .line("Loading config")
///     .line(LogLine::success("Ready"))
///     .max_lines(5);
///
/// // With tree connectors
/// let props = LogBoxProps::new()
///     .line("Task started")
///     .line(LogLine::error("Error: file not found"))
///     .line("Retrying...")
///     .tree_style(TreeStyle::Unicode)
///     .indent(2);
/// ```
pub struct LogBox;

impl Component for LogBox {
    type Props = LogBoxProps;

    fn render(props: &Self::Props) -> Element {
        if props.lines.is_empty() {
            return Element::Empty;
        }

        let (visible, overflow) = props.visible_lines();
        let indent_str = " ".repeat(props.indent);
        let (last_connector, mid_connector, _cont_connector) = props.tree_style.chars();

        let mut elements: Vec<Element> = Vec::new();

        for (i, line) in visible.iter().enumerate() {
            let is_last = i == visible.len() - 1 && overflow == 0;

            // Build prefix
            let prefix = if props.tree_style != TreeStyle::None {
                if is_last {
                    format!("{}{}", indent_str, last_connector)
                } else {
                    format!("{}{}", indent_str, mid_connector)
                }
            } else if let Some(ref custom_prefix) = line.prefix {
                format!("{}{}", indent_str, custom_prefix)
            } else {
                indent_str.clone()
            };

            let content = format!("{}{}", prefix, line.content);
            elements.push(Element::styled_text(&content, line.style));
        }

        // Add overflow indicator
        if overflow > 0 && props.show_overflow_count {
            let overflow_text = format!("{}+{} more", indent_str, overflow);
            let mut overflow_style = Style::new();
            if let Some(color) = props.overflow_color {
                overflow_style = overflow_style.fg(color);
            }
            overflow_style = overflow_style.add_modifier(Modifier::DIM);
            elements.push(Element::styled_text(&overflow_text, overflow_style));
        }

        if elements.len() == 1 {
            elements.remove(0)
        } else {
            Element::Fragment(elements)
        }
    }
}

/// Helper to create a simple log box.
pub fn log_box<I, T>(lines: I) -> LogBoxProps
where
    I: IntoIterator<Item = T>,
    T: Into<LogLine>,
{
    LogBoxProps::with_lines(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_line_new() {
        let line = LogLine::new("test");
        assert_eq!(line.content, "test");
    }

    #[test]
    fn test_log_line_styled() {
        let line = LogLine::styled("test", Style::new().fg(Color::Red));
        assert_eq!(line.style.fg, Color::Red);
    }

    #[test]
    fn test_log_line_builders() {
        let line = LogLine::new("test").color(Color::Green).bold();
        assert_eq!(line.style.fg, Color::Green);
        assert!(line.style.modifiers.contains(Modifier::BOLD));
    }

    #[test]
    fn test_log_line_helpers() {
        let error = LogLine::error("Error!");
        assert_eq!(error.style.fg, Color::Red);

        let success = LogLine::success("OK");
        assert_eq!(success.style.fg, Color::Green);

        let warning = LogLine::warning("Warn");
        assert_eq!(warning.style.fg, Color::Yellow);
    }

    #[test]
    fn test_log_line_from_str() {
        let line: LogLine = "hello".into();
        assert_eq!(line.content, "hello");
    }

    #[test]
    fn test_logbox_props_new() {
        let props = LogBoxProps::new();
        assert!(props.lines.is_empty());
        assert_eq!(props.max_lines, 5);
    }

    #[test]
    fn test_logbox_props_with_lines() {
        let props = LogBoxProps::with_lines(vec!["a", "b", "c"]);
        assert_eq!(props.lines.len(), 3);
    }

    #[test]
    fn test_logbox_props_builder() {
        let props = LogBoxProps::new()
            .line("first")
            .line("second")
            .max_lines(10)
            .indent(2);

        assert_eq!(props.lines.len(), 2);
        assert_eq!(props.max_lines, 10);
        assert_eq!(props.indent, 2);
    }

    #[test]
    fn test_logbox_visible_lines_under_max() {
        let props = LogBoxProps::with_lines(vec!["a", "b"]).max_lines(5);
        let (visible, overflow) = props.visible_lines();
        assert_eq!(visible.len(), 2);
        assert_eq!(overflow, 0);
    }

    #[test]
    fn test_logbox_visible_lines_over_max() {
        let props = LogBoxProps::with_lines(vec!["a", "b", "c", "d", "e", "f", "g"]).max_lines(3);
        let (visible, overflow) = props.visible_lines();
        assert_eq!(visible.len(), 3);
        assert_eq!(overflow, 4);
        // Should show last 3 (most recent)
        assert_eq!(visible[0].content, "e");
        assert_eq!(visible[1].content, "f");
        assert_eq!(visible[2].content, "g");
    }

    #[test]
    fn test_logbox_visible_lines_from_top() {
        let props = LogBoxProps::with_lines(vec!["a", "b", "c", "d", "e"])
            .max_lines(3)
            .show_from_top();
        let (visible, overflow) = props.visible_lines();
        assert_eq!(visible.len(), 3);
        assert_eq!(overflow, 2);
        // Should show first 3 (oldest)
        assert_eq!(visible[0].content, "a");
        assert_eq!(visible[1].content, "b");
        assert_eq!(visible[2].content, "c");
    }

    #[test]
    fn test_logbox_render_empty() {
        let props = LogBoxProps::new();
        let elem = LogBox::render(&props);
        assert!(elem.is_empty());
    }

    #[test]
    fn test_logbox_render_single() {
        let props = LogBoxProps::new().line("test");
        let elem = LogBox::render(&props);
        assert!(elem.is_text());
    }

    #[test]
    fn test_logbox_render_multiple() {
        let props = LogBoxProps::with_lines(vec!["a", "b", "c"]);
        let elem = LogBox::render(&props);
        assert!(elem.is_fragment());
    }

    #[test]
    fn test_logbox_render_with_overflow() {
        let props = LogBoxProps::with_lines(vec!["a", "b", "c", "d", "e"])
            .max_lines(2)
            .show_overflow_count(true);
        let elem = LogBox::render(&props);
        // Should have 2 visible lines + 1 overflow indicator
        if let Element::Fragment(children) = elem {
            assert_eq!(children.len(), 3);
        } else {
            panic!("Expected Fragment");
        }
    }

    #[test]
    fn test_tree_style_chars() {
        let (last, mid, cont) = TreeStyle::Unicode.chars();
        assert_eq!(last, "└ ");
        assert_eq!(mid, "├ ");
        assert_eq!(cont, "│ ");
    }

    #[test]
    fn test_log_box_helper() {
        let props = log_box(vec!["line 1", "line 2"]);
        assert_eq!(props.lines.len(), 2);
    }
}
