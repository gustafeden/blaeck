//! Input handling for interactive terminal applications.
//!
//! Provides key event types and an input reader that wraps crossterm.

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

/// A key press event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl Key {
    pub fn new(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::NONE,
        }
    }

    pub fn with_ctrl(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::CONTROL,
        }
    }

    pub fn with_alt(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::ALT,
        }
    }

    /// Check if this is Ctrl+C
    pub fn is_ctrl_c(&self) -> bool {
        self.code == KeyCode::Char('c') && self.modifiers.contains(KeyModifiers::CONTROL)
    }

    /// Check if this is the escape key
    pub fn is_escape(&self) -> bool {
        self.code == KeyCode::Esc
    }

    /// Check if this is the tab key
    pub fn is_tab(&self) -> bool {
        self.code == KeyCode::Tab
    }

    /// Check if this is shift+tab (backtab)
    pub fn is_backtab(&self) -> bool {
        self.code == KeyCode::BackTab
    }

    /// Check if this is enter/return
    pub fn is_enter(&self) -> bool {
        self.code == KeyCode::Enter
    }

    /// Check if this matches a specific character
    pub fn is_char(&self, c: char) -> bool {
        self.code == KeyCode::Char(c) && self.modifiers.is_empty()
    }

    /// Check if this is backspace
    pub fn is_backspace(&self) -> bool {
        self.code == KeyCode::Backspace
    }

    /// Get the character if this is a character key (no modifiers except shift)
    pub fn as_char(&self) -> Option<char> {
        match self.code {
            KeyCode::Char(c) if self.modifiers.is_empty() || self.modifiers == KeyModifiers::SHIFT => Some(c),
            _ => None,
        }
    }
}

impl From<KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        Self {
            code: event.code,
            modifiers: event.modifiers,
        }
    }
}

/// Polls for keyboard input with a timeout.
/// Returns Some(Key) if a key was pressed, None if timeout.
pub fn poll_key(timeout: Duration) -> std::io::Result<Option<Key>> {
    if event::poll(timeout)? {
        if let Event::Key(key_event) = event::read()? {
            return Ok(Some(Key::from(key_event)));
        }
    }
    Ok(None)
}

/// Blocks until a key is pressed.
pub fn read_key() -> std::io::Result<Key> {
    loop {
        if let Event::Key(key_event) = event::read()? {
            return Ok(Key::from(key_event));
        }
    }
}

/// Input handler that can be used with App::run.
pub trait InputHandler {
    fn handle(&mut self, key: &Key);
}

/// Arrow key direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arrow {
    Up,
    Down,
    Left,
    Right,
}

/// Builder for matching keys and executing handlers.
pub struct KeyMatcher<'a, T> {
    key: &'a Key,
    state: &'a mut T,
    handled: bool,
}

impl<'a, T> KeyMatcher<'a, T> {
    pub fn new(key: &'a Key, state: &'a mut T) -> Self {
        Self {
            key,
            state,
            handled: false,
        }
    }

    /// Execute handler if the key matches the given character.
    pub fn on_char<F>(mut self, c: char, f: F) -> Self
    where
        F: FnOnce(&mut T),
    {
        if !self.handled && self.key.is_char(c) {
            f(self.state);
            self.handled = true;
        }
        self
    }

    /// Execute handler if the key is Enter.
    pub fn on_enter<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut T),
    {
        if !self.handled && self.key.is_enter() {
            f(self.state);
            self.handled = true;
        }
        self
    }

    /// Execute handler if the key is Escape.
    pub fn on_escape<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut T),
    {
        if !self.handled && self.key.is_escape() {
            f(self.state);
            self.handled = true;
        }
        self
    }

    /// Execute handler if the key is Tab.
    pub fn on_tab<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut T),
    {
        if !self.handled && self.key.is_tab() {
            f(self.state);
            self.handled = true;
        }
        self
    }

    /// Execute handler if the key is Shift+Tab (backtab).
    pub fn on_backtab<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut T),
    {
        if !self.handled && self.key.is_backtab() {
            f(self.state);
            self.handled = true;
        }
        self
    }

    /// Execute handler if the key is an arrow key.
    pub fn on_arrow<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut T, Arrow),
    {
        if !self.handled {
            let arrow = match self.key.code {
                KeyCode::Up => Some(Arrow::Up),
                KeyCode::Down => Some(Arrow::Down),
                KeyCode::Left => Some(Arrow::Left),
                KeyCode::Right => Some(Arrow::Right),
                _ => None,
            };
            if let Some(dir) = arrow {
                f(self.state, dir);
                self.handled = true;
            }
        }
        self
    }

    /// Execute handler for any unhandled key.
    pub fn otherwise<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut T, &Key),
    {
        if !self.handled {
            f(self.state, self.key);
            self.handled = true;
        }
        self
    }

    /// Returns whether the key was handled.
    pub fn was_handled(&self) -> bool {
        self.handled
    }
}

/// Helper function to create a KeyMatcher.
pub fn match_key<'a, T>(key: &'a Key, state: &'a mut T) -> KeyMatcher<'a, T> {
    KeyMatcher::new(key, state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyModifiers};

    #[test]
    fn test_key_new() {
        let key = Key::new(KeyCode::Char('a'));
        assert_eq!(key.code, KeyCode::Char('a'));
        assert!(key.modifiers.is_empty());
    }

    #[test]
    fn test_key_with_ctrl() {
        let key = Key::with_ctrl(KeyCode::Char('c'));
        assert!(key.is_ctrl_c());
    }

    #[test]
    fn test_key_is_escape() {
        let key = Key::new(KeyCode::Esc);
        assert!(key.is_escape());
    }

    #[test]
    fn test_key_is_tab() {
        let key = Key::new(KeyCode::Tab);
        assert!(key.is_tab());
        assert!(!key.is_backtab());
    }

    #[test]
    fn test_key_is_backtab() {
        let key = Key {
            code: KeyCode::BackTab,
            modifiers: KeyModifiers::SHIFT,
        };
        assert!(key.is_backtab());
    }

    #[test]
    fn test_key_is_char() {
        let key = Key::new(KeyCode::Char('x'));
        assert!(key.is_char('x'));
        assert!(!key.is_char('y'));
    }

    #[test]
    fn test_key_from_key_event() {
        let event = crossterm::event::KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        let key = Key::from(event);
        assert!(key.is_enter());
    }

    #[test]
    fn test_key_matcher_on_char() {
        let key = Key::new(KeyCode::Char('q'));
        let mut called = false;
        match_key(&key, &mut called).on_char('q', |c| *c = true);
        assert!(called);
    }

    #[test]
    fn test_key_matcher_on_char_no_match() {
        let key = Key::new(KeyCode::Char('q'));
        let mut called = false;
        match_key(&key, &mut called).on_char('x', |c| *c = true);
        assert!(!called);
    }

    #[test]
    fn test_key_matcher_chain() {
        let key = Key::new(KeyCode::Enter);
        let mut result = 0;
        match_key(&key, &mut result)
            .on_char('a', |r| *r = 1)
            .on_enter(|r| *r = 2)
            .on_escape(|r| *r = 3);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_key_matcher_first_match_wins() {
        let key = Key::new(KeyCode::Char('a'));
        let mut result = 0;
        match_key(&key, &mut result)
            .on_char('a', |r| *r = 1)
            .on_char('a', |r| *r = 2); // Should not execute
        assert_eq!(result, 1);
    }

    #[test]
    fn test_arrow_enum() {
        let key = Key::new(KeyCode::Up);
        let mut dir = None;
        match_key(&key, &mut dir).on_arrow(|d, arrow| *d = Some(arrow));
        assert_eq!(dir, Some(Arrow::Up));
    }

    #[test]
    fn test_arrow_down() {
        let key = Key::new(KeyCode::Down);
        let mut dir = None;
        match_key(&key, &mut dir).on_arrow(|d, arrow| *d = Some(arrow));
        assert_eq!(dir, Some(Arrow::Down));
    }

    #[test]
    fn test_arrow_left() {
        let key = Key::new(KeyCode::Left);
        let mut dir = None;
        match_key(&key, &mut dir).on_arrow(|d, arrow| *d = Some(arrow));
        assert_eq!(dir, Some(Arrow::Left));
    }

    #[test]
    fn test_arrow_right() {
        let key = Key::new(KeyCode::Right);
        let mut dir = None;
        match_key(&key, &mut dir).on_arrow(|d, arrow| *d = Some(arrow));
        assert_eq!(dir, Some(Arrow::Right));
    }

    #[test]
    fn test_key_matcher_on_escape() {
        let key = Key::new(KeyCode::Esc);
        let mut called = false;
        match_key(&key, &mut called).on_escape(|c| *c = true);
        assert!(called);
    }

    #[test]
    fn test_key_matcher_on_tab() {
        let key = Key::new(KeyCode::Tab);
        let mut called = false;
        match_key(&key, &mut called).on_tab(|c| *c = true);
        assert!(called);
    }

    #[test]
    fn test_key_matcher_on_backtab() {
        let key = Key {
            code: KeyCode::BackTab,
            modifiers: KeyModifiers::SHIFT,
        };
        let mut called = false;
        match_key(&key, &mut called).on_backtab(|c| *c = true);
        assert!(called);
    }

    #[test]
    fn test_key_matcher_otherwise() {
        let key = Key::new(KeyCode::Char('z'));
        let mut result = None;
        match_key(&key, &mut result)
            .on_char('a', |_| {})
            .otherwise(|r, k| *r = Some(k.clone()));
        assert!(result.is_some());
        assert!(result.unwrap().is_char('z'));
    }

    #[test]
    fn test_key_matcher_otherwise_not_called_when_handled() {
        let key = Key::new(KeyCode::Char('a'));
        let mut result = 0;
        match_key(&key, &mut result)
            .on_char('a', |r| *r = 1)
            .otherwise(|r, _| *r = 999);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_key_matcher_was_handled() {
        let key = Key::new(KeyCode::Char('a'));
        let mut state = ();
        let matcher = match_key(&key, &mut state).on_char('a', |_| {});
        assert!(matcher.was_handled());
    }

    #[test]
    fn test_key_matcher_was_not_handled() {
        let key = Key::new(KeyCode::Char('a'));
        let mut state = ();
        let matcher = match_key(&key, &mut state).on_char('b', |_| {});
        assert!(!matcher.was_handled());
    }
}
