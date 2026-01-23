//! Timer/Stopwatch component for displaying elapsed or remaining time.
//!
//! ## When to use Timer
//!
//! - Showing elapsed time for operations
//! - Countdown timers with warnings
//! - Benchmarking or timing user actions
//!
//! ## See also
//!
//! - [`Progress`](super::Progress) — Visual progress bar (percentage-based)
//! - [`Spinner`](super::Spinner) — Indeterminate loading (no time shown)
//!
//! # Example
//!
//! ```ignore
//! use blaeck::prelude::*;
//! use std::time::Duration;
//!
//! // Stopwatch (counting up)
//! let elapsed = Duration::from_secs(125); // 2:05
//! let timer = Element::node::<Timer>(
//!     TimerProps::stopwatch(elapsed),
//!     vec![],
//! );
//!
//! // Countdown timer
//! let remaining = Duration::from_secs(30);
//! let timer = Element::node::<Timer>(
//!     TimerProps::countdown(remaining)
//!         .warn_at(Duration::from_secs(10))
//!         .danger_at(Duration::from_secs(5)),
//!     vec![],
//! );
//! ```

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};
use std::time::Duration;

/// Time display format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimeFormat {
    /// Seconds only: "125"
    Seconds,
    /// Minutes and seconds: "2:05"
    #[default]
    MinSec,
    /// Minutes and seconds with padding: "02:05"
    MinSecPadded,
    /// Hours, minutes, seconds: "1:02:05"
    HourMinSec,
    /// Hours, minutes, seconds with padding: "01:02:05"
    HourMinSecPadded,
    /// With milliseconds: "2:05.123"
    MinSecMs,
    /// With tenths: "2:05.1"
    MinSecTenths,
    /// Compact human readable: "2m 5s"
    Human,
    /// Long human readable: "2 minutes, 5 seconds"
    HumanLong,
}

impl TimeFormat {
    /// Format a duration using this format.
    pub fn format(&self, duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;
        let millis = duration.subsec_millis();
        let tenths = millis / 100;

        match self {
            TimeFormat::Seconds => format!("{}", total_secs),
            TimeFormat::MinSec => format!("{}:{:02}", mins + hours * 60, secs),
            TimeFormat::MinSecPadded => format!("{:02}:{:02}", mins + hours * 60, secs),
            TimeFormat::HourMinSec => {
                if hours > 0 {
                    format!("{}:{:02}:{:02}", hours, mins, secs)
                } else {
                    format!("{}:{:02}", mins, secs)
                }
            }
            TimeFormat::HourMinSecPadded => format!("{:02}:{:02}:{:02}", hours, mins, secs),
            TimeFormat::MinSecMs => format!("{}:{:02}.{:03}", mins + hours * 60, secs, millis),
            TimeFormat::MinSecTenths => format!("{}:{:02}.{}", mins + hours * 60, secs, tenths),
            TimeFormat::Human => {
                if hours > 0 {
                    format!("{}h {}m {}s", hours, mins, secs)
                } else if mins > 0 {
                    format!("{}m {}s", mins, secs)
                } else {
                    format!("{}s", secs)
                }
            }
            TimeFormat::HumanLong => {
                let mut parts = Vec::new();
                if hours > 0 {
                    parts.push(if hours == 1 {
                        "1 hour".to_string()
                    } else {
                        format!("{} hours", hours)
                    });
                }
                if mins > 0 {
                    parts.push(if mins == 1 {
                        "1 minute".to_string()
                    } else {
                        format!("{} minutes", mins)
                    });
                }
                if secs > 0 || parts.is_empty() {
                    parts.push(if secs == 1 {
                        "1 second".to_string()
                    } else {
                        format!("{} seconds", secs)
                    });
                }
                parts.join(", ")
            }
        }
    }
}

/// Timer mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimerMode {
    /// Count up from zero (stopwatch).
    #[default]
    Stopwatch,
    /// Count down to zero (countdown).
    Countdown,
}

/// Properties for the Timer component.
#[derive(Debug, Clone)]
pub struct TimerProps {
    /// Current duration to display.
    pub duration: Duration,
    /// Timer mode (stopwatch or countdown).
    pub mode: TimerMode,
    /// Display format.
    pub format: TimeFormat,
    /// Default color.
    pub color: Option<Color>,
    /// Color when warn threshold is reached (countdown only).
    pub warn_color: Option<Color>,
    /// Color when danger threshold is reached (countdown only).
    pub danger_color: Option<Color>,
    /// Color when completed (countdown reached zero).
    pub complete_color: Option<Color>,
    /// Warn threshold (countdown only).
    pub warn_threshold: Option<Duration>,
    /// Danger threshold (countdown only).
    pub danger_threshold: Option<Duration>,
    /// Whether to show blinking when in danger zone.
    pub blink_on_danger: bool,
    /// Current blink state (for external control).
    pub blink_visible: bool,
    /// Optional prefix text.
    pub prefix: Option<String>,
    /// Optional suffix text.
    pub suffix: Option<String>,
    /// Bold text.
    pub bold: bool,
    /// Dim text.
    pub dim: bool,
}

impl Default for TimerProps {
    fn default() -> Self {
        Self {
            duration: Duration::ZERO,
            mode: TimerMode::Stopwatch,
            format: TimeFormat::MinSec,
            color: None,
            warn_color: Some(Color::Yellow),
            danger_color: Some(Color::Red),
            complete_color: Some(Color::Green),
            warn_threshold: None,
            danger_threshold: None,
            blink_on_danger: false,
            blink_visible: true,
            prefix: None,
            suffix: None,
            bold: false,
            dim: false,
        }
    }
}

impl TimerProps {
    /// Create a new stopwatch timer.
    pub fn stopwatch(elapsed: Duration) -> Self {
        Self {
            duration: elapsed,
            mode: TimerMode::Stopwatch,
            ..Default::default()
        }
    }

    /// Create a new countdown timer.
    pub fn countdown(remaining: Duration) -> Self {
        Self {
            duration: remaining,
            mode: TimerMode::Countdown,
            ..Default::default()
        }
    }

    /// Set the display format.
    #[must_use]
    pub fn format(mut self, format: TimeFormat) -> Self {
        self.format = format;
        self
    }

    /// Set the default color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the warn threshold and color (countdown only).
    #[must_use]
    pub fn warn_at(mut self, threshold: Duration) -> Self {
        self.warn_threshold = Some(threshold);
        self
    }

    /// Set the danger threshold and color (countdown only).
    #[must_use]
    pub fn danger_at(mut self, threshold: Duration) -> Self {
        self.danger_threshold = Some(threshold);
        self
    }

    /// Set the warn color.
    #[must_use]
    pub fn warn_color(mut self, color: Color) -> Self {
        self.warn_color = Some(color);
        self
    }

    /// Set the danger color.
    #[must_use]
    pub fn danger_color(mut self, color: Color) -> Self {
        self.danger_color = Some(color);
        self
    }

    /// Set the complete color.
    #[must_use]
    pub fn complete_color(mut self, color: Color) -> Self {
        self.complete_color = Some(color);
        self
    }

    /// Enable blinking when in danger zone.
    #[must_use]
    pub fn blink_on_danger(mut self, blink: bool) -> Self {
        self.blink_on_danger = blink;
        self
    }

    /// Set current blink visibility state.
    #[must_use]
    pub fn blink_visible(mut self, visible: bool) -> Self {
        self.blink_visible = visible;
        self
    }

    /// Set prefix text.
    #[must_use]
    pub fn prefix(mut self, text: impl Into<String>) -> Self {
        self.prefix = Some(text.into());
        self
    }

    /// Set suffix text.
    #[must_use]
    pub fn suffix(mut self, text: impl Into<String>) -> Self {
        self.suffix = Some(text.into());
        self
    }

    /// Make text bold.
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Make text dim.
    #[must_use]
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Check if timer is in warn zone.
    pub fn is_warn(&self) -> bool {
        if self.mode != TimerMode::Countdown {
            return false;
        }
        if let Some(threshold) = self.warn_threshold {
            self.duration <= threshold && self.duration > Duration::ZERO
        } else {
            false
        }
    }

    /// Check if timer is in danger zone.
    pub fn is_danger(&self) -> bool {
        if self.mode != TimerMode::Countdown {
            return false;
        }
        if let Some(threshold) = self.danger_threshold {
            self.duration <= threshold && self.duration > Duration::ZERO
        } else {
            false
        }
    }

    /// Check if countdown is complete.
    pub fn is_complete(&self) -> bool {
        self.mode == TimerMode::Countdown && self.duration == Duration::ZERO
    }
}

/// A component that displays elapsed or remaining time.
///
/// # Example
///
/// ```ignore
/// // Simple stopwatch
/// let timer = timer_display(elapsed, TimeFormat::MinSec);
///
/// // Countdown with thresholds
/// let timer = Element::node::<Timer>(
///     TimerProps::countdown(remaining)
///         .format(TimeFormat::MinSecTenths)
///         .warn_at(Duration::from_secs(30))
///         .danger_at(Duration::from_secs(10)),
///     vec![],
/// );
/// ```
pub struct Timer;

impl Component for Timer {
    type Props = TimerProps;

    fn render(props: &Self::Props) -> Element {
        // Determine color based on state
        let color = if props.is_complete() {
            props.complete_color
        } else if props.is_danger() {
            props.danger_color
        } else if props.is_warn() {
            props.warn_color
        } else {
            props.color
        };

        // Handle blinking in danger zone
        let should_hide = props.blink_on_danger && props.is_danger() && !props.blink_visible;

        // Format the time
        let time_str = props.format.format(props.duration);

        // Build full display string
        let display = if should_hide {
            // Show spaces to maintain width
            let width = time_str.len();
            " ".repeat(width)
        } else {
            time_str
        };

        // Add prefix/suffix
        let mut full_display = String::new();
        if let Some(ref prefix) = props.prefix {
            full_display.push_str(prefix);
        }
        full_display.push_str(&display);
        if let Some(ref suffix) = props.suffix {
            full_display.push_str(suffix);
        }

        // Build style
        let mut style = Style::new();
        if let Some(c) = color {
            style = style.fg(c);
        }
        if props.bold {
            style = style.add_modifier(Modifier::BOLD);
        }
        if props.dim {
            style = style.add_modifier(Modifier::DIM);
        }

        Element::styled_text(&full_display, style)
    }
}

/// Helper to create a simple timer display.
pub fn timer_display(duration: Duration, format: TimeFormat) -> Element {
    Timer::render(&TimerProps {
        duration,
        format,
        ..Default::default()
    })
}

/// Helper to create a stopwatch display.
pub fn stopwatch(elapsed: Duration) -> Element {
    Timer::render(&TimerProps::stopwatch(elapsed))
}

/// Helper to create a countdown display.
pub fn countdown(remaining: Duration) -> Element {
    Timer::render(&TimerProps::countdown(remaining))
}

/// Helper to create a countdown with color thresholds.
pub fn countdown_with_thresholds(
    remaining: Duration,
    warn_at: Duration,
    danger_at: Duration,
) -> Element {
    Timer::render(
        &TimerProps::countdown(remaining)
            .warn_at(warn_at)
            .danger_at(danger_at),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_format_seconds() {
        assert_eq!(TimeFormat::Seconds.format(Duration::from_secs(125)), "125");
    }

    #[test]
    fn test_time_format_min_sec() {
        assert_eq!(TimeFormat::MinSec.format(Duration::from_secs(125)), "2:05");
        assert_eq!(TimeFormat::MinSec.format(Duration::from_secs(65)), "1:05");
        assert_eq!(TimeFormat::MinSec.format(Duration::from_secs(5)), "0:05");
    }

    #[test]
    fn test_time_format_min_sec_padded() {
        assert_eq!(
            TimeFormat::MinSecPadded.format(Duration::from_secs(65)),
            "01:05"
        );
        assert_eq!(
            TimeFormat::MinSecPadded.format(Duration::from_secs(5)),
            "00:05"
        );
    }

    #[test]
    fn test_time_format_hour_min_sec() {
        assert_eq!(
            TimeFormat::HourMinSec.format(Duration::from_secs(3665)),
            "1:01:05"
        );
        assert_eq!(
            TimeFormat::HourMinSec.format(Duration::from_secs(125)),
            "2:05"
        );
    }

    #[test]
    fn test_time_format_hour_min_sec_padded() {
        assert_eq!(
            TimeFormat::HourMinSecPadded.format(Duration::from_secs(3665)),
            "01:01:05"
        );
        assert_eq!(
            TimeFormat::HourMinSecPadded.format(Duration::from_secs(125)),
            "00:02:05"
        );
    }

    #[test]
    fn test_time_format_with_ms() {
        let dur = Duration::from_millis(125_456);
        assert_eq!(TimeFormat::MinSecMs.format(dur), "2:05.456");
    }

    #[test]
    fn test_time_format_with_tenths() {
        let dur = Duration::from_millis(125_456);
        assert_eq!(TimeFormat::MinSecTenths.format(dur), "2:05.4");
    }

    #[test]
    fn test_time_format_human() {
        assert_eq!(
            TimeFormat::Human.format(Duration::from_secs(3665)),
            "1h 1m 5s"
        );
        assert_eq!(TimeFormat::Human.format(Duration::from_secs(125)), "2m 5s");
        assert_eq!(TimeFormat::Human.format(Duration::from_secs(5)), "5s");
    }

    #[test]
    fn test_time_format_human_long() {
        assert_eq!(
            TimeFormat::HumanLong.format(Duration::from_secs(3665)),
            "1 hour, 1 minute, 5 seconds"
        );
        assert_eq!(
            TimeFormat::HumanLong.format(Duration::from_secs(61)),
            "1 minute, 1 second"
        );
    }

    #[test]
    fn test_timer_props_stopwatch() {
        let props = TimerProps::stopwatch(Duration::from_secs(60));
        assert_eq!(props.mode, TimerMode::Stopwatch);
        assert_eq!(props.duration, Duration::from_secs(60));
    }

    #[test]
    fn test_timer_props_countdown() {
        let props = TimerProps::countdown(Duration::from_secs(30));
        assert_eq!(props.mode, TimerMode::Countdown);
        assert_eq!(props.duration, Duration::from_secs(30));
    }

    #[test]
    fn test_timer_is_warn() {
        let props = TimerProps::countdown(Duration::from_secs(8)).warn_at(Duration::from_secs(10));
        assert!(props.is_warn());

        let props = TimerProps::countdown(Duration::from_secs(15)).warn_at(Duration::from_secs(10));
        assert!(!props.is_warn());
    }

    #[test]
    fn test_timer_is_danger() {
        let props =
            TimerProps::countdown(Duration::from_secs(3)).danger_at(Duration::from_secs(5));
        assert!(props.is_danger());
    }

    #[test]
    fn test_timer_is_complete() {
        let props = TimerProps::countdown(Duration::ZERO);
        assert!(props.is_complete());

        let props = TimerProps::stopwatch(Duration::ZERO);
        assert!(!props.is_complete()); // Stopwatch doesn't complete
    }

    #[test]
    fn test_timer_render() {
        let props = TimerProps::stopwatch(Duration::from_secs(65));
        let elem = Timer::render(&props);
        assert!(elem.is_text());
    }

    #[test]
    fn test_timer_helpers() {
        let elem = stopwatch(Duration::from_secs(60));
        assert!(elem.is_text());

        let elem = countdown(Duration::from_secs(30));
        assert!(elem.is_text());

        let elem = timer_display(Duration::from_secs(90), TimeFormat::Human);
        assert!(elem.is_text());
    }

    #[test]
    fn test_timer_with_prefix_suffix() {
        let props = TimerProps::stopwatch(Duration::from_secs(60))
            .prefix("Time: ")
            .suffix(" elapsed");
        let elem = Timer::render(&props);
        match elem {
            Element::Text { content, .. } => {
                assert!(content.contains("Time:"));
                assert!(content.contains("elapsed"));
            }
            _ => panic!("Expected Text element"),
        }
    }
}
