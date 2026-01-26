//! React-like hooks for the reactive system.
//!
//! Hooks provide a way to use state and side effects in functional components.
//! They use a cursor-based model where hooks must be called in the same order
//! every render.
//!
//! # Rules of Hooks
//!
//! 1. **Call hooks unconditionally** - Don't call hooks inside if statements or loops
//! 2. **Same order every render** - The hook cursor must advance consistently
//! 3. **Only in components** - Hooks can only be called from within a reactive component
//!
//! # Available Hooks (v0.2.0)
//!
//! - [`use_state`] - Create reactive state
//! - [`use_input`] - Register an input handler

use super::instance::HookSlot;
use super::runtime::{InputHandlerId, RuntimeHandle, TimelineId};
use super::scope::Scope;
use super::signal::Signal;
use crate::input::Key;
use crate::timeline::{Animatable, Timeline};
use std::marker::PhantomData;

/// Create a reactive state signal.
///
/// On the first render, `init` is called to create the initial value.
/// On subsequent renders, the existing signal is reused (init is not called).
///
/// # Example
///
/// ```ignore
/// fn counter(cx: Scope) -> Element {
///     let count = use_state(cx, || 0);
///
///     element! {
///         Text(content: format!("Count: {}", count.get()))
///     }
/// }
/// ```
///
/// # Panics
///
/// Panics if:
/// - Called outside of a reactive component render
/// - Hook order changes between renders (e.g., hook called conditionally)
pub fn use_state<T, F>(cx: Scope, init: F) -> Signal<T>
where
    T: Clone + 'static,
    F: FnOnce() -> T,
{
    let rt = cx.rt.clone();
    let component_id = cx.component_id;

    // Get current cursor position and advance
    let cursor = rt
        .with_instance_mut(component_id, |instance| instance.advance_cursor())
        .expect("Component instance not found");

    // Check if we already have a hook at this position
    let existing = rt.with_instance(component_id, |instance| instance.get_hook(cursor).cloned());

    match existing {
        Some(Some(HookSlot::State(id))) => {
            // Reuse existing signal
            Signal {
                id,
                rt,
                _marker: PhantomData,
            }
        }
        Some(Some(other)) => {
            // Wrong hook type - user changed hook order
            panic!(
                "Hook order changed: expected State hook at position {}, found {:?}. \
                 Hooks must be called unconditionally and in the same order every render.",
                cursor, other
            );
        }
        Some(None) | None => {
            // First render - create new signal
            let value = init();
            let signal_id = rt.create_signal(value);

            // Store the hook slot
            rt.with_instance_mut(component_id, |instance| {
                instance.push_hook(HookSlot::State(signal_id));
            });

            Signal {
                id: signal_id,
                rt,
                _marker: PhantomData,
            }
        }
    }
}

/// Register a keyboard input handler.
///
/// The handler is registered **once** on first render and persists across
/// re-renders. It will be called for every key press while the component
/// is mounted.
///
/// # Closure Requirements
///
/// The handler must be `'static`, meaning it cannot capture references to
/// local variables. Use `use_state` for reactive data that needs to be
/// accessed from the handler.
///
/// # Example
///
/// ```ignore
/// fn counter(cx: Scope) -> Element {
///     let count = use_state(cx, || 0);
///
///     use_input(cx, move |key| {
///         if key.is_char(' ') {
///             count.set(count.get() + 1);
///         }
///     });
///
///     element! {
///         Text(content: format!("Count: {}", count.get()))
///     }
/// }
/// ```
///
/// # Wrong Usage (Won't Compile)
///
/// ```ignore
/// fn broken(cx: Scope) -> Element {
///     let items = vec!["A", "B"];  // Stack variable
///
///     // ERROR: `items` does not live long enough
///     use_input(cx, move |_key| {
///         println!("{}", items.len());
///     });
///
///     // ...
/// }
/// ```
///
/// # Panics
///
/// Panics if:
/// - Called outside of a reactive component render
/// - Hook order changes between renders
pub fn use_input<F>(cx: Scope, handler: F)
where
    F: Fn(&Key) + 'static,
{
    let rt = cx.rt.clone();
    let component_id = cx.component_id;

    // Get current cursor position and advance
    let cursor = rt
        .with_instance_mut(component_id, |instance| instance.advance_cursor())
        .expect("Component instance not found");

    // Check if we already have a hook at this position
    let existing = rt.with_instance(component_id, |instance| instance.get_hook(cursor).cloned());

    match existing {
        Some(Some(HookSlot::Input(id))) => {
            // Handler already registered, verify it still exists
            if !rt.has_input_handler(id) {
                // This shouldn't happen in normal use, but let's be safe
                panic!("Input handler was unexpectedly removed");
            }
            // Nothing to do - handler persists from first render
        }
        Some(Some(other)) => {
            // Wrong hook type - user changed hook order
            panic!(
                "Hook order changed: expected Input hook at position {}, found {:?}. \
                 Hooks must be called unconditionally and in the same order every render.",
                cursor, other
            );
        }
        Some(None) | None => {
            // First render - register the handler
            let handler_id: InputHandlerId = rt.register_input_handler(handler);

            // Store the hook slot
            rt.with_instance_mut(component_id, |instance| {
                instance.push_hook(HookSlot::Input(handler_id));
            });
        }
    }
}

/// Create a reactive timeline from a Timeline definition.
///
/// On the first render, the timeline is started. On subsequent renders,
/// the existing timeline is reused, preserving playback state.
///
/// Returns a `TimelineHandle` that provides access to animated values
/// and playback controls.
///
/// # Example
///
/// ```ignore
/// use blaeck::prelude::*;
/// use blaeck::reactive::*;
/// use blaeck::animation::Easing;
///
/// fn animated_component(cx: Scope) -> Element {
///     let timeline = use_timeline(cx, Timeline::new()
///         .act(Act::new("fade_in")
///             .duration(1.0)
///             .animate("opacity", 0.0f64, 1.0, Easing::EaseOutCubic)));
///
///     let opacity = timeline.get_or("opacity", 1.0f64);
///
///     element! {
///         Text(content: format!("Opacity: {:.2}", opacity))
///     }
/// }
/// ```
///
/// # Panics
///
/// Panics if:
/// - Called outside of a reactive component render
/// - Hook order changes between renders
pub fn use_timeline(cx: Scope, timeline: Timeline) -> TimelineHandle {
    let rt = cx.rt.clone();
    let component_id = cx.component_id;

    // Get current cursor position and advance
    let cursor = rt
        .with_instance_mut(component_id, |instance| instance.advance_cursor())
        .expect("Component instance not found");

    // Check if we already have a hook at this position
    let existing = rt.with_instance(component_id, |instance| instance.get_hook(cursor).cloned());

    match existing {
        Some(Some(HookSlot::Timeline(id))) => {
            // Reuse existing timeline
            if !rt.has_timeline(id) {
                panic!("Timeline was unexpectedly removed");
            }
            TimelineHandle { id, rt }
        }
        Some(Some(other)) => {
            // Wrong hook type - user changed hook order
            panic!(
                "Hook order changed: expected Timeline hook at position {}, found {:?}. \
                 Hooks must be called unconditionally and in the same order every render.",
                cursor, other
            );
        }
        Some(None) | None => {
            // First render - create and start the timeline
            let playing = timeline.start();
            let timeline_id = rt.create_timeline(playing);

            // Store the hook slot
            rt.with_instance_mut(component_id, |instance| {
                instance.push_hook(HookSlot::Timeline(timeline_id));
            });

            TimelineHandle { id: timeline_id, rt }
        }
    }
}

/// Handle to a timeline in the reactive system.
///
/// Provides access to animated values and playback controls.
/// This handle is cheaply clonable.
#[derive(Clone)]
pub struct TimelineHandle {
    id: TimelineId,
    rt: RuntimeHandle,
}

impl TimelineHandle {
    /// Get the ID of this timeline.
    pub fn id(&self) -> TimelineId {
        self.id
    }

    /// Get an animated value by name at the current time.
    ///
    /// Returns `None` if the property doesn't exist in the current act.
    pub fn get<T: Animatable + Clone>(&self, property: &str) -> Option<T> {
        self.rt.with_timeline(self.id, |tl| tl.get::<T>(property))?
    }

    /// Get an animated value with a default.
    pub fn get_or<T: Animatable + Clone>(&self, property: &str, default: T) -> T {
        self.get(property).unwrap_or(default)
    }

    /// Get the current elapsed time in seconds.
    pub fn elapsed(&self) -> f64 {
        self.rt
            .with_timeline(self.id, |tl| tl.elapsed())
            .unwrap_or(0.0)
    }

    /// Get the name of the current act.
    pub fn current_act(&self) -> String {
        self.rt
            .with_timeline(self.id, |tl| tl.current_act())
            .unwrap_or_default()
    }

    /// Get progress through the current act (0.0 to 1.0).
    pub fn act_progress(&self) -> f64 {
        self.rt
            .with_timeline(self.id, |tl| tl.act_progress())
            .unwrap_or(0.0)
    }

    /// Get overall progress (0.0 to 1.0) for non-looping timelines.
    pub fn progress(&self) -> f64 {
        self.rt
            .with_timeline(self.id, |tl| tl.progress())
            .unwrap_or(0.0)
    }

    /// Get the total duration of the timeline.
    pub fn duration(&self) -> f64 {
        self.rt
            .with_timeline(self.id, |tl| tl.duration())
            .unwrap_or(0.0)
    }

    /// Check if the timeline is paused.
    pub fn is_paused(&self) -> bool {
        self.rt
            .with_timeline(self.id, |tl| tl.is_paused())
            .unwrap_or(false)
    }

    /// Check if the timeline is playing.
    pub fn is_playing(&self) -> bool {
        !self.is_paused()
    }

    /// Pause the timeline.
    pub fn pause(&self) {
        self.rt.with_timeline_mut(self.id, |tl| tl.pause());
    }

    /// Resume the timeline.
    pub fn play(&self) {
        self.rt.with_timeline_mut(self.id, |tl| tl.play());
    }

    /// Toggle pause/play.
    pub fn toggle_pause(&self) {
        self.rt.with_timeline_mut(self.id, |tl| tl.toggle_pause());
    }

    /// Seek to a specific time in seconds.
    pub fn seek(&self, time: f64) {
        self.rt.with_timeline_mut(self.id, |tl| tl.seek(time));
    }

    /// Restart the timeline from the beginning.
    pub fn restart(&self) {
        self.rt.with_timeline_mut(self.id, |tl| tl.restart());
    }

    /// Set playback speed (1.0 = normal, 2.0 = 2x, 0.5 = half speed).
    pub fn set_speed(&self, speed: f64) {
        self.rt.with_timeline_mut(self.id, |tl| tl.set_speed(speed));
    }

    /// Get the current playback speed.
    pub fn speed(&self) -> f64 {
        self.rt
            .with_timeline(self.id, |tl| tl.speed())
            .unwrap_or(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reactive::runtime::RuntimeHandle;
    use std::cell::Cell;
    use std::rc::Rc;

    fn setup_scope() -> (RuntimeHandle, Scope) {
        let rt = RuntimeHandle::new();
        let component_id = rt.create_instance();
        rt.set_current_instance(Some(component_id));
        let scope = Scope::new(rt.clone(), component_id);
        (rt, scope)
    }

    #[test]
    fn test_use_state_initial() {
        let (_rt, cx) = setup_scope();
        let signal = use_state(cx, || 42);
        assert_eq!(signal.get(), 42);
    }

    #[test]
    fn test_use_state_reuses_on_rerender() {
        let (rt, cx) = setup_scope();

        // First render
        let signal1 = use_state(cx.clone(), || 0);
        signal1.set(100);

        // Reset cursor for "re-render"
        rt.reset_hook_cursor(cx.component_id);

        // Second render - should reuse existing signal
        let signal2 = use_state(cx, || 999); // init ignored
        assert_eq!(signal2.get(), 100); // Still has value from first render
        assert_eq!(signal1.id(), signal2.id()); // Same signal
    }

    #[test]
    fn test_use_state_multiple() {
        let (rt, cx) = setup_scope();

        // First render
        let a = use_state(cx.clone(), || 1);
        let b = use_state(cx.clone(), || 2);
        let c = use_state(cx.clone(), || 3);

        assert_eq!(a.get(), 1);
        assert_eq!(b.get(), 2);
        assert_eq!(c.get(), 3);

        // Modify
        b.set(20);

        // Reset for re-render
        rt.reset_hook_cursor(cx.component_id);

        // Second render - values persist
        let a2 = use_state(cx.clone(), || 0);
        let b2 = use_state(cx.clone(), || 0);
        let c2 = use_state(cx, || 0);

        assert_eq!(a2.get(), 1);
        assert_eq!(b2.get(), 20);
        assert_eq!(c2.get(), 3);
    }

    #[test]
    #[should_panic(expected = "Hook order changed")]
    fn test_use_state_wrong_order_panics() {
        let (rt, cx) = setup_scope();

        // First render with state
        let _ = use_state(cx.clone(), || 0);

        // Reset for re-render
        rt.reset_hook_cursor(cx.component_id);

        // Second render tries to use input at same position
        use_input(cx, |_| {});
    }

    #[test]
    fn test_use_input_registers_once() {
        let (rt, cx) = setup_scope();
        let call_count = Rc::new(Cell::new(0));
        let count = call_count.clone();

        // First render - registers handler
        use_input(cx.clone(), move |_| {
            count.set(count.get() + 1);
        });

        // Dispatch input
        let key = Key::new(crossterm::event::KeyCode::Char('a'));
        rt.dispatch_input(&key);
        assert_eq!(call_count.get(), 1);

        // Reset for re-render
        rt.reset_hook_cursor(cx.component_id);

        // Second render - should NOT register again
        let count2 = call_count.clone();
        use_input(cx, move |_| {
            count2.set(count2.get() + 100);
        });

        // Dispatch again - should still be only 1 handler
        rt.dispatch_input(&key);
        assert_eq!(call_count.get(), 2); // Not 101!
    }

    #[test]
    fn test_use_input_receives_key() {
        let (_rt, cx) = setup_scope();
        let received = Rc::new(Cell::new(false));
        let r = received.clone();

        use_input(cx, move |key| {
            if key.is_char('x') {
                r.set(true);
            }
        });

        let key = Key::new(crossterm::event::KeyCode::Char('x'));
        _rt.dispatch_input(&key);

        assert!(received.get());
    }

    #[test]
    fn test_use_input_with_signal() {
        let (rt, cx) = setup_scope();

        let count = use_state(cx.clone(), || 0);
        let count_for_handler = count.clone();

        use_input(cx, move |key| {
            if key.is_char(' ') {
                count_for_handler.set(count_for_handler.get() + 1);
            }
        });

        assert_eq!(count.get(), 0);

        let space = Key::new(crossterm::event::KeyCode::Char(' '));
        rt.dispatch_input(&space);
        assert_eq!(count.get(), 1);

        rt.dispatch_input(&space);
        assert_eq!(count.get(), 2);

        let other = Key::new(crossterm::event::KeyCode::Char('a'));
        rt.dispatch_input(&other);
        assert_eq!(count.get(), 2); // Unchanged
    }

    #[test]
    #[should_panic(expected = "Hook order changed")]
    fn test_use_input_wrong_order_panics() {
        let (rt, cx) = setup_scope();

        // First render with input
        use_input(cx.clone(), |_| {});

        // Reset for re-render
        rt.reset_hook_cursor(cx.component_id);

        // Second render tries to use state at same position
        let _ = use_state(cx, || 0i32);
    }
}
