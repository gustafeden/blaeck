use blaeck::prelude::*;

pub fn build_ui() -> Element {
    let rust_code = r#"fn main() {
    let message = "Hello, world!";
    println!("{}", message);

    for i in 0..5 {
        println!("Count: {}", i);
    }
}"#;

    let python_code = r#"def greet(name):
    """Greet someone by name."""
    return f"Hello, {name}!"

if __name__ == "__main__":
    print(greet("World"))"#;

    let json_code = r#"{
    "name": "blaeck",
    "version": "0.1.0",
    "features": ["syntax", "animation"],
    "enabled": true
}"#;

    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            gap: 1.0,
            ..Default::default()
        },
        vec![
            // Title
            Element::node::<Text>(
                TextProps {
                    content: "Syntax Highlighting Demo".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            // Rust code with line numbers
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single,
                    padding: 1.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Rust (Ocean Dark theme):".into(),
                            bold: true,
                            color: Some(Color::Yellow),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<SyntaxHighlight>(
                        SyntaxHighlightProps::new(rust_code)
                            .language("rust")
                            .theme(SyntaxTheme::OceanDark)
                            .with_line_numbers(),
                        vec![],
                    ),
                ],
            ),
            // Python code
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single,
                    padding: 1.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Python (Ocean Dark theme):".into(),
                            bold: true,
                            color: Some(Color::Blue),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<SyntaxHighlight>(
                        SyntaxHighlightProps::new(python_code)
                            .language("python")
                            .theme(SyntaxTheme::OceanDark)
                            .line_numbers(LineNumberStyle::Padded),
                        vec![],
                    ),
                ],
            ),
            // JSON code
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single,
                    padding: 1.0,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "JSON (Eighties Dark theme):".into(),
                            bold: true,
                            color: Some(Color::Green),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<SyntaxHighlight>(
                        SyntaxHighlightProps::new(json_code)
                            .language("json")
                            .theme(SyntaxTheme::EightiesDark),
                        vec![],
                    ),
                ],
            ),
        ],
    )
}
