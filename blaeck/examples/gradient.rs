//! Gradient example - Colorful text gradients
//!
//! Run with: cargo run --example gradient

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let ui = Element::node::<Box>(
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
                    content: "Gradient Text Component".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            // Default rainbow gradient
            Element::node::<Text>(
                TextProps {
                    content: "Default (Rainbow):".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Gradient>(
                GradientProps::new("Hello, colorful world of gradients!"),
                vec![],
            ),
            Element::text(""),
            // Preset: Sunset
            Element::node::<Text>(
                TextProps {
                    content: "Preset - Sunset:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Gradient>(
                GradientProps::new("Warm sunset colors flowing across the text")
                    .preset(GradientPreset::Sunset),
                vec![],
            ),
            Element::text(""),
            // Preset: Ocean
            Element::node::<Text>(
                TextProps {
                    content: "Preset - Ocean:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Gradient>(
                GradientProps::new("Deep blue ocean waves of color")
                    .preset(GradientPreset::Ocean),
                vec![],
            ),
            Element::text(""),
            // Preset: Fire
            Element::node::<Text>(
                TextProps {
                    content: "Preset - Fire:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Gradient>(
                GradientProps::new("Burning flames from yellow to red")
                    .preset(GradientPreset::Fire),
                vec![],
            ),
            Element::text(""),
            // Preset: Forest
            Element::node::<Text>(
                TextProps {
                    content: "Preset - Forest:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Gradient>(
                GradientProps::new("Lush green forest shades")
                    .preset(GradientPreset::Forest),
                vec![],
            ),
            Element::text(""),
            // Preset: Neon
            Element::node::<Text>(
                TextProps {
                    content: "Preset - Neon:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Gradient>(
                GradientProps::new("Cyberpunk neon glow")
                    .preset(GradientPreset::Neon),
                vec![],
            ),
            Element::text(""),
            // Preset: Purple Haze
            Element::node::<Text>(
                TextProps {
                    content: "Preset - Purple Haze:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Gradient>(
                GradientProps::new("Mystical purple haze effect")
                    .preset(GradientPreset::PurpleHaze),
                vec![],
            ),
            Element::text(""),
            // Custom two-color gradient
            Element::node::<Text>(
                TextProps {
                    content: "Custom two-color (Red to Blue):".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Gradient>(
                GradientProps::new("Custom gradient from red to blue")
                    .two_colors(Color::Red, Color::Blue),
                vec![],
            ),
            Element::text(""),
            // Custom three-color gradient
            Element::node::<Text>(
                TextProps {
                    content: "Custom three-color (Yellow, Green, Cyan):".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Gradient>(
                GradientProps::new("Flowing through three colors smoothly")
                    .three_colors(Color::Yellow, Color::Green, Color::Cyan),
                vec![],
            ),
            Element::text(""),
            // With modifiers
            Element::node::<Text>(
                TextProps {
                    content: "With bold:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Gradient>(
                GradientProps::new("BOLD RAINBOW TEXT")
                    .bold(),
                vec![],
            ),
            Element::text(""),
            // All presets demo
            Element::node::<Divider>(
                DividerProps::new()
                    .width(50)
                    .line_style(DividerStyle::Dashed)
                    .color(Color::DarkGray),
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: "All Presets:".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            // Ice
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Ice:       ".into(),
                            dim: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Gradient>(
                        GradientProps::new("Frozen ice crystals")
                            .preset(GradientPreset::Ice),
                        vec![],
                    ),
                ],
            ),
            // Mint
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Mint:      ".into(),
                            dim: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Gradient>(
                        GradientProps::new("Fresh mint leaves")
                            .preset(GradientPreset::Mint),
                        vec![],
                    ),
                ],
            ),
            // Grayscale
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Grayscale: ".into(),
                            dim: true,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Gradient>(
                        GradientProps::new("Monochrome shades")
                            .preset(GradientPreset::Grayscale),
                        vec![],
                    ),
                ],
            ),
        ],
    );

    blaeck.render(ui)?;
    blaeck.unmount()?;

    Ok(())
}
