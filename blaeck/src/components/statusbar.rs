//! StatusBar component - Git-style status display.
//!
//! The StatusBar component displays multiple status segments inline,
//! commonly used for showing git status, build status, or other indicators.
//!
//! ## When to use StatusBar
//!
//! - Git branch and status display
//! - Build/test status indicators
//! - Multi-part status (e.g., "● API: OK │ ● DB: OK")
//!
//! ## See also
//!
//! - [`Badge`](super::Badge) — Single status indicator
//! - [`KeyHints`](super::KeyHints) — Keyboard shortcuts (often at bottom)

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// A single segment in the status bar.
#[derive(Debug, Clone)]
pub struct StatusSegment {
    /// The text content of this segment.
    pub text: String,
    /// Icon/symbol prefix (optional).
    pub icon: Option<String>,
    /// Color for this segment.
    pub color: Option<Color>,
    /// Background color for this segment.
    pub bg_color: Option<Color>,
    /// Whether this segment should be bold.
    pub bold: bool,
    /// Whether this segment should be dim.
    pub dim: bool,
}

impl StatusSegment {
    /// Create a new status segment.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon: None,
            color: None,
            bg_color: None,
            bold: false,
            dim: false,
        }
    }

    /// Create a segment with an icon.
    pub fn with_icon(icon: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon: Some(icon.into()),
            color: None,
            bg_color: None,
            bold: false,
            dim: false,
        }
    }

    /// Set the color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the background color.
    #[must_use]
    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Enable bold.
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Enable dim.
    #[must_use]
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Set the icon.
    #[must_use]
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Render to styled string.
    fn render_string(&self) -> String {
        match &self.icon {
            Some(icon) => format!("{} {}", icon, self.text),
            None => self.text.clone(),
        }
    }
}

impl<S: Into<String>> From<S> for StatusSegment {
    fn from(s: S) -> Self {
        StatusSegment::new(s)
    }
}

/// Separator style between segments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StatusSeparator {
    /// Space: " "
    #[default]
    Space,
    /// Double space: "  "
    DoubleSpace,
    /// Pipe with spaces: " | "
    Pipe,
    /// Bullet: " • "
    Bullet,
    /// Arrow: " > "
    Arrow,
    /// Slash: " / "
    Slash,
    /// None (no separator)
    None,
}

impl StatusSeparator {
    /// Get the separator string.
    pub fn as_str(&self) -> &str {
        match self {
            StatusSeparator::Space => " ",
            StatusSeparator::DoubleSpace => "  ",
            StatusSeparator::Pipe => " | ",
            StatusSeparator::Bullet => " • ",
            StatusSeparator::Arrow => " > ",
            StatusSeparator::Slash => " / ",
            StatusSeparator::None => "",
        }
    }
}

/// Preset icons for common status indicators.
pub mod icons {
    /// Git branch icon
    pub const BRANCH: &str = "";
    /// Git branch (ascii)
    pub const BRANCH_ASCII: &str = "*";
    /// Check mark
    pub const CHECK: &str = "✓";
    /// Cross mark
    pub const CROSS: &str = "✗";
    /// Warning
    pub const WARNING: &str = "⚠";
    /// Info
    pub const INFO: &str = "ℹ";
    /// Clock
    pub const CLOCK: &str = "⏱";
    /// User
    pub const USER: &str = "👤";
    /// Folder
    pub const FOLDER: &str = "📁";
    /// File
    pub const FILE: &str = "📄";
    /// Lock
    pub const LOCK: &str = "🔒";
    /// Unlock
    pub const UNLOCK: &str = "🔓";
    /// Arrow up
    pub const ARROW_UP: &str = "↑";
    /// Arrow down
    pub const ARROW_DOWN: &str = "↓";
    /// Sync/refresh
    pub const SYNC: &str = "⟳";
    /// Plus
    pub const PLUS: &str = "+";
    /// Minus
    pub const MINUS: &str = "-";
    /// Modified
    pub const MODIFIED: &str = "~";
}

/// Properties for the StatusBar component.
#[derive(Debug, Clone)]
pub struct StatusBarProps {
    /// The segments to display.
    pub segments: Vec<StatusSegment>,
    /// Separator between segments.
    pub separator: StatusSeparator,
    /// Color for the separator.
    pub separator_color: Option<Color>,
    /// Left bracket/prefix (optional).
    pub prefix: Option<String>,
    /// Right bracket/suffix (optional).
    pub suffix: Option<String>,
    /// Color for prefix/suffix.
    pub bracket_color: Option<Color>,
}

impl Default for StatusBarProps {
    fn default() -> Self {
        Self {
            segments: Vec::new(),
            separator: StatusSeparator::Space,
            separator_color: Some(Color::DarkGray),
            prefix: None,
            suffix: None,
            bracket_color: Some(Color::DarkGray),
        }
    }
}

impl StatusBarProps {
    /// Create new StatusBarProps with segments.
    pub fn new<I, T>(segments: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<StatusSegment>,
    {
        Self {
            segments: segments.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    /// Add a segment.
    #[must_use]
    pub fn segment(mut self, segment: impl Into<StatusSegment>) -> Self {
        self.segments.push(segment.into());
        self
    }

    /// Add a text segment with color.
    #[must_use]
    pub fn text(mut self, text: impl Into<String>, color: Color) -> Self {
        self.segments.push(StatusSegment::new(text).color(color));
        self
    }

    /// Add a segment with icon.
    #[must_use]
    pub fn with_icon(
        mut self,
        icon: impl Into<String>,
        text: impl Into<String>,
        color: Color,
    ) -> Self {
        self.segments
            .push(StatusSegment::with_icon(icon, text).color(color));
        self
    }

    /// Set the separator.
    #[must_use]
    pub fn separator(mut self, separator: StatusSeparator) -> Self {
        self.separator = separator;
        self
    }

    /// Set the separator color.
    #[must_use]
    pub fn separator_color(mut self, color: Color) -> Self {
        self.separator_color = Some(color);
        self
    }

    /// Set brackets around the status bar.
    #[must_use]
    pub fn brackets(mut self, prefix: impl Into<String>, suffix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self.suffix = Some(suffix.into());
        self
    }

    /// Set square brackets.
    #[must_use]
    pub fn square_brackets(self) -> Self {
        self.brackets("[", "]")
    }

    /// Set parentheses.
    #[must_use]
    pub fn parens(self) -> Self {
        self.brackets("(", ")")
    }

    /// Set bracket color.
    #[must_use]
    pub fn bracket_color(mut self, color: Color) -> Self {
        self.bracket_color = Some(color);
        self
    }

    /// Render the status bar as a plain string (no ANSI codes).
    pub fn render_string(&self) -> String {
        if self.segments.is_empty() {
            return String::new();
        }

        let sep = self.separator.as_str();
        let content: Vec<String> = self.segments.iter().map(|s| s.render_string()).collect();
        let joined = content.join(sep);

        match (&self.prefix, &self.suffix) {
            (Some(p), Some(s)) => format!("{}{}{}", p, joined, s),
            (Some(p), None) => format!("{}{}", p, joined),
            (None, Some(s)) => format!("{}{}", joined, s),
            (None, None) => joined,
        }
    }
}

/// A component that displays a status bar with multiple segments.
///
/// # Examples
///
/// ```ignore
/// // Git-style status
/// Element::node::<StatusBar>(
///     StatusBarProps::new([])
///         .with_icon("", "main", Color::Green)
///         .with_icon("+", "3", Color::Green)
///         .with_icon("-", "1", Color::Red)
///         .separator(StatusSeparator::Pipe),
///     vec![]
/// )
///
/// // Simple status
/// Element::node::<StatusBar>(
///     StatusBarProps::new(["ready", "3 tasks"])
///         .square_brackets(),
///     vec![]
/// )
/// ```
pub struct StatusBar;

impl Component for StatusBar {
    type Props = StatusBarProps;

    fn render(props: &Self::Props) -> Element {
        if props.segments.is_empty() {
            return Element::text("");
        }

        let sep = props.separator.as_str();
        let mut children = Vec::new();

        // Prefix
        if let Some(prefix) = &props.prefix {
            let mut style = Style::new();
            if let Some(color) = props.bracket_color {
                style = style.fg(color);
            }
            children.push(Element::styled_text(prefix, style));
        }

        // Segments
        for (i, segment) in props.segments.iter().enumerate() {
            // Separator
            if i > 0 && !sep.is_empty() {
                let mut sep_style = Style::new();
                if let Some(color) = props.separator_color {
                    sep_style = sep_style.fg(color);
                }
                children.push(Element::styled_text(sep, sep_style));
            }

            // Segment content
            let mut style = Style::new();
            if let Some(color) = segment.color {
                style = style.fg(color);
            }
            if let Some(bg) = segment.bg_color {
                style = style.bg(bg);
            }
            if segment.bold {
                style = style.add_modifier(Modifier::BOLD);
            }
            if segment.dim {
                style = style.add_modifier(Modifier::DIM);
            }

            children.push(Element::styled_text(segment.render_string(), style));
        }

        // Suffix
        if let Some(suffix) = &props.suffix {
            let mut style = Style::new();
            if let Some(color) = props.bracket_color {
                style = style.fg(color);
            }
            children.push(Element::styled_text(suffix, style));
        }

        Element::Fragment(children)
    }
}

/// Helper to create a git-style branch status.
pub fn git_branch(branch: &str, color: Color) -> StatusSegment {
    StatusSegment::with_icon(icons::BRANCH, branch).color(color)
}

/// Helper to create a status segment with a check mark.
pub fn status_ok(text: &str) -> StatusSegment {
    StatusSegment::with_icon(icons::CHECK, text).color(Color::Green)
}

/// Helper to create a status segment with a cross mark.
pub fn status_error(text: &str) -> StatusSegment {
    StatusSegment::with_icon(icons::CROSS, text).color(Color::Red)
}

/// Helper to create a status segment with a warning.
pub fn status_warning(text: &str) -> StatusSegment {
    StatusSegment::with_icon(icons::WARNING, text).color(Color::Yellow)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_new() {
        let seg = StatusSegment::new("test");
        assert_eq!(seg.text, "test");
        assert!(seg.icon.is_none());
    }

    #[test]
    fn test_segment_with_icon() {
        let seg = StatusSegment::with_icon("*", "main");
        assert_eq!(seg.text, "main");
        assert_eq!(seg.icon, Some("*".to_string()));
    }

    #[test]
    fn test_segment_builder() {
        let seg = StatusSegment::new("test")
            .color(Color::Green)
            .bold()
            .icon("+");
        assert_eq!(seg.color, Some(Color::Green));
        assert!(seg.bold);
        assert_eq!(seg.icon, Some("+".to_string()));
    }

    #[test]
    fn test_segment_render_string() {
        let seg = StatusSegment::new("text");
        assert_eq!(seg.render_string(), "text");

        let seg_icon = StatusSegment::with_icon("*", "branch");
        assert_eq!(seg_icon.render_string(), "* branch");
    }

    #[test]
    fn test_segment_from_string() {
        let seg: StatusSegment = "hello".into();
        assert_eq!(seg.text, "hello");
    }

    #[test]
    fn test_separator_as_str() {
        assert_eq!(StatusSeparator::Space.as_str(), " ");
        assert_eq!(StatusSeparator::Pipe.as_str(), " | ");
        assert_eq!(StatusSeparator::Bullet.as_str(), " • ");
        assert_eq!(StatusSeparator::None.as_str(), "");
    }

    #[test]
    fn test_props_new() {
        let props = StatusBarProps::new(["a", "b", "c"]);
        assert_eq!(props.segments.len(), 3);
    }

    #[test]
    fn test_props_builder() {
        let props = StatusBarProps::new(Vec::<&str>::new())
            .segment("one")
            .segment("two")
            .separator(StatusSeparator::Pipe)
            .square_brackets();

        assert_eq!(props.segments.len(), 2);
        assert_eq!(props.separator, StatusSeparator::Pipe);
        assert_eq!(props.prefix, Some("[".to_string()));
        assert_eq!(props.suffix, Some("]".to_string()));
    }

    #[test]
    fn test_props_text() {
        let props = StatusBarProps::new(Vec::<&str>::new())
            .text("ok", Color::Green)
            .text("warn", Color::Yellow);

        assert_eq!(props.segments.len(), 2);
        assert_eq!(props.segments[0].color, Some(Color::Green));
    }

    #[test]
    fn test_props_with_icon() {
        let props =
            StatusBarProps::new(Vec::<&str>::new()).with_icon(icons::CHECK, "done", Color::Green);

        assert_eq!(props.segments.len(), 1);
        assert_eq!(props.segments[0].icon, Some("✓".to_string()));
    }

    #[test]
    fn test_render_string() {
        let props = StatusBarProps::new(["a", "b", "c"]).separator(StatusSeparator::Pipe);
        assert_eq!(props.render_string(), "a | b | c");
    }

    #[test]
    fn test_render_string_with_brackets() {
        let props = StatusBarProps::new(["x", "y"]).square_brackets();
        assert_eq!(props.render_string(), "[x y]");
    }

    #[test]
    fn test_render_string_empty() {
        let props = StatusBarProps::new(Vec::<&str>::new());
        assert_eq!(props.render_string(), "");
    }

    #[test]
    fn test_component_render() {
        let props = StatusBarProps::new(["a", "b"]);
        let elem = StatusBar::render(&props);
        assert!(elem.is_fragment());
    }

    #[test]
    fn test_component_render_empty() {
        let props = StatusBarProps::new(Vec::<&str>::new());
        let elem = StatusBar::render(&props);
        assert!(elem.is_text());
    }

    #[test]
    fn test_helper_git_branch() {
        let seg = git_branch("main", Color::Green);
        assert!(seg.render_string().contains("main"));
        assert_eq!(seg.color, Some(Color::Green));
    }

    #[test]
    fn test_helper_status_ok() {
        let seg = status_ok("done");
        assert!(seg.render_string().contains("done"));
        assert_eq!(seg.color, Some(Color::Green));
    }

    #[test]
    fn test_helper_status_error() {
        let seg = status_error("failed");
        assert!(seg.render_string().contains("failed"));
        assert_eq!(seg.color, Some(Color::Red));
    }

    #[test]
    fn test_helper_status_warning() {
        let seg = status_warning("slow");
        assert!(seg.render_string().contains("slow"));
        assert_eq!(seg.color, Some(Color::Yellow));
    }
}
