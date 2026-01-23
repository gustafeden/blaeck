//! Select component - interactive single-item selection.
//!
//! The Select component displays a list of options that users can navigate
//! with arrow keys and select with Enter. Use `SelectState` to track selection.
//!
//! ## When to use Select
//!
//! - Single choice from a list of options
//! - Menu navigation
//! - Settings with discrete values
//!
//! ## See also
//!
//! - [`MultiSelect`](super::MultiSelect) — Multiple selections (checkboxes)
//! - [`Autocomplete`](super::Autocomplete) — Filterable selection with text input
//! - [`Confirm`](super::Confirm) — Simple yes/no choice
//! - [`Tabs`](super::Tabs) — Horizontal selection (tab bar style)

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// A single item in a select list.
#[derive(Debug, Clone)]
pub struct SelectItem {
    /// Display label for the item.
    pub label: String,
    /// Optional value (defaults to label if not set).
    pub value: Option<String>,
    /// Whether the item is disabled.
    pub disabled: bool,
}

impl SelectItem {
    /// Create a new select item with the given label.
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

impl<S: Into<String>> From<S> for SelectItem {
    fn from(s: S) -> Self {
        SelectItem::new(s)
    }
}

/// Style for the select indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectIndicator {
    /// Arrow indicator: ❯
    #[default]
    Arrow,
    /// Pointer: ▸
    Pointer,
    /// Bullet: •
    Bullet,
    /// Checkbox style: ○ / ●
    Radio,
    /// Simple: >
    Simple,
    /// Numbered with arrow: ❯ 1. / 2. / 3.
    Numbered,
}

impl SelectIndicator {
    /// Get the characters for selected and unselected states.
    pub fn chars(&self) -> (&'static str, &'static str) {
        match self {
            SelectIndicator::Arrow => ("❯", " "),
            SelectIndicator::Pointer => ("▸", " "),
            SelectIndicator::Bullet => ("•", " "),
            SelectIndicator::Radio => ("●", "○"),
            SelectIndicator::Simple => (">", " "),
            SelectIndicator::Numbered => ("❯", " "), // Base indicator, numbers handled separately
        }
    }

    /// Check if this indicator uses numbered items.
    pub fn is_numbered(&self) -> bool {
        matches!(self, SelectIndicator::Numbered)
    }
}

/// Properties for the Select component.
#[derive(Debug, Clone)]
pub struct SelectProps {
    /// List of items to display.
    pub items: Vec<SelectItem>,
    /// Currently highlighted index.
    pub selected: usize,
    /// Indicator style.
    pub indicator: SelectIndicator,
    /// Color for selected item.
    pub selected_color: Option<Color>,
    /// Color for unselected items.
    pub unselected_color: Option<Color>,
    /// Color for disabled items.
    pub disabled_color: Option<Color>,
    /// Maximum visible items (for scrolling).
    pub max_visible: Option<usize>,
    /// Scroll offset for long lists.
    pub scroll_offset: usize,
    /// Whether to show the indicator for unselected items.
    pub show_unselected_indicator: bool,
}

impl Default for SelectProps {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selected: 0,
            indicator: SelectIndicator::Arrow,
            selected_color: Some(Color::Cyan),
            unselected_color: None,
            disabled_color: Some(Color::DarkGray),
            max_visible: None,
            scroll_offset: 0,
            show_unselected_indicator: true,
        }
    }
}

impl SelectProps {
    /// Create new SelectProps with the given items.
    pub fn new<I, T>(items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<SelectItem>,
    {
        Self {
            items: items.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    /// Set the selected index.
    #[must_use]
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index.min(self.items.len().saturating_sub(1));
        self
    }

    /// Set the indicator style.
    #[must_use]
    pub fn indicator(mut self, indicator: SelectIndicator) -> Self {
        self.indicator = indicator;
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

    /// Hide the indicator for unselected items.
    #[must_use]
    pub fn hide_unselected_indicator(mut self) -> Self {
        self.show_unselected_indicator = false;
        self
    }

    /// Get the currently selected item.
    pub fn selected_item(&self) -> Option<&SelectItem> {
        self.items.get(self.selected)
    }

    /// Get the selected value.
    pub fn selected_value(&self) -> Option<&str> {
        self.selected_item().map(|item| item.get_value())
    }

    /// Find the next item starting with the given character (case-insensitive).
    /// Searches from current selection + 1, wrapping around.
    pub fn find_by_char(&self, c: char, current: usize) -> Option<usize> {
        let c_lower = c.to_ascii_lowercase();
        let len = self.items.len();
        if len == 0 {
            return None;
        }

        // Search from current+1 to end, then from 0 to current
        for i in 1..=len {
            let idx = (current + i) % len;
            if let Some(first_char) = self.items[idx].label.chars().next() {
                if first_char.to_ascii_lowercase() == c_lower && !self.items[idx].disabled {
                    return Some(idx);
                }
            }
        }
        None
    }

    /// Build the display strings for all visible items.
    pub fn render_lines(&self) -> Vec<(String, Style)> {
        let (selected_char, unselected_char) = self.indicator.chars();
        let visible_items = self.visible_items();
        let is_numbered = self.indicator.is_numbered();

        // Calculate max width from ALL items to prevent box resizing when scrolling
        let max_label_width = self
            .items
            .iter()
            .map(|item| item.label.chars().count())
            .max()
            .unwrap_or(0);

        // For numbered indicators, calculate the width needed for the largest number
        let max_num_width = if is_numbered {
            self.items.len().to_string().len()
        } else {
            0
        };

        visible_items
            .iter()
            .map(|(idx, item)| {
                let is_selected = *idx == self.selected;
                let indicator = if is_selected {
                    selected_char
                } else if self.show_unselected_indicator {
                    unselected_char
                } else {
                    " "
                };

                // Pad to max width so box doesn't resize when scrolling
                let padding = max_label_width.saturating_sub(item.label.chars().count());

                let line = if is_numbered {
                    // Format: "❯ 1. Label" or "  2. Label"
                    let num = idx + 1;
                    let num_str = format!("{:>width$}", num, width = max_num_width);
                    format!(
                        "{} {}. {}{}",
                        indicator,
                        num_str,
                        item.label,
                        " ".repeat(padding)
                    )
                } else {
                    format!("{} {}{}", indicator, item.label, " ".repeat(padding))
                };

                let mut style = Style::new();
                if item.disabled {
                    if let Some(color) = self.disabled_color {
                        style = style.fg(color);
                    }
                    style = style.add_modifier(Modifier::DIM);
                } else if is_selected {
                    if let Some(color) = self.selected_color {
                        style = style.fg(color);
                    }
                    style = style.add_modifier(Modifier::BOLD);
                } else if let Some(color) = self.unselected_color {
                    style = style.fg(color);
                }

                (line, style)
            })
            .collect()
    }

    /// Get the visible items based on scroll offset and max_visible.
    fn visible_items(&self) -> Vec<(usize, &SelectItem)> {
        let items: Vec<_> = self.items.iter().enumerate().collect();

        if let Some(max) = self.max_visible {
            if items.len() > max {
                let start = self.scroll_offset.min(items.len().saturating_sub(max));
                return items.into_iter().skip(start).take(max).collect();
            }
        }

        items
    }
}

/// A component that displays a selectable list.
///
/// # Examples
///
/// ```ignore
/// let items = vec!["Option 1", "Option 2", "Option 3"];
/// let props = SelectProps::new(items)
///     .selected(0)
///     .selected_color(Color::Green);
///
/// // Handle input in your event loop:
/// match key.code {
///     KeyCode::Up => selected = selected.saturating_sub(1),
///     KeyCode::Down => selected = (selected + 1).min(items.len() - 1),
///     KeyCode::Enter => return Some(selected),
///     _ => {}
/// }
/// ```
pub struct Select;

impl Component for Select {
    type Props = SelectProps;

    fn render(props: &Self::Props) -> Element {
        let lines = props.render_lines();

        // Join lines with newlines
        let content: String = lines
            .iter()
            .map(|(line, _)| line.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        // For now, use the first item's style (selected) or default
        // A more sophisticated version would render each line separately
        let style = lines
            .iter()
            .find(|(_, s)| s.modifiers.contains(Modifier::BOLD))
            .map(|(_, s)| *s)
            .unwrap_or_default();

        Element::styled_text(&content, style)
    }
}

/// Helper struct for managing select state.
#[derive(Debug, Clone)]
pub struct SelectState {
    /// Currently selected index.
    pub selected: usize,
    /// Number of items.
    pub count: usize,
    /// Scroll offset for long lists.
    pub scroll_offset: usize,
    /// Maximum visible items.
    pub max_visible: Option<usize>,
}

impl SelectState {
    /// Create a new select state.
    pub fn new(count: usize) -> Self {
        Self {
            selected: 0,
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

    /// Move selection up.
    pub fn up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            self.adjust_scroll();
        }
    }

    /// Move selection down.
    pub fn down(&mut self) {
        if self.selected < self.count.saturating_sub(1) {
            self.selected += 1;
            self.adjust_scroll();
        }
    }

    /// Move to first item.
    pub fn first(&mut self) {
        self.selected = 0;
        self.scroll_offset = 0;
    }

    /// Move to last item.
    pub fn last(&mut self) {
        self.selected = self.count.saturating_sub(1);
        self.adjust_scroll();
    }

    /// Jump to a specific index.
    pub fn jump_to(&mut self, index: usize) {
        if index < self.count {
            self.selected = index;
            self.adjust_scroll();
        }
    }

    /// Page up (move by max_visible or 5 items).
    pub fn page_up(&mut self) {
        let page_size = self.max_visible.unwrap_or(5);
        self.selected = self.selected.saturating_sub(page_size);
        self.adjust_scroll();
    }

    /// Page down (move by max_visible or 5 items).
    pub fn page_down(&mut self) {
        let page_size = self.max_visible.unwrap_or(5);
        self.selected = (self.selected + page_size).min(self.count.saturating_sub(1));
        self.adjust_scroll();
    }

    /// Adjust scroll offset to keep selection visible.
    fn adjust_scroll(&mut self) {
        if let Some(max) = self.max_visible {
            if self.selected < self.scroll_offset {
                self.scroll_offset = self.selected;
            } else if self.selected >= self.scroll_offset + max {
                self.scroll_offset = self.selected - max + 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_item_new() {
        let item = SelectItem::new("Test");
        assert_eq!(item.label, "Test");
        assert!(item.value.is_none());
        assert!(!item.disabled);
    }

    #[test]
    fn test_select_item_with_value() {
        let item = SelectItem::new("Display").value("actual_value");
        assert_eq!(item.label, "Display");
        assert_eq!(item.get_value(), "actual_value");
    }

    #[test]
    fn test_select_item_from_str() {
        let item: SelectItem = "Option".into();
        assert_eq!(item.label, "Option");
    }

    #[test]
    fn test_select_props_new() {
        let props = SelectProps::new(vec!["A", "B", "C"]);
        assert_eq!(props.items.len(), 3);
        assert_eq!(props.selected, 0);
    }

    #[test]
    fn test_select_props_selected() {
        let props = SelectProps::new(vec!["A", "B", "C"]).selected(1);
        assert_eq!(props.selected, 1);
    }

    #[test]
    fn test_select_props_selected_clamped() {
        let props = SelectProps::new(vec!["A", "B"]).selected(10);
        assert_eq!(props.selected, 1); // Clamped to last item
    }

    #[test]
    fn test_select_props_selected_item() {
        let props = SelectProps::new(vec!["A", "B", "C"]).selected(1);
        assert_eq!(props.selected_item().unwrap().label, "B");
    }

    #[test]
    fn test_select_props_selected_value() {
        let props = SelectProps::new(vec![
            SelectItem::new("Display").value("val"),
            SelectItem::new("Other"),
        ])
        .selected(0);
        assert_eq!(props.selected_value(), Some("val"));
    }

    #[test]
    fn test_select_indicator_arrow() {
        let (sel, unsel) = SelectIndicator::Arrow.chars();
        assert_eq!(sel, "❯");
        assert_eq!(unsel, " ");
    }

    #[test]
    fn test_select_indicator_radio() {
        let (sel, unsel) = SelectIndicator::Radio.chars();
        assert_eq!(sel, "●");
        assert_eq!(unsel, "○");
    }

    #[test]
    fn test_select_indicator_numbered() {
        assert!(SelectIndicator::Numbered.is_numbered());
        assert!(!SelectIndicator::Arrow.is_numbered());
    }

    #[test]
    fn test_select_numbered_render_lines() {
        let props = SelectProps::new(vec!["Yes", "No"])
            .selected(0)
            .indicator(SelectIndicator::Numbered);
        let lines = props.render_lines();
        assert_eq!(lines.len(), 2);
        // First line should be: "❯ 1. Yes"
        assert!(lines[0].0.contains("❯"));
        assert!(lines[0].0.contains("1."));
        assert!(lines[0].0.contains("Yes"));
        // Second line should be: "  2. No"
        assert!(lines[1].0.contains("2."));
        assert!(lines[1].0.contains("No"));
        assert!(!lines[1].0.contains("❯")); // Unselected should not have arrow
    }

    #[test]
    fn test_select_numbered_format() {
        let props = SelectProps::new(vec!["Yes", "Yes, and don't ask again", "No"])
            .selected(0)
            .indicator(SelectIndicator::Numbered);
        let lines = props.render_lines();
        // Verify the format matches expected: "❯ 1. Yes"
        assert!(lines[0].0.starts_with("❯ 1."));
        assert!(lines[1].0.starts_with("  2."));
        assert!(lines[2].0.starts_with("  3."));
    }

    #[test]
    fn test_select_render_lines() {
        let props = SelectProps::new(vec!["A", "B", "C"]).selected(1);
        let lines = props.render_lines();
        assert_eq!(lines.len(), 3);
        assert!(lines[0].0.contains("A"));
        assert!(lines[1].0.contains("B"));
        assert!(lines[2].0.contains("C"));
    }

    #[test]
    fn test_select_state_up_down() {
        let mut state = SelectState::new(3);
        assert_eq!(state.selected, 0);

        state.down();
        assert_eq!(state.selected, 1);

        state.down();
        assert_eq!(state.selected, 2);

        state.down(); // Should not go past end
        assert_eq!(state.selected, 2);

        state.up();
        assert_eq!(state.selected, 1);
    }

    #[test]
    fn test_select_state_first_last() {
        let mut state = SelectState::new(5);
        state.selected = 2;

        state.last();
        assert_eq!(state.selected, 4);

        state.first();
        assert_eq!(state.selected, 0);
    }

    #[test]
    fn test_select_state_scroll() {
        let mut state = SelectState::new(10).max_visible(3);
        assert_eq!(state.scroll_offset, 0);

        // Move down past visible area
        state.down();
        state.down();
        state.down(); // Now at index 3, should scroll
        assert_eq!(state.selected, 3);
        assert_eq!(state.scroll_offset, 1);
    }
}
