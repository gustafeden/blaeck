//! Autocomplete component - text input with filtered suggestions.
//!
//! The Autocomplete component combines a text input with a dropdown list
//! of suggestions that filter as you type. Supports multiple filter modes:
//! Contains, StartsWith, Fuzzy, and None.
//!
//! ## When to use Autocomplete
//!
//! - Large option lists where typing is faster than scrolling
//! - Command palettes or search interfaces
//! - File path or tag input
//!
//! ## See also
//!
//! - [`TextInput`](super::TextInput) — Plain text input (no suggestions)
//! - [`Select`](super::Select) — Small fixed lists (no typing needed)

use crate::element::{Component, Element};
use crate::style::{Color, Style};

/// A suggestion item for autocomplete.
#[derive(Debug, Clone)]
pub struct AutocompleteItem {
    /// Display label for the item.
    pub label: String,
    /// Optional value (defaults to label if not set).
    pub value: Option<String>,
}

impl AutocompleteItem {
    /// Create a new autocomplete item with the given label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: None,
        }
    }

    /// Set the value for this item.
    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Get the value (or label if no value set).
    pub fn get_value(&self) -> &str {
        self.value.as_deref().unwrap_or(&self.label)
    }
}

impl<S: Into<String>> From<S> for AutocompleteItem {
    fn from(s: S) -> Self {
        AutocompleteItem::new(s)
    }
}

/// Filtering mode for suggestions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FilterMode {
    /// Match if input is contained anywhere in the label (case-insensitive).
    #[default]
    Contains,
    /// Match if label starts with input (case-insensitive).
    StartsWith,
    /// Fuzzy match - all input chars appear in order (case-insensitive).
    Fuzzy,
    /// No filtering - show all items.
    None,
}

/// Properties for the Autocomplete component.
#[derive(Debug, Clone)]
pub struct AutocompleteProps {
    /// Current input value.
    pub input: String,
    /// Cursor position in input.
    pub cursor: usize,
    /// All available suggestions.
    pub items: Vec<AutocompleteItem>,
    /// Currently highlighted suggestion index (in filtered list).
    pub selected: usize,
    /// Whether the input is focused.
    pub focused: bool,
    /// Whether to show suggestions dropdown.
    pub show_suggestions: bool,
    /// Filter mode for suggestions.
    pub filter_mode: FilterMode,
    /// Maximum suggestions to show.
    pub max_suggestions: usize,
    /// Placeholder text when input is empty.
    pub placeholder: Option<String>,
    /// Input text color.
    pub input_color: Option<Color>,
    /// Placeholder color.
    pub placeholder_color: Option<Color>,
    /// Selected suggestion color.
    pub selected_color: Option<Color>,
    /// Unselected suggestion color.
    pub unselected_color: Option<Color>,
    /// Highlight color for matched characters.
    pub highlight_color: Option<Color>,
    /// Whether to highlight matched characters.
    pub highlight_matches: bool,
}

impl Default for AutocompleteProps {
    fn default() -> Self {
        Self {
            input: String::new(),
            cursor: 0,
            items: Vec::new(),
            selected: 0,
            focused: true,
            show_suggestions: true,
            filter_mode: FilterMode::Contains,
            max_suggestions: 5,
            placeholder: None,
            input_color: None,
            placeholder_color: Some(Color::DarkGray),
            selected_color: Some(Color::Cyan),
            unselected_color: None,
            highlight_color: Some(Color::Yellow),
            highlight_matches: false,
        }
    }
}

impl AutocompleteProps {
    /// Create new AutocompleteProps with suggestions.
    pub fn new<I, T>(items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AutocompleteItem>,
    {
        Self {
            items: items.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    /// Set the current input value.
    #[must_use]
    pub fn input(mut self, input: impl Into<String>) -> Self {
        self.input = input.into();
        self.cursor = self.input.len();
        self
    }

    /// Set cursor position.
    #[must_use]
    pub fn cursor(mut self, cursor: usize) -> Self {
        self.cursor = cursor.min(self.input.len());
        self
    }

    /// Set selected suggestion index.
    #[must_use]
    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }

    /// Set whether input is focused.
    #[must_use]
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set whether to show suggestions.
    #[must_use]
    pub fn show_suggestions(mut self, show: bool) -> Self {
        self.show_suggestions = show;
        self
    }

    /// Set filter mode.
    #[must_use]
    pub fn filter_mode(mut self, mode: FilterMode) -> Self {
        self.filter_mode = mode;
        self
    }

    /// Set maximum suggestions to display.
    #[must_use]
    pub fn max_suggestions(mut self, max: usize) -> Self {
        self.max_suggestions = max;
        self
    }

    /// Set placeholder text.
    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Set input text color.
    #[must_use]
    pub fn input_color(mut self, color: Color) -> Self {
        self.input_color = Some(color);
        self
    }

    /// Set selected suggestion color.
    #[must_use]
    pub fn selected_color(mut self, color: Color) -> Self {
        self.selected_color = Some(color);
        self
    }

    /// Enable highlight of matched characters.
    #[must_use]
    pub fn highlight_matches(mut self) -> Self {
        self.highlight_matches = true;
        self
    }

    /// Get filtered suggestions based on current input.
    pub fn filtered_suggestions(&self) -> Vec<&AutocompleteItem> {
        if self.input.is_empty() || self.filter_mode == FilterMode::None {
            return self.items.iter().take(self.max_suggestions).collect();
        }

        let input_lower = self.input.to_lowercase();

        self.items
            .iter()
            .filter(|item| {
                let label_lower = item.label.to_lowercase();
                match self.filter_mode {
                    FilterMode::Contains => label_lower.contains(&input_lower),
                    FilterMode::StartsWith => label_lower.starts_with(&input_lower),
                    FilterMode::Fuzzy => fuzzy_match(&input_lower, &label_lower),
                    FilterMode::None => true,
                }
            })
            .take(self.max_suggestions)
            .collect()
    }

    /// Get the currently selected suggestion.
    pub fn selected_suggestion(&self) -> Option<&AutocompleteItem> {
        let filtered = self.filtered_suggestions();
        filtered.get(self.selected).copied()
    }

    /// Get the selected value (or None if no selection).
    pub fn selected_value(&self) -> Option<&str> {
        self.selected_suggestion().map(|item| item.get_value())
    }

    /// Build the input line string.
    fn render_input(&self) -> String {
        if self.input.is_empty() {
            if let Some(ref placeholder) = self.placeholder {
                if self.focused {
                    format!("▏{}", placeholder)
                } else {
                    format!(" {}", placeholder)
                }
            } else if self.focused {
                "▏".to_string()
            } else {
                " ".to_string()
            }
        } else if self.focused {
            let cursor_pos = self.cursor.min(self.input.len());
            let before: String = self.input.chars().take(cursor_pos).collect();
            let after: String = self.input.chars().skip(cursor_pos).collect();
            format!("{}▏{}", before, after)
        } else {
            self.input.clone()
        }
    }

    /// Build the display strings for rendering.
    pub fn render_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();

        // Input line
        lines.push(self.render_input());

        // Suggestions (if showing and there are matches)
        if self.show_suggestions && self.focused {
            let filtered = self.filtered_suggestions();
            if !filtered.is_empty() {
                for (i, item) in filtered.iter().enumerate() {
                    let indicator = if i == self.selected { "❯" } else { " " };
                    lines.push(format!("{} {}", indicator, item.label));
                }
            }
        }

        lines
    }
}

/// Check if needle chars appear in haystack in order (fuzzy match).
fn fuzzy_match(needle: &str, haystack: &str) -> bool {
    let mut haystack_chars = haystack.chars();
    for needle_char in needle.chars() {
        loop {
            match haystack_chars.next() {
                Some(h) if h == needle_char => break,
                Some(_) => continue,
                None => return false,
            }
        }
    }
    true
}

/// A component that displays a text input with filtered suggestions.
///
/// # Examples
///
/// ```ignore
/// let suggestions = vec!["apple", "banana", "cherry", "date"];
/// let props = AutocompleteProps::new(suggestions)
///     .input(&state.input)
///     .selected(state.selected)
///     .placeholder("Search fruits...");
///
/// // Handle input in your event loop:
/// match key.code {
///     KeyCode::Char(c) => {
///         state.insert(c);
///         state.selected = 0; // Reset selection on input change
///     }
///     KeyCode::Up => state.prev(),
///     KeyCode::Down => state.next(),
///     KeyCode::Enter | KeyCode::Tab => {
///         if let Some(value) = props.selected_value() {
///             state.set_input(value);
///         }
///     }
///     _ => {}
/// }
/// ```
pub struct Autocomplete;

impl Component for Autocomplete {
    type Props = AutocompleteProps;

    fn render(props: &Self::Props) -> Element {
        let lines = props.render_lines();
        let filtered = props.filtered_suggestions();

        // Build content with each line
        let mut content_lines = Vec::new();

        // Input line (first line)
        content_lines.push(lines[0].clone());

        // Suggestion lines
        if props.show_suggestions && props.focused && !filtered.is_empty() {
            for (i, item) in filtered.iter().enumerate() {
                let indicator = if i == props.selected { "❯" } else { " " };
                content_lines.push(format!("{} {}", indicator, item.label));
            }
        }

        let content = content_lines.join("\n");

        // Use selected color for overall style when focused
        let mut style = Style::new();
        if let Some(color) = props.selected_color {
            style = style.fg(color);
        }

        Element::styled_text(&content, style)
    }
}

/// State helper for managing autocomplete input.
#[derive(Debug, Clone, Default)]
pub struct AutocompleteState {
    /// Current input value.
    pub input: String,
    /// Cursor position.
    pub cursor: usize,
    /// Selected suggestion index.
    pub selected: usize,
    /// Number of filtered items (updated externally).
    pub filtered_count: usize,
}

impl AutocompleteState {
    /// Create a new empty state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create state with initial suggestions count.
    pub fn with_count(count: usize) -> Self {
        Self {
            filtered_count: count,
            ..Default::default()
        }
    }

    /// Insert a character at cursor.
    pub fn insert(&mut self, c: char) {
        self.input.insert(self.cursor, c);
        self.cursor += 1;
        self.selected = 0; // Reset selection on input change
    }

    /// Delete character before cursor.
    pub fn backspace(&mut self) -> bool {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.input.remove(self.cursor);
            self.selected = 0;
            true
        } else {
            false
        }
    }

    /// Delete character at cursor.
    pub fn delete(&mut self) -> bool {
        if self.cursor < self.input.len() {
            self.input.remove(self.cursor);
            self.selected = 0;
            true
        } else {
            false
        }
    }

    /// Move cursor left.
    pub fn move_left(&mut self) -> bool {
        if self.cursor > 0 {
            self.cursor -= 1;
            true
        } else {
            false
        }
    }

    /// Move cursor right.
    pub fn move_right(&mut self) -> bool {
        if self.cursor < self.input.len() {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    /// Move to start.
    pub fn move_home(&mut self) {
        self.cursor = 0;
    }

    /// Move to end.
    pub fn move_end(&mut self) {
        self.cursor = self.input.len();
    }

    /// Select previous suggestion.
    pub fn prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Select next suggestion.
    pub fn next(&mut self) {
        if self.filtered_count > 0 && self.selected < self.filtered_count - 1 {
            self.selected += 1;
        }
    }

    /// Set input value (e.g., when selecting a suggestion).
    pub fn set_input(&mut self, value: impl Into<String>) {
        self.input = value.into();
        self.cursor = self.input.len();
        self.selected = 0;
    }

    /// Clear input.
    pub fn clear(&mut self) {
        self.input.clear();
        self.cursor = 0;
        self.selected = 0;
    }

    /// Get current input.
    pub fn value(&self) -> &str {
        &self.input
    }

    /// Check if input is empty.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    /// Update filtered count (call after filtering).
    pub fn set_filtered_count(&mut self, count: usize) {
        self.filtered_count = count;
        if self.selected >= count && count > 0 {
            self.selected = count - 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autocomplete_item_new() {
        let item = AutocompleteItem::new("test");
        assert_eq!(item.label, "test");
        assert!(item.value.is_none());
    }

    #[test]
    fn test_autocomplete_item_with_value() {
        let item = AutocompleteItem::new("Display").value("actual");
        assert_eq!(item.get_value(), "actual");
    }

    #[test]
    fn test_autocomplete_item_from_str() {
        let item: AutocompleteItem = "option".into();
        assert_eq!(item.label, "option");
    }

    #[test]
    fn test_autocomplete_props_new() {
        let props = AutocompleteProps::new(vec!["a", "b", "c"]);
        assert_eq!(props.items.len(), 3);
    }

    #[test]
    fn test_autocomplete_props_builder() {
        let props = AutocompleteProps::new(vec!["apple", "banana"])
            .input("app")
            .selected(0)
            .placeholder("Search...")
            .max_suggestions(10);

        assert_eq!(props.input, "app");
        assert_eq!(props.placeholder, Some("Search...".to_string()));
        assert_eq!(props.max_suggestions, 10);
    }

    #[test]
    fn test_filter_contains() {
        let props = AutocompleteProps::new(vec!["apple", "pineapple", "banana"])
            .input("apple")
            .filter_mode(FilterMode::Contains);

        let filtered = props.filtered_suggestions();
        assert_eq!(filtered.len(), 2); // apple, pineapple
    }

    #[test]
    fn test_filter_starts_with() {
        let props = AutocompleteProps::new(vec!["apple", "pineapple", "apricot"])
            .input("ap")
            .filter_mode(FilterMode::StartsWith);

        let filtered = props.filtered_suggestions();
        assert_eq!(filtered.len(), 2); // apple, apricot
    }

    #[test]
    fn test_filter_fuzzy() {
        let props = AutocompleteProps::new(vec!["flutter", "flask", "flex", "banana"])
            .input("fl")
            .filter_mode(FilterMode::Fuzzy);

        let filtered = props.filtered_suggestions();
        assert_eq!(filtered.len(), 3); // flutter, flask, flex
    }

    #[test]
    fn test_fuzzy_match() {
        assert!(fuzzy_match("fk", "flask"));
        assert!(fuzzy_match("fb", "foobar"));
        assert!(!fuzzy_match("fz", "flask"));
        assert!(fuzzy_match("", "anything"));
    }

    #[test]
    fn test_filter_none() {
        let props = AutocompleteProps::new(vec!["a", "b", "c"])
            .input("xyz")
            .filter_mode(FilterMode::None);

        let filtered = props.filtered_suggestions();
        assert_eq!(filtered.len(), 3); // All items
    }

    #[test]
    fn test_max_suggestions() {
        let props = AutocompleteProps::new(vec!["a", "b", "c", "d", "e", "f"]).max_suggestions(3);

        let filtered = props.filtered_suggestions();
        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn test_selected_suggestion() {
        let props = AutocompleteProps::new(vec!["apple", "banana", "cherry"]).selected(1);

        assert_eq!(props.selected_suggestion().unwrap().label, "banana");
    }

    #[test]
    fn test_render_lines_with_suggestions() {
        let props = AutocompleteProps::new(vec!["apple", "banana"])
            .input("a")
            .focused(true);

        let lines = props.render_lines();
        assert!(lines.len() >= 2); // Input + at least one suggestion
    }

    #[test]
    fn test_state_insert() {
        let mut state = AutocompleteState::new();
        state.insert('a');
        state.insert('b');
        assert_eq!(state.value(), "ab");
        assert_eq!(state.cursor, 2);
    }

    #[test]
    fn test_state_backspace() {
        let mut state = AutocompleteState::new();
        state.set_input("hello");
        state.backspace();
        assert_eq!(state.value(), "hell");
    }

    #[test]
    fn test_state_navigation() {
        let mut state = AutocompleteState::with_count(5);

        state.next();
        assert_eq!(state.selected, 1);

        state.next();
        state.next();
        assert_eq!(state.selected, 3);

        state.prev();
        assert_eq!(state.selected, 2);
    }

    #[test]
    fn test_state_set_input() {
        let mut state = AutocompleteState::new();
        state.set_input("selected value");
        assert_eq!(state.value(), "selected value");
        assert_eq!(state.cursor, 14);
        assert_eq!(state.selected, 0);
    }

    #[test]
    fn test_component_render() {
        let props = AutocompleteProps::new(vec!["apple", "banana"])
            .input("a")
            .focused(true);

        let elem = Autocomplete::render(&props);
        assert!(elem.is_text());
    }
}
