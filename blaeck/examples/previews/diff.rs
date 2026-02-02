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
                    content: "Diff Component".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            // Simple diff with builder
            Element::node::<Text>(
                TextProps {
                    content: "Simple diff (builder pattern):".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Diff>(
                DiffProps::new()
                    .context("fn calculate(x: i32) -> i32 {")
                    .removed("    x * 2")
                    .added("    x * 3  // Updated multiplier")
                    .context("}"),
                vec![],
            ),
            Element::text(""),
            // From unified diff string
            Element::node::<Text>(
                TextProps {
                    content: "From unified diff string:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Diff>(
                DiffProps::from_unified(
                    "@@ -1,4 +1,5 @@\n \n-use std::io;\n+use std::io::{self, Write};\n+use std::fs;\n \n fn main() {"
                ),
                vec![],
            ),
            Element::text(""),
            // Code change example
            Element::node::<Text>(
                TextProps {
                    content: "Code refactoring:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Diff>(
                DiffProps::new()
                    .header("@@ -88,4 +88,5 @@ impl Config {")
                    .context("    pub fn load() -> Self {")
                    .removed("        let data = fs::read(\"config.json\").unwrap();")
                    .removed("        serde_json::from_slice(&data).unwrap()")
                    .added("        let data = fs::read(\"config.json\")")
                    .added("            .expect(\"Failed to read config\");")
                    .added("        serde_json::from_slice(&data)")
                    .added("            .expect(\"Invalid config format\")")
                    .context("    }"),
                vec![],
            ),
            Element::text(""),
            // Minimal style
            Element::node::<Text>(
                TextProps {
                    content: "Minimal style (no context dimming):".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Diff>(
                DiffProps::new()
                    .removed("old_function()")
                    .added("new_function()")
                    .style(DiffStyle::Minimal)
                    .dim_context(false),
                vec![],
            ),
            Element::text(""),
            // With line numbers
            Element::node::<Text>(
                TextProps {
                    content: "With line numbers:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Diff>(
                DiffProps::new()
                    .line(DiffLine::context("impl Server {").line_nums(10, 10))
                    .line(DiffLine::removed("    port: 8080,").old_num(11))
                    .line(DiffLine::added("    port: 3000,").new_num(11))
                    .line(DiffLine::added("    host: \"localhost\",").new_num(12))
                    .line(DiffLine::context("}").line_nums(12, 13))
                    .style(DiffStyle::LineNumbers),
                vec![],
            ),
            Element::text(""),
            // Custom colors
            Element::node::<Text>(
                TextProps {
                    content: "Custom colors (yellow/magenta):".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Diff>(
                DiffProps::new()
                    .removed("yellow removed")
                    .added("magenta added")
                    .removed_color(Color::Yellow)
                    .added_color(Color::Magenta),
                vec![],
            ),
            Element::text(""),
            // Helper function
            Element::node::<Divider>(
                DividerProps::new()
                    .width(45)
                    .line_style(DividerStyle::Dashed)
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: "diff_lines helper (old vs new):".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Diff>(
                diff_lines(
                    &["version = \"0.1.0\"", "edition = \"2021\""],
                    &["version = \"0.2.0\"", "edition = \"2021\"", "license = \"MIT\""],
                ),
                vec![],
            ),
        ],
    )
}
