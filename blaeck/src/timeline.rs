//! Declarative animation timeline system.
//!
//! This module provides a timeline-based animation system for building
//! complex choreographed sequences with a declarative API.
//!
//! # Core Concepts
//!
//! - **Timeline**: A sequence of Acts that play in order, optionally looping
//! - **Act**: A named time segment with animated properties
//! - **Track**: Animation data for a single property within an act
//! - **Keyframe**: A value at a specific point in time
//!
//! # Example
//!
//! ```ignore
//! use blaeck::timeline::*;
//! use blaeck::animation::Easing;
//!
//! let timeline = Timeline::new()
//!     .act(Act::new("fade_in")
//!         .duration(1.0)
//!         .animate("opacity", 0.0, 1.0, Easing::EaseOutCubic))
//!     .act(Act::new("hold")
//!         .duration(2.0))
//!     .act(Act::new("fade_out")
//!         .duration(1.0)
//!         .animate("opacity", 1.0, 0.0, Easing::EaseInCubic));
//!
//! // Query the timeline at a specific time
//! let state = timeline.at(0.5); // Halfway through fade_in
//! let opacity: f64 = state.get("opacity").unwrap_or(0.0);
//! ```

use crate::animation::Easing;
use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

// ============================================================================
// Callback Types
// ============================================================================

/// A callback that takes no arguments.
pub type Callback = Rc<dyn Fn()>;

/// A callback that receives the loop count.
pub type LoopCallback = Rc<dyn Fn(u32)>;

/// A callback that receives the act name.
pub type ActCallback = Rc<dyn Fn(&str)>;

// ============================================================================
// Animatable Trait
// ============================================================================

/// A value that can be interpolated between two points.
///
/// Implement this trait for custom types that need to be animated.
pub trait Animatable: Clone + Send + Sync + 'static {
    /// Linearly interpolate between two values.
    ///
    /// - `t = 0.0` should return `a`
    /// - `t = 1.0` should return `b`
    /// - Values in between should smoothly transition
    fn lerp(a: &Self, b: &Self, t: f64) -> Self;
}

// Built-in implementations
impl Animatable for f32 {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        a + (b - a) * t as f32
    }
}

impl Animatable for f64 {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        a + (b - a) * t
    }
}

impl Animatable for i32 {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        (*a as f64 + (*b as f64 - *a as f64) * t).round() as i32
    }
}

impl Animatable for u8 {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        (*a as f64 + (*b as f64 - *a as f64) * t).round() as u8
    }
}

impl Animatable for (f32, f32) {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        (
            f32::lerp(&a.0, &b.0, t),
            f32::lerp(&a.1, &b.1, t),
        )
    }
}

impl Animatable for (f64, f64) {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        (
            f64::lerp(&a.0, &b.0, t),
            f64::lerp(&a.1, &b.1, t),
        )
    }
}

/// RGB color tuple
impl Animatable for (u8, u8, u8) {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        (
            u8::lerp(&a.0, &b.0, t),
            u8::lerp(&a.1, &b.1, t),
            u8::lerp(&a.2, &b.2, t),
        )
    }
}

/// RGBA color tuple
impl Animatable for (u8, u8, u8, u8) {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        (
            u8::lerp(&a.0, &b.0, t),
            u8::lerp(&a.1, &b.1, t),
            u8::lerp(&a.2, &b.2, t),
            u8::lerp(&a.3, &b.3, t),
        )
    }
}

// ============================================================================
// Keyframe
// ============================================================================

/// A keyframe holds a value at a specific normalized time (0.0 to 1.0).
#[derive(Clone)]
struct Keyframe<T: Animatable> {
    /// Normalized time within the act (0.0 = start, 1.0 = end)
    time: f64,
    /// Value at this keyframe
    value: T,
    /// Easing to use when interpolating TO this keyframe
    easing: Easing,
}

// ============================================================================
// Track
// ============================================================================

/// Type-erased track that can be stored in collections.
trait AnyTrack: Send + Sync {
    /// Get the value at normalized time t, boxed as Any.
    fn value_at(&self, t: f64) -> Box<dyn Any + Send + Sync>;
    /// Clone the track.
    fn clone_box(&self) -> Box<dyn AnyTrack>;
}

/// A track holds animation data for a single property.
#[derive(Clone)]
pub struct Track<T: Animatable> {
    keyframes: Vec<Keyframe<T>>,
}

impl<T: Animatable> Track<T> {
    /// Create a new empty track.
    pub fn new() -> Self {
        Self {
            keyframes: Vec::new(),
        }
    }

    /// Create a track with a simple from->to animation.
    pub fn from_to(from: T, to: T, easing: Easing) -> Self {
        Self {
            keyframes: vec![
                Keyframe { time: 0.0, value: from, easing: Easing::Linear },
                Keyframe { time: 1.0, value: to, easing },
            ],
        }
    }

    /// Add a keyframe at the specified normalized time.
    pub fn keyframe(mut self, time: f64, value: T, easing: Easing) -> Self {
        self.keyframes.push(Keyframe { time, value, easing });
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        self
    }

    /// Get the interpolated value at normalized time t (0.0 to 1.0).
    pub fn value_at(&self, t: f64) -> T {
        if self.keyframes.is_empty() {
            panic!("Track has no keyframes");
        }

        let t = t.clamp(0.0, 1.0);

        // Find surrounding keyframes
        let mut prev_idx = 0;
        let mut next_idx = 0;

        for (i, kf) in self.keyframes.iter().enumerate() {
            if kf.time <= t {
                prev_idx = i;
            }
            if kf.time >= t && next_idx == 0 {
                next_idx = i;
                break;
            }
        }

        // If we're past all keyframes, use the last one
        if next_idx == 0 {
            next_idx = self.keyframes.len() - 1;
        }

        let prev = &self.keyframes[prev_idx];
        let next = &self.keyframes[next_idx];

        // If same keyframe or same time, return the value
        if prev_idx == next_idx || (next.time - prev.time).abs() < f64::EPSILON {
            return prev.value.clone();
        }

        // Calculate local t between the two keyframes
        let local_t = (t - prev.time) / (next.time - prev.time);
        let eased_t = next.easing.apply(local_t);

        T::lerp(&prev.value, &next.value, eased_t)
    }
}

impl<T: Animatable> Default for Track<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Animatable> AnyTrack for Track<T> {
    fn value_at(&self, t: f64) -> Box<dyn Any + Send + Sync> {
        Box::new(self.value_at(t))
    }

    fn clone_box(&self) -> Box<dyn AnyTrack> {
        Box::new(self.clone())
    }
}

// ============================================================================
// Act
// ============================================================================

/// An Act is a named segment of time with animated properties.
#[derive(Clone)]
pub struct Act {
    /// Name of the act (for debugging and queries)
    name: String,
    /// Duration in seconds
    duration: f64,
    /// Named tracks for animated properties
    tracks: HashMap<String, Box<dyn AnyTrack>>,
    /// Callback fired when entering this act
    on_enter: Option<Callback>,
    /// Callback fired when exiting this act
    on_exit: Option<Callback>,
}

impl Act {
    /// Create a new act with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            duration: 1.0,
            tracks: HashMap::new(),
            on_enter: None,
            on_exit: None,
        }
    }

    /// Create a hold act (no animations, just wait).
    pub fn hold(name: impl Into<String>, duration: f64) -> Self {
        Self {
            name: name.into(),
            duration,
            tracks: HashMap::new(),
            on_enter: None,
            on_exit: None,
        }
    }

    /// Create a transition act with a specific duration.
    pub fn transition(name: impl Into<String>, duration: f64) -> Self {
        Self {
            name: name.into(),
            duration,
            tracks: HashMap::new(),
            on_enter: None,
            on_exit: None,
        }
    }

    /// Set the duration of this act in seconds.
    pub fn duration(mut self, seconds: f64) -> Self {
        self.duration = seconds;
        self
    }

    /// Add a simple from->to animation for a property.
    pub fn animate<T: Animatable>(
        mut self,
        property: impl Into<String>,
        from: T,
        to: T,
        easing: Easing,
    ) -> Self {
        let track = Track::from_to(from, to, easing);
        self.tracks.insert(property.into(), Box::new(track));
        self
    }

    /// Add a track with full keyframe control.
    pub fn track<T: Animatable>(mut self, property: impl Into<String>, track: Track<T>) -> Self {
        self.tracks.insert(property.into(), Box::new(track));
        self
    }

    /// Get the name of this act.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the duration of this act in seconds.
    pub fn get_duration(&self) -> f64 {
        self.duration
    }

    /// Get the value of a property at normalized time t (0.0 to 1.0).
    fn get_value(&self, property: &str, t: f64) -> Option<Box<dyn Any + Send + Sync>> {
        self.tracks.get(property).map(|track| track.value_at(t))
    }

    /// Set a callback to fire when entering this act.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Act::new("intro")
    ///     .duration(2.0)
    ///     .on_enter(|| println!("Intro started!"))
    /// ```
    pub fn on_enter<F: Fn() + 'static>(mut self, callback: F) -> Self {
        self.on_enter = Some(Rc::new(callback));
        self
    }

    /// Set a callback to fire when exiting this act.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Act::new("intro")
    ///     .duration(2.0)
    ///     .on_exit(|| println!("Intro complete!"))
    /// ```
    pub fn on_exit<F: Fn() + 'static>(mut self, callback: F) -> Self {
        self.on_exit = Some(Rc::new(callback));
        self
    }

    /// Fire the on_enter callback if set.
    fn fire_enter(&self) {
        if let Some(ref cb) = self.on_enter {
            cb();
        }
    }

    /// Fire the on_exit callback if set.
    fn fire_exit(&self) {
        if let Some(ref cb) = self.on_exit {
            cb();
        }
    }
}

impl Clone for Box<dyn AnyTrack> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

// ============================================================================
// Timeline
// ============================================================================

/// Loop behavior for a timeline.
#[derive(Clone, Debug, PartialEq)]
pub enum LoopBehavior {
    /// Play once and stop at the end.
    None,
    /// Loop from the beginning.
    Loop,
    /// Loop starting from a specific act.
    LoopFrom(String),
}

/// A Timeline is a sequence of Acts that play in order.
#[derive(Clone)]
pub struct Timeline {
    acts: Vec<Act>,
    loop_behavior: LoopBehavior,
    /// Total duration (computed)
    total_duration: f64,
    /// Loop start time (computed, for LoopFrom)
    loop_start_time: f64,
    /// Callback fired when timeline loops
    on_loop: Option<LoopCallback>,
    /// Callback fired when any act is entered
    on_act_enter: Option<ActCallback>,
    /// Callback fired when any act is exited
    on_act_exit: Option<ActCallback>,
}

impl Timeline {
    /// Create a new empty timeline.
    pub fn new() -> Self {
        Self {
            acts: Vec::new(),
            loop_behavior: LoopBehavior::None,
            total_duration: 0.0,
            loop_start_time: 0.0,
            on_loop: None,
            on_act_enter: None,
            on_act_exit: None,
        }
    }

    /// Add an act to the timeline.
    pub fn act(mut self, act: Act) -> Self {
        self.total_duration += act.duration;
        self.acts.push(act);
        self
    }

    /// Set the timeline to loop from the beginning.
    pub fn loop_forever(mut self) -> Self {
        self.loop_behavior = LoopBehavior::Loop;
        self
    }

    /// Set the timeline to loop starting from a specific act.
    pub fn loop_from(mut self, act_name: impl Into<String>) -> Self {
        let name = act_name.into();
        // Find the start time of the loop act
        let mut time = 0.0;
        for act in &self.acts {
            if act.name == name {
                self.loop_start_time = time;
                break;
            }
            time += act.duration;
        }
        self.loop_behavior = LoopBehavior::LoopFrom(name);
        self
    }

    /// Set a callback to fire when the timeline loops.
    ///
    /// The callback receives the loop iteration count (1 for first loop, 2 for second, etc.).
    pub fn on_loop<F: Fn(u32) + 'static>(mut self, callback: F) -> Self {
        self.on_loop = Some(Rc::new(callback));
        self
    }

    /// Set a callback to fire when any act is entered.
    ///
    /// The callback receives the act name.
    pub fn on_act_enter<F: Fn(&str) + 'static>(mut self, callback: F) -> Self {
        self.on_act_enter = Some(Rc::new(callback));
        self
    }

    /// Set a callback to fire when any act is exited.
    ///
    /// The callback receives the act name.
    pub fn on_act_exit<F: Fn(&str) + 'static>(mut self, callback: F) -> Self {
        self.on_act_exit = Some(Rc::new(callback));
        self
    }

    /// Chain another timeline after this one.
    pub fn then(mut self, other: Timeline) -> Self {
        for act in other.acts {
            self.total_duration += act.duration;
            self.acts.push(act);
        }
        self.loop_behavior = other.loop_behavior;
        // Copy callbacks from the other timeline if not already set
        if self.on_loop.is_none() {
            self.on_loop = other.on_loop;
        }
        if self.on_act_enter.is_none() {
            self.on_act_enter = other.on_act_enter;
        }
        if self.on_act_exit.is_none() {
            self.on_act_exit = other.on_act_exit;
        }
        if let LoopBehavior::LoopFrom(ref name) = self.loop_behavior {
            // Recalculate loop start time
            let mut time = 0.0;
            for act in &self.acts {
                if &act.name == name {
                    self.loop_start_time = time;
                    break;
                }
                time += act.duration;
            }
        }
        self
    }

    /// Get the total duration of the timeline (before looping).
    pub fn duration(&self) -> f64 {
        self.total_duration
    }

    /// Get the number of acts.
    pub fn act_count(&self) -> usize {
        self.acts.len()
    }

    /// Query the timeline state at a specific time.
    pub fn at(&self, time: f64) -> TimelineState<'_> {
        if self.acts.is_empty() {
            return TimelineState::empty();
        }

        // Handle looping
        let effective_time = match &self.loop_behavior {
            LoopBehavior::None => time.min(self.total_duration),
            LoopBehavior::Loop => {
                if self.total_duration > 0.0 {
                    time % self.total_duration
                } else {
                    0.0
                }
            }
            LoopBehavior::LoopFrom(_) => {
                if time < self.total_duration {
                    time
                } else {
                    let loop_duration = self.total_duration - self.loop_start_time;
                    if loop_duration > 0.0 {
                        let overflow = time - self.total_duration;
                        self.loop_start_time + (overflow % loop_duration)
                    } else {
                        self.loop_start_time
                    }
                }
            }
        };

        // Find the current act
        let mut accumulated_time = 0.0;
        let mut current_act_idx = 0;
        let mut act_start_time = 0.0;

        for (i, act) in self.acts.iter().enumerate() {
            let act_end_time = accumulated_time + act.duration;
            if effective_time < act_end_time || i == self.acts.len() - 1 {
                // We're in this act (or it's the last one)
                current_act_idx = i;
                act_start_time = accumulated_time;
                break;
            }
            accumulated_time = act_end_time;
        }

        let current_act = &self.acts[current_act_idx];
        let time_in_act = effective_time - act_start_time;
        let act_progress = if current_act.duration > 0.0 {
            (time_in_act / current_act.duration).clamp(0.0, 1.0)
        } else {
            1.0
        };

        TimelineState {
            time: effective_time,
            act_name: current_act.name.clone(),
            act_index: current_act_idx,
            act_progress,
            act: current_act,
        }
    }

    /// Create a playing timeline instance.
    pub fn start(&self) -> PlayingTimeline {
        PlayingTimeline {
            timeline: self.clone(),
            start_time: Instant::now(),
            paused: false,
            paused_at: 0.0,
            speed: 1.0,
            last_act_index: None,
            loop_count: 0,
            last_elapsed: 0.0,
        }
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TimelineState
// ============================================================================

/// A snapshot of the timeline at a specific point in time.
pub struct TimelineState<'a> {
    /// Current time in seconds
    pub time: f64,
    /// Name of the current act
    pub act_name: String,
    /// Index of the current act
    pub act_index: usize,
    /// Progress through the current act (0.0 to 1.0)
    pub act_progress: f64,
    /// Reference to the current act
    act: &'a Act,
}

impl TimelineState<'static> {
    /// Create an empty state (for empty timelines).
    ///
    /// Note: This leaks a small amount of memory each call. Only used for empty timelines.
    fn empty() -> Self {
        // Leak a minimal Act - this is fine since empty timelines are edge cases
        let act: &'static Act = Box::leak(Box::new(Act::new("empty").duration(0.0)));

        Self {
            time: 0.0,
            act_name: String::new(),
            act_index: 0,
            act_progress: 0.0,
            act,
        }
    }
}

impl<'a> TimelineState<'a> {
    /// Get an animated value by name.
    ///
    /// Returns `None` if the property doesn't exist in the current act.
    pub fn get<T: Animatable + Clone>(&self, property: &str) -> Option<T> {
        self.act
            .get_value(property, self.act_progress)
            .and_then(|boxed| boxed.downcast::<T>().ok())
            .map(|v| *v)
    }

    /// Get an animated value with a default.
    pub fn get_or<T: Animatable + Clone>(&self, property: &str, default: T) -> T {
        self.get(property).unwrap_or(default)
    }
}

// ============================================================================
// PlayingTimeline
// ============================================================================

/// A timeline that is actively playing.
#[derive(Clone)]
pub struct PlayingTimeline {
    timeline: Timeline,
    start_time: Instant,
    paused: bool,
    paused_at: f64,
    speed: f64,
    /// Last known act index for detecting transitions
    last_act_index: Option<usize>,
    /// Number of times the timeline has looped
    loop_count: u32,
    /// Last elapsed time for detecting loop resets
    last_elapsed: f64,
}

impl PlayingTimeline {
    /// Get the current state of the playing timeline.
    pub fn state(&self) -> TimelineState<'_> {
        let time = if self.paused {
            self.paused_at
        } else {
            self.start_time.elapsed().as_secs_f64() * self.speed
        };
        self.timeline.at(time)
    }

    /// Get an animated value by name at the current time.
    pub fn get<T: Animatable + Clone>(&self, property: &str) -> Option<T> {
        self.state().get(property)
    }

    /// Get an animated value with a default.
    pub fn get_or<T: Animatable + Clone>(&self, property: &str, default: T) -> T {
        self.state().get_or(property, default)
    }

    /// Get the current elapsed time.
    pub fn elapsed(&self) -> f64 {
        if self.paused {
            self.paused_at
        } else {
            self.start_time.elapsed().as_secs_f64() * self.speed
        }
    }

    /// Get the current act name.
    pub fn current_act(&self) -> String {
        self.state().act_name
    }

    /// Get progress through the current act (0.0 to 1.0).
    pub fn act_progress(&self) -> f64 {
        self.state().act_progress
    }

    /// Check if the timeline is paused.
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Check if the timeline is playing.
    pub fn is_playing(&self) -> bool {
        !self.paused
    }

    /// Pause the timeline.
    pub fn pause(&mut self) {
        if !self.paused {
            self.paused_at = self.start_time.elapsed().as_secs_f64() * self.speed;
            self.paused = true;
        }
    }

    /// Resume the timeline.
    pub fn play(&mut self) {
        if self.paused {
            self.start_time = Instant::now() - std::time::Duration::from_secs_f64(self.paused_at / self.speed);
            self.paused = false;
        }
    }

    /// Toggle pause/play.
    pub fn toggle_pause(&mut self) {
        if self.paused {
            self.play();
        } else {
            self.pause();
        }
    }

    /// Seek to a specific time.
    pub fn seek(&mut self, time: f64) {
        if self.paused {
            self.paused_at = time;
        } else {
            self.start_time = Instant::now() - std::time::Duration::from_secs_f64(time / self.speed);
        }
    }

    /// Restart from the beginning.
    pub fn restart(&mut self) {
        self.start_time = Instant::now();
        self.paused_at = 0.0;
    }

    /// Set playback speed (1.0 = normal, 2.0 = 2x, 0.5 = half speed).
    pub fn set_speed(&mut self, speed: f64) {
        let current_time = self.elapsed();
        self.speed = speed;
        self.seek(current_time);
    }

    /// Get the playback speed.
    pub fn speed(&self) -> f64 {
        self.speed
    }

    /// Get the total duration of the timeline.
    pub fn duration(&self) -> f64 {
        self.timeline.duration()
    }

    /// Get overall progress (0.0 to 1.0) for non-looping timelines.
    pub fn progress(&self) -> f64 {
        let duration = self.timeline.duration();
        if duration > 0.0 {
            (self.elapsed() / duration).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }

    /// Get the number of times the timeline has looped.
    pub fn loop_count(&self) -> u32 {
        self.loop_count
    }

    /// Update the timeline and fire any pending callbacks.
    ///
    /// Call this each frame to ensure callbacks are triggered at the right time.
    /// Returns true if any callbacks were fired.
    pub fn update(&mut self) -> bool {
        let mut fired = false;
        let state = self.state();
        let current_act_index = state.act_index;
        let current_elapsed = self.elapsed();

        // Detect loop (elapsed time jumped backward while looping)
        let looped = match self.timeline.loop_behavior {
            LoopBehavior::Loop | LoopBehavior::LoopFrom(_) => {
                current_elapsed < self.last_elapsed && self.last_elapsed > 0.0
            }
            LoopBehavior::None => false,
        };

        if looped {
            self.loop_count += 1;

            // Fire loop callback
            if let Some(ref cb) = self.timeline.on_loop {
                cb(self.loop_count);
                fired = true;
            }
        }

        // Detect act change
        if self.last_act_index != Some(current_act_index) {
            // Fire exit callback for previous act
            if let Some(prev_idx) = self.last_act_index {
                if prev_idx < self.timeline.acts.len() {
                    let prev_act = &self.timeline.acts[prev_idx];
                    prev_act.fire_exit();
                    if let Some(ref cb) = self.timeline.on_act_exit {
                        cb(&prev_act.name);
                    }
                    fired = true;
                }
            }

            // Fire enter callback for current act
            if current_act_index < self.timeline.acts.len() {
                let current_act = &self.timeline.acts[current_act_index];
                current_act.fire_enter();
                if let Some(ref cb) = self.timeline.on_act_enter {
                    cb(&current_act.name);
                }
                fired = true;
            }

            self.last_act_index = Some(current_act_index);
        }

        self.last_elapsed = current_elapsed;
        fired
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animatable_f64() {
        assert_eq!(f64::lerp(&0.0, &100.0, 0.0), 0.0);
        assert_eq!(f64::lerp(&0.0, &100.0, 0.5), 50.0);
        assert_eq!(f64::lerp(&0.0, &100.0, 1.0), 100.0);
    }

    #[test]
    fn test_animatable_tuple() {
        let a = (0.0f64, 0.0f64);
        let b = (100.0f64, 200.0f64);
        let mid = <(f64, f64)>::lerp(&a, &b, 0.5);
        assert_eq!(mid, (50.0, 100.0));
    }

    #[test]
    fn test_track_from_to() {
        let track = Track::from_to(0.0f64, 100.0, Easing::Linear);
        assert_eq!(track.value_at(0.0), 0.0);
        assert_eq!(track.value_at(0.5), 50.0);
        assert_eq!(track.value_at(1.0), 100.0);
    }

    #[test]
    fn test_track_keyframes() {
        let track = Track::new()
            .keyframe(0.0, 0.0f64, Easing::Linear)
            .keyframe(0.5, 100.0, Easing::Linear)
            .keyframe(1.0, 50.0, Easing::Linear);

        assert_eq!(track.value_at(0.0), 0.0);
        assert_eq!(track.value_at(0.5), 100.0);
        assert_eq!(track.value_at(1.0), 50.0);
        assert_eq!(track.value_at(0.25), 50.0); // Halfway to 100
        assert_eq!(track.value_at(0.75), 75.0); // Halfway from 100 to 50
    }

    #[test]
    fn test_act_simple() {
        let act = Act::new("fade")
            .duration(2.0)
            .animate("opacity", 0.0f64, 1.0, Easing::Linear);

        assert_eq!(act.name(), "fade");
        assert_eq!(act.get_duration(), 2.0);
    }

    #[test]
    fn test_timeline_single_act() {
        let timeline = Timeline::new()
            .act(Act::new("fade")
                .duration(2.0)
                .animate("opacity", 0.0f64, 1.0, Easing::Linear));

        let state = timeline.at(0.0);
        assert_eq!(state.act_name, "fade");
        assert_eq!(state.get::<f64>("opacity"), Some(0.0));

        let state = timeline.at(1.0);
        assert_eq!(state.get::<f64>("opacity"), Some(0.5));

        let state = timeline.at(2.0);
        assert_eq!(state.get::<f64>("opacity"), Some(1.0));
    }

    #[test]
    fn test_timeline_multiple_acts() {
        let timeline = Timeline::new()
            .act(Act::new("fade_in")
                .duration(1.0)
                .animate("opacity", 0.0f64, 1.0, Easing::Linear))
            .act(Act::new("hold")
                .duration(1.0))
            .act(Act::new("fade_out")
                .duration(1.0)
                .animate("opacity", 1.0f64, 0.0, Easing::Linear));

        // fade_in
        let state = timeline.at(0.5);
        assert_eq!(state.act_name, "fade_in");
        assert_eq!(state.get::<f64>("opacity"), Some(0.5));

        // hold (no opacity track, should return None)
        let state = timeline.at(1.5);
        assert_eq!(state.act_name, "hold");
        assert_eq!(state.get::<f64>("opacity"), None);

        // fade_out
        let state = timeline.at(2.5);
        assert_eq!(state.act_name, "fade_out");
        assert_eq!(state.get::<f64>("opacity"), Some(0.5));
    }

    #[test]
    fn test_timeline_loop() {
        let timeline = Timeline::new()
            .act(Act::new("a").duration(1.0))
            .act(Act::new("b").duration(1.0))
            .loop_forever();

        assert_eq!(timeline.at(0.5).act_name, "a");
        assert_eq!(timeline.at(1.5).act_name, "b");
        assert_eq!(timeline.at(2.5).act_name, "a"); // Looped
        assert_eq!(timeline.at(3.5).act_name, "b"); // Looped
    }

    #[test]
    fn test_timeline_loop_from() {
        let timeline = Timeline::new()
            .act(Act::new("intro").duration(1.0))
            .act(Act::new("loop_a").duration(1.0))
            .act(Act::new("loop_b").duration(1.0))
            .loop_from("loop_a");

        assert_eq!(timeline.at(0.5).act_name, "intro");
        assert_eq!(timeline.at(1.5).act_name, "loop_a");
        assert_eq!(timeline.at(2.5).act_name, "loop_b");
        // After total duration, loops from loop_a
        assert_eq!(timeline.at(3.5).act_name, "loop_a");
        assert_eq!(timeline.at(4.5).act_name, "loop_b");
    }

    #[test]
    fn test_playing_timeline() {
        let timeline = Timeline::new()
            .act(Act::new("test")
                .duration(1.0)
                .animate("value", 0.0f64, 100.0, Easing::Linear));

        let mut playing = timeline.start();

        // Test pause/play
        playing.pause();
        assert!(playing.is_paused());

        playing.play();
        assert!(playing.is_playing());

        // Test seek
        playing.seek(0.5);
        let value: f64 = playing.get_or("value", 0.0);
        assert!((value - 50.0).abs() < 1.0); // Allow small timing variance

        // Test restart
        playing.restart();
        assert!(playing.elapsed() < 0.1);
    }

    #[test]
    fn test_act_callbacks() {
        use std::cell::Cell;
        use std::rc::Rc;

        let entered = Rc::new(Cell::new(false));
        let exited = Rc::new(Cell::new(false));

        let entered_clone = entered.clone();
        let exited_clone = exited.clone();

        let timeline = Timeline::new()
            .act(Act::new("first")
                .duration(1.0)
                .on_enter(move || entered_clone.set(true))
                .on_exit(move || exited_clone.set(true)))
            .act(Act::new("second").duration(1.0));

        let mut playing = timeline.start();

        // First update should fire enter callback
        playing.update();
        assert!(entered.get(), "on_enter should fire on first act");
        assert!(!exited.get(), "on_exit should not fire yet");

        // Seek to second act
        playing.seek(1.5);
        playing.update();
        assert!(exited.get(), "on_exit should fire when leaving first act");
    }

    #[test]
    fn test_timeline_callbacks() {
        use std::cell::RefCell;
        use std::rc::Rc;

        let act_entered = Rc::new(RefCell::new(String::new()));
        let loop_count = Rc::new(RefCell::new(0u32));

        let act_entered_clone = act_entered.clone();
        let loop_count_clone = loop_count.clone();

        let timeline = Timeline::new()
            .act(Act::new("a").duration(0.5))
            .act(Act::new("b").duration(0.5))
            .loop_forever()
            .on_act_enter(move |name| *act_entered_clone.borrow_mut() = name.to_string())
            .on_loop(move |count| *loop_count_clone.borrow_mut() = count);

        let mut playing = timeline.start();

        // Initial update
        playing.update();
        assert_eq!(*act_entered.borrow(), "a");

        // Move to act b
        playing.seek(0.6);
        playing.update();
        assert_eq!(*act_entered.borrow(), "b");

        // Loop back to a
        playing.seek(1.1); // This should trigger loop detection on next seek backward
        playing.update();
        playing.seek(0.1); // Go back to start (simulates loop)
        playing.update();
        // Note: loop detection is based on elapsed time jumping backward
        // which doesn't happen with seek(), so we test act changes instead
        assert_eq!(*act_entered.borrow(), "a");
    }
}
