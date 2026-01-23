//! MultiSelect component - checkbox list with multiple selections.
//!
//! The MultiSelect component displays a list of options with checkboxes
//! that users can toggle to select multiple items. Use `MultiSelectState`
//! to track which items are selected.
//!
//! ## When to use MultiSelect
//!
//! - Multiple choices from a list (select all that apply)
//! - Feature toggles or option selection
//! - Batch operations (select items to delete/process)
//!
//! ## See also
//!
//! - [`Select`](super::Select) — Single selection only
//! - [`Checkbox`](super::Checkbox) — Individual toggle (not in a list)

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};
use std::collections::HashSet;

/// A single item in a multiselect list.
#[derive(Debug, Clone)]
pub struct MultiSelectItem {
    /// Display label for the item.
    pub label: String,
    /// Optional value (defaults to label if not set).
    pub value: Option<String>,
    /// Whether the item is disabled.
    pub disabled: bool,
}

impl MultiSelectItem {
    /// Create a new multiselect item with the given label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: None,
            disabled: false,
        }
    }

    /// Set the value for this item.
    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Mark this item as disabled.
    #[must_use]
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Get the value (or label if no value set).
    pub fn get_value(&self) -> &str {
        self.value.as_deref().unwrap_or(&self.label)
    }
}

impl<S: Into<String>> From<S> for MultiSelectItem {
    fn from(s: S) -> Self {
        MultiSelectItem::new(s)
    }
}

/// Style for the checkbox indicators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MultiSelectStyle {
    /// Bracket style: [x] / [ ]
    #[default]
    Bracket,
    /// Unicode checkmarks: ✓ / ○
    Unicode,
    /// Filled circles: ● / ○
    Circle,
    /// Check marks: ☑ / ☐
    Check,
}

impl MultiSelectStyle {
    /// Get the characters for checked and unchecked states.
    pub fn chars(&self) -> (&'static str, &'static str) {
        match self {
            MultiSelectStyle::Bracket => ("[x]", "[ ]"),
            MultiSelectStyle::Unicode => (" ✓ ", " ○ "),
            MultiSelectStyle::Circle => (" ● ", " ○ "),
            MultiSelectStyle::Check => (" ☑ ", " ☐ "),
        }
    }
}

/// Properties for the MultiSelect component.
#[derive(Debug, Clone)]
pub struct MultiSelectProps {
    /// List of items to display.
    pub items: Vec<MultiSelectItem>,
    /// Currently highlighted index (cursor position).
    pub cursor: usize,
    /// Set of selected indices.
    pub selected: HashSet<usize>,
    /// Checkbox style.
    pub style: MultiSelectStyle,
    /// Color for the cursor/highlighted item.
    pub cursor_color: Option<Color>,
    /// Color for selected (checked) items.
    pub selected_color: Option<Color>,
    /// Color for unselected items.
    pub unselected_color: Option<Color>,
    /// Color for disabled items.
    pub disabled_color: Option<Color>,
    /// Maximum visible items (for scrolling).
    pub max_visible: Option<usize>,
    /// Scroll offset for long lists.
    pub scroll_offset: usize,
    /// Cursor indicator character.
    pub cursor_indicator: &'static str,
}

impl Default for MultiSelectProps {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            cursor: 0,
            selected: HashSet::new(),
            style: MultiSelectStyle::Bracket,
            cursor_color: Some(Color::Cyan),
            selected_color: Some(Color::Green),
            unselected_color: None,
            disabled_color: Some(Color::DarkGray),
            max_visible: None,
            scroll_offset: 0,
            cursor_indicator: "❯",
        }
    }
}

impl MultiSelectProps {
    /// Create new MultiSelectProps with the given items.
    pub fn new<I, T>(items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<MultiSelectItem>,
    {
        Self {
            items: items.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    /// Set the cursor position.
    #[must_use]
    pub fn cursor(mut self, cursor: usize) -> Self {
        self.cursor = cursor.min(self.items.len().saturating_sub(1));
        self
    }

    /// Set the selected indices.
    #[must_use]
    pub fn selected(mut self, selected: HashSet<usize>) -> Self {
        self.selected = selected;
        self
    }

    /// Set the checkbox style.
    #[must_use]
    pub fn style(mut self, style: MultiSelectStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the cursor/highlight color.
    #[must_use]
    pub fn cursor_color(mut self, color: Color) -> Self {
        self.cursor_color = Some(color);
        self
    }

    /// Set the selected item color.
    #[must_use]
    pub fn selected_color(mut self, color: Color) -> Self {
        self.selected_color = Some(color);
        self
    }

    /// Set the unselected item color.
    #[must_use]
    pub fn unselected_color(mut self, color: Color) -> Self {
        self.unselected_color = Some(color);
        self
    }

    /// Set the maximum visible items.
    #[must_use]
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = Some(max);
        self
    }

    /// Set the scroll offset.
    #[must_use]
    pub fn scroll_offset(mut self, offset: usize) -> Self {
        self.scroll_offset = offset;
        self
    }

    /// Check if an index is selected.
    pub fn is_selected(&self, index: usize) -> bool {
        self.selected.contains(&index)
    }

    /// Get selected items.
    pub fn selected_items(&self) -> Vec<&MultiSelectItem> {
        self.selected
            .iter()
            .filter_map(|&i| self.items.get(i))
            .collect()
    }

    /// Get selected values.
    pub fn selected_values(&self) -> Vec<&str> {
        self.selected
            .iter()
            .filter_map(|&i| self.items.get(i).map(|item| item.get_value()))
            .collect()
    }

    /// Get the visible items based on scroll offset and max_visible.
    fn visible_items(&self) -> Vec<(usize, &MultiSelectItem)> {
        let items: Vec<_> = self.items.iter().enumerate().collect();

        if let Some(max) = self.max_visible {
            if items.len() > max {
                let start = self.scroll_offset.min(items.len().saturating_sub(max));
                return items.into_iter().skip(start).take(max).collect();
            }
        }

        items
    }

    /// Build the display strings for all visible items.
    pub fn render_lines(&self) -> Vec<(String, Style)> {
        let (checked_char, unchecked_char) = self.style.chars();
        let visible_items = self.visible_items();

        // Calculate max width from ALL items to prevent resizing when scrolling
        let max_label_width = self
            .items
            .iter()
            .map(|item| item.label.chars().count())
            .max()
            .unwrap_or(0);

        visible_items
            .iter()
            .map(|(idx, item)| {
                let is_cursor = *idx == self.cursor;
                let is_checked = self.selected.contains(idx);

                let cursor_char = if is_cursor {
                    self.cursor_indicator
                } else {
                    " "
                };
                let checkbox = if is_checked {
                    checked_char
                } else {
                    unchecked_char
                };

                // Pad to max width
                let padding = max_label_width.saturating_sub(item.label.chars().count());
                let line = format!(
                    "{} {} {}{}",
                    cursor_char,
                    checkbox,
                    item.label,
                    " ".repeat(padding)
                );

                let mut style = Style::new();
                if item.disabled {
                    if let Some(color) = self.disabled_color {
                        style = style.fg(color);
                    }
                    style = style.add_modifier(Modifier::DIM);
                } else if is_cursor {
                    if let Some(color) = self.cursor_color {
                        style = style.fg(color);
                    }
                    style = style.add_modifier(Modifier::BOLD);
                } else if is_checked {
                    if let Some(color) = self.selected_color {
                        style = style.fg(color);
                    }
                } else if let Some(color) = self.unselected_color {
                    style = style.fg(color);
                }

                (line, style)
            })
            .collect()
    }
}

/// A component that displays a multi-select checkbox list.
///
/// # Examples
///
/// ```ignore
/// let items = vec!["Option 1", "Option 2", "Option 3"];
/// let props = MultiSelectProps::new(items)
///     .cursor(state.cursor)
///     .selected(state.selected.clone())
///     .selected_color(Color::Green);
///
/// // Handle input in your event loop:
/// match key.code {
///     KeyCode::Up => state.up(),
///     KeyCode::Down => state.down(),
///     KeyCode::Char(' ') => state.toggle(),
///     KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => state.select_all(),
///     KeyCode::Enter => return Some(state.selected_indices()),
///     _ => {}
/// }
/// ```
pub struct MultiSelect;

impl Component for MultiSelect {
    type Props = MultiSelectProps;

    fn render(props: &Self::Props) -> Element {
        let lines = props.render_lines();

        // Join lines with newlines
        let content: String = lines
            .iter()
            .map(|(line, _)| line.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        // Use cursor line's style as the overall style
        let style = lines
            .iter()
            .find(|(_, s)| s.modifiers.contains(Modifier::BOLD))
            .map(|(_, s)| *s)
            .unwrap_or_default();

        Element::styled_text(&content, style)
    }
}

/// Helper struct for managing multiselect state.
#[derive(Debug, Clone)]
pub struct MultiSelectState {
    /// Current cursor position.
    pub cursor: usize,
    /// Set of selected indices.
    pub selected: HashSet<usize>,
    /// Number of items.
    pub count: usize,
    /// Scroll offset for long lists.
    pub scroll_offset: usize,
    /// Maximum visible items.
    pub max_visible: Option<usize>,
}

impl MultiSelectState {
    /// Create a new multiselect state.
    pub fn new(count: usize) -> Self {
        Self {
            cursor: 0,
            selected: HashSet::new(),
            count,
            scroll_offset: 0,
            max_visible: None,
        }
    }

    /// Set maximum visible items.
    #[must_use]
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = Some(max);
        self
    }

    /// Set initially selected indices.
    #[must_use]
    pub fn with_selected(mut self, selected: impl IntoIterator<Item = usize>) -> Self {
        self.selected = selected.into_iter().filter(|&i| i < self.count).collect();
        self
    }

    /// Move cursor up.
    pub fn up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.adjust_scroll();
        }
    }

    /// Move cursor down.
    pub fn down(&mut self) {
        if self.cursor < self.count.saturating_sub(1) {
            self.cursor += 1;
            self.adjust_scroll();
        }
    }

    /// Move to first item.
    pub fn first(&mut self) {
        self.cursor = 0;
        self.scroll_offset = 0;
    }

    /// Move to last item.
    pub fn last(&mut self) {
        self.cursor = self.count.saturating_sub(1);
        self.adjust_scroll();
    }

    /// Toggle selection of current item.
    pub fn toggle(&mut self) {
        if self.selected.contains(&self.cursor) {
            self.selected.remove(&self.cursor);
        } else {
            self.selected.insert(self.cursor);
        }
    }

    /// Select current item.
    pub fn select(&mut self) {
        self.selected.insert(self.cursor);
    }

    /// Deselect current item.
    pub fn deselect(&mut self) {
        self.selected.remove(&self.cursor);
    }

    /// Select all items.
    pub fn select_all(&mut self) {
        self.selected = (0..self.count).collect();
    }

    /// Deselect all items.
    pub fn deselect_all(&mut self) {
        self.selected.clear();
    }

    /// Toggle all items (invert selection).
    pub fn toggle_all(&mut self) {
        let all: HashSet<usize> = (0..self.count).collect();
        self.selected = all.difference(&self.selected).copied().collect();
    }

    /// Check if an index is selected.
    pub fn is_selected(&self, index: usize) -> bool {
        self.selected.contains(&index)
    }

    /// Get number of selected items.
    pub fn selected_count(&self) -> usize {
        self.selected.len()
    }

    /// Get selected indices as a sorted vector.
    pub fn selected_indices(&self) -> Vec<usize> {
        let mut indices: Vec<_> = self.selected.iter().copied().collect();
        indices.sort();
        indices
    }

    /// Check if any item is selected.
    pub fn has_selection(&self) -> bool {
        !self.selected.is_empty()
    }

    /// Check if all items are selected.
    pub fn all_selected(&self) -> bool {
        self.selected.len() == self.count
    }

    /// Adjust scroll offset to keep cursor visible.
    fn adjust_scroll(&mut self) {
        if let Some(max) = self.max_visible {
            if self.cursor < self.scroll_offset {
                self.scroll_offset = self.cursor;
            } else if self.cursor >= self.scroll_offset + max {
                self.scroll_offset = self.cursor - max + 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiselect_item_new() {
        let item = MultiSelectItem::new("Test");
        assert_eq!(item.label, "Test");
        assert!(item.value.is_none());
        assert!(!item.disabled);
    }

    #[test]
    fn test_multiselect_item_with_value() {
        let item = MultiSelectItem::new("Display").value("actual_value");
        assert_eq!(item.label, "Display");
        assert_eq!(item.get_value(), "actual_value");
    }

    #[test]
    fn test_multiselect_item_from_str() {
        let item: MultiSelectItem = "Option".into();
        assert_eq!(item.label, "Option");
    }

    #[test]
    fn test_multiselect_props_new() {
        let props = MultiSelectProps::new(vec!["A", "B", "C"]);
        assert_eq!(props.items.len(), 3);
        assert_eq!(props.cursor, 0);
        assert!(props.selected.is_empty());
    }

    #[test]
    fn test_multiselect_props_selected() {
        let mut selected = HashSet::new();
        selected.insert(0);
        selected.insert(2);

        let props = MultiSelectProps::new(vec!["A", "B", "C"]).selected(selected);
        assert!(props.is_selected(0));
        assert!(!props.is_selected(1));
        assert!(props.is_selected(2));
    }

    #[test]
    fn test_multiselect_style_chars() {
        assert_eq!(MultiSelectStyle::Bracket.chars(), ("[x]", "[ ]"));
        assert_eq!(MultiSelectStyle::Unicode.chars(), (" ✓ ", " ○ "));
        assert_eq!(MultiSelectStyle::Circle.chars(), (" ● ", " ○ "));
        assert_eq!(MultiSelectStyle::Check.chars(), (" ☑ ", " ☐ "));
    }

    #[test]
    fn test_multiselect_selected_values() {
        let mut selected = HashSet::new();
        selected.insert(0);
        selected.insert(1);

        let props = MultiSelectProps::new(vec![
            MultiSelectItem::new("A").value("val_a"),
            MultiSelectItem::new("B").value("val_b"),
            MultiSelectItem::new("C"),
        ])
        .selected(selected);

        let values = props.selected_values();
        assert_eq!(values.len(), 2);
        assert!(values.contains(&"val_a"));
        assert!(values.contains(&"val_b"));
    }

    #[test]
    fn test_multiselect_state_navigation() {
        let mut state = MultiSelectState::new(5);
        assert_eq!(state.cursor, 0);

        state.down();
        assert_eq!(state.cursor, 1);

        state.down();
        state.down();
        assert_eq!(state.cursor, 3);

        state.up();
        assert_eq!(state.cursor, 2);

        state.first();
        assert_eq!(state.cursor, 0);

        state.last();
        assert_eq!(state.cursor, 4);
    }

    #[test]
    fn test_multiselect_state_toggle() {
        let mut state = MultiSelectState::new(3);

        state.toggle();
        assert!(state.is_selected(0));

        state.toggle();
        assert!(!state.is_selected(0));

        state.down();
        state.toggle();
        assert!(state.is_selected(1));
        assert!(!state.is_selected(0));
    }

    #[test]
    fn test_multiselect_state_select_all() {
        let mut state = MultiSelectState::new(3);

        state.select_all();
        assert!(state.all_selected());
        assert_eq!(state.selected_count(), 3);

        state.deselect_all();
        assert!(!state.has_selection());
        assert_eq!(state.selected_count(), 0);
    }

    #[test]
    fn test_multiselect_state_toggle_all() {
        let mut state = MultiSelectState::new(4);
        state.selected.insert(0);
        state.selected.insert(2);

        state.toggle_all();
        assert!(!state.is_selected(0));
        assert!(state.is_selected(1));
        assert!(!state.is_selected(2));
        assert!(state.is_selected(3));
    }

    #[test]
    fn test_multiselect_state_selected_indices() {
        let mut state = MultiSelectState::new(5);
        state.selected.insert(3);
        state.selected.insert(1);
        state.selected.insert(4);

        let indices = state.selected_indices();
        assert_eq!(indices, vec![1, 3, 4]); // Sorted
    }

    #[test]
    fn test_multiselect_render_lines() {
        let mut selected = HashSet::new();
        selected.insert(1);

        let props = MultiSelectProps::new(vec!["A", "B", "C"])
            .cursor(0)
            .selected(selected);

        let lines = props.render_lines();
        assert_eq!(lines.len(), 3);
        assert!(lines[0].0.contains("[ ]")); // A not selected
        assert!(lines[1].0.contains("[x]")); // B selected
        assert!(lines[2].0.contains("[ ]")); // C not selected
    }

    #[test]
    fn test_multiselect_component_render() {
        let props = MultiSelectProps::new(vec!["A", "B"]);
        let elem = MultiSelect::render(&props);
        assert!(elem.is_text());
    }
}
