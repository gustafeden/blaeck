//! TextInput component - interactive text field.
//!
//! The TextInput component displays an editable text field with cursor,
//! placeholder text, and optional password masking. Use `TextInputState`
//! to manage the text value and cursor position.
//!
//! ## When to use TextInput
//!
//! - Free-form text entry (names, paths, search queries)
//! - Password fields (with `mask: true`)
//! - Any user input that isn't a fixed set of choices
//!
//! ## See also
//!
//! - [`Autocomplete`](super::Autocomplete) — Text input with suggestions
//! - [`Select`](super::Select) — Fixed choices (no typing needed)
//! - [`Confirm`](super::Confirm) — Yes/no questions

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// Properties for the TextInput component.
#[derive(Debug, Clone)]
pub struct TextInputProps {
    /// The current text value.
    pub value: String,
    /// Placeholder text shown when value is empty.
    pub placeholder: Option<String>,
    /// Cursor position (character index).
    pub cursor: usize,
    /// Selection anchor (where selection started). None = no selection.
    pub selection_anchor: Option<usize>,
    /// Whether the input is focused (shows cursor).
    pub focused: bool,
    /// Whether to mask the input (for passwords).
    pub mask: bool,
    /// Character to use for masking (default: '•').
    pub mask_char: char,
    /// Text color.
    pub color: Option<Color>,
    /// Placeholder text color.
    pub placeholder_color: Option<Color>,
    /// Cursor color/style.
    pub cursor_color: Option<Color>,
    /// Selection background color.
    pub selection_color: Option<Color>,
    /// Whether the text should be bold.
    pub bold: bool,
    /// Whether the text should be dimmed.
    pub dim: bool,
    /// Minimum width of the input field.
    pub min_width: Option<usize>,
}

impl Default for TextInputProps {
    fn default() -> Self {
        Self {
            value: String::new(),
            placeholder: None,
            cursor: 0,
            selection_anchor: None,
            focused: true,
            mask: false,
            mask_char: '•',
            color: None,
            placeholder_color: None,
            cursor_color: None,
            selection_color: None,
            bold: false,
            dim: false,
            min_width: None,
        }
    }
}

impl TextInputProps {
    /// Create new TextInputProps with the given value.
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        let cursor = value.len();
        Self {
            value,
            cursor,
            ..Default::default()
        }
    }

    /// Set the current value.
    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.cursor = self.cursor.min(self.value.len());
        self
    }

    /// Set the placeholder text.
    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Set the cursor position.
    #[must_use]
    pub fn cursor(mut self, cursor: usize) -> Self {
        self.cursor = cursor.min(self.value.len());
        self
    }

    /// Set whether the input is focused.
    #[must_use]
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Enable password masking.
    #[must_use]
    pub fn mask(mut self) -> Self {
        self.mask = true;
        self
    }

    /// Set the mask character.
    #[must_use]
    pub fn mask_char(mut self, c: char) -> Self {
        self.mask_char = c;
        self
    }

    /// Set the text color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the placeholder color.
    #[must_use]
    pub fn placeholder_color(mut self, color: Color) -> Self {
        self.placeholder_color = Some(color);
        self
    }

    /// Set the cursor color.
    #[must_use]
    pub fn cursor_color(mut self, color: Color) -> Self {
        self.cursor_color = Some(color);
        self
    }

    /// Make the text bold.
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Make the text dimmed.
    #[must_use]
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Set minimum width.
    #[must_use]
    pub fn min_width(mut self, width: usize) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Set selection anchor for text selection.
    #[must_use]
    pub fn selection(mut self, anchor: Option<usize>) -> Self {
        self.selection_anchor = anchor;
        self
    }

    /// Get selection range (start, end) where start <= end.
    pub fn selection_range(&self) -> Option<(usize, usize)> {
        self.selection_anchor.and_then(|anchor| {
            if anchor == self.cursor {
                None // No actual selection
            } else if anchor <= self.cursor {
                Some((anchor, self.cursor))
            } else {
                Some((self.cursor, anchor))
            }
        })
    }

    /// Build the display string with cursor and selection.
    pub fn render_string(&self) -> String {
        if self.value.is_empty() {
            // Show placeholder or empty with cursor
            if let Some(ref placeholder) = self.placeholder {
                if self.focused {
                    format!("▏{}", placeholder)
                } else {
                    // Add space to maintain alignment with cursor
                    format!(" {}", placeholder)
                }
            } else if self.focused {
                "▏".to_string()
            } else {
                " ".to_string()
            }
        } else {
            let display_value = if self.mask {
                self.mask_char.to_string().repeat(self.value.chars().count())
            } else {
                self.value.clone()
            };

            if self.focused {
                let char_count = display_value.chars().count();
                let cursor_pos = self.cursor.min(char_count);

                // Check for selection
                if let Some((sel_start, sel_end)) = self.selection_range() {
                    let sel_start = sel_start.min(char_count);
                    let sel_end = sel_end.min(char_count);

                    // Build string with selection markers [selected]
                    let before: String = display_value.chars().take(sel_start).collect();
                    let selected: String = display_value.chars().skip(sel_start).take(sel_end - sel_start).collect();
                    let after: String = display_value.chars().skip(sel_end).collect();

                    // Show cursor position within the selection context
                    if cursor_pos == sel_end {
                        format!("{}[{}]▏{}", before, selected, after)
                    } else {
                        format!("{}▏[{}]{}", before, selected, after)
                    }
                } else {
                    // No selection, just cursor
                    let before: String = display_value.chars().take(cursor_pos).collect();
                    let after: String = display_value.chars().skip(cursor_pos).collect();
                    format!("{}▏{}", before, after)
                }
            } else {
                display_value
            }
        }
    }
}

/// A component that displays an editable text input.
///
/// The TextInput renders the current value with a cursor indicator when focused.
/// The caller is responsible for handling key events and updating the value/cursor.
///
/// # Examples
///
/// ```ignore
/// let props = TextInputProps::new("Hello")
///     .placeholder("Enter text...")
///     .focused(true)
///     .color(Color::White);
///
/// // Handle input in your event loop:
/// match key.code {
///     KeyCode::Char(c) => {
///         value.insert(cursor, c);
///         cursor += 1;
///     }
///     KeyCode::Backspace if cursor > 0 => {
///         cursor -= 1;
///         value.remove(cursor);
///     }
///     KeyCode::Left if cursor > 0 => cursor -= 1,
///     KeyCode::Right if cursor < value.len() => cursor += 1,
///     _ => {}
/// }
/// ```
pub struct TextInput;

impl Component for TextInput {
    type Props = TextInputProps;

    fn render(props: &Self::Props) -> Element {
        let content = props.render_string();

        let mut style = Style::new();

        // Choose color based on state
        // Only apply placeholder styling when NOT focused (so cursor stays visible when focused)
        if props.value.is_empty() && props.placeholder.is_some() && !props.focused {
            // Placeholder style (unfocused)
            if let Some(color) = props.placeholder_color {
                style = style.fg(color);
            } else {
                style = style.add_modifier(Modifier::DIM);
            }
        } else {
            // Normal text style (or focused with placeholder)
            if let Some(color) = props.color {
                style = style.fg(color);
            }
            if props.bold {
                style = style.add_modifier(Modifier::BOLD);
            }
            if props.dim {
                style = style.add_modifier(Modifier::DIM);
            }
        }

        Element::styled_text(&content, style)
    }
}

/// Helper struct for managing text input state.
///
/// This provides a convenient way to manage the value, cursor position,
/// selection, and handle common editing operations.
#[derive(Debug, Clone, Default)]
pub struct TextInputState {
    /// The current text value.
    pub value: String,
    /// Cursor position (character index).
    pub cursor: usize,
    /// Selection anchor (where selection started). None = no selection.
    pub selection_anchor: Option<usize>,
}

impl TextInputState {
    /// Create a new empty state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new state with initial value.
    pub fn with_value(value: impl Into<String>) -> Self {
        let value = value.into();
        let cursor = value.len();
        Self { value, cursor, selection_anchor: None }
    }

    /// Insert a character at the cursor position.
    /// If there's a selection, it's deleted first.
    pub fn insert(&mut self, c: char) {
        self.delete_selection();
        self.value.insert(self.cursor, c);
        self.cursor += 1;
    }

    /// Insert a string at the cursor position.
    /// If there's a selection, it's deleted first.
    pub fn insert_str(&mut self, s: &str) {
        self.delete_selection();
        self.value.insert_str(self.cursor, s);
        self.cursor += s.len();
    }

    /// Delete the character before the cursor (backspace).
    /// If there's a selection, deletes the selection instead.
    pub fn backspace(&mut self) -> bool {
        if self.has_selection() {
            self.delete_selection();
            return true;
        }
        if self.cursor > 0 {
            self.cursor -= 1;
            self.value.remove(self.cursor);
            true
        } else {
            false
        }
    }

    /// Delete the character at the cursor position (delete).
    /// If there's a selection, deletes the selection instead.
    pub fn delete(&mut self) -> bool {
        if self.has_selection() {
            self.delete_selection();
            return true;
        }
        if self.cursor < self.value.len() {
            self.value.remove(self.cursor);
            true
        } else {
            false
        }
    }

    /// Move cursor left (clears selection).
    pub fn move_left(&mut self) -> bool {
        self.clear_selection();
        if self.cursor > 0 {
            self.cursor -= 1;
            true
        } else {
            false
        }
    }

    /// Move cursor right (clears selection).
    pub fn move_right(&mut self) -> bool {
        self.clear_selection();
        if self.cursor < self.value.len() {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    /// Move cursor to the start (clears selection).
    pub fn move_home(&mut self) {
        self.clear_selection();
        self.cursor = 0;
    }

    /// Move cursor to the end (clears selection).
    pub fn move_end(&mut self) {
        self.clear_selection();
        self.cursor = self.value.len();
    }

    /// Clear the input.
    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor = 0;
    }

    /// Set the value and move cursor to end.
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
        self.cursor = self.value.len();
    }

    /// Get the current value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Check if the input is empty.
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Check if there's an active selection.
    pub fn has_selection(&self) -> bool {
        self.selection_anchor.is_some() && self.selection_anchor != Some(self.cursor)
    }

    /// Get selection range (start, end) where start <= end.
    pub fn selection_range(&self) -> Option<(usize, usize)> {
        self.selection_anchor.map(|anchor| {
            if anchor <= self.cursor {
                (anchor, self.cursor)
            } else {
                (self.cursor, anchor)
            }
        })
    }

    /// Get the selected text.
    pub fn selected_text(&self) -> Option<&str> {
        self.selection_range().map(|(start, end)| {
            &self.value[start..end]
        })
    }

    /// Clear the selection without deleting text.
    pub fn clear_selection(&mut self) {
        self.selection_anchor = None;
    }

    /// Delete the selected text and return it.
    pub fn delete_selection(&mut self) -> Option<String> {
        if let Some((start, end)) = self.selection_range() {
            if start != end {
                let deleted: String = self.value.drain(start..end).collect();
                self.cursor = start;
                self.selection_anchor = None;
                return Some(deleted);
            }
        }
        self.selection_anchor = None;
        None
    }

    /// Select all text.
    pub fn select_all(&mut self) {
        self.selection_anchor = Some(0);
        self.cursor = self.value.len();
    }

    /// Move cursor left with selection (shift+left).
    pub fn select_left(&mut self) -> bool {
        if self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor);
        }
        if self.cursor > 0 {
            self.cursor -= 1;
            true
        } else {
            false
        }
    }

    /// Move cursor right with selection (shift+right).
    pub fn select_right(&mut self) -> bool {
        if self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor);
        }
        if self.cursor < self.value.len() {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    /// Select to beginning (shift+home).
    pub fn select_to_home(&mut self) {
        if self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor);
        }
        self.cursor = 0;
    }

    /// Select to end (shift+end).
    pub fn select_to_end(&mut self) {
        if self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor);
        }
        self.cursor = self.value.len();
    }

    /// Convert to props for rendering.
    pub fn to_props(&self) -> TextInputProps {
        TextInputProps {
            value: self.value.clone(),
            cursor: self.cursor,
            selection_anchor: self.selection_anchor,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_input_props_default() {
        let props = TextInputProps::default();
        assert!(props.value.is_empty());
        assert_eq!(props.cursor, 0);
        assert!(props.focused);
        assert!(!props.mask);
    }

    #[test]
    fn test_text_input_props_new() {
        let props = TextInputProps::new("Hello");
        assert_eq!(props.value, "Hello");
        assert_eq!(props.cursor, 5); // At end
    }

    #[test]
    fn test_text_input_props_builder() {
        let props = TextInputProps::new("test")
            .placeholder("Enter text")
            .cursor(2)
            .color(Color::Green)
            .mask();

        assert_eq!(props.value, "test");
        assert_eq!(props.placeholder, Some("Enter text".to_string()));
        assert_eq!(props.cursor, 2);
        assert_eq!(props.color, Some(Color::Green));
        assert!(props.mask);
    }

    #[test]
    fn test_text_input_render_string_empty() {
        let props = TextInputProps::default().focused(true);
        assert_eq!(props.render_string(), "▏");
    }

    #[test]
    fn test_text_input_render_string_with_placeholder() {
        let props = TextInputProps::default()
            .placeholder("Type here")
            .focused(true);
        assert_eq!(props.render_string(), "▏Type here");
    }

    #[test]
    fn test_text_input_render_string_with_value() {
        let props = TextInputProps::new("Hello").cursor(2).focused(true);
        assert_eq!(props.render_string(), "He▏llo");
    }

    #[test]
    fn test_text_input_render_string_cursor_at_end() {
        let props = TextInputProps::new("Hello").focused(true);
        assert_eq!(props.render_string(), "Hello▏");
    }

    #[test]
    fn test_text_input_render_string_not_focused() {
        let props = TextInputProps::new("Hello").focused(false);
        // Value without cursor, no leading space needed
        assert_eq!(props.render_string(), "Hello");
    }

    #[test]
    fn test_text_input_render_string_placeholder_not_focused() {
        let props = TextInputProps::default()
            .placeholder("Type here")
            .focused(false);
        // Space added for alignment with cursor
        assert_eq!(props.render_string(), " Type here");
    }

    #[test]
    fn test_text_input_render_string_masked() {
        let props = TextInputProps::new("secret").mask().focused(true);
        assert_eq!(props.render_string(), "••••••▏");
    }

    #[test]
    fn test_text_input_render_string_masked_custom_char() {
        let props = TextInputProps::new("abc")
            .mask()
            .mask_char('•')
            .focused(false);
        assert_eq!(props.render_string(), "•••");
    }

    #[test]
    fn test_text_input_state_new() {
        let state = TextInputState::new();
        assert!(state.is_empty());
        assert_eq!(state.cursor, 0);
    }

    #[test]
    fn test_text_input_state_with_value() {
        let state = TextInputState::with_value("Hello");
        assert_eq!(state.value(), "Hello");
        assert_eq!(state.cursor, 5);
    }

    #[test]
    fn test_text_input_state_insert() {
        let mut state = TextInputState::new();
        state.insert('H');
        state.insert('i');
        assert_eq!(state.value(), "Hi");
        assert_eq!(state.cursor, 2);
    }

    #[test]
    fn test_text_input_state_backspace() {
        let mut state = TextInputState::with_value("Hello");
        assert!(state.backspace());
        assert_eq!(state.value(), "Hell");
        assert_eq!(state.cursor, 4);
    }

    #[test]
    fn test_text_input_state_backspace_at_start() {
        let mut state = TextInputState::with_value("Hello");
        state.cursor = 0;
        assert!(!state.backspace());
        assert_eq!(state.value(), "Hello");
    }

    #[test]
    fn test_text_input_state_delete() {
        let mut state = TextInputState::with_value("Hello");
        state.cursor = 0;
        assert!(state.delete());
        assert_eq!(state.value(), "ello");
    }

    #[test]
    fn test_text_input_state_delete_at_end() {
        let mut state = TextInputState::with_value("Hello");
        assert!(!state.delete());
        assert_eq!(state.value(), "Hello");
    }

    #[test]
    fn test_text_input_state_move_left() {
        let mut state = TextInputState::with_value("Hello");
        assert!(state.move_left());
        assert_eq!(state.cursor, 4);
    }

    #[test]
    fn test_text_input_state_move_right() {
        let mut state = TextInputState::with_value("Hello");
        state.cursor = 0;
        assert!(state.move_right());
        assert_eq!(state.cursor, 1);
    }

    #[test]
    fn test_text_input_state_move_home_end() {
        let mut state = TextInputState::with_value("Hello");
        state.move_home();
        assert_eq!(state.cursor, 0);
        state.move_end();
        assert_eq!(state.cursor, 5);
    }

    #[test]
    fn test_text_input_state_clear() {
        let mut state = TextInputState::with_value("Hello");
        state.clear();
        assert!(state.is_empty());
        assert_eq!(state.cursor, 0);
    }

    #[test]
    fn test_text_input_component_render() {
        let props = TextInputProps::new("Test").focused(true);
        let elem = TextInput::render(&props);
        match elem {
            Element::Text { content, .. } => {
                assert_eq!(content, "Test▏");
            }
            _ => panic!("Expected Text element"),
        }
    }
}
