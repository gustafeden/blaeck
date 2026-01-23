//! Blaeck procedural macros.
//!
//! This crate provides the `element!` macro for declaring UI elements.

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::{Brace, Comma, Paren},
    Expr, FieldValue, Result, Token, Type,
};

/// A parsed child element - either an element or an expression for dynamic children.
enum ParsedChild {
    Element(ParsedElement),
    Expr(Expr),
}

/// A parsed element declaration.
///
/// Elements have the form:
/// ```ignore
/// ComponentType(prop1: value1, prop2: value2) {
///     ChildElement1
///     ChildElement2
/// }
/// ```
struct ParsedElement {
    /// The component type (e.g., Box, Text, Spacer)
    ty: Type,
    /// Property assignments (e.g., content: "Hello", bold: true)
    props: Punctuated<FieldValue, Comma>,
    /// Child elements
    children: Vec<ParsedChild>,
}

impl Parse for ParsedElement {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse the component type
        let ty: Type = input.parse()?;

        // Parse optional props in parentheses
        let props = if input.peek(Paren) {
            let props_input;
            parenthesized!(props_input in input);
            Punctuated::parse_terminated(&props_input)?
        } else {
            Punctuated::new()
        };

        // Parse optional children in braces
        let mut children = Vec::new();
        if input.peek(Brace) {
            let children_input;
            braced!(children_input in input);
            while !children_input.is_empty() {
                if children_input.peek(Token![#]) {
                    // Dynamic child: #(expr)
                    children_input.parse::<Token![#]>()?;
                    let expr_input;
                    parenthesized!(expr_input in children_input);
                    children.push(ParsedChild::Expr(expr_input.parse()?));
                } else {
                    // Static child element
                    children.push(ParsedChild::Element(children_input.parse()?));
                }
            }
        }

        Ok(Self {
            ty,
            props,
            children,
        })
    }
}

impl ToTokens for ParsedElement {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ty = &self.ty;

        // Generate property assignments
        let prop_assignments = self.props.iter().map(|fv| {
            let member = &fv.member;
            let expr = &fv.expr;
            quote! { _quill_props.#member = (#expr).into(); }
        });

        // Generate children
        let has_children = !self.children.is_empty();
        let children_code = if has_children {
            let child_elements = self.children.iter().map(|child| match child {
                ParsedChild::Element(elem) => {
                    quote! { #elem }
                }
                ParsedChild::Expr(expr) => {
                    quote! { #expr }
                }
            });
            Some(quote! {
                let _quill_children: ::std::vec::Vec<Element> = ::std::vec![#(#child_elements),*];
            })
        } else {
            None
        };

        let children_vec = if has_children {
            quote! { _quill_children }
        } else {
            quote! { ::std::vec![] }
        };

        // Generate the element creation code
        // We use types that must be in scope from the prelude
        tokens.extend(quote! {
            {
                type Props = <#ty as Component>::Props;
                let mut _quill_props: Props = ::std::default::Default::default();
                #(#prop_assignments)*
                #children_code
                Element::node::<#ty>(_quill_props, #children_vec)
            }
        });
    }
}

/// Used to declare an element and its properties.
///
/// **Important:** The following types must be in scope for the macro to work:
/// - `Component` - the component trait
/// - `Element` - the element type
/// - Any component types used (e.g., `Box`, `Text`, `Spacer`)
///
/// The easiest way to ensure this is to use `use quill::prelude::*;`
///
/// Elements are declared starting with their type. All properties are optional, so the simplest
/// use of this macro is just a type name:
///
/// ```ignore
/// element!(Text)
/// ```
///
/// This will evaluate to an `Element` with no properties set.
///
/// To specify properties, you can add them in a parenthesized block after the type name:
///
/// ```ignore
/// element! {
///     Text(content: "Hello, World!", color: Color::Green, bold: true)
/// }
/// ```
///
/// If the element has children, you can pass one or more child elements in braces:
///
/// ```ignore
/// element! {
///     Box {
///         Text(content: "Hello")
///         Text(content: "World")
///     }
/// }
/// ```
///
/// You can also use Rust expressions to conditionally add child elements via `#()` blocks:
///
/// ```ignore
/// element! {
///     Box {
///         #(if show_greeting {
///             element! { Text(content: "Hello!") }
///         } else {
///             Element::empty()
///         })
///     }
/// }
/// ```
///
/// # Examples
///
/// Simple text:
/// ```ignore
/// element! {
///     Text(content: "Hello")
/// }
/// ```
///
/// Box with border and children:
/// ```ignore
/// element! {
///     Box(border_style: BorderStyle::Single, padding: 1.0) {
///         Text(content: "Title", bold: true)
///         Spacer
///         Text(content: "Footer")
///     }
/// }
/// ```
///
/// Nested layout:
/// ```ignore
/// element! {
///     Box(flex_direction: FlexDirection::Column) {
///         Box(flex_direction: FlexDirection::Row) {
///             Text(content: "Left")
///             Spacer
///             Text(content: "Right")
///         }
///         Text(content: "Bottom")
///     }
/// }
/// ```
#[proc_macro]
pub fn element(input: TokenStream) -> TokenStream {
    let element = parse_macro_input!(input as ParsedElement);
    quote!(#element).into()
}

#[cfg(test)]
mod tests {
    // Tests run through the quill crate's integration tests
}
