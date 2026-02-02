use blaeck::prelude::*;

pub fn build_ui() -> Element {
    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            border_style: BorderStyle::Round,
            ..Default::default()
        },
        vec![
            // Title
            Element::node::<Text>(
                TextProps {
                    content: "Markdown Component".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            // Headers
            Element::node::<Text>(
                TextProps {
                    content: "Headers:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Markdown>(
                MarkdownProps::new("# Heading 1\n## Heading 2\n### Heading 3"),
                vec![],
            ),
            Element::text(""),
            // Inline formatting
            Element::node::<Text>(
                TextProps {
                    content: "Inline formatting:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Markdown>(
                MarkdownProps::new("This has **bold**, *italic*, and ~~strikethrough~~ text."),
                vec![],
            ),
            Element::text(""),
            // Code
            Element::node::<Text>(
                TextProps {
                    content: "Code spans:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Markdown>(
                MarkdownProps::new("Use `println!` or `eprintln!` for output."),
                vec![],
            ),
            Element::text(""),
            // Code blocks
            Element::node::<Text>(
                TextProps {
                    content: "Code blocks:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Markdown>(
                MarkdownProps::new("```rust\nfn main() {\n    println!(\"Hello!\");\n}\n```"),
                vec![],
            ),
            Element::text(""),
            // Lists
            Element::node::<Text>(
                TextProps {
                    content: "Bulleted list:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Markdown>(
                MarkdownProps::new("- First item\n- Second item\n- Third item"),
                vec![],
            ),
            Element::text(""),
            Element::node::<Text>(
                TextProps {
                    content: "Numbered list:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Markdown>(
                MarkdownProps::new("1. Step one\n2. Step two\n3. Step three"),
                vec![],
            ),
            Element::text(""),
            // Blockquote
            Element::node::<Text>(
                TextProps {
                    content: "Blockquote:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Markdown>(
                MarkdownProps::new("> This is a blockquote.\n> It can span multiple lines."),
                vec![],
            ),
            Element::text(""),
            // Links
            Element::node::<Text>(
                TextProps {
                    content: "Links (OSC 8 hyperlinks in supported terminals):".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Markdown>(
                MarkdownProps::new("Check out [Rust](https://www.rust-lang.org) and [Anthropic](https://www.anthropic.com)!"),
                vec![],
            ),
            Element::text(""),
            // Horizontal rule
            Element::node::<Text>(
                TextProps {
                    content: "Horizontal rule:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Markdown>(
                MarkdownProps::new("Above the line\n\n---\n\nBelow the line"),
                vec![],
            ),
            Element::text(""),
            // Custom colors
            Element::node::<Divider>(
                DividerProps::new()
                    .width(50)
                    .line_style(DividerStyle::Dashed)
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: "Custom colors (green headers, magenta code):".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Markdown>(
                MarkdownProps::new("# Custom Header\n\nWith `custom` code styling.")
                    .header_color(Color::Green)
                    .code_color(Color::Magenta),
                vec![],
            ),
            Element::text(""),
            // Combined example
            Element::node::<Text>(
                TextProps {
                    content: "Combined example:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Markdown>(
                MarkdownProps::new(r#"## Getting Started

1. Install the **dependencies**
2. Run `cargo build`
3. Check the *documentation*

> Remember: Code quality matters!

---

Happy coding!"#),
                vec![],
            ),
        ],
    )
}
