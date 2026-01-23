//! Static component - renders content that scrolls up and persists.
//!
//! The Static component is used to display content that should remain visible
//! above the dynamically updating UI. This is useful for things like completed
//! tasks, logs, or any output that shouldn't be re-rendered.
//!
//! Based on Ink's Static component pattern.

use crate::element::{Component, Element};
use crate::layout::LayoutStyle;
use crate::style::Style;

/// Properties for the Static component.
#[derive(Debug, Clone, Default)]
pub struct StaticProps {
    /// Items to render statically.
    /// Each item in this list will be rendered once and then scroll up
    /// as new items are added.
    items: Vec<StaticItem>,
}

/// An item that has been rendered statically.
#[derive(Debug, Clone)]
pub struct StaticItem {
    /// Unique key for this item
    pub key: String,
    /// The text content to render
    pub content: String,
    /// Optional style for this item
    pub style: Style,
}

impl StaticItem {
    /// Create a new static item with just content.
    pub fn new(key: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            content: content.into(),
            style: Style::default(),
        }
    }

    /// Create a new static item with content and style.
    pub fn with_style(key: impl Into<String>, content: impl Into<String>, style: Style) -> Self {
        Self {
            key: key.into(),
            content: content.into(),
            style,
        }
    }
}

impl StaticProps {
    /// Create new empty StaticProps.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an item to the static list.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, item: StaticItem) -> Self {
        self.items.push(item);
        self
    }

    /// Add a text item to the static list.
    #[must_use]
    pub fn add_text(mut self, key: impl Into<String>, content: impl Into<String>) -> Self {
        self.items.push(StaticItem::new(key, content));
        self
    }

    /// Get the items.
    pub fn items(&self) -> &[StaticItem] {
        &self.items
    }
}

/// A component that permanently renders its output above everything else.
///
/// Static content scrolls up and persists, unlike normal content which is
/// re-rendered in place. This is useful for displaying completed tasks,
/// logs, or any output that shouldn't change.
///
/// # Example
///
/// ```ignore
/// let props = StaticProps::new()
///     .add_text("task1", "✓ Task 1 complete")
///     .add_text("task2", "✓ Task 2 complete");
/// element! {
///     Box {
///         Static(props)
///         Text(content: "Current progress...")
///     }
/// }
/// ```
pub struct Static;

impl Static {
    /// Get the layout style for Static components.
    /// Static uses flex-direction: column by default.
    pub fn layout_style() -> LayoutStyle {
        LayoutStyle {
            flex_direction: crate::layout::FlexDirection::Column,
            ..Default::default()
        }
    }
}

impl Component for Static {
    type Props = StaticProps;

    fn render(props: &Self::Props) -> Element {
        // Static component renders its items as a vertical list of text elements
        if props.items.is_empty() {
            return Element::Empty;
        }

        // Return a Box containing all the static items as text
        let children: Vec<Element> = props
            .items
            .iter()
            .map(|item| Element::styled_text(&item.content, item.style))
            .collect();

        Element::node_with_layout::<crate::components::Box>(
            crate::components::BoxProps {
                flex_direction: crate::layout::FlexDirection::Column,
                ..Default::default()
            },
            Self::layout_style(),
            children,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    #[test]
    fn test_static_props_new() {
        let props = StaticProps::new();
        assert!(props.items().is_empty());
    }

    #[test]
    fn test_static_props_add_item() {
        let props = StaticProps::new()
            .add(StaticItem::new("key1", "Item 1"))
            .add(StaticItem::new("key2", "Item 2"));

        assert_eq!(props.items().len(), 2);
        assert_eq!(props.items()[0].key, "key1");
        assert_eq!(props.items()[1].key, "key2");
    }

    #[test]
    fn test_static_props_add_text() {
        let props = StaticProps::new()
            .add_text("1", "First")
            .add_text("2", "Second");

        assert_eq!(props.items().len(), 2);
        assert_eq!(props.items()[0].content, "First");
        assert_eq!(props.items()[1].content, "Second");
    }

    #[test]
    fn test_static_render_empty() {
        let props = StaticProps::new();
        let elem = Static::render(&props);
        assert!(elem.is_empty());
    }

    #[test]
    fn test_static_render_with_items() {
        let props = StaticProps::new()
            .add_text("1", "First")
            .add_text("2", "Second");

        let elem = Static::render(&props);
        assert!(elem.is_node());
        assert_eq!(elem.children().len(), 2);
    }

    #[test]
    fn test_static_render_with_styled_item() {
        let style = Style::new().fg(Color::Green);
        let props = StaticProps::new().add(StaticItem::with_style("task", "Completed task", style));

        let elem = Static::render(&props);
        assert!(elem.is_node());
        assert_eq!(elem.children().len(), 1);
    }

    #[test]
    fn test_static_layout_style() {
        let style = Static::layout_style();
        assert_eq!(style.flex_direction, crate::layout::FlexDirection::Column);
    }

    #[test]
    fn test_static_item_new() {
        let item = StaticItem::new("key1", "Content");
        assert_eq!(item.key, "key1");
        assert_eq!(item.content, "Content");
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_static_item_with_style() {
        let style = Style::new().fg(Color::Red);
        let item = StaticItem::with_style("key1", "Content", style);
        assert_eq!(item.key, "key1");
        assert_eq!(item.content, "Content");
        assert_eq!(item.style.fg, Color::Red);
    }

    #[test]
    fn test_static_item_clone() {
        let item = StaticItem::new("test", "Hello");
        let cloned = item.clone();
        assert_eq!(cloned.key, "test");
        assert_eq!(cloned.content, "Hello");
    }
}
