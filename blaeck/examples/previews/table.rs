use blaeck::prelude::*;

pub fn build_ui() -> Element {
    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            ..Default::default()
        },
        vec![
            Element::node::<Text>(
                TextProps {
                    content: "Table Component Demo".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            Element::node::<Divider>(
                DividerProps::new()
                    .label("Basic Table")
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::text(""),
            Element::node::<Table>(
                TableProps::new(vec![
                    vec!["Alice", "alice@example.com", "Admin"],
                    vec!["Bob", "bob@example.com", "User"],
                    vec!["Charlie", "charlie@example.com", "User"],
                ])
                .header(vec!["Name", "Email", "Role"])
                .fixed_widths([12, 25, 10])
                .header_color(Color::Cyan)
                .border(BorderStyle::Round)
                .border_color(Color::DarkGray),
                vec![],
            ),
            Element::text(""),
            Element::node::<Divider>(
                DividerProps::new()
                    .label("Striped Rows")
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::text(""),
            Element::node::<Table>(
                TableProps::new(vec![
                    vec!["1", "Rust", "Systems"],
                    vec!["2", "Python", "Scripting"],
                    vec!["3", "JavaScript", "Web"],
                    vec!["4", "Go", "Backend"],
                    vec!["5", "C++", "Performance"],
                ])
                .header(vec!["#", "Language", "Domain"])
                .fixed_widths([5, 15, 15])
                .striped()
                .header_color(Color::Yellow)
                .header_bold(true),
                vec![],
            ),
            Element::text(""),
            Element::node::<Divider>(
                DividerProps::new()
                    .label("Styled Cells")
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::text(""),
            Element::node::<Table>(
                TableProps::new(vec![
                    Row::new(vec![
                        TableCell::new("Active").color(Color::Green).bold(),
                        TableCell::new("web-server-01"),
                        TableCell::new("100%"),
                    ]),
                    Row::new(vec![
                        TableCell::new("Warning").color(Color::Yellow).bold(),
                        TableCell::new("db-primary"),
                        TableCell::new("85%"),
                    ]),
                    Row::new(vec![
                        TableCell::new("Down").color(Color::Red).bold(),
                        TableCell::new("cache-node-03"),
                        TableCell::new("0%"),
                    ]),
                    Row::new(vec![
                        TableCell::new("Active").color(Color::Green).bold(),
                        TableCell::new("api-gateway"),
                        TableCell::new("72%"),
                    ]),
                ])
                .header(vec!["Status", "Server", "CPU"])
                .fixed_widths([10, 20, 8])
                .header_color(Color::Magenta)
                .border(BorderStyle::Single),
                vec![],
            ),
            Element::text(""),
            Element::node::<Divider>(
                DividerProps::new()
                    .label("Minimal Style")
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::text(""),
            Element::node::<Table>(
                TableProps::new(vec![
                    vec!["GET", "/api/users", "200"],
                    vec!["POST", "/api/login", "201"],
                    vec!["DELETE", "/api/users/5", "404"],
                ])
                .header(vec!["Method", "Endpoint", "Status"])
                .fixed_widths([8, 20, 8])
                .header_color(Color::White)
                .column_spacing(3),
                vec![],
            ),
        ],
    )
}
