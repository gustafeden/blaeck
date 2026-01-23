//! Tabs component - horizontal tab bar with selection.
//!
//! The Tabs component displays a horizontal set of tabs with a single tab selected.
//! Use `TabsState` to track which tab is active.
//!
//! ## When to use Tabs
//!
//! - Navigation between views/sections
//! - Mode switching (e.g., "Files | Search | Settings")
//! - When options are few and should be visible at once
//!
//! ## See also
//!
//! - [`Select`](super::Select) — Vertical list selection
//! - [`Breadcrumbs`](super::Breadcrumbs) — Path-based navigation

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// A single tab item.
#[derive(Debug, Clone)]
pub struct Tab {
    /// Tab label.
    pub label: String,
    /// Optional custom color for this tab.
    pub color: Option<Color>,
    /// Whether the tab is disabled.
    pub disabled: bool,
}

impl Tab {
    /// Create a new tab with label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            color: None,
            disabled: false,
        }
    }

    /// Set tab color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Mark tab as disabled.
    #[must_use]
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl<S: Into<String>> From<S> for Tab {
    fn from(s: S) -> Self {
        Tab::new(s)
    }
}

/// Style for tab dividers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabDivider {
    /// Vertical line: │
    #[default]
    Line,
    /// Dot: •
    Dot,
    /// Space (no visible divider)
    Space,
    /// Slash: /
    Slash,
    /// Custom character
    Custom(char),
}

impl TabDivider {
    /// Get the divider character.
    pub fn char(&self) -> char {
        match self {
            TabDivider::Line => '│',
            TabDivider::Dot => '•',
            TabDivider::Space => ' ',
            TabDivider::Slash => '/',
            TabDivider::Custom(c) => *c,
        }
    }
}

/// Style for the tab bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabStyle {
    /// Simple text with highlight
    #[default]
    Simple,
    /// Boxed tabs with borders
    Boxed,
    /// Underline active tab
    Underline,
}

/// Properties for the Tabs component.
#[derive(Debug, Clone)]
pub struct TabsProps {
    /// Tab items.
    pub tabs: Vec<Tab>,
    /// Currently selected tab index.
    pub selected: usize,
    /// Divider between tabs.
    pub divider: TabDivider,
    /// Color for selected tab.
    pub selected_color: Option<Color>,
    /// Background color for selected tab.
    pub selected_bg_color: Option<Color>,
    /// Color for unselected tabs.
    pub unselected_color: Option<Color>,
    /// Color for disabled tabs.
    pub disabled_color: Option<Color>,
    /// Color for dividers.
    pub divider_color: Option<Color>,
    /// Tab style.
    pub style: TabStyle,
    /// Padding around tab labels (spaces).
    pub padding: u8,
    /// Whether selected tab is bold.
    pub selected_bold: bool,
    /// Whether to underline selected tab.
    pub selected_underline: bool,
}

impl Default for TabsProps {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            selected: 0,
            divider: TabDivider::Line,
            selected_color: Some(Color::Cyan),
            selected_bg_color: None,
            unselected_color: None,
            disabled_color: Some(Color::DarkGray),
            divider_color: Some(Color::DarkGray),
            style: TabStyle::Simple,
            padding: 1,
            selected_bold: true,
            selected_underline: false,
        }
    }
}

impl TabsProps {
    /// Create new tabs with items.
    pub fn new<I, T>(tabs: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Tab>,
    {
        Self {
            tabs: tabs.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    /// Set selected tab index.
    #[must_use]
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    /// Set divider style.
    #[must_use]
    pub fn divider(mut self, divider: TabDivider) -> Self {
        self.divider = divider;
        self
    }

    /// Set selected tab color.
    #[must_use]
    pub fn selected_color(mut self, color: Color) -> Self {
        self.selected_color = Some(color);
        self
    }

    /// Set selected tab background color.
    #[must_use]
    pub fn selected_bg_color(mut self, color: Color) -> Self {
        self.selected_bg_color = Some(color);
        self
    }

    /// Set unselected tab color.
    #[must_use]
    pub fn unselected_color(mut self, color: Color) -> Self {
        self.unselected_color = Some(color);
        self
    }

    /// Set divider color.
    #[must_use]
    pub fn divider_color(mut self, color: Color) -> Self {
        self.divider_color = Some(color);
        self
    }

    /// Set tab style.
    #[must_use]
    pub fn style(mut self, style: TabStyle) -> Self {
        self.style = style;
        self
    }

    /// Set padding around labels.
    #[must_use]
    pub fn padding(mut self, padding: u8) -> Self {
        self.padding = padding;
        self
    }

    /// Set whether selected tab is bold.
    #[must_use]
    pub fn selected_bold(mut self, bold: bool) -> Self {
        self.selected_bold = bold;
        self
    }

    /// Set whether selected tab is underlined.
    #[must_use]
    pub fn selected_underline(mut self, underline: bool) -> Self {
        self.selected_underline = underline;
        self
    }

    /// No divider between tabs.
    #[must_use]
    pub fn no_divider(mut self) -> Self {
        self.divider = TabDivider::Space;
        self
    }

    /// Get selected tab.
    pub fn selected_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.selected)
    }

    /// Get selected tab label.
    pub fn selected_label(&self) -> Option<&str> {
        self.selected_tab().map(|t| t.label.as_str())
    }
}

/// A component that displays a horizontal tab bar.
///
/// # Examples
///
/// ```ignore
/// Element::node::<Tabs>(
///     TabsProps::new(vec!["Home", "Settings", "Help"])
///         .selected(0)
///         .selected_color(Color::Cyan),
///     vec![]
/// )
/// ```
pub struct Tabs;

impl Component for Tabs {
    type Props = TabsProps;

    fn render(props: &Self::Props) -> Element {
        if props.tabs.is_empty() {
            return Element::text("");
        }

        let padding = " ".repeat(props.padding as usize);
        let divider_char = props.divider.char();
        let mut parts: Vec<String> = Vec::new();

        for (i, tab) in props.tabs.iter().enumerate() {
            let is_selected = i == props.selected;

            // Add divider before tab (except first)
            if i > 0 {
                parts.push(format!(" {} ", divider_char));
            }

            // Format tab - use brackets for selected, spaces for unselected (same width)
            if is_selected {
                parts.push(format!("[{}{}{}]", padding, tab.label, padding));
            } else {
                parts.push(format!(" {}{}{} ", padding, tab.label, padding));
            }
        }

        let content = parts.join("");

        // Apply overall style based on selected tab's properties
        let mut style = Style::new();
        if let Some(color) = props.selected_color {
            style = style.fg(color);
        }
        if props.selected_bold {
            style = style.add_modifier(Modifier::BOLD);
        }

        Element::Text { content, style }
    }
}

/// State for tab navigation.
#[derive(Debug, Clone, Default)]
pub struct TabsState {
    /// Currently selected tab index.
    pub selected: usize,
    /// Total number of tabs.
    pub tab_count: usize,
}

impl TabsState {
    /// Create new state for tabs.
    pub fn new(tab_count: usize) -> Self {
        Self {
            selected: 0,
            tab_count,
        }
    }

    /// Select next tab (wraps around).
    pub fn next(&mut self) {
        if self.tab_count > 0 {
            self.selected = (self.selected + 1) % self.tab_count;
        }
    }

    /// Select previous tab (wraps around).
    pub fn prev(&mut self) {
        if self.tab_count > 0 {
            self.selected = if self.selected == 0 {
                self.tab_count - 1
            } else {
                self.selected - 1
            };
        }
    }

    /// Select first tab.
    pub fn first(&mut self) {
        self.selected = 0;
    }

    /// Select last tab.
    pub fn last(&mut self) {
        if self.tab_count > 0 {
            self.selected = self.tab_count - 1;
        }
    }

    /// Select specific tab by index.
    pub fn select(&mut self, index: usize) {
        if index < self.tab_count {
            self.selected = index;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_new() {
        let tab = Tab::new("Home");
        assert_eq!(tab.label, "Home");
        assert!(tab.color.is_none());
        assert!(!tab.disabled);
    }

    #[test]
    fn test_tab_builder() {
        let tab = Tab::new("Settings").color(Color::Blue).disabled();
        assert_eq!(tab.label, "Settings");
        assert_eq!(tab.color, Some(Color::Blue));
        assert!(tab.disabled);
    }

    #[test]
    fn test_tab_from_string() {
        let tab: Tab = "Help".into();
        assert_eq!(tab.label, "Help");
    }

    #[test]
    fn test_tabs_props_new() {
        let props = TabsProps::new(vec!["A", "B", "C"]);
        assert_eq!(props.tabs.len(), 3);
        assert_eq!(props.selected, 0);
    }

    #[test]
    fn test_tabs_props_builder() {
        let props = TabsProps::new(vec!["A", "B"])
            .selected(1)
            .selected_color(Color::Yellow)
            .divider(TabDivider::Dot);
        assert_eq!(props.selected, 1);
        assert_eq!(props.selected_color, Some(Color::Yellow));
        assert_eq!(props.divider, TabDivider::Dot);
    }

    #[test]
    fn test_tabs_state_navigation() {
        let mut state = TabsState::new(4);
        assert_eq!(state.selected, 0);

        state.next();
        assert_eq!(state.selected, 1);

        state.next();
        state.next();
        assert_eq!(state.selected, 3);

        state.next(); // Wrap around
        assert_eq!(state.selected, 0);

        state.prev(); // Wrap back
        assert_eq!(state.selected, 3);

        state.first();
        assert_eq!(state.selected, 0);

        state.last();
        assert_eq!(state.selected, 3);
    }

    #[test]
    fn test_tabs_state_select() {
        let mut state = TabsState::new(5);
        state.select(3);
        assert_eq!(state.selected, 3);

        state.select(10); // Out of bounds, should not change
        assert_eq!(state.selected, 3);
    }

    #[test]
    fn test_tabs_render_empty() {
        let props = TabsProps::new(Vec::<&str>::new());
        let elem = Tabs::render(&props);
        assert!(elem.is_text());
    }

    #[test]
    fn test_tabs_render_basic() {
        let props = TabsProps::new(vec!["A", "B", "C"]);
        let elem = Tabs::render(&props);
        assert!(elem.is_text());
    }

    #[test]
    fn test_tabs_selected_label() {
        let props = TabsProps::new(vec!["Home", "Settings"]).selected(1);
        assert_eq!(props.selected_label(), Some("Settings"));
    }

    #[test]
    fn test_tab_divider_chars() {
        assert_eq!(TabDivider::Line.char(), '│');
        assert_eq!(TabDivider::Dot.char(), '•');
        assert_eq!(TabDivider::Space.char(), ' ');
        assert_eq!(TabDivider::Slash.char(), '/');
        assert_eq!(TabDivider::Custom('X').char(), 'X');
    }
}
