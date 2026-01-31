//! Bar Chart component for horizontal bar visualization.
//!
//! ## When to use BarChart
//!
//! - Comparing values across categories
//! - Resource usage visualization
//! - Survey results or voting data
//!
//! ## See also
//!
//! - [`Sparkline`](super::Sparkline) — Compact inline trend chart
//! - [`Progress`](super::Progress) — Single progress value
//! - [`Table`](super::Table) — Tabular data without visual bars
//!
//! # Example
//!
//! ```ignore
//! use blaeck::prelude::*;
//!
//! let data = vec![
//!     BarData::new("Rust", 85.0).color(Color::Yellow),
//!     BarData::new("Python", 72.0).color(Color::Blue),
//!     BarData::new("JavaScript", 68.0).color(Color::Green),
//! ];
//!
//! Element::node::<BarChart>(
//!     BarChartProps::new(data)
//!         .max_value(100.0)
//!         .bar_width(20)
//!         .show_values(true),
//!     vec![],
//! )
//! ```

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// Data for a single bar.
#[derive(Debug, Clone)]
pub struct BarData {
    /// Label for the bar.
    pub label: String,
    /// Value (determines bar length).
    pub value: f64,
    /// Optional color for this bar.
    pub color: Option<Color>,
}

impl BarData {
    /// Create a new bar data point.
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    /// Set the color for this bar.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

/// Bar chart style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BarStyle {
    /// Solid block: ████████
    #[default]
    Block,
    /// Hash marks: ########
    Hash,
    /// Equals signs: ========
    Equals,
    /// Dots: ········
    Dot,
    /// Thin blocks: ▏▏▏▏▏▏▏▏
    Thin,
    /// Half blocks with gradient: ▓▓▓▒▒░░░
    Gradient,
}

impl BarStyle {
    /// Get the filled character for this style.
    pub fn filled_char(&self) -> char {
        match self {
            BarStyle::Block => '█',
            BarStyle::Hash => '#',
            BarStyle::Equals => '=',
            BarStyle::Dot => '●',
            BarStyle::Thin => '▏',
            BarStyle::Gradient => '█',
        }
    }

    /// Get the empty character for this style.
    pub fn empty_char(&self) -> char {
        match self {
            BarStyle::Block => ' ',
            BarStyle::Hash => ' ',
            BarStyle::Equals => ' ',
            BarStyle::Dot => '○',
            BarStyle::Thin => ' ',
            BarStyle::Gradient => ' ',
        }
    }

    /// Render a bar of the given fill ratio.
    pub fn render(&self, width: usize, ratio: f64) -> String {
        let ratio = ratio.clamp(0.0, 1.0);
        let filled = (width as f64 * ratio).round() as usize;
        let empty = width.saturating_sub(filled);

        match self {
            BarStyle::Gradient => {
                // Use gradient characters for smoother appearance
                let mut result = String::new();
                let full_blocks = (width as f64 * ratio) as usize;
                let remainder = (width as f64 * ratio) - full_blocks as f64;

                // Full blocks
                for _ in 0..full_blocks {
                    result.push('█');
                }

                // Partial block based on remainder
                if full_blocks < width {
                    let partial = if remainder > 0.75 {
                        '▓'
                    } else if remainder > 0.5 {
                        '▒'
                    } else if remainder > 0.25 {
                        '░'
                    } else {
                        ' '
                    };
                    result.push(partial);
                }

                // Empty space
                let current_len = result.chars().count();
                for _ in current_len..width {
                    result.push(' ');
                }
                result
            }
            _ => {
                let filled_str: String = std::iter::repeat_n(self.filled_char(), filled).collect();
                let empty_str: String = std::iter::repeat_n(self.empty_char(), empty).collect();
                format!("{}{}", filled_str, empty_str)
            }
        }
    }
}

/// Value display format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValueFormat {
    /// No value display.
    #[default]
    None,
    /// Raw value: "85"
    Raw,
    /// With decimal: "85.0"
    Decimal1,
    /// With two decimals: "85.00"
    Decimal2,
    /// Percentage: "85%"
    Percent,
    /// Percentage with decimal: "85.0%"
    PercentDecimal,
}

impl ValueFormat {
    /// Format a value.
    pub fn format(&self, value: f64, max_value: f64) -> String {
        match self {
            ValueFormat::None => String::new(),
            ValueFormat::Raw => format!("{:.0}", value),
            ValueFormat::Decimal1 => format!("{:.1}", value),
            ValueFormat::Decimal2 => format!("{:.2}", value),
            ValueFormat::Percent => {
                let pct = (value / max_value) * 100.0;
                format!("{:.0}%", pct)
            }
            ValueFormat::PercentDecimal => {
                let pct = (value / max_value) * 100.0;
                format!("{:.1}%", pct)
            }
        }
    }
}

/// Properties for the BarChart component.
#[derive(Debug, Clone)]
pub struct BarChartProps {
    /// Data points to display.
    pub data: Vec<BarData>,
    /// Maximum value (for scaling). If None, uses max from data.
    pub max_value: Option<f64>,
    /// Width of the bars in characters.
    pub bar_width: usize,
    /// Bar style.
    pub style: BarStyle,
    /// Value display format.
    pub value_format: ValueFormat,
    /// Default bar color (if not set per-bar).
    pub default_color: Option<Color>,
    /// Label color.
    pub label_color: Option<Color>,
    /// Value color.
    pub value_color: Option<Color>,
    /// Background color.
    pub bg_color: Option<Color>,
    /// Show brackets around bars: [████    ]
    pub brackets: bool,
    /// Gap between label and bar.
    pub label_gap: usize,
    /// Minimum label width (for alignment).
    pub min_label_width: Option<usize>,
    /// Show a legend/scale.
    pub show_scale: bool,
}

impl Default for BarChartProps {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            max_value: None,
            bar_width: 20,
            style: BarStyle::Block,
            value_format: ValueFormat::None,
            default_color: None,
            label_color: None,
            value_color: None,
            bg_color: None,
            brackets: false,
            label_gap: 1,
            min_label_width: None,
            show_scale: false,
        }
    }
}

impl BarChartProps {
    /// Create new BarChartProps with data.
    pub fn new(data: Vec<BarData>) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }

    /// Set maximum value for scaling.
    #[must_use]
    pub fn max_value(mut self, max: f64) -> Self {
        self.max_value = Some(max);
        self
    }

    /// Set bar width.
    #[must_use]
    pub fn bar_width(mut self, width: usize) -> Self {
        self.bar_width = width.max(1);
        self
    }

    /// Set bar style.
    #[must_use]
    pub fn style(mut self, style: BarStyle) -> Self {
        self.style = style;
        self
    }

    /// Set value display format.
    #[must_use]
    pub fn value_format(mut self, format: ValueFormat) -> Self {
        self.value_format = format;
        self
    }

    /// Show values.
    #[must_use]
    pub fn show_values(mut self) -> Self {
        self.value_format = ValueFormat::Raw;
        self
    }

    /// Show percentages.
    #[must_use]
    pub fn show_percent(mut self) -> Self {
        self.value_format = ValueFormat::Percent;
        self
    }

    /// Set default bar color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.default_color = Some(color);
        self
    }

    /// Set label color.
    #[must_use]
    pub fn label_color(mut self, color: Color) -> Self {
        self.label_color = Some(color);
        self
    }

    /// Set value color.
    #[must_use]
    pub fn value_color(mut self, color: Color) -> Self {
        self.value_color = Some(color);
        self
    }

    /// Set background color.
    #[must_use]
    pub fn bg_color(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Show brackets around bars.
    #[must_use]
    pub fn brackets(mut self, show: bool) -> Self {
        self.brackets = show;
        self
    }

    /// Set minimum label width for alignment.
    #[must_use]
    pub fn min_label_width(mut self, width: usize) -> Self {
        self.min_label_width = Some(width);
        self
    }

    /// Show scale at bottom.
    #[must_use]
    pub fn show_scale(mut self, show: bool) -> Self {
        self.show_scale = show;
        self
    }

    /// Get the effective max value.
    fn effective_max(&self) -> f64 {
        self.max_value.unwrap_or_else(|| {
            self.data
                .iter()
                .map(|d| d.value)
                .fold(0.0, f64::max)
                .max(1.0)
        })
    }

    /// Get the max label width.
    fn max_label_width(&self) -> usize {
        let data_max = self.data.iter().map(|d| d.label.len()).max().unwrap_or(0);
        self.min_label_width.unwrap_or(0).max(data_max)
    }
}

/// A component that displays horizontal bar charts.
pub struct BarChart;

impl Component for BarChart {
    type Props = BarChartProps;

    fn render(props: &Self::Props) -> Element {
        if props.data.is_empty() {
            return Element::Empty;
        }

        let max_value = props.effective_max();
        let label_width = props.max_label_width();

        // Helper to apply bg_color to a style
        let with_bg = |mut style: Style| -> Style {
            if let Some(bg) = props.bg_color {
                style = style.bg(bg);
            }
            style
        };

        let mut lines: Vec<Element> = Vec::new();

        for bar in &props.data {
            let ratio = if max_value > 0.0 {
                bar.value / max_value
            } else {
                0.0
            };

            // Build the line
            let mut segments: Vec<Element> = Vec::new();

            // Label (right-aligned to label_width)
            let label_padded = format!("{:>width$}", bar.label, width = label_width);
            let label_style = with_bg(if let Some(color) = props.label_color {
                Style::new().fg(color)
            } else {
                Style::new()
            });
            segments.push(Element::styled_text(&label_padded, label_style));

            // Gap
            segments.push(Element::styled_text(" ".repeat(props.label_gap), with_bg(Style::new())));

            // Opening bracket
            if props.brackets {
                segments.push(Element::styled_text("[", with_bg(Style::new())));
            }

            // Bar
            let bar_str = props.style.render(props.bar_width, ratio);
            let bar_color = bar.color.or(props.default_color);
            let bar_style = with_bg(if let Some(color) = bar_color {
                Style::new().fg(color)
            } else {
                Style::new()
            });
            segments.push(Element::styled_text(&bar_str, bar_style));

            // Closing bracket
            if props.brackets {
                segments.push(Element::styled_text("]", with_bg(Style::new())));
            }

            // Value
            if props.value_format != ValueFormat::None {
                let value_str = props.value_format.format(bar.value, max_value);
                let value_style = with_bg(if let Some(color) = props.value_color {
                    Style::new().fg(color)
                } else {
                    Style::new().add_modifier(Modifier::DIM)
                });
                segments.push(Element::styled_text(" ", with_bg(Style::new())));
                segments.push(Element::styled_text(&value_str, value_style));
            }

            // Combine segments into a line
            lines.push(Element::Fragment(segments));
        }

        // Scale line
        if props.show_scale {
            let scale_offset = label_width + props.label_gap + if props.brackets { 1 } else { 0 };
            let mut scale_line = " ".repeat(scale_offset);
            scale_line.push_str(&format!(
                "0{:>width$}",
                max_value,
                width = props.bar_width - 1
            ));
            if props.brackets {
                scale_line.push(' ');
            }
            lines.push(Element::styled_text(
                &scale_line,
                Style::new().add_modifier(Modifier::DIM),
            ));
        }

        if lines.len() == 1 {
            lines.remove(0)
        } else {
            Element::Fragment(lines)
        }
    }
}

/// Helper to create a simple bar chart.
pub fn bar_chart(data: Vec<BarData>) -> Element {
    BarChart::render(&BarChartProps::new(data))
}

/// Helper to create a bar chart with values shown.
pub fn bar_chart_with_values(data: Vec<BarData>, max: f64) -> Element {
    BarChart::render(&BarChartProps::new(data).max_value(max).show_values())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_data_new() {
        let bar = BarData::new("Test", 50.0);
        assert_eq!(bar.label, "Test");
        assert_eq!(bar.value, 50.0);
        assert!(bar.color.is_none());
    }

    #[test]
    fn test_bar_data_color() {
        let bar = BarData::new("Test", 50.0).color(Color::Red);
        assert_eq!(bar.color, Some(Color::Red));
    }

    #[test]
    fn test_bar_style_render_block() {
        let bar = BarStyle::Block.render(10, 0.5);
        assert_eq!(bar.chars().count(), 10);
        assert!(bar.starts_with("█████"));
    }

    #[test]
    fn test_bar_style_render_hash() {
        let bar = BarStyle::Hash.render(10, 0.3);
        assert_eq!(bar.chars().count(), 10);
        assert!(bar.starts_with("###"));
    }

    #[test]
    fn test_bar_style_render_zero() {
        let bar = BarStyle::Block.render(10, 0.0);
        assert_eq!(bar, "          ");
    }

    #[test]
    fn test_bar_style_render_full() {
        let bar = BarStyle::Block.render(10, 1.0);
        assert_eq!(bar, "██████████");
    }

    #[test]
    fn test_value_format_raw() {
        assert_eq!(ValueFormat::Raw.format(85.5, 100.0), "86");
    }

    #[test]
    fn test_value_format_decimal() {
        assert_eq!(ValueFormat::Decimal1.format(85.5, 100.0), "85.5");
    }

    #[test]
    fn test_value_format_percent() {
        assert_eq!(ValueFormat::Percent.format(50.0, 100.0), "50%");
    }

    #[test]
    fn test_bar_chart_props_new() {
        let data = vec![BarData::new("A", 10.0)];
        let props = BarChartProps::new(data);
        assert_eq!(props.data.len(), 1);
        assert_eq!(props.bar_width, 20);
    }

    #[test]
    fn test_bar_chart_props_builder() {
        let data = vec![BarData::new("A", 10.0)];
        let props = BarChartProps::new(data)
            .max_value(100.0)
            .bar_width(30)
            .style(BarStyle::Hash)
            .show_values();
        assert_eq!(props.max_value, Some(100.0));
        assert_eq!(props.bar_width, 30);
        assert_eq!(props.style, BarStyle::Hash);
        assert_eq!(props.value_format, ValueFormat::Raw);
    }

    #[test]
    fn test_bar_chart_effective_max() {
        let data = vec![BarData::new("A", 30.0), BarData::new("B", 50.0)];
        let props = BarChartProps::new(data);
        assert_eq!(props.effective_max(), 50.0);

        let props = props.max_value(100.0);
        assert_eq!(props.effective_max(), 100.0);
    }

    #[test]
    fn test_bar_chart_render_empty() {
        let props = BarChartProps::new(vec![]);
        let elem = BarChart::render(&props);
        assert!(elem.is_empty());
    }

    #[test]
    fn test_bar_chart_render_single() {
        let data = vec![BarData::new("Test", 50.0)];
        let props = BarChartProps::new(data).max_value(100.0);
        let elem = BarChart::render(&props);
        assert!(elem.is_fragment());
    }

    #[test]
    fn test_bar_chart_render_multiple() {
        let data = vec![
            BarData::new("A", 30.0),
            BarData::new("B", 50.0),
            BarData::new("C", 80.0),
        ];
        let props = BarChartProps::new(data).max_value(100.0);
        let elem = BarChart::render(&props);
        assert!(elem.is_fragment());
    }

    #[test]
    fn test_bar_chart_helper() {
        let data = vec![BarData::new("Test", 50.0)];
        let elem = bar_chart(data);
        assert!(elem.is_fragment());
    }

    #[test]
    fn test_bar_chart_with_values_helper() {
        let data = vec![BarData::new("Test", 50.0)];
        let elem = bar_chart_with_values(data, 100.0);
        assert!(elem.is_fragment());
    }
}
