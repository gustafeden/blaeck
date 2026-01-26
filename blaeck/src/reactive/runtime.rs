//! Runtime state management for the reactive system.
//!
//! The runtime holds all signal values, component instances, and input handlers.
//! It uses a slot-map arena for efficient ID-based storage.

use super::instance::ComponentInstance;
use crate::input::Key;
use crate::timeline::PlayingTimeline;
use slotmap::{new_key_type, SlotMap};
use std::any::Any;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

// Define typed keys for the slot maps
new_key_type! {
    /// Unique identifier for a signal in the runtime.
    pub struct SignalId;

    /// Unique identifier for a component instance.
    pub struct ComponentId;

    /// Unique identifier for an input handler.
    pub struct InputHandlerId;

    /// Unique identifier for a timeline in the runtime.
    pub struct TimelineId;
}

/// Handle to the runtime, cheaply clonable.
///
/// This is the main entry point for accessing runtime state.
/// Uses `Rc<RefCell<...>>` for interior mutability in single-threaded context.
#[derive(Clone)]
pub struct RuntimeHandle(pub(crate) Rc<RefCell<RuntimeInner>>);

impl RuntimeHandle {
    /// Create a new runtime.
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(RuntimeInner::new())))
    }

    /// Create a new signal with an initial value.
    pub fn create_signal<T: 'static>(&self, value: T) -> SignalId {
        self.0.borrow_mut().signals.insert(Box::new(value))
    }

    /// Get a signal's value by cloning it.
    ///
    /// # Panics
    /// Panics if the signal ID is invalid or the type doesn't match.
    pub fn get_signal<T: Clone + 'static>(&self, id: SignalId) -> T {
        let inner = self.0.borrow();
        let boxed = inner
            .signals
            .get(id)
            .expect("Signal not found - invalid SignalId");
        boxed
            .downcast_ref::<T>()
            .expect("Signal type mismatch")
            .clone()
    }

    /// Set a signal's value and mark the runtime as needing a render.
    ///
    /// # Panics
    /// Panics if the signal ID is invalid or the type doesn't match.
    pub fn set_signal<T: 'static>(&self, id: SignalId, value: T) {
        {
            let mut inner = self.0.borrow_mut();
            let boxed = inner
                .signals
                .get_mut(id)
                .expect("Signal not found - invalid SignalId");
            *boxed
                .downcast_mut::<T>()
                .expect("Signal type mismatch") = value;
        }
        self.mark_dirty();
    }

    /// Mark the runtime as needing a re-render.
    pub fn mark_dirty(&self) {
        self.0.borrow().needs_render.set(true);
    }

    /// Check if the runtime needs a re-render.
    pub fn needs_render(&self) -> bool {
        self.0.borrow().needs_render.get()
    }

    /// Clear the dirty flag after rendering.
    pub fn clear_dirty(&self) {
        self.0.borrow().needs_render.set(false);
    }

    /// Create a new component instance.
    pub fn create_instance(&self) -> ComponentId {
        self.0
            .borrow_mut()
            .instances
            .insert(ComponentInstance::new())
    }

    /// Set the current component instance being rendered.
    pub fn set_current_instance(&self, id: Option<ComponentId>) {
        self.0.borrow_mut().current_instance = id;
    }

    /// Get the current component instance ID.
    pub fn current_instance(&self) -> Option<ComponentId> {
        self.0.borrow().current_instance
    }

    /// Reset the hook cursor for a component instance before rendering.
    pub fn reset_hook_cursor(&self, id: ComponentId) {
        if let Some(instance) = self.0.borrow_mut().instances.get_mut(id) {
            instance.reset_cursor();
        }
    }

    /// Register an input handler. Returns the handler ID.
    ///
    /// The handler is stored and will be called when input is received.
    pub fn register_input_handler<F>(&self, handler: F) -> InputHandlerId
    where
        F: Fn(&Key) + 'static,
    {
        self.0
            .borrow_mut()
            .input_handlers
            .insert(Box::new(handler))
    }

    /// Check if an input handler with the given ID exists.
    pub fn has_input_handler(&self, id: InputHandlerId) -> bool {
        self.0.borrow().input_handlers.contains_key(id)
    }

    /// Dispatch a key event to all registered input handlers.
    pub fn dispatch_input(&self, key: &Key) {
        // Collect handler pointers to avoid borrow issues during dispatch.
        // We need to collect because a handler might trigger state changes
        // that would require borrowing the runtime again.
        let handlers: Vec<*const Box<dyn Fn(&Key)>> = self
            .0
            .borrow()
            .input_handlers
            .values()
            .map(|h| h as *const _)
            .collect();

        // Call each handler.
        // SAFETY: We still hold the Rc, handlers are 'static, and we're single-threaded.
        // The handlers cannot be removed during this iteration because we hold the only
        // reference and removals would require mutable access.
        for handler_ptr in handlers {
            let handler = unsafe { &*handler_ptr };
            handler(key);
        }
    }

    /// Create a new timeline from a PlayingTimeline.
    pub fn create_timeline(&self, timeline: PlayingTimeline) -> TimelineId {
        self.0.borrow_mut().timelines.insert(timeline)
    }

    /// Check if a timeline with the given ID exists.
    pub fn has_timeline(&self, id: TimelineId) -> bool {
        self.0.borrow().timelines.contains_key(id)
    }

    /// Access a timeline immutably.
    pub fn with_timeline<R, F: FnOnce(&PlayingTimeline) -> R>(
        &self,
        id: TimelineId,
        f: F,
    ) -> Option<R> {
        self.0.borrow().timelines.get(id).map(f)
    }

    /// Access a timeline mutably.
    pub fn with_timeline_mut<R, F: FnOnce(&mut PlayingTimeline) -> R>(
        &self,
        id: TimelineId,
        f: F,
    ) -> Option<R> {
        self.0.borrow_mut().timelines.get_mut(id).map(f)
    }

    /// Access a component instance.
    pub fn with_instance<R, F: FnOnce(&ComponentInstance) -> R>(
        &self,
        id: ComponentId,
        f: F,
    ) -> Option<R> {
        self.0.borrow().instances.get(id).map(f)
    }

    /// Mutably access a component instance.
    pub fn with_instance_mut<R, F: FnOnce(&mut ComponentInstance) -> R>(
        &self,
        id: ComponentId,
        f: F,
    ) -> Option<R> {
        self.0.borrow_mut().instances.get_mut(id).map(f)
    }
}

impl Default for RuntimeHandle {
    fn default() -> Self {
        Self::new()
    }
}

/// Inner runtime state.
///
/// This struct holds all the actual data; `RuntimeHandle` provides safe access.
pub struct RuntimeInner {
    /// Signal storage - maps SignalId to boxed values.
    pub(crate) signals: SlotMap<SignalId, Box<dyn Any>>,

    /// Component instances - maps ComponentId to instance data.
    pub(crate) instances: SlotMap<ComponentId, ComponentInstance>,

    /// Currently rendering component instance.
    pub(crate) current_instance: Option<ComponentId>,

    /// Input handlers - maps InputHandlerId to handler functions.
    pub(crate) input_handlers: SlotMap<InputHandlerId, Box<dyn Fn(&Key)>>,

    /// Timeline storage - maps TimelineId to playing timelines.
    pub(crate) timelines: SlotMap<TimelineId, PlayingTimeline>,

    /// Whether the UI needs to be re-rendered.
    ///
    /// Uses `Cell` for interior mutability without full borrow.
    pub(crate) needs_render: Cell<bool>,
}

impl RuntimeInner {
    /// Create a new runtime inner state.
    pub fn new() -> Self {
        Self {
            signals: SlotMap::with_key(),
            instances: SlotMap::with_key(),
            current_instance: None,
            input_handlers: SlotMap::with_key(),
            timelines: SlotMap::with_key(),
            needs_render: Cell::new(false),
        }
    }
}

impl Default for RuntimeInner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_create() {
        let rt = RuntimeHandle::new();
        assert!(!rt.needs_render());
    }

    #[test]
    fn test_signal_create_and_get() {
        let rt = RuntimeHandle::new();
        let id = rt.create_signal(42i32);
        assert_eq!(rt.get_signal::<i32>(id), 42);
    }

    #[test]
    fn test_signal_set() {
        let rt = RuntimeHandle::new();
        let id = rt.create_signal(0i32);
        rt.set_signal(id, 100);
        assert_eq!(rt.get_signal::<i32>(id), 100);
    }

    #[test]
    fn test_signal_set_marks_dirty() {
        let rt = RuntimeHandle::new();
        let id = rt.create_signal(0i32);
        assert!(!rt.needs_render());
        rt.set_signal(id, 1);
        assert!(rt.needs_render());
    }

    #[test]
    fn test_clear_dirty() {
        let rt = RuntimeHandle::new();
        rt.mark_dirty();
        assert!(rt.needs_render());
        rt.clear_dirty();
        assert!(!rt.needs_render());
    }

    #[test]
    fn test_signal_string() {
        let rt = RuntimeHandle::new();
        let id = rt.create_signal(String::from("hello"));
        assert_eq!(rt.get_signal::<String>(id), "hello");
        rt.set_signal(id, String::from("world"));
        assert_eq!(rt.get_signal::<String>(id), "world");
    }

    #[test]
    fn test_multiple_signals() {
        let rt = RuntimeHandle::new();
        let id1 = rt.create_signal(10i32);
        let id2 = rt.create_signal(20i32);
        let id3 = rt.create_signal(30i32);

        assert_eq!(rt.get_signal::<i32>(id1), 10);
        assert_eq!(rt.get_signal::<i32>(id2), 20);
        assert_eq!(rt.get_signal::<i32>(id3), 30);
    }

    #[test]
    fn test_instance_create() {
        let rt = RuntimeHandle::new();
        let id = rt.create_instance();
        assert!(rt.with_instance(id, |_| ()).is_some());
    }

    #[test]
    fn test_current_instance() {
        let rt = RuntimeHandle::new();
        let id = rt.create_instance();
        assert!(rt.current_instance().is_none());
        rt.set_current_instance(Some(id));
        assert_eq!(rt.current_instance(), Some(id));
        rt.set_current_instance(None);
        assert!(rt.current_instance().is_none());
    }

    #[test]
    fn test_input_handler_registration() {
        let rt = RuntimeHandle::new();
        let called = Rc::new(Cell::new(false));
        let called_clone = called.clone();

        let handler_id = rt.register_input_handler(move |_key| {
            called_clone.set(true);
        });

        assert!(rt.has_input_handler(handler_id));

        let key = Key::new(crossterm::event::KeyCode::Char('a'));
        rt.dispatch_input(&key);

        assert!(called.get());
    }

    #[test]
    fn test_multiple_input_handlers() {
        let rt = RuntimeHandle::new();
        let count = Rc::new(Cell::new(0));
        let count1 = count.clone();
        let count2 = count.clone();

        rt.register_input_handler(move |_| {
            count1.set(count1.get() + 1);
        });
        rt.register_input_handler(move |_| {
            count2.set(count2.get() + 1);
        });

        let key = Key::new(crossterm::event::KeyCode::Char('a'));
        rt.dispatch_input(&key);

        assert_eq!(count.get(), 2);
    }

    #[test]
    fn test_runtime_clone_shares_state() {
        let rt1 = RuntimeHandle::new();
        let rt2 = rt1.clone();

        let id = rt1.create_signal(0i32);
        rt2.set_signal(id, 42);

        assert_eq!(rt1.get_signal::<i32>(id), 42);
    }
}
