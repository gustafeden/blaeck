//! Animation utilities for Blaeck.
//!
//! This module provides tools for creating animations in terminal UIs:
//! - `AnimationTimer` for tracking elapsed time and calculating frames
//! - Easing functions for smooth transitions
//! - Frame helpers for common patterns (blink, cycle, pulse)
//!
//! # Example
//!
//! ```ignore
//! use blaeck::animation::{AnimationTimer, Easing};
//!
//! let timer = AnimationTimer::new();
//!
//! // In render loop:
//! let blink_visible = timer.blink(500); // Toggle every 500ms
//! let frame = timer.cycle(4, 200);      // Cycle through 4 frames, 200ms each
//! let progress = timer.progress(1000, Easing::EaseInOut); // 0.0 to 1.0 over 1s
//! ```

use std::time::{Duration, Instant};

/// Timer for tracking animation state.
///
/// Create once at the start of your animation and query it each frame
/// to get the current animation state.
#[derive(Debug, Clone)]
pub struct AnimationTimer {
    start: Instant,
}

impl Default for AnimationTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationTimer {
    /// Create a new animation timer starting now.
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// Create a timer with a specific start time.
    pub fn with_start(start: Instant) -> Self {
        Self { start }
    }

    /// Get elapsed time since timer started.
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Get elapsed time in milliseconds.
    pub fn elapsed_ms(&self) -> u128 {
        self.start.elapsed().as_millis()
    }

    /// Reset the timer to now.
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }

    /// Returns true/false alternating at the given interval.
    ///
    /// # Arguments
    /// * `interval_ms` - Time in milliseconds for each on/off phase
    ///
    /// # Example
    /// ```ignore
    /// let visible = timer.blink(500); // Toggle every 500ms
    /// ```
    pub fn blink(&self, interval_ms: u64) -> bool {
        (self.elapsed_ms() / interval_ms as u128).is_multiple_of(2)
    }

    /// Returns true/false with separate on and off durations.
    ///
    /// # Arguments
    /// * `on_ms` - Time visible in milliseconds
    /// * `off_ms` - Time hidden in milliseconds
    ///
    /// # Example
    /// ```ignore
    /// let visible = timer.blink_asymmetric(800, 200); // On 800ms, off 200ms
    /// ```
    pub fn blink_asymmetric(&self, on_ms: u64, off_ms: u64) -> bool {
        let cycle = on_ms + off_ms;
        let pos = (self.elapsed_ms() % cycle as u128) as u64;
        pos < on_ms
    }

    /// Cycle through N frames at the given interval.
    ///
    /// # Arguments
    /// * `frame_count` - Number of frames to cycle through
    /// * `interval_ms` - Time per frame in milliseconds
    ///
    /// # Returns
    /// Frame index (0 to frame_count - 1)
    ///
    /// # Example
    /// ```ignore
    /// let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    /// let frame = timer.cycle(frames.len(), 80);
    /// let spinner = frames[frame];
    /// ```
    pub fn cycle(&self, frame_count: usize, interval_ms: u64) -> usize {
        if frame_count == 0 {
            return 0;
        }
        ((self.elapsed_ms() / interval_ms as u128) % frame_count as u128) as usize
    }

    /// Get progress from 0.0 to 1.0 over the given duration.
    ///
    /// # Arguments
    /// * `duration_ms` - Total duration in milliseconds
    /// * `easing` - Easing function to apply
    ///
    /// # Returns
    /// Progress value from 0.0 to 1.0, clamped at 1.0 after duration
    ///
    /// # Example
    /// ```ignore
    /// let progress = timer.progress(1000, Easing::EaseInOut);
    /// let width = (progress * max_width as f64) as usize;
    /// ```
    pub fn progress(&self, duration_ms: u64, easing: Easing) -> f64 {
        let elapsed = self.elapsed_ms() as f64;
        let duration = duration_ms as f64;
        let t = (elapsed / duration).min(1.0);
        easing.apply(t)
    }

    /// Get looping progress from 0.0 to 1.0, repeating.
    ///
    /// # Arguments
    /// * `duration_ms` - Duration of one cycle in milliseconds
    /// * `easing` - Easing function to apply
    ///
    /// # Example
    /// ```ignore
    /// let pulse = timer.progress_loop(1000, Easing::EaseInOut);
    /// ```
    pub fn progress_loop(&self, duration_ms: u64, easing: Easing) -> f64 {
        let elapsed = self.elapsed_ms() as f64;
        let duration = duration_ms as f64;
        let t = (elapsed % duration) / duration;
        easing.apply(t)
    }

    /// Get ping-pong progress (0 -> 1 -> 0 -> 1...).
    ///
    /// # Arguments
    /// * `duration_ms` - Duration of one direction (half cycle)
    /// * `easing` - Easing function to apply
    ///
    /// # Example
    /// ```ignore
    /// let pulse = timer.progress_pingpong(500, Easing::EaseInOut);
    /// // Goes 0->1 over 500ms, then 1->0 over 500ms, repeating
    /// ```
    pub fn progress_pingpong(&self, duration_ms: u64, easing: Easing) -> f64 {
        let elapsed = self.elapsed_ms() as f64;
        let duration = duration_ms as f64;
        let cycle = duration * 2.0;
        let pos = elapsed % cycle;
        let t = if pos < duration {
            pos / duration
        } else {
            1.0 - ((pos - duration) / duration)
        };
        easing.apply(t)
    }

    /// Check if a duration has elapsed.
    pub fn is_elapsed(&self, duration_ms: u64) -> bool {
        self.elapsed_ms() >= duration_ms as u128
    }
}

/// Easing functions for smooth animations.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Easing {
    /// Linear interpolation (no easing).
    #[default]
    Linear,
    /// Slow start, fast end.
    EaseIn,
    /// Fast start, slow end.
    EaseOut,
    /// Slow start and end, fast middle.
    EaseInOut,
    /// Quadratic ease in.
    EaseInQuad,
    /// Quadratic ease out.
    EaseOutQuad,
    /// Quadratic ease in and out.
    EaseInOutQuad,
    /// Cubic ease in.
    EaseInCubic,
    /// Cubic ease out.
    EaseOutCubic,
    /// Cubic ease in and out.
    EaseInOutCubic,
    /// Elastic bounce at end.
    EaseOutElastic,
    /// Bounce at end.
    EaseOutBounce,
}

impl Easing {
    /// Apply the easing function to a value t in range [0, 1].
    pub fn apply(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t * t,
            Easing::EaseOut => 1.0 - (1.0 - t).powi(3),
            Easing::EaseInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Easing::EaseInQuad => t * t,
            Easing::EaseOutQuad => 1.0 - (1.0 - t).powi(2),
            Easing::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Easing::EaseInCubic => t * t * t,
            Easing::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            Easing::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Easing::EaseOutElastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let c4 = (2.0 * std::f64::consts::PI) / 3.0;
                    2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
                }
            }
            Easing::EaseOutBounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    let t = t - 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    let t = t - 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
        }
    }

    /// Interpolate between two values using this easing.
    pub fn interpolate(&self, from: f64, to: f64, t: f64) -> f64 {
        let eased_t = self.apply(t);
        from + (to - from) * eased_t
    }
}

/// Interpolate between two u8 values (useful for colors).
pub fn lerp_u8(from: u8, to: u8, t: f64) -> u8 {
    let t = t.clamp(0.0, 1.0);
    (from as f64 + (to as f64 - from as f64) * t).round() as u8
}

/// Interpolate between two RGB colors.
pub fn lerp_rgb(from: (u8, u8, u8), to: (u8, u8, u8), t: f64) -> (u8, u8, u8) {
    (
        lerp_u8(from.0, to.0, t),
        lerp_u8(from.1, to.1, t),
        lerp_u8(from.2, to.2, t),
    )
}

/// Built-in blink patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlinkPattern {
    /// Standard 500ms on/off blink.
    Standard,
    /// Fast 250ms blink.
    Fast,
    /// Slow 1000ms blink.
    Slow,
    /// Quick pulse: 200ms on, 800ms off.
    Pulse,
    /// Heartbeat: two quick blinks then pause.
    Heartbeat,
}

impl BlinkPattern {
    /// Get whether visible at current time.
    pub fn is_visible(&self, timer: &AnimationTimer) -> bool {
        match self {
            BlinkPattern::Standard => timer.blink(500),
            BlinkPattern::Fast => timer.blink(250),
            BlinkPattern::Slow => timer.blink(1000),
            BlinkPattern::Pulse => timer.blink_asymmetric(200, 800),
            BlinkPattern::Heartbeat => {
                // Two quick blinks (100ms on, 100ms off) then 600ms pause
                let cycle_ms = 1000u64;
                let pos = (timer.elapsed_ms() % cycle_ms as u128) as u64;
                matches!(pos, 0..=99 | 200..=299)
            }
        }
    }
}

/// Built-in indicator styles that animate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicatorStyle {
    /// Blinking dot: ● / (space)
    BlinkingDot,
    /// Pulsing dot: ● / ○
    PulsingDot,
    /// Spinning dots: ⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏
    SpinnerDots,
    /// Spinning line: |/-\
    SpinnerLine,
    /// Bouncing bar: [=   ] -> [  = ]
    BouncingBar,
    /// Growing dots: . -> .. -> ...
    GrowingDots,
}

impl IndicatorStyle {
    /// Get the indicator string at current time.
    pub fn render(&self, timer: &AnimationTimer) -> &'static str {
        match self {
            IndicatorStyle::BlinkingDot => {
                if timer.blink(500) {
                    "●"
                } else {
                    " "
                }
            }
            IndicatorStyle::PulsingDot => {
                if timer.blink(500) {
                    "●"
                } else {
                    "○"
                }
            }
            IndicatorStyle::SpinnerDots => {
                const FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
                FRAMES[timer.cycle(10, 80)]
            }
            IndicatorStyle::SpinnerLine => {
                const FRAMES: [&str; 4] = ["|", "/", "-", "\\"];
                FRAMES[timer.cycle(4, 100)]
            }
            IndicatorStyle::BouncingBar => {
                const FRAMES: [&str; 8] = [
                    "[=   ]", "[ =  ]", "[  = ]", "[   =]", "[  = ]", "[ =  ]", "[=   ]", "[=   ]",
                ];
                FRAMES[timer.cycle(8, 150)]
            }
            IndicatorStyle::GrowingDots => {
                const FRAMES: [&str; 4] = ["", ".", "..", "..."];
                FRAMES[timer.cycle(4, 400)]
            }
        }
    }

    /// Get the recommended tick interval for this style in milliseconds.
    pub fn tick_interval_ms(&self) -> u64 {
        match self {
            IndicatorStyle::BlinkingDot => 100,
            IndicatorStyle::PulsingDot => 100,
            IndicatorStyle::SpinnerDots => 80,
            IndicatorStyle::SpinnerLine => 100,
            IndicatorStyle::BouncingBar => 150,
            IndicatorStyle::GrowingDots => 400,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_animation_timer_new() {
        let timer = AnimationTimer::new();
        assert!(timer.elapsed_ms() < 100);
    }

    #[test]
    fn test_animation_timer_elapsed() {
        let timer = AnimationTimer::new();
        thread::sleep(Duration::from_millis(50));
        assert!(timer.elapsed_ms() >= 50);
    }

    #[test]
    fn test_blink() {
        let timer = AnimationTimer::new();
        // At start, should be visible
        assert!(timer.blink(500));
    }

    #[test]
    fn test_cycle() {
        let timer = AnimationTimer::new();
        let frame = timer.cycle(4, 100);
        assert!(frame < 4);
    }

    #[test]
    fn test_cycle_zero_frames() {
        let timer = AnimationTimer::new();
        assert_eq!(timer.cycle(0, 100), 0);
    }

    #[test]
    fn test_progress() {
        let timer = AnimationTimer::new();
        let progress = timer.progress(1000, Easing::Linear);
        assert!((0.0..=1.0).contains(&progress));
    }

    #[test]
    fn test_easing_linear() {
        assert_eq!(Easing::Linear.apply(0.0), 0.0);
        assert_eq!(Easing::Linear.apply(0.5), 0.5);
        assert_eq!(Easing::Linear.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_in() {
        assert_eq!(Easing::EaseIn.apply(0.0), 0.0);
        assert!(Easing::EaseIn.apply(0.5) < 0.5); // Should be slower at start
        assert_eq!(Easing::EaseIn.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_out() {
        assert_eq!(Easing::EaseOut.apply(0.0), 0.0);
        assert!(Easing::EaseOut.apply(0.5) > 0.5); // Should be faster at start
        assert_eq!(Easing::EaseOut.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_clamp() {
        assert_eq!(Easing::Linear.apply(-0.5), 0.0);
        assert_eq!(Easing::Linear.apply(1.5), 1.0);
    }

    #[test]
    fn test_interpolate() {
        assert_eq!(Easing::Linear.interpolate(0.0, 100.0, 0.5), 50.0);
        assert_eq!(Easing::Linear.interpolate(10.0, 20.0, 0.0), 10.0);
        assert_eq!(Easing::Linear.interpolate(10.0, 20.0, 1.0), 20.0);
    }

    #[test]
    fn test_lerp_u8() {
        assert_eq!(lerp_u8(0, 255, 0.0), 0);
        assert_eq!(lerp_u8(0, 255, 1.0), 255);
        assert_eq!(lerp_u8(0, 255, 0.5), 128);
    }

    #[test]
    fn test_lerp_rgb() {
        assert_eq!(lerp_rgb((0, 0, 0), (255, 255, 255), 0.5), (128, 128, 128));
    }

    #[test]
    fn test_blink_pattern_standard() {
        let timer = AnimationTimer::new();
        // Just verify it doesn't panic
        let _ = BlinkPattern::Standard.is_visible(&timer);
    }

    #[test]
    fn test_indicator_style_render() {
        let timer = AnimationTimer::new();
        // Verify all styles render something
        // BlinkingDot may be empty depending on timing, so we just verify it doesn't panic
        let _ = IndicatorStyle::BlinkingDot.render(&timer);
        assert!(!IndicatorStyle::SpinnerDots.render(&timer).is_empty());
        assert!(!IndicatorStyle::SpinnerLine.render(&timer).is_empty());
    }

    #[test]
    fn test_indicator_tick_interval() {
        assert!(IndicatorStyle::SpinnerDots.tick_interval_ms() > 0);
        assert!(IndicatorStyle::BlinkingDot.tick_interval_ms() > 0);
    }

    #[test]
    fn test_progress_pingpong() {
        let timer = AnimationTimer::new();
        let value = timer.progress_pingpong(500, Easing::Linear);
        assert!((0.0..=1.0).contains(&value));
    }
}
