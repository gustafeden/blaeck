//! Transform component for applying text transformations.

use crate::element::{Component, Element};

/// Text transformation function type.
pub type TransformFn = fn(&str) -> String;

/// Built-in transformations.
pub mod transforms {
    /// Convert text to uppercase.
    pub fn uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    /// Convert text to lowercase.
    pub fn lowercase(s: &str) -> String {
        s.to_lowercase()
    }

    /// Capitalize first letter of each word.
    pub fn capitalize(s: &str) -> String {
        s.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(chars).collect(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Reverse the text.
    pub fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }

    /// Trim whitespace.
    pub fn trim(s: &str) -> String {
        s.trim().to_string()
    }
}

/// Properties for the Transform component.
#[derive(Default)]
pub struct TransformProps {
    /// The transformation function to apply.
    pub transform: Option<TransformFn>,
}

/// Transform component that applies a transformation to its children's text.
pub struct Transform;

impl Component for Transform {
    type Props = TransformProps;

    fn render(_props: &Self::Props) -> Element {
        // Transform is a pass-through - actual transformation happens during rendering
        // The transform prop is used by the renderer to modify text content
        Element::empty()
    }
}

impl Transform {
    /// Apply transformation to text content.
    pub fn apply(transform: Option<TransformFn>, text: &str) -> String {
        match transform {
            Some(f) => f(text),
            None => text.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::transforms::*;

    #[test]
    fn test_uppercase() {
        assert_eq!(uppercase("hello"), "HELLO");
    }

    #[test]
    fn test_lowercase() {
        assert_eq!(lowercase("HELLO"), "hello");
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello world"), "Hello World");
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("hello"), "olleh");
    }

    #[test]
    fn test_trim() {
        assert_eq!(trim("  hello  "), "hello");
    }

    #[test]
    fn test_transform_apply() {
        assert_eq!(Transform::apply(Some(uppercase), "hello"), "HELLO");
        assert_eq!(Transform::apply(None, "hello"), "hello");
    }

    #[test]
    fn test_transform_props_default() {
        let props = TransformProps::default();
        assert!(props.transform.is_none());
    }

    #[test]
    fn test_transform_component_render() {
        let props = TransformProps {
            transform: Some(uppercase),
        };
        let elem = Transform::render(&props);
        assert!(elem.is_empty());
    }

    #[test]
    fn test_uppercase_with_mixed_case() {
        assert_eq!(uppercase("HeLLo WoRLd"), "HELLO WORLD");
    }

    #[test]
    fn test_lowercase_with_mixed_case() {
        assert_eq!(lowercase("HeLLo WoRLd"), "hello world");
    }

    #[test]
    fn test_capitalize_single_word() {
        assert_eq!(capitalize("hello"), "Hello");
    }

    #[test]
    fn test_capitalize_empty() {
        assert_eq!(capitalize(""), "");
    }

    #[test]
    fn test_reverse_empty() {
        assert_eq!(reverse(""), "");
    }

    #[test]
    fn test_trim_no_whitespace() {
        assert_eq!(trim("hello"), "hello");
    }

    #[test]
    fn test_trim_only_whitespace() {
        assert_eq!(trim("   "), "");
    }
}
