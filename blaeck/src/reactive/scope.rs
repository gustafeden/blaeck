//! Scope - the context passed to reactive components.
//!
//! A `Scope` provides access to the runtime and identifies which
//! component instance is currently rendering. It's passed to every
//! reactive component function.

use super::runtime::{ComponentId, RuntimeHandle};

/// Context passed to reactive component functions.
///
/// The scope provides:
/// - Access to the runtime for hooks
/// - Component instance identity for hook storage
///
/// # Example
///
/// ```ignore
/// fn my_component(cx: Scope) -> Element {
///     let state = use_state(cx, || 0);
///     // ...
/// }
/// ```
///
/// Scopes are lightweight (just two small values) and can be freely copied.
#[derive(Clone)]
pub struct Scope {
    /// Handle to the runtime.
    pub(crate) rt: RuntimeHandle,

    /// The component instance this scope belongs to.
    pub(crate) component_id: ComponentId,
}

impl Scope {
    /// Create a new scope for a component.
    pub fn new(rt: RuntimeHandle, component_id: ComponentId) -> Self {
        Self { rt, component_id }
    }

    /// Get a clone of the runtime handle.
    ///
    /// This is useful when you need to access the runtime from
    /// within a closure or callback.
    pub fn runtime(&self) -> RuntimeHandle {
        self.rt.clone()
    }

    /// Get the component instance ID.
    ///
    /// This identifies which component this scope belongs to.
    pub fn component_id(&self) -> ComponentId {
        self.component_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_creation() {
        let rt = RuntimeHandle::new();
        let component_id = rt.create_instance();
        let scope = Scope::new(rt.clone(), component_id);

        assert_eq!(scope.component_id(), component_id);
    }

    #[test]
    fn test_scope_clone() {
        let rt = RuntimeHandle::new();
        let component_id = rt.create_instance();
        let scope1 = Scope::new(rt.clone(), component_id);
        let scope2 = scope1.clone();

        assert_eq!(scope1.component_id(), scope2.component_id());
    }

    #[test]
    fn test_scope_runtime_access() {
        let rt = RuntimeHandle::new();
        let component_id = rt.create_instance();
        let scope = Scope::new(rt.clone(), component_id);

        // Create a signal through the scope's runtime
        let signal_id = scope.runtime().create_signal(42i32);
        assert_eq!(rt.get_signal::<i32>(signal_id), 42);
    }
}
