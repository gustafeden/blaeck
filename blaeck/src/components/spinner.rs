//! Spinner component - animated loading indicator.
//!
//! The Spinner component displays an animated spinning indicator
//! to show that an operation is in progress. Includes 15 built-in styles.
//!
//! ## When to use Spinner
//!
//! - Indeterminate loading states (unknown duration)
//! - Background operations in progress
//! - Alongside status text ("Loading...", "Connecting...")
//!
//! ## See also
//!
//! - [`Progress`](super::Progress) — Use when you know the percentage complete
//! - [`Timer`](super::Timer) — Show elapsed/remaining time

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// Built-in spinner animation styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpinnerStyle {
    /// Braille dots pattern: ⠋ ⠙ ⠹ ⠸ ⠼ ⠴ ⠦ ⠧ ⠇ ⠏
    #[default]
    Dots,
    /// Line rotation: | / - \
    Line,
    /// Circular dots: ◐ ◓ ◑ ◒
    Circle,
    /// Growing dots: .  .. ...
    GrowingDots,
    /// Arrow rotation: ← ↖ ↑ ↗ → ↘ ↓ ↙
    Arrow,
    /// Bouncing bar: [=   ] [ =  ] [  = ] [   =]
    BouncingBar,
    /// Arc rotation: ◜ ◝ ◞ ◟
    Arc,
    /// Box corners: ◰ ◳ ◲ ◱
    BoxCorners,
    /// Triangle: ◢ ◣ ◤ ◥
    Triangle,
    /// Binary: 010010 101101 etc (random-ish)
    Binary,
    /// Clock: 🕐 🕑 🕒 ...
    Clock,
    /// Moon phases: 🌑 🌒 🌓 🌔 🌕 🌖 🌗 🌘
    Moon,
    /// Earth: 🌍 🌎 🌏
    Earth,
    /// Simple dots: ⠁ ⠂ ⠄ ⠂
    SimpleDots,
    /// Flip: _ _ _ - ‾ ‾ ‾ -
    Flip,
}

impl SpinnerStyle {
    /// Get the frames for this spinner style.
    pub fn frames(&self) -> &'static [&'static str] {
        match self {
            SpinnerStyle::Dots => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            SpinnerStyle::Line => &["|", "/", "-", "\\"],
            SpinnerStyle::Circle => &["◐", "◓", "◑", "◒"],
            SpinnerStyle::GrowingDots => &[".  ", ".. ", "...", " ..", "  .", "   "],
            SpinnerStyle::Arrow => &["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"],
            SpinnerStyle::BouncingBar => {
                &["[=   ]", "[ =  ]", "[  = ]", "[   =]", "[  = ]", "[ =  ]"]
            }
            SpinnerStyle::Arc => &["◜", "◝", "◞", "◟"],
            SpinnerStyle::BoxCorners => &["◰", "◳", "◲", "◱"],
            SpinnerStyle::Triangle => &["◢", "◣", "◤", "◥"],
            SpinnerStyle::Binary => &["010010", "001100", "100101", "111010", "101011", "011001"],
            SpinnerStyle::Clock => &[
                "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛",
            ],
            SpinnerStyle::Moon => &["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"],
            SpinnerStyle::Earth => &["🌍", "🌎", "🌏"],
            SpinnerStyle::SimpleDots => &["⠁", "⠂", "⠄", "⠂"],
            SpinnerStyle::Flip => &["_", "_", "_", "-", "`", "`", "'", "´", "-", "_", "_", "_"],
        }
    }

    /// Get the recommended interval in milliseconds for this spinner.
    pub fn interval_ms(&self) -> u64 {
        match self {
            SpinnerStyle::Dots => 80,
            SpinnerStyle::Line => 100,
            SpinnerStyle::Circle => 100,
            SpinnerStyle::GrowingDots => 200,
            SpinnerStyle::Arrow => 100,
            SpinnerStyle::BouncingBar => 80,
            SpinnerStyle::Arc => 100,
            SpinnerStyle::BoxCorners => 100,
            SpinnerStyle::Triangle => 100,
            SpinnerStyle::Binary => 100,
            SpinnerStyle::Clock => 200,
            SpinnerStyle::Moon => 150,
            SpinnerStyle::Earth => 200,
            SpinnerStyle::SimpleDots => 120,
            SpinnerStyle::Flip => 80,
        }
    }

    /// Get the frame at the given index (wraps around).
    pub fn frame_at(&self, index: usize) -> &'static str {
        let frames = self.frames();
        frames[index % frames.len()]
    }

    /// Get the number of frames in this spinner.
    pub fn frame_count(&self) -> usize {
        self.frames().len()
    }
}

/// Properties for the Spinner component.
#[derive(Debug, Clone, Default)]
pub struct SpinnerProps {
    /// The spinner animation style.
    pub style: SpinnerStyle,
    /// The current frame index (caller manages animation).
    pub frame: usize,
    /// Optional label text to show after the spinner.
    pub label: Option<String>,
    /// Spinner color.
    pub color: Option<Color>,
    /// Label color (defaults to spinner color if not set).
    pub label_color: Option<Color>,
    /// Whether the spinner should be bold.
    pub bold: bool,
    /// Whether the spinner should be dimmed.
    pub dim: bool,
    /// Custom frames (overrides style if set).
    pub custom_frames: Option<Vec<String>>,
}

impl SpinnerProps {
    /// Create new SpinnerProps with the default style.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the spinner style.
    #[must_use]
    pub fn style(mut self, style: SpinnerStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the current frame index.
    #[must_use]
    pub fn frame(mut self, frame: usize) -> Self {
        self.frame = frame;
        self
    }

    /// Set the label text.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the spinner color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the label color.
    #[must_use]
    pub fn label_color(mut self, color: Color) -> Self {
        self.label_color = Some(color);
        self
    }

    /// Set bold styling.
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set dim styling.
    #[must_use]
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Set custom animation frames.
    #[must_use]
    pub fn custom_frames(mut self, frames: Vec<String>) -> Self {
        self.custom_frames = Some(frames);
        self
    }

    /// Get the current frame string.
    pub fn current_frame(&self) -> &str {
        if let Some(ref custom) = self.custom_frames {
            if custom.is_empty() {
                return " ";
            }
            &custom[self.frame % custom.len()]
        } else {
            self.style.frame_at(self.frame)
        }
    }

    /// Get the recommended interval for the current style.
    pub fn interval_ms(&self) -> u64 {
        self.style.interval_ms()
    }

    fn spinner_style(&self) -> Style {
        let mut style = Style::new();
        if let Some(color) = self.color {
            style = style.fg(color);
        }
        if self.bold {
            style = style.add_modifier(Modifier::BOLD);
        }
        if self.dim {
            style = style.add_modifier(Modifier::DIM);
        }
        style
    }
}

/// A component that displays an animated spinner.
///
/// The Spinner component shows a rotating/animated indicator. The caller
/// is responsible for updating the `frame` prop to animate the spinner.
///
/// # Examples
///
/// ```ignore
/// use std::time::Instant;
///
/// let start = Instant::now();
/// let frame = (start.elapsed().as_millis() / 80) as usize;
///
/// let props = SpinnerProps::new()
///     .style(SpinnerStyle::Dots)
///     .frame(frame)
///     .label("Loading...")
///     .color(Color::Cyan);
///
/// let elem = Spinner::render(&props);
/// ```
pub struct Spinner;

impl Component for Spinner {
    type Props = SpinnerProps;

    fn render(props: &Self::Props) -> Element {
        let frame_str = props.current_frame();
        let spinner_style = props.spinner_style();

        if let Some(ref label) = props.label {
            // Spinner with label: combine spinner + space + label
            let content = format!("{} {}", frame_str, label);

            // For now, render as single styled text
            // A more sophisticated version could use separate styles for spinner vs label
            Element::styled_text(&content, spinner_style)
        } else {
            Element::styled_text(frame_str, spinner_style)
        }
    }
}

/// Helper to calculate frame index from elapsed time.
///
/// # Example
///
/// ```ignore
/// use std::time::Instant;
///
/// let start = Instant::now();
/// let frame = spinner_frame(start, SpinnerStyle::Dots);
/// ```
pub fn spinner_frame(start: std::time::Instant, style: SpinnerStyle) -> usize {
    let elapsed_ms = start.elapsed().as_millis() as u64;
    (elapsed_ms / style.interval_ms()) as usize
}

/// Helper to calculate frame index from elapsed time with custom interval.
pub fn spinner_frame_interval(start: std::time::Instant, interval_ms: u64) -> usize {
    let elapsed_ms = start.elapsed().as_millis() as u64;
    (elapsed_ms / interval_ms) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_style_dots_frames() {
        let style = SpinnerStyle::Dots;
        assert_eq!(style.frames().len(), 10);
        assert_eq!(style.frame_at(0), "⠋");
        assert_eq!(style.frame_at(10), "⠋"); // wraps
    }

    #[test]
    fn test_spinner_style_line_frames() {
        let style = SpinnerStyle::Line;
        assert_eq!(style.frames().len(), 4);
        assert_eq!(style.frame_at(0), "|");
        assert_eq!(style.frame_at(1), "/");
    }

    #[test]
    fn test_spinner_style_intervals() {
        assert_eq!(SpinnerStyle::Dots.interval_ms(), 80);
        assert_eq!(SpinnerStyle::Clock.interval_ms(), 200);
    }

    #[test]
    fn test_spinner_props_default() {
        let props = SpinnerProps::default();
        assert_eq!(props.style, SpinnerStyle::Dots);
        assert_eq!(props.frame, 0);
        assert!(props.label.is_none());
    }

    #[test]
    fn test_spinner_props_builder() {
        let props = SpinnerProps::new()
            .style(SpinnerStyle::Arrow)
            .frame(5)
            .label("Loading")
            .color(Color::Green)
            .bold();

        assert_eq!(props.style, SpinnerStyle::Arrow);
        assert_eq!(props.frame, 5);
        assert_eq!(props.label, Some("Loading".to_string()));
        assert_eq!(props.color, Some(Color::Green));
        assert!(props.bold);
    }

    #[test]
    fn test_spinner_current_frame() {
        let props = SpinnerProps::new().style(SpinnerStyle::Line).frame(2);
        assert_eq!(props.current_frame(), "-");
    }

    #[test]
    fn test_spinner_custom_frames() {
        let props = SpinnerProps::new()
            .custom_frames(vec!["A".into(), "B".into(), "C".into()])
            .frame(1);
        assert_eq!(props.current_frame(), "B");
    }

    #[test]
    fn test_spinner_custom_frames_wrap() {
        let props = SpinnerProps::new()
            .custom_frames(vec!["X".into(), "Y".into()])
            .frame(5);
        assert_eq!(props.current_frame(), "Y"); // 5 % 2 = 1
    }

    #[test]
    fn test_spinner_render_without_label() {
        let props = SpinnerProps::new().frame(0);
        let elem = Spinner::render(&props);
        match elem {
            Element::Text { content, .. } => {
                assert_eq!(content, "⠋");
            }
            _ => panic!("Expected Text element"),
        }
    }

    #[test]
    fn test_spinner_render_with_label() {
        let props = SpinnerProps::new().frame(0).label("Loading...");
        let elem = Spinner::render(&props);
        match elem {
            Element::Text { content, .. } => {
                assert_eq!(content, "⠋ Loading...");
            }
            _ => panic!("Expected Text element"),
        }
    }

    #[test]
    fn test_all_spinner_styles_have_frames() {
        let styles = [
            SpinnerStyle::Dots,
            SpinnerStyle::Line,
            SpinnerStyle::Circle,
            SpinnerStyle::GrowingDots,
            SpinnerStyle::Arrow,
            SpinnerStyle::BouncingBar,
            SpinnerStyle::Arc,
            SpinnerStyle::BoxCorners,
            SpinnerStyle::Triangle,
            SpinnerStyle::Binary,
            SpinnerStyle::Clock,
            SpinnerStyle::Moon,
            SpinnerStyle::Earth,
            SpinnerStyle::SimpleDots,
            SpinnerStyle::Flip,
        ];

        for style in &styles {
            assert!(
                style.frames().len() >= 2,
                "Style {:?} has too few frames",
                style
            );
            assert!(
                style.interval_ms() > 0,
                "Style {:?} has invalid interval",
                style
            );
        }
    }
}
