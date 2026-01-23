//! Gradient component - Horizontal color gradients for text.
//!
//! The Gradient component renders text with smooth color transitions.
//! Includes 10 preset gradients (Rainbow, Sunset, Ocean, Fire, etc.)
//! or define custom color stops.
//!
//! ## When to use Gradient
//!
//! - Eye-catching headers or titles
//! - Branding elements
//! - Visual emphasis without plain bold/color
//!
//! ## See also
//!
//! - [`Text`](super::Text) — Plain styled text (single color)

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// A color stop in a gradient.
#[derive(Debug, Clone, Copy)]
pub struct ColorStop {
    /// Position in gradient (0.0 to 1.0).
    pub position: f32,
    /// Color at this position.
    pub color: Color,
}

impl ColorStop {
    /// Create a new color stop.
    pub fn new(position: f32, color: Color) -> Self {
        Self {
            position: position.clamp(0.0, 1.0),
            color,
        }
    }
}

/// Preset gradient themes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GradientPreset {
    /// Rainbow: red -> orange -> yellow -> green -> cyan -> blue -> magenta
    #[default]
    Rainbow,
    /// Sunset: red -> orange -> yellow
    Sunset,
    /// Ocean: cyan -> blue -> dark blue
    Ocean,
    /// Forest: light green -> green -> dark green
    Forest,
    /// Fire: yellow -> orange -> red
    Fire,
    /// Ice: white -> cyan -> blue
    Ice,
    /// Purple haze: magenta -> purple -> blue
    PurpleHaze,
    /// Grayscale: white -> gray -> dark gray
    Grayscale,
    /// Neon: cyan -> magenta
    Neon,
    /// Mint: cyan -> green
    Mint,
}

impl GradientPreset {
    /// Get the color stops for this preset.
    pub fn stops(&self) -> Vec<ColorStop> {
        match self {
            GradientPreset::Rainbow => vec![
                ColorStop::new(0.0, Color::Red),
                ColorStop::new(0.17, Color::Rgb(255, 165, 0)), // Orange
                ColorStop::new(0.33, Color::Yellow),
                ColorStop::new(0.5, Color::Green),
                ColorStop::new(0.67, Color::Cyan),
                ColorStop::new(0.83, Color::Blue),
                ColorStop::new(1.0, Color::Magenta),
            ],
            GradientPreset::Sunset => vec![
                ColorStop::new(0.0, Color::Red),
                ColorStop::new(0.5, Color::Rgb(255, 165, 0)), // Orange
                ColorStop::new(1.0, Color::Yellow),
            ],
            GradientPreset::Ocean => vec![
                ColorStop::new(0.0, Color::Cyan),
                ColorStop::new(0.5, Color::Blue),
                ColorStop::new(1.0, Color::Rgb(0, 0, 139)), // Dark blue
            ],
            GradientPreset::Forest => vec![
                ColorStop::new(0.0, Color::Rgb(144, 238, 144)), // Light green
                ColorStop::new(0.5, Color::Green),
                ColorStop::new(1.0, Color::Rgb(0, 100, 0)), // Dark green
            ],
            GradientPreset::Fire => vec![
                ColorStop::new(0.0, Color::Yellow),
                ColorStop::new(0.5, Color::Rgb(255, 165, 0)), // Orange
                ColorStop::new(1.0, Color::Red),
            ],
            GradientPreset::Ice => vec![
                ColorStop::new(0.0, Color::White),
                ColorStop::new(0.5, Color::Cyan),
                ColorStop::new(1.0, Color::Blue),
            ],
            GradientPreset::PurpleHaze => vec![
                ColorStop::new(0.0, Color::Magenta),
                ColorStop::new(0.5, Color::Rgb(128, 0, 128)), // Purple
                ColorStop::new(1.0, Color::Blue),
            ],
            GradientPreset::Grayscale => vec![
                ColorStop::new(0.0, Color::White),
                ColorStop::new(0.5, Color::Gray),
                ColorStop::new(1.0, Color::DarkGray),
            ],
            GradientPreset::Neon => vec![
                ColorStop::new(0.0, Color::Cyan),
                ColorStop::new(1.0, Color::Magenta),
            ],
            GradientPreset::Mint => vec![
                ColorStop::new(0.0, Color::Cyan),
                ColorStop::new(1.0, Color::Green),
            ],
        }
    }
}

/// Properties for the Gradient component.
#[derive(Debug, Clone)]
pub struct GradientProps {
    /// The text content to render.
    pub content: String,
    /// Preset gradient to use.
    pub preset: Option<GradientPreset>,
    /// Custom color stops (overrides preset).
    pub stops: Vec<ColorStop>,
    /// Whether text should be bold.
    pub bold: bool,
    /// Whether text should be italic.
    pub italic: bool,
    /// Whether text should be underlined.
    pub underline: bool,
}

impl Default for GradientProps {
    fn default() -> Self {
        Self {
            content: String::new(),
            preset: Some(GradientPreset::Rainbow),
            stops: Vec::new(),
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

impl GradientProps {
    /// Create new GradientProps with content.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    /// Set the preset gradient.
    #[must_use]
    pub fn preset(mut self, preset: GradientPreset) -> Self {
        self.preset = Some(preset);
        self.stops.clear();
        self
    }

    /// Add a custom color stop.
    #[must_use]
    pub fn stop(mut self, position: f32, color: Color) -> Self {
        self.preset = None;
        self.stops.push(ColorStop::new(position, color));
        self
    }

    /// Set custom color stops (clears preset).
    #[must_use]
    pub fn stops(mut self, stops: Vec<ColorStop>) -> Self {
        self.preset = None;
        self.stops = stops;
        self
    }

    /// Set two-color gradient (start to end).
    #[must_use]
    pub fn two_colors(mut self, start: Color, end: Color) -> Self {
        self.preset = None;
        self.stops = vec![ColorStop::new(0.0, start), ColorStop::new(1.0, end)];
        self
    }

    /// Set three-color gradient (start, middle, end).
    #[must_use]
    pub fn three_colors(mut self, start: Color, middle: Color, end: Color) -> Self {
        self.preset = None;
        self.stops = vec![
            ColorStop::new(0.0, start),
            ColorStop::new(0.5, middle),
            ColorStop::new(1.0, end),
        ];
        self
    }

    /// Enable bold text.
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Enable italic text.
    #[must_use]
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Enable underlined text.
    #[must_use]
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Get the effective color stops (from preset or custom).
    fn effective_stops(&self) -> Vec<ColorStop> {
        if !self.stops.is_empty() {
            let mut stops = self.stops.clone();
            stops.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
            stops
        } else if let Some(preset) = self.preset {
            preset.stops()
        } else {
            // Default to white if nothing specified
            vec![ColorStop::new(0.0, Color::White)]
        }
    }

    /// Interpolate color at a given position.
    fn interpolate_color(stops: &[ColorStop], position: f32) -> Color {
        if stops.is_empty() {
            return Color::White;
        }
        if stops.len() == 1 {
            return stops[0].color;
        }

        let position = position.clamp(0.0, 1.0);

        // Find the two stops we're between
        let mut lower = &stops[0];
        let mut upper = &stops[stops.len() - 1];

        for i in 0..stops.len() - 1 {
            if position >= stops[i].position && position <= stops[i + 1].position {
                lower = &stops[i];
                upper = &stops[i + 1];
                break;
            }
        }

        // Interpolate between the two colors
        let range = upper.position - lower.position;
        let t = if range > 0.0 {
            (position - lower.position) / range
        } else {
            0.0
        };

        Self::lerp_color(lower.color, upper.color, t)
    }

    /// Linear interpolation between two colors.
    fn lerp_color(a: Color, b: Color, t: f32) -> Color {
        let (r1, g1, b1) = Self::color_to_rgb(a);
        let (r2, g2, b2) = Self::color_to_rgb(b);

        let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8;
        let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8;
        let b_val = (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8;

        Color::Rgb(r, g, b_val)
    }

    /// Convert any Color to RGB values.
    fn color_to_rgb(color: Color) -> (u8, u8, u8) {
        match color {
            Color::Rgb(r, g, b) => (r, g, b),
            Color::Red => (255, 0, 0),
            Color::Green => (0, 255, 0),
            Color::Blue => (0, 0, 255),
            Color::Yellow => (255, 255, 0),
            Color::Cyan => (0, 255, 255),
            Color::Magenta => (255, 0, 255),
            Color::White => (255, 255, 255),
            Color::Black => (0, 0, 0),
            Color::Gray => (128, 128, 128),
            Color::DarkGray => (64, 64, 64),
            Color::LightRed => (255, 128, 128),
            Color::LightGreen => (128, 255, 128),
            Color::LightBlue => (128, 128, 255),
            Color::LightYellow => (255, 255, 128),
            Color::LightCyan => (128, 255, 255),
            Color::LightMagenta => (255, 128, 255),
            Color::Indexed(idx) => {
                // Approximate 256-color palette
                if idx < 16 {
                    // Basic colors
                    match idx {
                        0 => (0, 0, 0),
                        1 => (128, 0, 0),
                        2 => (0, 128, 0),
                        3 => (128, 128, 0),
                        4 => (0, 0, 128),
                        5 => (128, 0, 128),
                        6 => (0, 128, 128),
                        7 => (192, 192, 192),
                        8 => (128, 128, 128),
                        9 => (255, 0, 0),
                        10 => (0, 255, 0),
                        11 => (255, 255, 0),
                        12 => (0, 0, 255),
                        13 => (255, 0, 255),
                        14 => (0, 255, 255),
                        15 => (255, 255, 255),
                        _ => (128, 128, 128),
                    }
                } else if idx < 232 {
                    // 216 color cube
                    let idx = idx - 16;
                    let r = (idx / 36) * 51;
                    let g = ((idx % 36) / 6) * 51;
                    let b = (idx % 6) * 51;
                    (r, g, b)
                } else {
                    // Grayscale
                    let gray = (idx - 232) * 10 + 8;
                    (gray, gray, gray)
                }
            }
            Color::Reset => (255, 255, 255), // Default to white
        }
    }

    /// Build base modifiers from props.
    fn base_modifiers(&self) -> Modifier {
        let mut modifiers = Modifier::empty();
        if self.bold {
            modifiers |= Modifier::BOLD;
        }
        if self.italic {
            modifiers |= Modifier::ITALIC;
        }
        if self.underline {
            modifiers |= Modifier::UNDERLINED;
        }
        modifiers
    }
}

/// A component that renders text with a horizontal color gradient.
///
/// # Examples
///
/// ```ignore
/// // Rainbow gradient
/// Element::node::<Gradient>(
///     GradientProps::new("Hello, World!"),
///     vec![]
/// )
///
/// // Preset gradient
/// Element::node::<Gradient>(
///     GradientProps::new("Sunset text")
///         .preset(GradientPreset::Sunset),
///     vec![]
/// )
///
/// // Custom two-color gradient
/// Element::node::<Gradient>(
///     GradientProps::new("Custom colors")
///         .two_colors(Color::Red, Color::Blue),
///     vec![]
/// )
/// ```
pub struct Gradient;

impl Component for Gradient {
    type Props = GradientProps;

    fn render(props: &Self::Props) -> Element {
        let chars: Vec<char> = props.content.chars().collect();
        let len = chars.len();

        if len == 0 {
            return Element::text("");
        }

        let stops = props.effective_stops();
        let base_modifiers = props.base_modifiers();

        // Create a Fragment with individually styled characters
        let children: Vec<Element> = chars
            .into_iter()
            .enumerate()
            .map(|(i, ch)| {
                let position = if len > 1 {
                    i as f32 / (len - 1) as f32
                } else {
                    0.0
                };
                let color = GradientProps::interpolate_color(&stops, position);
                let style = Style::new().fg(color).add_modifier(base_modifiers);
                Element::styled_text(&ch.to_string(), style)
            })
            .collect();

        Element::Fragment(children)
    }
}

/// Helper function to create gradient text.
pub fn gradient(content: impl Into<String>) -> String {
    // This returns the raw text since we can't return styled elements from a helper
    // For actual gradient rendering, use the component
    content.into()
}

/// Helper function to create gradient text with a preset.
pub fn gradient_preset(content: impl Into<String>, preset: GradientPreset) -> GradientProps {
    GradientProps::new(content).preset(preset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_stop_new() {
        let stop = ColorStop::new(0.5, Color::Red);
        assert_eq!(stop.position, 0.5);
        assert_eq!(stop.color, Color::Red);
    }

    #[test]
    fn test_color_stop_clamp() {
        let stop = ColorStop::new(1.5, Color::Red);
        assert_eq!(stop.position, 1.0);

        let stop = ColorStop::new(-0.5, Color::Blue);
        assert_eq!(stop.position, 0.0);
    }

    #[test]
    fn test_gradient_preset_rainbow() {
        let stops = GradientPreset::Rainbow.stops();
        assert_eq!(stops.len(), 7);
        assert_eq!(stops[0].color, Color::Red);
    }

    #[test]
    fn test_gradient_preset_sunset() {
        let stops = GradientPreset::Sunset.stops();
        assert_eq!(stops.len(), 3);
        assert_eq!(stops[0].color, Color::Red);
    }

    #[test]
    fn test_gradient_props_new() {
        let props = GradientProps::new("test");
        assert_eq!(props.content, "test");
        assert_eq!(props.preset, Some(GradientPreset::Rainbow));
    }

    #[test]
    fn test_gradient_props_preset() {
        let props = GradientProps::new("test").preset(GradientPreset::Ocean);
        assert_eq!(props.preset, Some(GradientPreset::Ocean));
    }

    #[test]
    fn test_gradient_props_two_colors() {
        let props = GradientProps::new("test").two_colors(Color::Red, Color::Blue);
        assert!(props.preset.is_none());
        assert_eq!(props.stops.len(), 2);
    }

    #[test]
    fn test_gradient_props_three_colors() {
        let props = GradientProps::new("test").three_colors(Color::Red, Color::Green, Color::Blue);
        assert!(props.preset.is_none());
        assert_eq!(props.stops.len(), 3);
    }

    #[test]
    fn test_gradient_props_builder() {
        let props = GradientProps::new("test")
            .bold()
            .italic()
            .underline();
        assert!(props.bold);
        assert!(props.italic);
        assert!(props.underline);
    }

    #[test]
    fn test_gradient_interpolate_simple() {
        let stops = vec![
            ColorStop::new(0.0, Color::Rgb(0, 0, 0)),
            ColorStop::new(1.0, Color::Rgb(255, 255, 255)),
        ];

        let mid = GradientProps::interpolate_color(&stops, 0.5);
        if let Color::Rgb(r, g, b) = mid {
            assert!(r > 100 && r < 150); // Should be around 127
            assert!(g > 100 && g < 150);
            assert!(b > 100 && b < 150);
        } else {
            panic!("Expected RGB color");
        }
    }

    #[test]
    fn test_gradient_interpolate_edge() {
        let stops = vec![
            ColorStop::new(0.0, Color::Red),
            ColorStop::new(1.0, Color::Blue),
        ];

        let start = GradientProps::interpolate_color(&stops, 0.0);
        if let Color::Rgb(r, _, _) = start {
            assert_eq!(r, 255);
        }

        let end = GradientProps::interpolate_color(&stops, 1.0);
        if let Color::Rgb(_, _, b) = end {
            assert_eq!(b, 255);
        }
    }

    #[test]
    fn test_gradient_component_render() {
        let props = GradientProps::new("Hi");
        let elem = Gradient::render(&props);
        match elem {
            Element::Fragment(children) => {
                assert_eq!(children.len(), 2);
            }
            _ => panic!("Expected Fragment"),
        }
    }

    #[test]
    fn test_gradient_component_render_empty() {
        let props = GradientProps::new("");
        let elem = Gradient::render(&props);
        assert!(elem.is_text());
    }

    #[test]
    fn test_gradient_helper() {
        let text = gradient("Hello");
        assert_eq!(text, "Hello");
    }

    #[test]
    fn test_gradient_preset_helper() {
        let props = gradient_preset("Test", GradientPreset::Fire);
        assert_eq!(props.content, "Test");
        assert_eq!(props.preset, Some(GradientPreset::Fire));
    }

    #[test]
    fn test_color_to_rgb() {
        assert_eq!(GradientProps::color_to_rgb(Color::Red), (255, 0, 0));
        assert_eq!(GradientProps::color_to_rgb(Color::Green), (0, 255, 0));
        assert_eq!(GradientProps::color_to_rgb(Color::Blue), (0, 0, 255));
        assert_eq!(GradientProps::color_to_rgb(Color::White), (255, 255, 255));
        assert_eq!(GradientProps::color_to_rgb(Color::Black), (0, 0, 0));
    }
}
