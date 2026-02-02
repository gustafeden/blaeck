use blaeck::prelude::*;

pub fn build_ui() -> Element {
    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            gap: 1.0,
            ..Default::default()
        },
        vec![
            Element::node::<Text>(
                TextProps {
                    content: "Modal/Dialog Demo".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            alert(
                "Information",
                "This is an informational message.\nIt can span multiple lines.",
            ),
            confirm_modal(
                "Confirm Action",
                "Are you sure you want to proceed?\nThis action cannot be undone.",
            ),
            error_modal(
                "Error Occurred",
                "Failed to connect to the server.\nPlease check your network.",
            ),
            success_modal("Success!", "Your changes have been saved."),
            Element::node::<Modal>(
                ModalProps::new("Custom Dialog")
                    .body("This is a custom modal with\nmultiple buttons and styling.")
                    .style(ModalStyle::Default)
                    .border_style(BorderStyle::Double)
                    .buttons(vec![
                        ModalButton::new("Help").color(Color::Blue),
                        ModalButton::cancel(),
                        ModalButton::new("Save").color(Color::Green).primary(),
                    ])
                    .min_width(40)
                    .center_title(true),
                vec![],
            ),
        ],
    )
}
