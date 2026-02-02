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
// Stagger Configuration
// ============================================================================

/// Order in which staggered items animate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StaggerOrder {
    /// Animate from first to last (index 0, 1, 2, ...)
    #[default]
    Forward,
    /// Animate from last to first (index n-1, n-2, ..., 0)
    Reverse,
    /// Animate from center outward
    CenterOut,
    /// Animate from edges toward center
    EdgesIn,
    /// Random order (deterministic based on index)
    Random,
}

impl StaggerOrder {
    /// Calculate the delay multiplier for an item at the given index.
    /// Returns a value between 0.0 and 1.0 representing the relative start position.
    pub fn delay_factor(&self, index: usize, count: usize) -> f64 {
        if count <= 1 {
            return 0.0;
        }
        let max_idx = count - 1;
        match self {
            StaggerOrder::Forward => index as f64 / max_idx as f64,
            StaggerOrder::Reverse => (max_idx - index) as f64 / max_idx as f64,
            StaggerOrder::CenterOut => {
                let center = (count - 1) as f64 / 2.0;
                let distance = (index as f64 - center).abs();
                let max_distance = center;
                if max_distance > 0.0 {
                    distance / max_distance
                } else {
                    0.0
                }
            }
            StaggerOrder::EdgesIn => {
                let center = (count - 1) as f64 / 2.0;
                let distance = (index as f64 - center).abs();
                let max_distance = center;
                if max_distance > 0.0 {
                    1.0 - (distance / max_distance)
                } else {
                    0.0
                }
            }
            StaggerOrder::Random => {
                // Simple hash-based pseudo-random but deterministic
                let hash = (index.wrapping_mul(2654435761) ^ index.wrapping_mul(0x5bd1e995)) % 1000;
                hash as f64 / 999.0
            }
        }
    }
}

/// Configuration for a staggered animation.
#[derive(Clone)]
pub struct StaggerConfig<T: Animatable> {
    /// Number of items to stagger
    pub count: usize,
    /// Delay between each item (as a fraction of act duration, 0.0-1.0)
    pub delay: f64,
    /// Order of animation
    pub order: StaggerOrder,
    /// Start value
    pub from: T,
    /// End value
    pub to: T,
    /// Easing function
    pub easing: Easing,
}

impl<T: Animatable> StaggerConfig<T> {
    /// Create a new stagger config with defaults.
    pub fn new(count: usize, from: T, to: T) -> Self {
        Self {
            count,
            delay: 0.1, // 10% of act duration between each
            order: StaggerOrder::Forward,
            from,
            to,
            easing: Easing::EaseOutCubic,
        }
    }

    /// Set the delay between items (as fraction of act duration).
    pub fn delay(mut self, delay: f64) -> Self {
        self.delay = delay.clamp(0.0, 1.0);
        self
    }

    /// Set the stagger order.
    pub fn order(mut self, order: StaggerOrder) -> Self {
        self.order = order;
        self
    }

    /// Set the easing function.
    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// Get the animated value for a specific item at normalized time t.
    pub fn value_at(&self, index: usize, t: f64) -> T {
        if self.count == 0 {
            return self.from.clone();
        }

        // Calculate when this item starts and ends
        let delay_factor = self.order.delay_factor(index, self.count);
        let total_stagger_time = self.delay * (self.count - 1) as f64;
        let item_start = delay_factor * total_stagger_time;
        let item_duration = 1.0 - total_stagger_time;

        if item_duration <= 0.0 {
            // If stagger takes up all the time, just return based on whether we've started
            return if t >= item_start {
                self.to.clone()
            } else {
                self.from.clone()
            };
        }

        // Calculate local progress for this item
        let local_t = if t < item_start {
            0.0
        } else if t >= item_start + item_duration {
            1.0
        } else {
            (t - item_start) / item_duration
        };

        let eased_t = self.easing.apply(local_t);
        T::lerp(&self.from, &self.to, eased_t)
    }
}

// ============================================================================
// Spring Physics
// ============================================================================

/// Spring configuration for physics-based animation.
///
/// Springs provide natural, organic motion that responds to physical parameters
/// rather than predefined easing curves.
///
/// # Example
///
/// ```ignore
/// let bouncy = Spring::new(170.0, 26.0); // Stiff and bouncy
/// let smooth = Spring::new(100.0, 20.0); // Smoother motion
/// let gentle = Spring::preset_gentle();   // Use a preset
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spring {
    /// Stiffness (spring constant k). Higher = faster oscillation.
    /// Typical range: 100-500
    pub stiffness: f64,
    /// Damping coefficient. Higher = less bouncing.
    /// Typical range: 10-40
    pub damping: f64,
    /// Mass of the object. Higher = more inertia.
    /// Usually kept at 1.0
    pub mass: f64,
}

impl Spring {
    /// Create a new spring with the given parameters.
    pub fn new(stiffness: f64, damping: f64) -> Self {
        Self {
            stiffness,
            damping,
            mass: 1.0,
        }
    }

    /// Create a spring with custom mass.
    pub fn with_mass(stiffness: f64, damping: f64, mass: f64) -> Self {
        Self {
            stiffness,
            damping,
            mass,
        }
    }

    /// Gentle spring - smooth with minimal overshoot.
    pub fn preset_gentle() -> Self {
        Self::new(120.0, 20.0)
    }

    /// Bouncy spring - playful with visible overshoot.
    pub fn preset_bouncy() -> Self {
        Self::new(180.0, 12.0)
    }

    /// Stiff spring - quick and snappy.
    pub fn preset_stiff() -> Self {
        Self::new(300.0, 30.0)
    }

    /// Slow spring - gradual, heavy feeling.
    pub fn preset_slow() -> Self {
        Self::new(80.0, 20.0)
    }

    /// Calculate the damping ratio (ζ).
    /// - ζ < 1: underdamped (bouncy)
    /// - ζ = 1: critically damped (fastest without overshoot)
    /// - ζ > 1: overdamped (slow approach)
    fn damping_ratio(&self) -> f64 {
        self.damping / (2.0 * (self.stiffness * self.mass).sqrt())
    }

    /// Calculate the natural frequency (ω₀).
    fn natural_frequency(&self) -> f64 {
        (self.stiffness / self.mass).sqrt()
    }

    /// Compute the spring position at time t, starting from 0 and moving to 1.
    ///
    /// Returns a value that starts at 0, oscillates toward 1, and settles at 1.
    pub fn evaluate(&self, t: f64) -> f64 {
        if t <= 0.0 {
            return 0.0;
        }

        let zeta = self.damping_ratio();
        let omega0 = self.natural_frequency();

        if zeta < 1.0 {
            // Underdamped - oscillates
            let omega_d = omega0 * (1.0 - zeta * zeta).sqrt();
            let decay = (-zeta * omega0 * t).exp();
            let oscillation = (omega_d * t).cos() + (zeta * omega0 / omega_d) * (omega_d * t).sin();
            1.0 - decay * oscillation
        } else if (zeta - 1.0).abs() < 0.0001 {
            // Critically damped - fastest without overshoot
            let decay = (-omega0 * t).exp();
            1.0 - decay * (1.0 + omega0 * t)
        } else {
            // Overdamped - slow approach
            let sqrt_term = (zeta * zeta - 1.0).sqrt();
            let r1 = -omega0 * (zeta - sqrt_term);
            let r2 = -omega0 * (zeta + sqrt_term);
            let c2 = -r1 / (r2 - r1);
            let c1 = 1.0 - c2;
            1.0 - c1 * (r1 * t).exp() - c2 * (r2 * t).exp()
        }
    }

    /// Estimate the settling time (time to reach ~99% of target).
    pub fn settling_time(&self) -> f64 {
        let zeta = self.damping_ratio();
        let omega0 = self.natural_frequency();

        if zeta < 1.0 {
            // Underdamped: ~4/(ζω₀) for 2% settling
            4.6 / (zeta * omega0)
        } else {
            // Critically/overdamped: ~4/ω₀
            4.6 / omega0
        }
    }
}

impl Default for Spring {
    fn default() -> Self {
        Self::preset_gentle()
    }
}

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
        (f32::lerp(&a.0, &b.0, t), f32::lerp(&a.1, &b.1, t))
    }
}

impl Animatable for (f64, f64) {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        (f64::lerp(&a.0, &b.0, t), f64::lerp(&a.1, &b.1, t))
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
                Keyframe {
                    time: 0.0,
                    value: from,
                    easing: Easing::Linear,
                },
                Keyframe {
                    time: 1.0,
                    value: to,
                    easing,
                },
            ],
        }
    }

    /// Add a keyframe at the specified normalized time.
    pub fn keyframe(mut self, time: f64, value: T, easing: Easing) -> Self {
        self.keyframes.push(Keyframe {
            time,
            value,
            easing,
        });
        self.keyframes
            .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
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
// SpringTrack
// ============================================================================

/// A track that uses spring physics for animation.
///
/// Unlike keyframe-based tracks, spring tracks simulate physical spring motion
/// from a start value to a target value.
#[derive(Clone)]
pub struct SpringTrack<T: Animatable> {
    from: T,
    to: T,
    spring: Spring,
}

impl<T: Animatable> SpringTrack<T> {
    /// Create a new spring track.
    pub fn new(from: T, to: T, spring: Spring) -> Self {
        Self { from, to, spring }
    }

    /// Get the interpolated value at normalized time t (0.0 to 1.0).
    ///
    /// The spring animation is scaled to fit within the act duration,
    /// with t=1.0 corresponding to the spring's settling time.
    pub fn value_at(&self, t: f64) -> T {
        let t = t.clamp(0.0, 1.0);
        // Scale t to spring settling time
        let spring_t = t * self.spring.settling_time();
        let spring_progress = self.spring.evaluate(spring_t);
        T::lerp(&self.from, &self.to, spring_progress)
    }
}

impl<T: Animatable> AnyTrack for SpringTrack<T> {
    fn value_at(&self, t: f64) -> Box<dyn Any + Send + Sync> {
        Box::new(self.value_at(t))
    }

    fn clone_box(&self) -> Box<dyn AnyTrack> {
        Box::new(self.clone())
    }
}

// ============================================================================
// StaggerTrack
// ============================================================================

/// Type-erased stagger track that can be stored in collections.
trait AnyStaggerTrack: Send + Sync {
    /// Get the value at normalized time t for a specific item index, boxed as Any.
    fn value_at_index(&self, index: usize, t: f64) -> Box<dyn Any + Send + Sync>;
    /// Get the number of items.
    fn count(&self) -> usize;
    /// Clone the track.
    fn clone_box(&self) -> Box<dyn AnyStaggerTrack>;
}

/// A track that staggers animation across multiple items.
#[derive(Clone)]
pub struct StaggerTrack<T: Animatable> {
    config: StaggerConfig<T>,
}

impl<T: Animatable> StaggerTrack<T> {
    /// Create a new stagger track from configuration.
    pub fn new(config: StaggerConfig<T>) -> Self {
        Self { config }
    }

    /// Create a simple stagger track with default settings.
    pub fn simple(count: usize, from: T, to: T, easing: Easing) -> Self {
        Self {
            config: StaggerConfig::new(count, from, to).easing(easing),
        }
    }

    /// Get the value for a specific item at normalized time t.
    pub fn value_at(&self, index: usize, t: f64) -> T {
        self.config.value_at(index, t)
    }

    /// Get the number of items.
    pub fn count(&self) -> usize {
        self.config.count
    }
}

impl<T: Animatable> AnyStaggerTrack for StaggerTrack<T> {
    fn value_at_index(&self, index: usize, t: f64) -> Box<dyn Any + Send + Sync> {
        Box::new(self.value_at(index, t))
    }

    fn count(&self) -> usize {
        self.config.count
    }

    fn clone_box(&self) -> Box<dyn AnyStaggerTrack> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AnyStaggerTrack> {
    fn clone(&self) -> Self {
        self.clone_box()
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
    /// Named stagger tracks for multi-item animations
    stagger_tracks: HashMap<String, Box<dyn AnyStaggerTrack>>,
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
            stagger_tracks: HashMap::new(),
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
            stagger_tracks: HashMap::new(),
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
            stagger_tracks: HashMap::new(),
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

    /// Add a spring-based animation for a property.
    ///
    /// Spring animations provide natural, physics-based motion with overshoots
    /// and settling behavior.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Act::new("bounce_in")
    ///     .duration(1.0)
    ///     .spring("position", 0.0, 100.0, Spring::preset_bouncy())
    /// ```
    pub fn spring<T: Animatable>(
        mut self,
        property: impl Into<String>,
        from: T,
        to: T,
        spring: Spring,
    ) -> Self {
        let track = SpringTrack::new(from, to, spring);
        self.tracks.insert(property.into(), Box::new(track));
        self
    }

    /// Add a spring track with full control.
    pub fn spring_track<T: Animatable>(
        mut self,
        property: impl Into<String>,
        track: SpringTrack<T>,
    ) -> Self {
        self.tracks.insert(property.into(), Box::new(track));
        self
    }

    /// Add a staggered animation for multiple items.
    ///
    /// Stagger animations apply the same animation to multiple items with
    /// a delay between each one, creating a wave or cascade effect.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Act::new("panels_enter")
    ///     .duration(2.0)
    ///     .stagger("panel_opacity", 5, 0.0f64, 1.0, Easing::EaseOutCubic)
    /// ```
    pub fn stagger<T: Animatable>(
        mut self,
        property: impl Into<String>,
        count: usize,
        from: T,
        to: T,
        easing: Easing,
    ) -> Self {
        let track = StaggerTrack::simple(count, from, to, easing);
        self.stagger_tracks.insert(property.into(), Box::new(track));
        self
    }

    /// Add a staggered animation with full configuration.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Act::new("panels_enter")
    ///     .duration(2.0)
    ///     .stagger_config("panel_opacity", StaggerConfig::new(5, 0.0f64, 1.0)
    ///         .delay(0.15)
    ///         .order(StaggerOrder::CenterOut)
    ///         .easing(Easing::EaseOutElastic))
    /// ```
    pub fn stagger_config<T: Animatable>(
        mut self,
        property: impl Into<String>,
        config: StaggerConfig<T>,
    ) -> Self {
        let track = StaggerTrack::new(config);
        self.stagger_tracks.insert(property.into(), Box::new(track));
        self
    }

    /// Add a stagger track directly.
    pub fn stagger_track<T: Animatable>(
        mut self,
        property: impl Into<String>,
        track: StaggerTrack<T>,
    ) -> Self {
        self.stagger_tracks.insert(property.into(), Box::new(track));
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

    /// Get the stagger value for an item at normalized time t (0.0 to 1.0).
    fn get_stagger_value(
        &self,
        property: &str,
        index: usize,
        t: f64,
    ) -> Option<Box<dyn Any + Send + Sync>> {
        self.stagger_tracks
            .get(property)
            .map(|track| track.value_at_index(index, t))
    }

    /// Get the count of items in a stagger track.
    fn get_stagger_count(&self, property: &str) -> Option<usize> {
        self.stagger_tracks.get(property).map(|track| track.count())
    }

    /// Check if this act has a stagger track for the given property.
    pub fn has_stagger(&self, property: &str) -> bool {
        self.stagger_tracks.contains_key(property)
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

    /// Get a staggered animation value for a specific item.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let state = timeline.at(1.5);
    /// for i in 0..5 {
    ///     let opacity: f64 = state.get_stagger("panel_opacity", i).unwrap_or(0.0);
    ///     // Use opacity for panel i
    /// }
    /// ```
    pub fn get_stagger<T: Animatable + Clone>(&self, property: &str, index: usize) -> Option<T> {
        self.act
            .get_stagger_value(property, index, self.act_progress)
            .and_then(|boxed| boxed.downcast::<T>().ok())
            .map(|v| *v)
    }

    /// Get a staggered animation value with a default.
    pub fn get_stagger_or<T: Animatable + Clone>(
        &self,
        property: &str,
        index: usize,
        default: T,
    ) -> T {
        self.get_stagger(property, index).unwrap_or(default)
    }

    /// Get all stagger values as a Vec.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let state = timeline.at(1.5);
    /// let opacities: Vec<f64> = state.get_stagger_all("panel_opacity", 0.0);
    /// for (i, opacity) in opacities.iter().enumerate() {
    ///     // Use opacity for panel i
    /// }
    /// ```
    pub fn get_stagger_all<T: Animatable + Clone>(&self, property: &str, default: T) -> Vec<T> {
        let count = self.act.get_stagger_count(property).unwrap_or(0);
        (0..count)
            .map(|i| {
                self.get_stagger(property, i)
                    .unwrap_or_else(|| default.clone())
            })
            .collect()
    }

    /// Get the number of items in a stagger track.
    pub fn stagger_count(&self, property: &str) -> usize {
        self.act.get_stagger_count(property).unwrap_or(0)
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

    /// Get a staggered animation value for a specific item.
    pub fn get_stagger<T: Animatable + Clone>(&self, property: &str, index: usize) -> Option<T> {
        self.state().get_stagger(property, index)
    }

    /// Get a staggered animation value with a default.
    pub fn get_stagger_or<T: Animatable + Clone>(
        &self,
        property: &str,
        index: usize,
        default: T,
    ) -> T {
        self.state().get_stagger_or(property, index, default)
    }

    /// Get all stagger values as a Vec.
    pub fn get_stagger_all<T: Animatable + Clone>(&self, property: &str, default: T) -> Vec<T> {
        self.state().get_stagger_all(property, default)
    }

    /// Get the number of items in a stagger track.
    pub fn stagger_count(&self, property: &str) -> usize {
        self.state().stagger_count(property)
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
            self.start_time =
                Instant::now() - std::time::Duration::from_secs_f64(self.paused_at / self.speed);
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
            self.start_time =
                Instant::now() - std::time::Duration::from_secs_f64(time / self.speed);
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
// Timeline Visualization (Phase 4: Developer Experience)
// ============================================================================

/// A debug visualization of a timeline's current state.
///
/// Use this to display timeline info in a debug panel or logging output.
#[derive(Debug, Clone)]
pub struct TimelineDebugInfo {
    /// Total duration of the timeline
    pub duration: f64,
    /// Current elapsed time
    pub elapsed: f64,
    /// Overall progress (0.0 to 1.0)
    pub progress: f64,
    /// Name of the current act
    pub current_act: String,
    /// Index of the current act
    pub act_index: usize,
    /// Total number of acts
    pub act_count: usize,
    /// Progress within the current act (0.0 to 1.0)
    pub act_progress: f64,
    /// Duration of the current act
    pub act_duration: f64,
    /// Whether the timeline is paused
    pub is_paused: bool,
    /// Current playback speed
    pub speed: f64,
    /// Number of loop iterations completed
    pub loop_count: u32,
    /// Loop behavior description
    pub loop_behavior: String,
    /// List of all act names with their durations
    pub acts: Vec<(String, f64)>,
}

impl TimelineDebugInfo {
    /// Format as a compact single-line string.
    pub fn to_compact_string(&self) -> String {
        format!(
            "[{:.2}s/{:.2}s] {} ({:.0}%) {} {:.1}x",
            self.elapsed,
            self.duration,
            self.current_act,
            self.act_progress * 100.0,
            if self.is_paused { "PAUSED" } else { "PLAYING" },
            self.speed,
        )
    }

    /// Format as a multi-line debug display.
    pub fn to_debug_string(&self) -> String {
        let mut lines = vec![
            format!("Timeline Debug Info"),
            format!("=================="),
            format!(
                "Time: {:.2}s / {:.2}s ({:.1}%)",
                self.elapsed,
                self.duration,
                self.progress * 100.0
            ),
            format!(
                "Act: {} [{}/{}]",
                self.current_act,
                self.act_index + 1,
                self.act_count
            ),
            format!("Act Progress: {:.1}%", self.act_progress * 100.0),
            format!(
                "Status: {} at {:.1}x speed",
                if self.is_paused { "Paused" } else { "Playing" },
                self.speed
            ),
            format!("Loop: {} (count: {})", self.loop_behavior, self.loop_count),
            format!(""),
            format!("Acts:"),
        ];

        let mut time_offset = 0.0;
        for (i, (name, duration)) in self.acts.iter().enumerate() {
            let marker = if i == self.act_index { ">>>" } else { "   " };
            lines.push(format!(
                "{} {:2}. {:20} ({:.1}s) @ {:.1}s",
                marker,
                i + 1,
                name,
                duration,
                time_offset
            ));
            time_offset += duration;
        }

        lines.join("\n")
    }

    /// Create an ASCII progress bar showing overall timeline progress.
    pub fn progress_bar(&self, width: usize) -> String {
        let filled = (self.progress * width as f64) as usize;
        let empty = width.saturating_sub(filled);
        format!("[{}{}]", "=".repeat(filled), "-".repeat(empty))
    }

    /// Create an ASCII visualization of the act layout.
    pub fn act_visualization(&self, width: usize) -> String {
        if self.duration == 0.0 || self.acts.is_empty() {
            return format!("[{}]", "-".repeat(width));
        }

        let mut result = String::new();

        for (i, (_name, duration)) in self.acts.iter().enumerate() {
            let act_width = ((duration / self.duration) * width as f64).round() as usize;
            let act_width = act_width.max(1);

            let char_to_use = if i == self.act_index { '=' } else { '-' };
            let segment = char_to_use.to_string().repeat(act_width);

            // Add separator between acts
            if !result.is_empty() {
                result.push('|');
            }
            result.push_str(&segment);
        }

        // Mark current position
        let pos = ((self.elapsed / self.duration) * width as f64) as usize;
        let pos = pos.min(width.saturating_sub(1));

        // Replace character at position with marker
        let mut chars: Vec<char> = result.chars().collect();
        if pos < chars.len() {
            chars[pos] = '*';
        }

        format!("[{}]", chars.into_iter().collect::<String>())
    }
}

impl PlayingTimeline {
    /// Get debug information about the current timeline state.
    ///
    /// Useful for debugging and visualization tools.
    pub fn debug_info(&self) -> TimelineDebugInfo {
        let state = self.state();

        let acts: Vec<(String, f64)> = self
            .timeline
            .acts
            .iter()
            .map(|a| (a.name.clone(), a.duration))
            .collect();

        let loop_behavior = match &self.timeline.loop_behavior {
            LoopBehavior::None => "None".to_string(),
            LoopBehavior::Loop => "Loop forever".to_string(),
            LoopBehavior::LoopFrom(name) => format!("Loop from '{}'", name),
        };

        let act_duration = self
            .timeline
            .acts
            .get(state.act_index)
            .map(|a| a.duration)
            .unwrap_or(0.0);

        TimelineDebugInfo {
            duration: self.timeline.duration(),
            elapsed: self.elapsed(),
            progress: self.progress(),
            current_act: state.act_name,
            act_index: state.act_index,
            act_count: self.timeline.acts.len(),
            act_progress: state.act_progress,
            act_duration,
            is_paused: self.paused,
            speed: self.speed,
            loop_count: self.loop_count,
            loop_behavior,
            acts,
        }
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
        let timeline = Timeline::new().act(Act::new("fade").duration(2.0).animate(
            "opacity",
            0.0f64,
            1.0,
            Easing::Linear,
        ));

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
            .act(
                Act::new("fade_in")
                    .duration(1.0)
                    .animate("opacity", 0.0f64, 1.0, Easing::Linear),
            )
            .act(Act::new("hold").duration(1.0))
            .act(Act::new("fade_out").duration(1.0).animate(
                "opacity",
                1.0f64,
                0.0,
                Easing::Linear,
            ));

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
        let timeline = Timeline::new().act(Act::new("test").duration(1.0).animate(
            "value",
            0.0f64,
            100.0,
            Easing::Linear,
        ));

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
            .act(
                Act::new("first")
                    .duration(1.0)
                    .on_enter(move || entered_clone.set(true))
                    .on_exit(move || exited_clone.set(true)),
            )
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

    #[test]
    fn test_spring_evaluate() {
        let spring = Spring::preset_bouncy();

        // At t=0, should be at start (0)
        assert!((spring.evaluate(0.0) - 0.0).abs() < 0.001);

        // At settling time, should be very close to target (1)
        let settling = spring.settling_time();
        assert!((spring.evaluate(settling) - 1.0).abs() < 0.05);
    }

    #[test]
    fn test_spring_presets() {
        let gentle = Spring::preset_gentle();
        let bouncy = Spring::preset_bouncy();
        let stiff = Spring::preset_stiff();
        let slow = Spring::preset_slow();

        // All should reach target eventually
        for spring in [gentle, bouncy, stiff, slow] {
            let settling = spring.settling_time();
            let final_val = spring.evaluate(settling);
            assert!(
                (final_val - 1.0).abs() < 0.1,
                "Spring should settle near target"
            );
        }
    }

    #[test]
    fn test_spring_track() {
        let track = SpringTrack::new(0.0f64, 100.0, Spring::preset_gentle());

        // At t=0, should be at start
        assert!((track.value_at(0.0) - 0.0).abs() < 0.1);

        // At t=1, should be at or very near target
        assert!((track.value_at(1.0) - 100.0).abs() < 5.0);
    }

    #[test]
    fn test_act_with_spring() {
        let timeline = Timeline::new().act(Act::new("bounce").duration(1.0).spring(
            "position",
            0.0f64,
            100.0,
            Spring::preset_bouncy(),
        ));

        // At start
        let state = timeline.at(0.0);
        let pos: f64 = state.get("position").unwrap();
        assert!((pos - 0.0).abs() < 0.1);

        // At end
        let state = timeline.at(1.0);
        let pos: f64 = state.get("position").unwrap();
        assert!((pos - 100.0).abs() < 5.0);
    }

    #[test]
    fn test_stagger_order_forward() {
        let order = StaggerOrder::Forward;
        assert_eq!(order.delay_factor(0, 5), 0.0);
        assert_eq!(order.delay_factor(2, 5), 0.5);
        assert_eq!(order.delay_factor(4, 5), 1.0);
    }

    #[test]
    fn test_stagger_order_reverse() {
        let order = StaggerOrder::Reverse;
        assert_eq!(order.delay_factor(0, 5), 1.0);
        assert_eq!(order.delay_factor(2, 5), 0.5);
        assert_eq!(order.delay_factor(4, 5), 0.0);
    }

    #[test]
    fn test_stagger_order_center_out() {
        let order = StaggerOrder::CenterOut;
        // For 5 items, center is index 2
        assert_eq!(order.delay_factor(2, 5), 0.0); // Center starts first
        assert_eq!(order.delay_factor(0, 5), 1.0); // Edges start last
        assert_eq!(order.delay_factor(4, 5), 1.0);
    }

    #[test]
    fn test_stagger_order_edges_in() {
        let order = StaggerOrder::EdgesIn;
        // For 5 items, edges start first
        assert_eq!(order.delay_factor(0, 5), 0.0); // Edge starts first
        assert_eq!(order.delay_factor(4, 5), 0.0);
        assert_eq!(order.delay_factor(2, 5), 1.0); // Center starts last
    }

    #[test]
    fn test_stagger_config_basic() {
        let config = StaggerConfig::new(3, 0.0f64, 1.0)
            .delay(0.2)
            .order(StaggerOrder::Forward);

        // At t=0, first item should start (no delay)
        // All items should be at 0.0
        assert!((config.value_at(0, 0.0) - 0.0).abs() < 0.01);
        assert!((config.value_at(1, 0.0) - 0.0).abs() < 0.01);
        assert!((config.value_at(2, 0.0) - 0.0).abs() < 0.01);

        // At t=1, all items should be at 1.0
        assert!((config.value_at(0, 1.0) - 1.0).abs() < 0.01);
        assert!((config.value_at(1, 1.0) - 1.0).abs() < 0.01);
        assert!((config.value_at(2, 1.0) - 1.0).abs() < 0.01);

        // At t=0.5, items should be at different stages
        let v0 = config.value_at(0, 0.5);
        let v1 = config.value_at(1, 0.5);
        let v2 = config.value_at(2, 0.5);
        // Item 0 started first, should be furthest along
        assert!(v0 > v1);
        assert!(v1 > v2);
    }

    #[test]
    fn test_stagger_track() {
        let track = StaggerTrack::simple(4, 0.0f64, 100.0, Easing::Linear);

        assert_eq!(track.count(), 4);

        // At t=0, all should be at start
        for i in 0..4 {
            assert!((track.value_at(i, 0.0) - 0.0).abs() < 0.1);
        }

        // At t=1, all should be at end
        for i in 0..4 {
            assert!((track.value_at(i, 1.0) - 100.0).abs() < 0.1);
        }
    }

    #[test]
    fn test_act_with_stagger() {
        let timeline = Timeline::new().act(Act::new("panels_enter").duration(2.0).stagger(
            "opacity",
            3,
            0.0f64,
            1.0,
            Easing::Linear,
        ));

        // At start
        let state = timeline.at(0.0);
        assert_eq!(state.stagger_count("opacity"), 3);
        assert!((state.get_stagger::<f64>("opacity", 0).unwrap() - 0.0).abs() < 0.01);

        // At end
        let state = timeline.at(2.0);
        assert!((state.get_stagger::<f64>("opacity", 0).unwrap() - 1.0).abs() < 0.01);
        assert!((state.get_stagger::<f64>("opacity", 1).unwrap() - 1.0).abs() < 0.01);
        assert!((state.get_stagger::<f64>("opacity", 2).unwrap() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_stagger_all() {
        let timeline = Timeline::new().act(Act::new("fade").duration(1.0).stagger(
            "alpha",
            4,
            0.0f64,
            1.0,
            Easing::Linear,
        ));

        let state = timeline.at(1.0);
        let values: Vec<f64> = state.get_stagger_all("alpha", 0.0);
        assert_eq!(values.len(), 4);
        for v in &values {
            assert!((v - 1.0).abs() < 0.1);
        }
    }

    #[test]
    fn test_playing_timeline_stagger() {
        let timeline = Timeline::new().act(Act::new("test").duration(1.0).stagger(
            "value",
            3,
            0.0f64,
            100.0,
            Easing::Linear,
        ));

        let mut playing = timeline.start();
        playing.seek(1.0);

        assert_eq!(playing.stagger_count("value"), 3);

        let v0: f64 = playing.get_stagger_or("value", 0, 0.0);
        let v1: f64 = playing.get_stagger_or("value", 1, 0.0);
        let v2: f64 = playing.get_stagger_or("value", 2, 0.0);

        assert!((v0 - 100.0).abs() < 1.0);
        assert!((v1 - 100.0).abs() < 1.0);
        assert!((v2 - 100.0).abs() < 1.0);

        let all: Vec<f64> = playing.get_stagger_all("value", 0.0);
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn test_debug_info_basic() {
        let timeline = Timeline::new()
            .act(Act::new("intro").duration(2.0))
            .act(Act::new("main").duration(3.0))
            .act(Act::new("outro").duration(1.0));

        let mut playing = timeline.start();
        playing.seek(2.5); // Halfway through "main"

        let debug = playing.debug_info();
        assert_eq!(debug.duration, 6.0);
        assert!((debug.elapsed - 2.5).abs() < 0.1);
        assert_eq!(debug.current_act, "main");
        assert_eq!(debug.act_index, 1);
        assert_eq!(debug.act_count, 3);
        assert_eq!(debug.acts.len(), 3);
        assert_eq!(debug.loop_behavior, "None");
    }

    #[test]
    fn test_debug_info_compact_string() {
        let timeline = Timeline::new().act(Act::new("test").duration(1.0));

        let mut playing = timeline.start();
        playing.seek(0.5);

        let debug = playing.debug_info();
        let compact = debug.to_compact_string();

        assert!(compact.contains("0.50s"));
        assert!(compact.contains("1.00s"));
        assert!(compact.contains("test"));
        assert!(compact.contains("PLAYING"));
    }

    #[test]
    fn test_debug_info_progress_bar() {
        let timeline = Timeline::new().act(Act::new("test").duration(1.0));

        let mut playing = timeline.start();
        playing.seek(0.5);

        let debug = playing.debug_info();
        let bar = debug.progress_bar(10);

        assert!(bar.starts_with('['));
        assert!(bar.ends_with(']'));
        assert_eq!(bar.len(), 12); // [ + 10 chars + ]
    }

    #[test]
    fn test_debug_info_loop() {
        let timeline = Timeline::new()
            .act(Act::new("a").duration(1.0))
            .act(Act::new("b").duration(1.0))
            .loop_from("a");

        let playing = timeline.start();
        let debug = playing.debug_info();

        assert!(debug.loop_behavior.contains("Loop from"));
        assert!(debug.loop_behavior.contains("'a'"));
    }
}
