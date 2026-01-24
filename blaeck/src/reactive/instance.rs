//! Component instance state for the reactive system.
//!
//! Each component has an instance that tracks:
//! - Hook storage (signals, input handlers, etc.)
//! - Hook cursor (for consistent hook ordering)
//! - Cleanup callbacks (for future use_effect support)

use super::runtime::{InputHandlerId, SignalId};

/// Represents a slot in the hooks array.
///
/// Each hook type has its own variant to ensure type safety
/// and detect hook order changes.
#[derive(Debug, Clone)]
pub enum HookSlot {
    /// A state signal created by `use_state`.
    State(SignalId),

    /// An input handler created by `use_input`.
    Input(InputHandlerId),
    // Future hooks (v0.3.0+):
    // Effect { cleanup: Option<Box<dyn FnOnce()>>, deps: Vec<...> },
    // Memo { value: Box<dyn Any>, deps: Vec<...> },
    // Const(Box<dyn Any>),
}

/// State for a single component instance.
///
/// This tracks all hooks used by the component and ensures
/// they are called in consistent order across renders.
pub struct ComponentInstance {
    /// Hook storage - each hook gets a slot.
    pub(crate) hooks: Vec<HookSlot>,

    /// Current position in hooks array during render.
    /// Reset to 0 at start of each render.
    pub(crate) hook_cursor: usize,

    /// Cleanup callbacks to run when component unmounts.
    /// Reserved for future use_effect support.
    #[allow(dead_code)]
    pub(crate) cleanup: Vec<Box<dyn FnOnce()>>,
}

impl ComponentInstance {
    /// Create a new component instance.
    pub fn new() -> Self {
        Self {
            hooks: Vec::new(),
            hook_cursor: 0,
            cleanup: Vec::new(),
        }
    }

    /// Reset the hook cursor before rendering.
    ///
    /// This must be called at the start of each render cycle
    /// so hooks are read from the beginning.
    pub fn reset_cursor(&mut self) {
        self.hook_cursor = 0;
    }

    /// Get the current hook cursor position.
    pub fn cursor(&self) -> usize {
        self.hook_cursor
    }

    /// Advance the hook cursor and return the previous position.
    pub fn advance_cursor(&mut self) -> usize {
        let pos = self.hook_cursor;
        self.hook_cursor += 1;
        pos
    }

    /// Get the hook at the current cursor position, if it exists.
    pub fn current_hook(&self) -> Option<&HookSlot> {
        self.hooks.get(self.hook_cursor)
    }

    /// Push a new hook slot.
    pub fn push_hook(&mut self, slot: HookSlot) {
        self.hooks.push(slot);
    }

    /// Get a hook at a specific index.
    pub fn get_hook(&self, index: usize) -> Option<&HookSlot> {
        self.hooks.get(index)
    }

    /// Get the total number of hooks.
    pub fn hook_count(&self) -> usize {
        self.hooks.len()
    }

    /// Verify that the hook cursor matches the expected count.
    ///
    /// This catches bugs where hooks are called conditionally,
    /// which would break the cursor-based lookup.
    ///
    /// # Panics
    /// Panics if the cursor doesn't match the hook count.
    #[allow(dead_code)]
    pub fn verify_hook_count(&self) {
        if self.hook_cursor != self.hooks.len() {
            panic!(
                "Hook count mismatch: expected {} hooks, but {} were called. \
                 Hooks must be called unconditionally and in the same order every render.",
                self.hooks.len(),
                self.hook_cursor
            );
        }
    }
}

impl Default for ComponentInstance {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slotmap::SlotMap;

    #[test]
    fn test_instance_new() {
        let instance = ComponentInstance::new();
        assert_eq!(instance.hook_cursor, 0);
        assert!(instance.hooks.is_empty());
    }

    #[test]
    fn test_reset_cursor() {
        let mut instance = ComponentInstance::new();
        instance.hook_cursor = 5;
        instance.reset_cursor();
        assert_eq!(instance.hook_cursor, 0);
    }

    #[test]
    fn test_advance_cursor() {
        let mut instance = ComponentInstance::new();
        assert_eq!(instance.advance_cursor(), 0);
        assert_eq!(instance.advance_cursor(), 1);
        assert_eq!(instance.advance_cursor(), 2);
        assert_eq!(instance.cursor(), 3);
    }

    #[test]
    fn test_push_and_get_hook() {
        let mut signals: SlotMap<SignalId, ()> = SlotMap::with_key();
        let signal_id = signals.insert(());

        let mut instance = ComponentInstance::new();
        instance.push_hook(HookSlot::State(signal_id));

        assert_eq!(instance.hook_count(), 1);
        assert!(matches!(instance.get_hook(0), Some(HookSlot::State(_))));
    }

    #[test]
    fn test_current_hook() {
        let mut signals: SlotMap<SignalId, ()> = SlotMap::with_key();
        let signal_id = signals.insert(());

        let mut instance = ComponentInstance::new();
        assert!(instance.current_hook().is_none());

        instance.push_hook(HookSlot::State(signal_id));
        assert!(instance.current_hook().is_some());

        instance.advance_cursor();
        assert!(instance.current_hook().is_none());
    }

    #[test]
    fn test_hook_slot_clone() {
        let mut signals: SlotMap<SignalId, ()> = SlotMap::with_key();
        let signal_id = signals.insert(());

        let slot = HookSlot::State(signal_id);
        let cloned = slot.clone();

        if let (HookSlot::State(id1), HookSlot::State(id2)) = (&slot, &cloned) {
            assert_eq!(id1, id2);
        } else {
            panic!("Clone should preserve variant");
        }
    }

    #[test]
    fn test_hook_slot_debug() {
        let mut signals: SlotMap<SignalId, ()> = SlotMap::with_key();
        let signal_id = signals.insert(());

        let slot = HookSlot::State(signal_id);
        let debug_str = format!("{:?}", slot);
        assert!(debug_str.contains("State"));
    }
}
