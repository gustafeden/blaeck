//! Blink helpers - visibility toggling for animations.
//!
//! Provides helper functions for conditional rendering based on animation state.
//!
//! # Example
//!
//! ```ignore
//! use blaeck::prelude::*;
//!
//! let timer = AnimationTimer::new();
//!
//! // Simple blink - shows element or empty
//! let indicator = blink(timer.blink(500), Element::text("●"));
//!
//! // Blink with placeholder - maintains layout
//! let indicator = blink_or(
//!     timer.blink(500),
//!     Element::text("●"),
//!     Element::text(" "),
//! );
//!
//! // Animated indicator with built-in patterns
//! let indicator = animated_indicator(IndicatorStyle::BlinkingDot, &timer);
//! ```

use crate::animation::{AnimationTimer, BlinkPattern, IndicatorStyle};
use crate::element::Element;
use crate::style::{Color, Style};

/// Conditionally render an element based on visibility.
///
/// Returns the element when visible, or `Element::Empty` when hidden.
///
/// # Example
/// ```ignore
/// let dot = blink(timer.blink(500), Element::text("●"));
/// ```
pub fn blink(visible: bool, element: Element) -> Element {
    if visible {
        element
    } else {
        Element::Empty
    }
}

/// Conditionally render one of two elements based on visibility.
///
/// Returns `when_visible` when visible, or `when_hidden` when hidden.
/// Useful for maintaining layout width.
///
/// # Example
/// ```ignore
/// let dot = blink_or(
///     timer.blink(500),
///     Element::text("●"),  // When visible
///     Element::text(" "),  // When hidden (maintains width)
/// );
/// ```
pub fn blink_or(visible: bool, when_visible: Element, when_hidden: Element) -> Element {
    if visible {
        when_visible
    } else {
        when_hidden
    }
}

/// Render an animated indicator using a BlinkPattern.
///
/// # Example
/// ```ignore
/// let indicator = blink_pattern(BlinkPattern::Heartbeat, &timer, "●", " ");
/// ```
pub fn blink_pattern(
    pattern: BlinkPattern,
    timer: &AnimationTimer,
    visible_text: &str,
    hidden_text: &str,
) -> Element {
    if pattern.is_visible(timer) {
        Element::text(visible_text)
    } else {
        Element::text(hidden_text)
    }
}

/// Render an animated indicator using a built-in style.
///
/// Returns a Text element with the current frame of the indicator animation.
///
/// # Example
/// ```ignore
/// let spinner = animated_indicator(IndicatorStyle::SpinnerDots, &timer);
/// ```
pub fn animated_indicator(style: IndicatorStyle, timer: &AnimationTimer) -> Element {
    Element::text(style.render(timer))
}

/// Render an animated indicator with a specific color.
///
/// # Example
/// ```ignore
/// let spinner = animated_indicator_colored(
///     IndicatorStyle::SpinnerDots,
///     &timer,
///     Color::Cyan,
/// );
/// ```
pub fn animated_indicator_colored(
    style: IndicatorStyle,
    timer: &AnimationTimer,
    color: Color,
) -> Element {
    Element::styled_text(style.render(timer), Style::new().fg(color))
}

/// Render a blinking dot indicator.
///
/// Shorthand for `blink_or(timer.blink(interval), Element::text("●"), Element::text(" "))`.
///
/// # Example
/// ```ignore
/// let dot = blinking_dot(&timer, 500, Color::Green);
/// ```
pub fn blinking_dot(timer: &AnimationTimer, interval_ms: u64, color: Color) -> Element {
    if timer.blink(interval_ms) {
        Element::styled_text("●", Style::new().fg(color))
    } else {
        Element::text(" ")
    }
}

/// Render a pulsing dot that alternates between filled and empty.
///
/// # Example
/// ```ignore
/// let dot = pulsing_dot(&timer, 500, Color::Yellow);
/// ```
pub fn pulsing_dot(timer: &AnimationTimer, interval_ms: u64, color: Color) -> Element {
    if timer.blink(interval_ms) {
        Element::styled_text("●", Style::new().fg(color))
    } else {
        Element::styled_text("○", Style::new().fg(color))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blink_visible() {
        let elem = blink(true, Element::text("test"));
        assert!(elem.is_text());
    }

    #[test]
    fn test_blink_hidden() {
        let elem = blink(false, Element::text("test"));
        assert!(elem.is_empty());
    }

    #[test]
    fn test_blink_or_visible() {
        let elem = blink_or(true, Element::text("on"), Element::text("off"));
        match elem {
            Element::Text { content, .. } => assert_eq!(content, "on"),
            _ => panic!("Expected Text"),
        }
    }

    #[test]
    fn test_blink_or_hidden() {
        let elem = blink_or(false, Element::text("on"), Element::text("off"));
        match elem {
            Element::Text { content, .. } => assert_eq!(content, "off"),
            _ => panic!("Expected Text"),
        }
    }

    #[test]
    fn test_animated_indicator() {
        let timer = AnimationTimer::new();
        let elem = animated_indicator(IndicatorStyle::SpinnerDots, &timer);
        assert!(elem.is_text());
    }

    #[test]
    fn test_animated_indicator_colored() {
        let timer = AnimationTimer::new();
        let elem = animated_indicator_colored(IndicatorStyle::SpinnerDots, &timer, Color::Cyan);
        assert!(elem.is_text());
    }

    #[test]
    fn test_blinking_dot() {
        let timer = AnimationTimer::new();
        let elem = blinking_dot(&timer, 500, Color::Green);
        assert!(elem.is_text());
    }

    #[test]
    fn test_pulsing_dot() {
        let timer = AnimationTimer::new();
        let elem = pulsing_dot(&timer, 500, Color::Yellow);
        assert!(elem.is_text());
    }

    #[test]
    fn test_blink_pattern() {
        let timer = AnimationTimer::new();
        let elem = blink_pattern(BlinkPattern::Standard, &timer, "●", " ");
        assert!(elem.is_text());
    }
}
