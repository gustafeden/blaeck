//! Focus management for interactive elements.
//!
//! Allows tab navigation between focusable components with optional
//! callbacks for focus/blur events.
//!
//! # Example
//!
//! ```ignore
//! let mut focus = FocusManager::new();
//!
//! // Register elements
//! focus.register(FocusId(1));
//! focus.register(FocusId(2));
//!
//! // Set up focus change callback
//! focus.on_focus_change(|event| {
//!     if let Some(id) = event.focused {
//!         println!("Focused: {:?}", id);
//!     }
//!     if let Some(id) = event.blurred {
//!         println!("Blurred: {:?}", id);
//!     }
//! });
//!
//! // Navigate - callbacks fire automatically
//! focus.focus_next();
//! ```

/// Unique identifier for a focusable element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FocusId(pub usize);

impl FocusId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

/// Focus state for an element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FocusState {
    /// Whether this element is currently focused.
    pub is_focused: bool,
    /// Whether focus is enabled for navigation.
    pub focus_enabled: bool,
}

impl Default for FocusState {
    fn default() -> Self {
        Self {
            is_focused: false,
            focus_enabled: true,
        }
    }
}

/// Event emitted when focus changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FocusEvent {
    /// The element that lost focus (if any).
    pub blurred: Option<FocusId>,
    /// The element that gained focus (if any).
    pub focused: Option<FocusId>,
}

impl FocusEvent {
    /// Returns true if this event represents a focus gain.
    pub fn is_focus(&self) -> bool {
        self.focused.is_some()
    }

    /// Returns true if this event represents a focus loss.
    pub fn is_blur(&self) -> bool {
        self.blurred.is_some()
    }
}

/// Type alias for focus change callback.
pub type FocusCallback = Box<dyn FnMut(FocusEvent)>;

/// Manages focus state across multiple elements.
#[derive(Default)]
pub struct FocusManager {
    /// List of focusable element IDs in order.
    elements: Vec<FocusId>,
    /// Currently focused element index.
    current_index: Option<usize>,
    /// Callback for focus changes.
    on_change: Option<FocusCallback>,
}

impl std::fmt::Debug for FocusManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FocusManager")
            .field("elements", &self.elements)
            .field("current_index", &self.current_index)
            .field("on_change", &self.on_change.as_ref().map(|_| "..."))
            .finish()
    }
}


impl FocusManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a callback that fires when focus changes.
    ///
    /// The callback receives a `FocusEvent` with information about
    /// which element lost focus and which gained focus.
    ///
    /// # Example
    ///
    /// ```ignore
    /// focus.on_focus_change(|event| {
    ///     if let Some(id) = event.focused {
    ///         println!("Element {:?} gained focus", id);
    ///     }
    ///     if let Some(id) = event.blurred {
    ///         println!("Element {:?} lost focus", id);
    ///     }
    /// });
    /// ```
    pub fn on_focus_change<F>(&mut self, callback: F)
    where
        F: FnMut(FocusEvent) + 'static,
    {
        self.on_change = Some(Box::new(callback));
    }

    /// Clear the focus change callback.
    pub fn clear_callback(&mut self) {
        self.on_change = None;
    }

    /// Emit a focus change event.
    fn emit_change(&mut self, blurred: Option<FocusId>, focused: Option<FocusId>) {
        if blurred == focused {
            return; // No actual change
        }
        if let Some(ref mut callback) = self.on_change {
            callback(FocusEvent { blurred, focused });
        }
    }

    /// Register a focusable element.
    pub fn register(&mut self, id: FocusId) {
        if !self.elements.contains(&id) {
            self.elements.push(id);
            // If this is the first element, focus it
            if self.current_index.is_none() {
                self.current_index = Some(0);
                self.emit_change(None, Some(id));
            }
        }
    }

    /// Unregister a focusable element.
    pub fn unregister(&mut self, id: FocusId) {
        if let Some(pos) = self.elements.iter().position(|&x| x == id) {
            let was_focused = self.is_focused(id);
            self.elements.remove(pos);

            // Adjust current index
            let new_focused = if let Some(current) = self.current_index {
                if current >= self.elements.len() {
                    self.current_index = if self.elements.is_empty() {
                        None
                    } else {
                        Some(self.elements.len() - 1)
                    };
                }
                self.focused()
            } else {
                None
            };

            // Emit event if focus changed
            if was_focused {
                self.emit_change(Some(id), new_focused);
            }
        }
    }

    /// Get the currently focused element ID.
    pub fn focused(&self) -> Option<FocusId> {
        self.current_index.map(|i| self.elements[i])
    }

    /// Check if a specific element is focused.
    pub fn is_focused(&self, id: FocusId) -> bool {
        self.focused() == Some(id)
    }

    /// Move focus to the next element.
    pub fn focus_next(&mut self) {
        if self.elements.is_empty() {
            return;
        }
        let old_focused = self.focused();
        self.current_index = Some(match self.current_index {
            Some(i) => (i + 1) % self.elements.len(),
            None => 0,
        });
        let new_focused = self.focused();
        self.emit_change(old_focused, new_focused);
    }

    /// Move focus to the previous element.
    pub fn focus_previous(&mut self) {
        if self.elements.is_empty() {
            return;
        }
        let old_focused = self.focused();
        self.current_index = Some(match self.current_index {
            Some(0) => self.elements.len() - 1,
            Some(i) => i - 1,
            None => self.elements.len() - 1,
        });
        let new_focused = self.focused();
        self.emit_change(old_focused, new_focused);
    }

    /// Focus a specific element by ID.
    pub fn focus(&mut self, id: FocusId) {
        if let Some(pos) = self.elements.iter().position(|&x| x == id) {
            let old_focused = self.focused();
            self.current_index = Some(pos);
            self.emit_change(old_focused, Some(id));
        }
    }

    /// Clear focus (blur all elements).
    pub fn blur(&mut self) {
        let old_focused = self.focused();
        self.current_index = None;
        self.emit_change(old_focused, None);
    }

    /// Get the number of registered focusable elements.
    pub fn count(&self) -> usize {
        self.elements.len()
    }

    /// Check if any element is currently focused.
    pub fn has_focus(&self) -> bool {
        self.current_index.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focus_id_new() {
        let id = FocusId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn test_focus_id_eq() {
        let id1 = FocusId(1);
        let id2 = FocusId(1);
        let id3 = FocusId(2);
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_focus_id_clone() {
        let id = FocusId(5);
        let cloned = id;
        assert_eq!(id, cloned);
    }

    #[test]
    fn test_focus_state_default() {
        let state = FocusState::default();
        assert!(!state.is_focused);
        assert!(state.focus_enabled);
    }

    #[test]
    fn test_focus_state_custom() {
        let state = FocusState {
            is_focused: true,
            focus_enabled: false,
        };
        assert!(state.is_focused);
        assert!(!state.focus_enabled);
    }

    #[test]
    fn test_focus_manager_new() {
        let fm = FocusManager::new();
        assert_eq!(fm.count(), 0);
        assert_eq!(fm.focused(), None);
    }

    #[test]
    fn test_focus_manager_register() {
        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.register(FocusId(2));
        assert_eq!(fm.count(), 2);
    }

    #[test]
    fn test_focus_manager_register_duplicate() {
        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.register(FocusId(1)); // Duplicate
        assert_eq!(fm.count(), 1);
    }

    #[test]
    fn test_focus_manager_first_element_focused() {
        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        assert_eq!(fm.focused(), Some(FocusId(1)));
    }

    #[test]
    fn test_focus_manager_focus_next() {
        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.register(FocusId(2));
        fm.register(FocusId(3));

        assert_eq!(fm.focused(), Some(FocusId(1)));
        fm.focus_next();
        assert_eq!(fm.focused(), Some(FocusId(2)));
        fm.focus_next();
        assert_eq!(fm.focused(), Some(FocusId(3)));
        fm.focus_next();
        assert_eq!(fm.focused(), Some(FocusId(1))); // Wraps around
    }

    #[test]
    fn test_focus_manager_focus_next_empty() {
        let mut fm = FocusManager::new();
        fm.focus_next(); // Should not panic
        assert_eq!(fm.focused(), None);
    }

    #[test]
    fn test_focus_manager_focus_next_from_none() {
        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.blur();
        fm.focus_next();
        assert_eq!(fm.focused(), Some(FocusId(1)));
    }

    #[test]
    fn test_focus_manager_focus_previous() {
        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.register(FocusId(2));
        fm.register(FocusId(3));

        fm.focus_previous();
        assert_eq!(fm.focused(), Some(FocusId(3))); // Wraps around
        fm.focus_previous();
        assert_eq!(fm.focused(), Some(FocusId(2)));
    }

    #[test]
    fn test_focus_manager_focus_previous_empty() {
        let mut fm = FocusManager::new();
        fm.focus_previous(); // Should not panic
        assert_eq!(fm.focused(), None);
    }

    #[test]
    fn test_focus_manager_focus_previous_from_none() {
        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.register(FocusId(2));
        fm.blur();
        fm.focus_previous();
        assert_eq!(fm.focused(), Some(FocusId(2)));
    }

    #[test]
    fn test_focus_manager_is_focused() {
        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.register(FocusId(2));

        assert!(fm.is_focused(FocusId(1)));
        assert!(!fm.is_focused(FocusId(2)));
    }

    #[test]
    fn test_focus_manager_focus_specific() {
        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.register(FocusId(2));
        fm.register(FocusId(3));

        fm.focus(FocusId(3));
        assert_eq!(fm.focused(), Some(FocusId(3)));
    }

    #[test]
    fn test_focus_manager_focus_nonexistent() {
        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.focus(FocusId(99)); // Doesn't exist
        assert_eq!(fm.focused(), Some(FocusId(1))); // Focus unchanged
    }

    #[test]
    fn test_focus_manager_blur() {
        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.blur();
        assert_eq!(fm.focused(), None);
    }

    #[test]
    fn test_focus_manager_unregister() {
        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.register(FocusId(2));
        fm.unregister(FocusId(1));
        assert_eq!(fm.count(), 1);
        assert_eq!(fm.focused(), Some(FocusId(2)));
    }

    #[test]
    fn test_focus_manager_unregister_nonexistent() {
        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.unregister(FocusId(99)); // Doesn't exist
        assert_eq!(fm.count(), 1);
    }

    #[test]
    fn test_focus_manager_unregister_focused() {
        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.register(FocusId(2));
        fm.register(FocusId(3));
        fm.focus(FocusId(3)); // Focus last element
        fm.unregister(FocusId(3)); // Remove focused element
        assert_eq!(fm.focused(), Some(FocusId(2))); // Focus moves to previous last
    }

    #[test]
    fn test_focus_manager_unregister_all() {
        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.unregister(FocusId(1));
        assert_eq!(fm.count(), 0);
        assert_eq!(fm.focused(), None);
    }

    #[test]
    fn test_focus_manager_count() {
        let mut fm = FocusManager::new();
        assert_eq!(fm.count(), 0);
        fm.register(FocusId(1));
        assert_eq!(fm.count(), 1);
        fm.register(FocusId(2));
        assert_eq!(fm.count(), 2);
        fm.unregister(FocusId(1));
        assert_eq!(fm.count(), 1);
    }

    #[test]
    fn test_focus_event_is_focus() {
        let event = FocusEvent {
            blurred: None,
            focused: Some(FocusId(1)),
        };
        assert!(event.is_focus());
        assert!(!event.is_blur());
    }

    #[test]
    fn test_focus_event_is_blur() {
        let event = FocusEvent {
            blurred: Some(FocusId(1)),
            focused: None,
        };
        assert!(!event.is_focus());
        assert!(event.is_blur());
    }

    #[test]
    fn test_focus_event_both() {
        let event = FocusEvent {
            blurred: Some(FocusId(1)),
            focused: Some(FocusId(2)),
        };
        assert!(event.is_focus());
        assert!(event.is_blur());
    }

    #[test]
    fn test_focus_manager_has_focus() {
        let mut fm = FocusManager::new();
        assert!(!fm.has_focus());
        fm.register(FocusId(1));
        assert!(fm.has_focus());
        fm.blur();
        assert!(!fm.has_focus());
    }

    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_focus_callback_on_register() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let events_clone = events.clone();

        let mut fm = FocusManager::new();
        fm.on_focus_change(move |e| {
            events_clone.borrow_mut().push(e);
        });

        fm.register(FocusId(1));

        let captured = events.borrow();
        assert_eq!(captured.len(), 1);
        assert_eq!(captured[0].blurred, None);
        assert_eq!(captured[0].focused, Some(FocusId(1)));
    }

    #[test]
    fn test_focus_callback_on_focus_next() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let events_clone = events.clone();

        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.register(FocusId(2));

        fm.on_focus_change(move |e| {
            events_clone.borrow_mut().push(e);
        });

        fm.focus_next();

        let captured = events.borrow();
        assert_eq!(captured.len(), 1);
        assert_eq!(captured[0].blurred, Some(FocusId(1)));
        assert_eq!(captured[0].focused, Some(FocusId(2)));
    }

    #[test]
    fn test_focus_callback_on_focus_previous() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let events_clone = events.clone();

        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.register(FocusId(2));

        fm.on_focus_change(move |e| {
            events_clone.borrow_mut().push(e);
        });

        fm.focus_previous(); // Wraps to last element

        let captured = events.borrow();
        assert_eq!(captured.len(), 1);
        assert_eq!(captured[0].blurred, Some(FocusId(1)));
        assert_eq!(captured[0].focused, Some(FocusId(2)));
    }

    #[test]
    fn test_focus_callback_on_blur() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let events_clone = events.clone();

        let mut fm = FocusManager::new();
        fm.register(FocusId(1));

        fm.on_focus_change(move |e| {
            events_clone.borrow_mut().push(e);
        });

        fm.blur();

        let captured = events.borrow();
        assert_eq!(captured.len(), 1);
        assert_eq!(captured[0].blurred, Some(FocusId(1)));
        assert_eq!(captured[0].focused, None);
    }

    #[test]
    fn test_focus_callback_on_focus_specific() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let events_clone = events.clone();

        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.register(FocusId(2));
        fm.register(FocusId(3));

        fm.on_focus_change(move |e| {
            events_clone.borrow_mut().push(e);
        });

        fm.focus(FocusId(3));

        let captured = events.borrow();
        assert_eq!(captured.len(), 1);
        assert_eq!(captured[0].blurred, Some(FocusId(1)));
        assert_eq!(captured[0].focused, Some(FocusId(3)));
    }

    #[test]
    fn test_focus_callback_no_change_no_event() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let events_clone = events.clone();

        let mut fm = FocusManager::new();
        fm.register(FocusId(1));

        fm.on_focus_change(move |e| {
            events_clone.borrow_mut().push(e);
        });

        // Focus same element - no change
        fm.focus(FocusId(1));

        let captured = events.borrow();
        assert_eq!(captured.len(), 0);
    }

    #[test]
    fn test_focus_callback_clear() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let events_clone = events.clone();

        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.register(FocusId(2));

        fm.on_focus_change(move |e| {
            events_clone.borrow_mut().push(e);
        });

        fm.clear_callback();
        fm.focus_next();

        let captured = events.borrow();
        assert_eq!(captured.len(), 0); // No events after clearing callback
    }

    #[test]
    fn test_focus_callback_on_unregister_focused() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let events_clone = events.clone();

        let mut fm = FocusManager::new();
        fm.register(FocusId(1));
        fm.register(FocusId(2));
        fm.focus(FocusId(2));

        // Clear events from setup
        events.borrow_mut().clear();

        fm.on_focus_change(move |e| {
            events_clone.borrow_mut().push(e);
        });

        fm.unregister(FocusId(2)); // Remove focused element

        let captured = events.borrow();
        assert_eq!(captured.len(), 1);
        assert_eq!(captured[0].blurred, Some(FocusId(2)));
        assert_eq!(captured[0].focused, Some(FocusId(1))); // Focus moves to remaining element
    }
}
