//! Progress bar component - visual progress indicator.
//!
//! The Progress component displays a progress bar showing
//! completion percentage with customizable styling. Includes 6 built-in styles.
//!
//! ## When to use Progress
//!
//! - Determinate operations (known percentage complete)
//! - File downloads, uploads, processing tasks
//! - Multi-step wizards showing completion
//!
//! ## See also
//!
//! - [`Spinner`](super::Spinner) — Use for indeterminate loading (unknown duration)
//! - [`Timer`](super::Timer) — Show elapsed/remaining time alongside progress

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// Built-in progress bar styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressStyle {
    /// Block style: ████████░░░░
    #[default]
    Block,
    /// ASCII style: ========----
    Ascii,
    /// Thin style: ───────○────
    Thin,
    /// Thick style: ▓▓▓▓▓▓▓░░░░░
    Thick,
    /// Dots style: ●●●●●○○○○○
    Dots,
    /// Braille style: ⣿⣿⣿⣿⣀⣀⣀⣀
    Braille,
}

impl ProgressStyle {
    /// Get the characters for filled and empty portions.
    pub fn chars(&self) -> ProgressChars {
        match self {
            ProgressStyle::Block => ProgressChars {
                filled: '█',
                empty: '░',
                head: None,
            },
            ProgressStyle::Ascii => ProgressChars {
                filled: '=',
                empty: '-',
                head: Some('>'),
            },
            ProgressStyle::Thin => ProgressChars {
                filled: '─',
                empty: '─',
                head: Some('○'),
            },
            ProgressStyle::Thick => ProgressChars {
                filled: '▓',
                empty: '░',
                head: None,
            },
            ProgressStyle::Dots => ProgressChars {
                filled: '●',
                empty: '○',
                head: None,
            },
            ProgressStyle::Braille => ProgressChars {
                filled: '⣿',
                empty: '⣀',
                head: None,
            },
        }
    }
}

/// Characters used to render the progress bar.
#[derive(Debug, Clone, Copy)]
pub struct ProgressChars {
    /// Character for filled portion.
    pub filled: char,
    /// Character for empty portion.
    pub empty: char,
    /// Optional character for the head/cursor position.
    pub head: Option<char>,
}

impl Default for ProgressChars {
    fn default() -> Self {
        ProgressStyle::Block.chars()
    }
}

/// Properties for the Progress component.
#[derive(Debug, Clone)]
pub struct ProgressProps {
    /// Progress value from 0.0 to 1.0.
    pub progress: f32,
    /// Width of the progress bar in characters.
    pub width: usize,
    /// Progress bar style.
    pub style: ProgressStyle,
    /// Custom characters (overrides style if set).
    pub custom_chars: Option<ProgressChars>,
    /// Whether to show percentage text.
    pub show_percentage: bool,
    /// Optional label to show before the bar.
    pub label: Option<String>,
    /// Color for filled portion.
    pub filled_color: Option<Color>,
    /// Color for empty portion.
    pub empty_color: Option<Color>,
    /// Color for percentage text.
    pub text_color: Option<Color>,
    /// Whether to show brackets around the bar.
    pub brackets: bool,
    /// Whether the filled portion should be bold.
    pub bold: bool,
    /// Whether the empty portion should be dimmed.
    pub dim_empty: bool,
}

impl Default for ProgressProps {
    fn default() -> Self {
        Self {
            progress: 0.0,
            width: 20,
            style: ProgressStyle::Block,
            custom_chars: None,
            show_percentage: false,
            label: None,
            filled_color: None,
            empty_color: None,
            text_color: None,
            brackets: false,
            bold: false,
            dim_empty: true,
        }
    }
}

impl ProgressProps {
    /// Create new ProgressProps with the given progress value.
    pub fn new(progress: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            ..Default::default()
        }
    }

    /// Set the progress value (0.0 to 1.0).
    #[must_use]
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Set the progress from a percentage (0 to 100).
    #[must_use]
    pub fn percent(mut self, percent: u32) -> Self {
        self.progress = (percent.min(100) as f32) / 100.0;
        self
    }

    /// Set the width in characters.
    #[must_use]
    pub fn width(mut self, width: usize) -> Self {
        self.width = width.max(1);
        self
    }

    /// Set the progress bar style.
    #[must_use]
    pub fn style(mut self, style: ProgressStyle) -> Self {
        self.style = style;
        self
    }

    /// Set custom characters.
    #[must_use]
    pub fn custom_chars(mut self, chars: ProgressChars) -> Self {
        self.custom_chars = Some(chars);
        self
    }

    /// Show percentage text after the bar.
    #[must_use]
    pub fn show_percentage(mut self) -> Self {
        self.show_percentage = true;
        self
    }

    /// Set a label to show before the bar.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the color for the filled portion.
    #[must_use]
    pub fn filled_color(mut self, color: Color) -> Self {
        self.filled_color = Some(color);
        self
    }

    /// Set the color for the empty portion.
    #[must_use]
    pub fn empty_color(mut self, color: Color) -> Self {
        self.empty_color = Some(color);
        self
    }

    /// Set the color for percentage/label text.
    #[must_use]
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Set a single color for the entire bar.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.filled_color = Some(color);
        self.empty_color = Some(color);
        self.text_color = Some(color);
        self
    }

    /// Show brackets around the bar.
    #[must_use]
    pub fn brackets(mut self) -> Self {
        self.brackets = true;
        self
    }

    /// Make the filled portion bold.
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Dim the empty portion.
    #[must_use]
    pub fn dim_empty(mut self, dim: bool) -> Self {
        self.dim_empty = dim;
        self
    }

    /// Get the characters to use for rendering.
    fn chars(&self) -> ProgressChars {
        self.custom_chars.unwrap_or_else(|| self.style.chars())
    }

    /// Get the percentage as an integer (0-100).
    pub fn percentage(&self) -> u32 {
        (self.progress * 100.0).round() as u32
    }

    /// Build the progress bar string.
    pub fn render_string(&self) -> String {
        let chars = self.chars();
        let filled_count = ((self.progress * self.width as f32).round() as usize).min(self.width);
        let empty_count = self.width - filled_count;

        let mut result = String::new();

        // Label
        if let Some(ref label) = self.label {
            result.push_str(label);
            result.push(' ');
        }

        // Opening bracket
        if self.brackets {
            result.push('[');
        }

        // Filled portion
        if let Some(head) = chars.head {
            // With head character
            if filled_count > 0 {
                result.extend(std::iter::repeat(chars.filled).take(filled_count - 1));
                if filled_count < self.width {
                    result.push(head);
                } else {
                    result.push(chars.filled);
                }
            }
        } else {
            // Without head character
            result.extend(std::iter::repeat(chars.filled).take(filled_count));
        }

        // Empty portion
        result.extend(std::iter::repeat(chars.empty).take(empty_count));

        // Closing bracket
        if self.brackets {
            result.push(']');
        }

        // Percentage
        if self.show_percentage {
            result.push_str(&format!(" {:>3}%", self.percentage()));
        }

        result
    }
}

/// A component that displays a progress bar.
///
/// # Examples
///
/// ```ignore
/// // Simple progress bar
/// let props = ProgressProps::new(0.75)
///     .width(30)
///     .color(Color::Green)
///     .show_percentage();
///
/// let elem = Progress::render(&props);
/// // Renders: ██████████████████████░░░░░░░░ 75%
/// ```
pub struct Progress;

impl Component for Progress {
    type Props = ProgressProps;

    fn render(props: &Self::Props) -> Element {
        let content = props.render_string();

        // Build style
        let mut style = Style::new();
        if let Some(color) = props.filled_color {
            style = style.fg(color);
        }
        if props.bold {
            style = style.add_modifier(Modifier::BOLD);
        }

        Element::styled_text(&content, style)
    }
}

/// Helper to create a simple progress bar string.
///
/// # Example
///
/// ```ignore
/// let bar = progress_bar(75, 20); // 75%, 20 chars wide
/// // Returns: "███████████████░░░░░"
/// ```
pub fn progress_bar(percent: u32, width: usize) -> String {
    ProgressProps::new(percent as f32 / 100.0)
        .width(width)
        .render_string()
}

/// Helper to create a bracketed progress bar string.
///
/// # Example
///
/// ```ignore
/// let bar = progress_bar_bracketed(50, 15); // 50%, 15 chars wide
/// // Returns: "[███████░░░░░░░░]"
/// ```
pub fn progress_bar_bracketed(percent: u32, width: usize) -> String {
    ProgressProps::new(percent as f32 / 100.0)
        .width(width)
        .brackets()
        .render_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_props_default() {
        let props = ProgressProps::default();
        assert_eq!(props.progress, 0.0);
        assert_eq!(props.width, 20);
        assert_eq!(props.style, ProgressStyle::Block);
    }

    #[test]
    fn test_progress_props_new() {
        let props = ProgressProps::new(0.5);
        assert_eq!(props.progress, 0.5);
    }

    #[test]
    fn test_progress_props_clamp() {
        let props = ProgressProps::new(1.5);
        assert_eq!(props.progress, 1.0);

        let props = ProgressProps::new(-0.5);
        assert_eq!(props.progress, 0.0);
    }

    #[test]
    fn test_progress_props_percent() {
        let props = ProgressProps::default().percent(75);
        assert_eq!(props.progress, 0.75);
    }

    #[test]
    fn test_progress_props_builder() {
        let props = ProgressProps::new(0.5)
            .width(30)
            .style(ProgressStyle::Ascii)
            .filled_color(Color::Green)
            .show_percentage()
            .brackets();

        assert_eq!(props.progress, 0.5);
        assert_eq!(props.width, 30);
        assert_eq!(props.style, ProgressStyle::Ascii);
        assert_eq!(props.filled_color, Some(Color::Green));
        assert!(props.show_percentage);
        assert!(props.brackets);
    }

    #[test]
    fn test_progress_percentage() {
        let props = ProgressProps::new(0.756);
        assert_eq!(props.percentage(), 76);
    }

    #[test]
    fn test_progress_render_string_empty() {
        let props = ProgressProps::new(0.0).width(10);
        let bar = props.render_string();
        assert_eq!(bar, "░░░░░░░░░░");
    }

    #[test]
    fn test_progress_render_string_full() {
        let props = ProgressProps::new(1.0).width(10);
        let bar = props.render_string();
        assert_eq!(bar, "██████████");
    }

    #[test]
    fn test_progress_render_string_half() {
        let props = ProgressProps::new(0.5).width(10);
        let bar = props.render_string();
        assert_eq!(bar, "█████░░░░░");
    }

    #[test]
    fn test_progress_render_string_with_brackets() {
        let props = ProgressProps::new(0.5).width(10).brackets();
        let bar = props.render_string();
        assert_eq!(bar, "[█████░░░░░]");
    }

    #[test]
    fn test_progress_render_string_with_percentage() {
        let props = ProgressProps::new(0.75).width(10).show_percentage();
        let bar = props.render_string();
        assert!(bar.ends_with(" 75%"));
    }

    #[test]
    fn test_progress_render_string_with_label() {
        let props = ProgressProps::new(0.5).width(10).label("Loading");
        let bar = props.render_string();
        assert!(bar.starts_with("Loading "));
    }

    #[test]
    fn test_progress_render_string_ascii_style() {
        let props = ProgressProps::new(0.5)
            .width(10)
            .style(ProgressStyle::Ascii);
        let bar = props.render_string();
        // ASCII style has a head character '>'
        assert!(bar.contains('='));
        assert!(bar.contains('>'));
        assert!(bar.contains('-'));
    }

    #[test]
    fn test_progress_style_block_chars() {
        let chars = ProgressStyle::Block.chars();
        assert_eq!(chars.filled, '█');
        assert_eq!(chars.empty, '░');
        assert!(chars.head.is_none());
    }

    #[test]
    fn test_progress_style_ascii_chars() {
        let chars = ProgressStyle::Ascii.chars();
        assert_eq!(chars.filled, '=');
        assert_eq!(chars.empty, '-');
        assert_eq!(chars.head, Some('>'));
    }

    #[test]
    fn test_progress_helper_function() {
        let bar = progress_bar(50, 10);
        assert_eq!(bar, "█████░░░░░");
    }

    #[test]
    fn test_progress_helper_bracketed() {
        let bar = progress_bar_bracketed(50, 10);
        assert_eq!(bar, "[█████░░░░░]");
    }

    #[test]
    fn test_progress_render_component() {
        let props = ProgressProps::new(0.5).width(10);
        let elem = Progress::render(&props);
        match elem {
            Element::Text { content, .. } => {
                assert_eq!(content, "█████░░░░░");
            }
            _ => panic!("Expected Text element"),
        }
    }

    #[test]
    fn test_all_progress_styles() {
        let styles = [
            ProgressStyle::Block,
            ProgressStyle::Ascii,
            ProgressStyle::Thin,
            ProgressStyle::Thick,
            ProgressStyle::Dots,
            ProgressStyle::Braille,
        ];

        for style in &styles {
            let props = ProgressProps::new(0.5).width(10).style(*style);
            let bar = props.render_string();
            assert_eq!(
                bar.chars().count(),
                10,
                "Style {:?} has wrong length",
                style
            );
        }
    }
}
