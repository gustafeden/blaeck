//! Component and Element types for building declarative terminal UIs.
//!
//! This module defines the core abstractions:
//!
//! - **`Element`** — A node in the UI tree. Can be `Text`, `Node` (component), `Fragment`, or `Empty`.
//! - **`Component`** — A trait for defining UI pieces. Has a `Props` type and `render()` method.
//!
//! Components are type-erased via `TypeId` + `Box<dyn Any>` so the element tree can hold
//! any component type. Props are downcast at render time.
//!
//! Pattern from Iocraft. See `ARCHITECTURE.md` for why type erasure is used.

use crate::layout::LayoutStyle;
use crate::style::Style;
use std::any::{Any, TypeId};

/// A component that can be rendered.
///
/// Components are the building blocks of a Blaeck UI. Each component
/// defines how to render itself given a set of properties.
pub trait Component: 'static {
    /// The type of properties this component accepts.
    type Props: Default + 'static;

    /// Render this component with the given props, returning an Element tree.
    fn render(props: &Self::Props) -> Element;
}

/// An element in the UI tree.
///
/// Elements are lightweight descriptions of what to render.
/// They can be text, styled text, component nodes, or fragments.
#[derive(Default)]
pub enum Element {
    /// An empty element (renders nothing)
    #[default]
    Empty,
    /// A text element with optional styling
    Text {
        /// The text content
        content: String,
        /// The style to apply
        style: Style,
    },
    /// A component node with props and children
    Node {
        /// The TypeId of the component
        type_id: TypeId,
        /// The props as a boxed Any
        props: Box<dyn Any>,
        /// The layout style for this node
        layout_style: LayoutStyle,
        /// Child elements
        children: Vec<Element>,
        /// Render function for this component
        render_fn: fn(&dyn Any) -> Element,
    },
    /// A fragment containing multiple elements (no wrapping container)
    Fragment(Vec<Element>),
}

impl Element {
    /// Create an empty element.
    pub fn empty() -> Self {
        Element::Empty
    }

    /// Create a text element with default style.
    pub fn text(s: impl Into<String>) -> Self {
        Element::Text {
            content: s.into(),
            style: Style::default(),
        }
    }

    /// Create a text element with a specific style.
    pub fn styled_text(s: impl Into<String>, style: Style) -> Self {
        Element::Text {
            content: s.into(),
            style,
        }
    }

    /// Create a component node.
    ///
    /// ## Why type erasure?
    ///
    /// The element tree needs to hold any component type in a single `Vec<Element>`.
    /// Rust doesn't have heterogeneous collections, so we have three options:
    ///
    /// 1. **Enum with all variants** — Doesn't scale, can't add custom components
    /// 2. **Trait objects (`dyn Component`)** — Loses the `Props` type information
    /// 3. **Type erasure** — Store `TypeId` + `Box<dyn Any>` + a render function
    ///
    /// We use option 3. The `render_fn` closure captures the concrete type `C`
    /// at construction time, so it can downcast `props` back to `C::Props` at
    /// render time. This gives us type safety (wrong props = panic) while
    /// allowing heterogeneous trees.
    ///
    /// Pattern copied from Iocraft.
    pub fn node<C: Component>(props: C::Props, children: Vec<Element>) -> Self {
        Element::Node {
            type_id: TypeId::of::<C>(),
            props: Box::new(props),
            layout_style: LayoutStyle::default(),
            children,
            render_fn: |props_any| {
                let props = props_any.downcast_ref::<C::Props>().unwrap();
                C::render(props)
            },
        }
    }

    /// Create a component node with layout style.
    pub fn node_with_layout<C: Component>(
        props: C::Props,
        layout_style: LayoutStyle,
        children: Vec<Element>,
    ) -> Self {
        Element::Node {
            type_id: TypeId::of::<C>(),
            props: Box::new(props),
            layout_style,
            children,
            render_fn: |props_any| {
                let props = props_any.downcast_ref::<C::Props>().unwrap();
                C::render(props)
            },
        }
    }

    /// Get the type ID if this is a node element.
    pub fn type_id(&self) -> Option<TypeId> {
        match self {
            Element::Node { type_id, .. } => Some(*type_id),
            _ => None,
        }
    }

    /// Check if this element is empty.
    pub fn is_empty(&self) -> bool {
        matches!(self, Element::Empty)
    }

    /// Check if this element is text.
    pub fn is_text(&self) -> bool {
        matches!(self, Element::Text { .. })
    }

    /// Check if this element is a node.
    pub fn is_node(&self) -> bool {
        matches!(self, Element::Node { .. })
    }

    /// Check if this element is a fragment.
    pub fn is_fragment(&self) -> bool {
        matches!(self, Element::Fragment(_))
    }

    /// Create a fragment from multiple elements.
    ///
    /// **Note:** Fragments render children **horizontally** (inline).
    /// For vertical stacking, use [`Element::column`] instead.
    pub fn fragment(elements: Vec<Element>) -> Self {
        Element::Fragment(elements)
    }

    /// Create a vertical column of elements.
    ///
    /// This is the recommended way to render a dynamic list of items vertically.
    /// It creates a `Box` with `flex_direction: Column`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let items = vec!["Apple", "Banana", "Cherry"];
    /// Element::column(
    ///     items.iter().map(|item| {
    ///         element! { Text(content: item) }
    ///     }).collect()
    /// )
    /// ```
    pub fn column(children: Vec<Element>) -> Self {
        use crate::components::{Box, BoxProps};
        Element::node::<Box>(
            BoxProps {
                flex_direction: crate::layout::FlexDirection::Column,
                ..Default::default()
            },
            children,
        )
    }

    /// Create a horizontal row of elements.
    ///
    /// This creates a `Box` with `flex_direction: Row`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Element::row(vec![
    ///     element! { Text(content: "Left") },
    ///     element! { Spacer },
    ///     element! { Text(content: "Right") },
    /// ])
    /// ```
    pub fn row(children: Vec<Element>) -> Self {
        use crate::components::{Box, BoxProps};
        Element::node::<Box>(
            BoxProps {
                flex_direction: crate::layout::FlexDirection::Row,
                ..Default::default()
            },
            children,
        )
    }

    /// Get the children of this element (empty for non-nodes).
    pub fn children(&self) -> &[Element] {
        match self {
            Element::Node { children, .. } => children,
            _ => &[],
        }
    }

    /// Get the layout style of this element.
    pub fn layout_style(&self) -> &LayoutStyle {
        static DEFAULT_LAYOUT: LayoutStyle = LayoutStyle {
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            flex_direction: crate::layout::FlexDirection::Column,
            flex_grow: 0.0,
            flex_shrink: 0.0,
            padding: 0.0,
            padding_left: None,
            padding_right: None,
            padding_top: None,
            padding_bottom: None,
            margin: 0.0,
            margin_left: None,
            margin_right: None,
            margin_top: None,
            margin_bottom: None,
            gap: 0.0,
            align_items: None,
            align_self: None,
            align_content: None,
            justify_content: None,
        };
        match self {
            Element::Node { layout_style, .. } => layout_style,
            _ => &DEFAULT_LAYOUT,
        }
    }

    /// Invoke the render function for a node element.
    pub fn render_component(&self) -> Option<Element> {
        match self {
            Element::Node {
                props, render_fn, ..
            } => Some(render_fn(props.as_ref())),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    // Test component
    struct TestComponent;

    #[derive(Default)]
    struct TestProps {
        value: i32,
    }

    impl Component for TestComponent {
        type Props = TestProps;

        fn render(props: &Self::Props) -> Element {
            Element::text(format!("Value: {}", props.value))
        }
    }

    // Another test component for nested testing
    struct ContainerComponent;

    #[derive(Default)]
    struct ContainerProps {
        label: String,
    }

    impl Component for ContainerComponent {
        type Props = ContainerProps;

        fn render(props: &Self::Props) -> Element {
            Element::text(format!("Container: {}", props.label))
        }
    }

    #[test]
    fn test_element_empty() {
        let elem = Element::empty();
        assert!(elem.is_empty());
        assert!(!elem.is_text());
        assert!(!elem.is_node());
    }

    #[test]
    fn test_element_text() {
        let elem = Element::text("Hello");
        match &elem {
            Element::Text { content, style } => {
                assert_eq!(content, "Hello");
                assert_eq!(*style, Style::default());
            }
            _ => panic!("Expected Text"),
        }
        assert!(elem.is_text());
        assert!(!elem.is_empty());
        assert!(!elem.is_node());
    }

    #[test]
    fn test_element_styled_text() {
        let style = Style::new().fg(Color::Red).bold();
        let elem = Element::styled_text("Styled", style);
        match &elem {
            Element::Text {
                content, style: s, ..
            } => {
                assert_eq!(content, "Styled");
                assert_eq!(s.fg, Color::Red);
            }
            _ => panic!("Expected Text"),
        }
    }

    #[test]
    fn test_element_node() {
        let elem = Element::node::<TestComponent>(TestProps { value: 42 }, vec![]);
        match &elem {
            Element::Node { children, .. } => assert!(children.is_empty()),
            _ => panic!("Expected Node"),
        }
        assert!(elem.is_node());
        assert!(!elem.is_empty());
        assert!(!elem.is_text());
    }

    #[test]
    fn test_element_with_children() {
        let child = Element::text("Child");
        let elem = Element::node::<TestComponent>(TestProps::default(), vec![child]);
        match &elem {
            Element::Node { children, .. } => assert_eq!(children.len(), 1),
            _ => panic!("Expected Node"),
        }
    }

    #[test]
    fn test_element_type_id() {
        let elem = Element::node::<TestComponent>(TestProps::default(), vec![]);
        assert_eq!(elem.type_id(), Some(TypeId::of::<TestComponent>()));

        let text_elem = Element::text("Hello");
        assert_eq!(text_elem.type_id(), None);
    }

    #[test]
    fn test_element_children() {
        let child1 = Element::text("A");
        let child2 = Element::text("B");
        let elem = Element::node::<TestComponent>(TestProps::default(), vec![child1, child2]);

        assert_eq!(elem.children().len(), 2);

        let empty = Element::empty();
        assert!(empty.children().is_empty());
    }

    #[test]
    fn test_element_default() {
        let elem = Element::default();
        assert!(elem.is_empty());
    }

    #[test]
    fn test_component_render() {
        let elem = Element::node::<TestComponent>(TestProps { value: 99 }, vec![]);
        let rendered = elem.render_component().unwrap();

        match rendered {
            Element::Text { content, .. } => {
                assert_eq!(content, "Value: 99");
            }
            _ => panic!("Expected Text from render"),
        }
    }

    #[test]
    fn test_nested_components() {
        let inner = Element::node::<TestComponent>(TestProps { value: 10 }, vec![]);
        let outer = Element::node::<ContainerComponent>(
            ContainerProps {
                label: "Outer".to_string(),
            },
            vec![inner],
        );

        assert!(outer.is_node());
        assert_eq!(outer.children().len(), 1);
        assert!(outer.children()[0].is_node());
    }

    #[test]
    fn test_element_node_with_layout() {
        use crate::layout::FlexDirection;

        let layout = LayoutStyle {
            width: Some(100.0),
            height: Some(50.0),
            flex_direction: FlexDirection::Row,
            ..Default::default()
        };
        let elem = Element::node_with_layout::<TestComponent>(TestProps::default(), layout, vec![]);

        let style = elem.layout_style();
        assert_eq!(style.width, Some(100.0));
        assert_eq!(style.height, Some(50.0));
        assert_eq!(style.flex_direction, FlexDirection::Row);
    }

    #[test]
    fn test_render_component_on_non_node() {
        let text = Element::text("Hello");
        assert!(text.render_component().is_none());

        let empty = Element::empty();
        assert!(empty.render_component().is_none());
    }
}
