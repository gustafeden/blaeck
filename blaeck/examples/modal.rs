//! Modal/Dialog example - Overlay prompts and alerts
//!
//! Run with: cargo run --example modal

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let ui = Element::node::<Box>(
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
                    content: "Modal/Dialog Demo".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            // Info alert
            alert(
                "Information",
                "This is an informational message.\nIt can span multiple lines.",
            ),
            // Confirm dialog
            confirm_modal(
                "Confirm Action",
                "Are you sure you want to proceed?\nThis action cannot be undone.",
            ),
            // Error modal
            error_modal(
                "Error Occurred",
                "Failed to connect to the server.\nPlease check your network.",
            ),
            // Success modal
            success_modal("Success!", "Your changes have been saved."),
            // Custom modal
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
    );

    blaeck.render(ui)?;
    blaeck.unmount()?;

    Ok(())
}
