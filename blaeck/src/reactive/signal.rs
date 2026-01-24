//! Reactive signals for state management.
//!
//! Signals are the primary primitive for reactive state. When a signal's
//! value changes via `set()`, the runtime is marked dirty and will re-render.

use super::runtime::{RuntimeHandle, SignalId};
use std::fmt;
use std::marker::PhantomData;

/// A reactive signal holding a value of type `T`.
///
/// Signals are created via [`use_state`](super::use_state) and provide
/// `get()` and `set()` methods for reading and updating state.
///
/// # Type Safety
///
/// The type `T` is tracked at compile time via `PhantomData`. Attempting
/// to `get()` or `set()` with the wrong type will panic at runtime.
///
/// # Example
///
/// ```ignore
/// let count = use_state(cx, || 0);
/// let current = count.get();  // Read value
/// count.set(current + 1);     // Update value (triggers re-render)
/// ```
///
/// # Clone Behavior
///
/// Signals are cheap to clone - they only contain an ID and a reference
/// to the runtime. The actual value lives in the runtime's arena.
pub struct Signal<T> {
    /// Unique identifier for this signal in the runtime.
    pub(crate) id: SignalId,

    /// Handle to the runtime that owns the signal's value.
    pub(crate) rt: RuntimeHandle,

    /// Phantom type marker for the value type.
    pub(crate) _marker: PhantomData<T>,
}

impl<T: Clone + 'static> Signal<T> {
    /// Get the current value by cloning it.
    ///
    /// # Panics
    ///
    /// Panics if the signal was created with a different type `T`.
    /// This shouldn't happen in normal use since `use_state` returns
    /// a properly typed `Signal<T>`.
    pub fn get(&self) -> T {
        self.rt.get_signal(self.id)
    }
}

impl<T: 'static> Signal<T> {
    /// Set a new value and trigger a re-render.
    ///
    /// After calling `set()`, the runtime is marked dirty and the
    /// component will re-render on the next frame.
    ///
    /// # Panics
    ///
    /// Panics if the signal was created with a different type `T`.
    pub fn set(&self, value: T) {
        self.rt.set_signal(self.id, value);
    }

    /// Get the signal's ID.
    ///
    /// This is primarily useful for debugging and testing.
    pub fn id(&self) -> SignalId {
        self.id
    }
}

impl<T: Clone + 'static> Signal<T> {
    /// Update the value using a function.
    ///
    /// This is a convenience method that gets the current value,
    /// applies the function, and sets the result.
    ///
    /// # Example
    ///
    /// ```ignore
    /// count.update(|n| n + 1);
    /// // Equivalent to:
    /// // count.set(count.get() + 1);
    /// ```
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(T) -> T,
    {
        let current = self.get();
        self.set(f(current));
    }
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            rt: self.rt.clone(),
            _marker: PhantomData,
        }
    }
}

// Note: Signal cannot implement Copy because RuntimeHandle contains Rc.
// However, Signal is cheap to clone and can be used in closures with `move`.

impl<T> fmt::Debug for Signal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Signal")
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_get_set() {
        let rt = RuntimeHandle::new();
        let id = rt.create_signal(42i32);
        let signal: Signal<i32> = Signal {
            id,
            rt,
            _marker: PhantomData,
        };

        assert_eq!(signal.get(), 42);
        signal.set(100);
        assert_eq!(signal.get(), 100);
    }

    #[test]
    fn test_signal_update() {
        let rt = RuntimeHandle::new();
        let id = rt.create_signal(5i32);
        let signal: Signal<i32> = Signal {
            id,
            rt,
            _marker: PhantomData,
        };

        signal.update(|n| n * 2);
        assert_eq!(signal.get(), 10);
    }

    #[test]
    fn test_signal_clone() {
        let rt = RuntimeHandle::new();
        let id = rt.create_signal(0i32);
        let signal1: Signal<i32> = Signal {
            id,
            rt,
            _marker: PhantomData,
        };
        let signal2 = signal1.clone();

        signal1.set(42);
        assert_eq!(signal2.get(), 42);
    }

    #[test]
    fn test_signal_multiple_clones_share_state() {
        let rt = RuntimeHandle::new();
        let id = rt.create_signal(0i32);
        let signal1: Signal<i32> = Signal {
            id,
            rt,
            _marker: PhantomData,
        };
        let signal2 = signal1.clone();

        signal1.set(10);
        assert_eq!(signal2.get(), 10);
    }

    #[test]
    fn test_signal_string() {
        let rt = RuntimeHandle::new();
        let id = rt.create_signal(String::from("hello"));
        let signal: Signal<String> = Signal {
            id,
            rt,
            _marker: PhantomData,
        };

        assert_eq!(signal.get(), "hello");
        signal.set(String::from("world"));
        assert_eq!(signal.get(), "world");
    }

    #[test]
    fn test_signal_vec() {
        let rt = RuntimeHandle::new();
        let id = rt.create_signal(vec![1, 2, 3]);
        let signal: Signal<Vec<i32>> = Signal {
            id,
            rt,
            _marker: PhantomData,
        };

        assert_eq!(signal.get(), vec![1, 2, 3]);
        signal.update(|mut v| {
            v.push(4);
            v
        });
        assert_eq!(signal.get(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_signal_debug() {
        let rt = RuntimeHandle::new();
        let id = rt.create_signal(0i32);
        let signal: Signal<i32> = Signal {
            id,
            rt,
            _marker: PhantomData,
        };

        let debug_str = format!("{:?}", signal);
        assert!(debug_str.contains("Signal"));
    }

    #[test]
    fn test_signal_marks_dirty() {
        let rt = RuntimeHandle::new();
        let id = rt.create_signal(0i32);
        let signal: Signal<i32> = Signal {
            id,
            rt: rt.clone(),
            _marker: PhantomData,
        };

        assert!(!rt.needs_render());
        signal.set(1);
        assert!(rt.needs_render());
    }
}
