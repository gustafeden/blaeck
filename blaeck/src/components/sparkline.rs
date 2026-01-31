//! Sparkline component - mini inline charts.
//!
//! The Sparkline component displays a compact inline chart using
//! Unicode block characters (▁▂▃▅▇) to visualize data trends.
//!
//! ## When to use Sparkline
//!
//! - Quick trend visualization (CPU, memory over time)
//! - Inline data summary in a dashboard
//! - Small-scale data that doesn't need full chart
//!
//! ## See also
//!
//! - [`BarChart`](super::BarChart) — Full horizontal bar charts with labels
//! - [`Progress`](super::Progress) — Single value (not a series)

use crate::element::{Component, Element};
use crate::style::{Color, Style};

/// Block characters for sparkline from lowest to highest.
const BLOCKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

/// Dot characters for sparkline.
const DOTS: [char; 4] = ['⣀', '⠤', '⠒', '⠉'];

/// Style for the sparkline visualization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SparklineStyle {
    /// Block bars (▁▂▃▄▅▆▇█)
    #[default]
    Block,
    /// Braille dots (⣀⠤⠒⠉)
    Dot,
    /// Simple ASCII (#)
    Ascii,
    /// Thin bars (│)
    Thin,
}

impl SparklineStyle {
    /// Get the characters for this style.
    pub fn chars(&self) -> &'static [char] {
        match self {
            SparklineStyle::Block => &BLOCKS,
            SparklineStyle::Dot => &DOTS,
            SparklineStyle::Ascii => &['_', '.', '-', '=', '#'],
            SparklineStyle::Thin => &[' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉'],
        }
    }
}

/// Properties for the Sparkline component.
#[derive(Debug, Clone)]
pub struct SparklineProps {
    /// Data values to display.
    pub data: Vec<f64>,
    /// Minimum value for scaling (auto-calculated if None).
    pub min: Option<f64>,
    /// Maximum value for scaling (auto-calculated if None).
    pub max: Option<f64>,
    /// Sparkline style.
    pub style: SparklineStyle,
    /// Color for the sparkline.
    pub color: Option<Color>,
    /// Background color.
    pub bg_color: Option<Color>,
    /// Color for values above a threshold.
    pub high_color: Option<Color>,
    /// Color for values below a threshold.
    pub low_color: Option<Color>,
    /// Threshold for high/low coloring.
    pub threshold: Option<f64>,
    /// Optional label prefix.
    pub label: Option<String>,
    /// Show min/max values.
    pub show_minmax: bool,
}

impl Default for SparklineProps {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            min: None,
            max: None,
            style: SparklineStyle::Block,
            color: None,
            bg_color: None,
            high_color: None,
            low_color: None,
            threshold: None,
            label: None,
            show_minmax: false,
        }
    }
}

impl SparklineProps {
    /// Create a new sparkline with data.
    pub fn new(data: impl IntoIterator<Item = impl Into<f64>>) -> Self {
        Self {
            data: data.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    /// Set minimum value for scaling.
    #[must_use]
    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// Set maximum value for scaling.
    #[must_use]
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// Set both min and max values.
    #[must_use]
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    /// Set the sparkline style.
    #[must_use]
    pub fn style(mut self, style: SparklineStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the background color.
    #[must_use]
    pub fn bg_color(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set high color (for values above threshold).
    #[must_use]
    pub fn high_color(mut self, color: Color) -> Self {
        self.high_color = Some(color);
        self
    }

    /// Set low color (for values below threshold).
    #[must_use]
    pub fn low_color(mut self, color: Color) -> Self {
        self.low_color = Some(color);
        self
    }

    /// Set threshold for high/low coloring.
    #[must_use]
    pub fn threshold(mut self, threshold: f64) -> Self {
        self.threshold = Some(threshold);
        self
    }

    /// Set a label prefix.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Show min/max values at the end.
    #[must_use]
    pub fn show_minmax(mut self) -> Self {
        self.show_minmax = true;
        self
    }

    /// Get the effective min value.
    pub fn effective_min(&self) -> f64 {
        self.min
            .unwrap_or_else(|| self.data.iter().copied().fold(f64::INFINITY, f64::min))
    }

    /// Get the effective max value.
    pub fn effective_max(&self) -> f64 {
        self.max
            .unwrap_or_else(|| self.data.iter().copied().fold(f64::NEG_INFINITY, f64::max))
    }

    /// Render the sparkline string.
    pub fn render_string(&self) -> String {
        if self.data.is_empty() {
            return String::new();
        }

        let min = self.effective_min();
        let max = self.effective_max();
        let range = max - min;
        let chars = self.style.chars();
        let num_chars = chars.len();

        let mut result = String::new();

        // Add label if present
        if let Some(ref label) = self.label {
            result.push_str(label);
            result.push(' ');
        }

        // Generate sparkline
        for &value in &self.data {
            let normalized = if range == 0.0 {
                0.5 // All values are the same
            } else {
                (value - min) / range
            };

            // Map to character index
            let idx = ((normalized * (num_chars - 1) as f64).round() as usize).min(num_chars - 1);
            result.push(chars[idx]);
        }

        // Add min/max if requested
        if self.show_minmax {
            result.push_str(&format!(" ({:.1}-{:.1})", min, max));
        }

        result
    }
}

/// A component that displays a mini inline chart.
///
/// # Examples
///
/// ```ignore
/// // Simple sparkline
/// Element::node::<Sparkline>(
///     SparklineProps::new(vec![1, 4, 2, 8, 5, 7, 3])
///         .color(Color::Cyan),
///     vec![]
/// )
///
/// // With label and range
/// Element::node::<Sparkline>(
///     SparklineProps::new(cpu_history)
///         .label("CPU:")
///         .range(0.0, 100.0)
///         .color(Color::Green),
///     vec![]
/// )
/// ```
pub struct Sparkline;

impl Component for Sparkline {
    type Props = SparklineProps;

    fn render(props: &Self::Props) -> Element {
        let content = props.render_string();

        let mut style = Style::new();
        if let Some(color) = props.color {
            style = style.fg(color);
        }
        if let Some(bg) = props.bg_color {
            style = style.bg(bg);
        }

        Element::styled_text(&content, style)
    }
}

/// Helper function to create a sparkline string.
pub fn sparkline(data: impl IntoIterator<Item = impl Into<f64>>) -> String {
    SparklineProps::new(data).render_string()
}

/// Helper function to create a sparkline with a label.
pub fn sparkline_labeled(label: &str, data: impl IntoIterator<Item = impl Into<f64>>) -> String {
    SparklineProps::new(data).label(label).render_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparkline_props_new() {
        let props = SparklineProps::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(props.data.len(), 5);
    }

    #[test]
    fn test_sparkline_props_builder() {
        let props = SparklineProps::new(vec![1.0, 2.0, 3.0])
            .min(0.0)
            .max(10.0)
            .color(Color::Cyan)
            .label("Test:");

        assert_eq!(props.min, Some(0.0));
        assert_eq!(props.max, Some(10.0));
        assert_eq!(props.color, Some(Color::Cyan));
        assert_eq!(props.label, Some("Test:".to_string()));
    }

    #[test]
    fn test_sparkline_effective_minmax() {
        let props = SparklineProps::new(vec![2.0, 5.0, 1.0, 8.0, 3.0]);
        assert_eq!(props.effective_min(), 1.0);
        assert_eq!(props.effective_max(), 8.0);
    }

    #[test]
    fn test_sparkline_effective_minmax_override() {
        let props = SparklineProps::new(vec![2.0, 5.0, 1.0, 8.0, 3.0])
            .min(0.0)
            .max(10.0);
        assert_eq!(props.effective_min(), 0.0);
        assert_eq!(props.effective_max(), 10.0);
    }

    #[test]
    fn test_sparkline_render_empty() {
        let props = SparklineProps::new(Vec::<f64>::new());
        assert_eq!(props.render_string(), "");
    }

    #[test]
    fn test_sparkline_render_basic() {
        let props = SparklineProps::new(vec![0, 4, 7, 3]);
        let result = props.render_string();
        // Should contain block characters
        assert!(!result.is_empty());
        assert_eq!(result.chars().count(), 4);
    }

    #[test]
    fn test_sparkline_render_with_label() {
        let props = SparklineProps::new(vec![1, 2, 3]).label("Data:");
        let result = props.render_string();
        assert!(result.starts_with("Data: "));
    }

    #[test]
    fn test_sparkline_render_with_minmax() {
        let props = SparklineProps::new(vec![1.0, 5.0, 3.0]).show_minmax();
        let result = props.render_string();
        assert!(result.contains("(1.0-5.0)"));
    }

    #[test]
    fn test_sparkline_render_constant() {
        // All same values should work
        let props = SparklineProps::new(vec![5, 5, 5, 5]);
        let result = props.render_string();
        assert_eq!(result.chars().count(), 4);
    }

    #[test]
    fn test_sparkline_style_block() {
        let chars = SparklineStyle::Block.chars();
        assert_eq!(chars.len(), 8);
        assert_eq!(chars[0], '▁');
        assert_eq!(chars[7], '█');
    }

    #[test]
    fn test_sparkline_style_dot() {
        let chars = SparklineStyle::Dot.chars();
        assert_eq!(chars.len(), 4);
    }

    #[test]
    fn test_sparkline_helper() {
        let result = sparkline(vec![1, 2, 3, 4]);
        assert_eq!(result.chars().count(), 4);
    }

    #[test]
    fn test_sparkline_labeled_helper() {
        let result = sparkline_labeled("CPU:", vec![10, 20, 30]);
        assert!(result.starts_with("CPU: "));
    }

    #[test]
    fn test_sparkline_component_render() {
        let props = SparklineProps::new(vec![1, 2, 3]);
        let elem = Sparkline::render(&props);
        assert!(elem.is_text());
    }

    #[test]
    fn test_sparkline_range() {
        let props = SparklineProps::new(vec![50.0]).range(0.0, 100.0);
        let result = props.render_string();
        // 50% should be middle character
        assert_eq!(result.chars().count(), 1);
        let ch = result.chars().next().unwrap();
        // Should be roughly middle block character
        assert!(BLOCKS.contains(&ch));
    }
}
