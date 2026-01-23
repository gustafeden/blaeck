//! Style primitives for terminal UI rendering.
//!
//! This module provides Color, Modifier, and Style types for styling text
//! in the terminal, following patterns from Ratatui.

use bitflags::bitflags;

/// ANSI Color for terminal rendering.
///
/// Supports the standard 16 ANSI colors, 256-color palette (Indexed),
/// and 24-bit true color (Rgb).
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Color {
    /// Resets the foreground or background color
    #[default]
    Reset,
    /// ANSI Color: Black
    Black,
    /// ANSI Color: Red
    Red,
    /// ANSI Color: Green
    Green,
    /// ANSI Color: Yellow
    Yellow,
    /// ANSI Color: Blue
    Blue,
    /// ANSI Color: Magenta
    Magenta,
    /// ANSI Color: Cyan
    Cyan,
    /// ANSI Color: White (bright white)
    White,
    /// ANSI Color: Gray (dark white)
    Gray,
    /// ANSI Color: Dark Gray (bright black)
    DarkGray,
    /// ANSI Color: Light Red
    LightRed,
    /// ANSI Color: Light Green
    LightGreen,
    /// ANSI Color: Light Yellow
    LightYellow,
    /// ANSI Color: Light Blue
    LightBlue,
    /// ANSI Color: Light Magenta
    LightMagenta,
    /// ANSI Color: Light Cyan
    LightCyan,
    /// An RGB color (24-bit true color)
    Rgb(u8, u8, u8),
    /// An indexed color from the 256-color palette
    Indexed(u8),
}

impl Color {
    /// Converts this color to an ANSI foreground color code.
    /// Returns None for Reset (no change needed).
    pub fn to_ansi_fg(self) -> Option<String> {
        match self {
            Color::Reset => None,
            Color::Black => Some("30".to_string()),
            Color::Red => Some("31".to_string()),
            Color::Green => Some("32".to_string()),
            Color::Yellow => Some("33".to_string()),
            Color::Blue => Some("34".to_string()),
            Color::Magenta => Some("35".to_string()),
            Color::Cyan => Some("36".to_string()),
            Color::White => Some("37".to_string()),
            Color::Gray => Some("37".to_string()), // Same as White for basic ANSI
            Color::DarkGray => Some("90".to_string()),
            Color::LightRed => Some("91".to_string()),
            Color::LightGreen => Some("92".to_string()),
            Color::LightYellow => Some("93".to_string()),
            Color::LightBlue => Some("94".to_string()),
            Color::LightMagenta => Some("95".to_string()),
            Color::LightCyan => Some("96".to_string()),
            Color::Rgb(r, g, b) => Some(format!("38;2;{};{};{}", r, g, b)),
            Color::Indexed(n) => Some(format!("38;5;{}", n)),
        }
    }

    /// Converts this color to an ANSI background color code.
    /// Returns None for Reset (no change needed).
    pub fn to_ansi_bg(self) -> Option<String> {
        match self {
            Color::Reset => None,
            Color::Black => Some("40".to_string()),
            Color::Red => Some("41".to_string()),
            Color::Green => Some("42".to_string()),
            Color::Yellow => Some("43".to_string()),
            Color::Blue => Some("44".to_string()),
            Color::Magenta => Some("45".to_string()),
            Color::Cyan => Some("46".to_string()),
            Color::White => Some("47".to_string()),
            Color::Gray => Some("47".to_string()), // Same as White for basic ANSI
            Color::DarkGray => Some("100".to_string()),
            Color::LightRed => Some("101".to_string()),
            Color::LightGreen => Some("102".to_string()),
            Color::LightYellow => Some("103".to_string()),
            Color::LightBlue => Some("104".to_string()),
            Color::LightMagenta => Some("105".to_string()),
            Color::LightCyan => Some("106".to_string()),
            Color::Rgb(r, g, b) => Some(format!("48;2;{};{};{}", r, g, b)),
            Color::Indexed(n) => Some(format!("48;5;{}", n)),
        }
    }
}

bitflags! {
    /// Modifier changes the way a piece of text is displayed.
    ///
    /// They are bitflags so they can easily be composed.
    #[derive(Default, Clone, Copy, Eq, PartialEq, Hash, Debug)]
    pub struct Modifier: u16 {
        const BOLD              = 0b0000_0000_0001;
        const DIM               = 0b0000_0000_0010;
        const ITALIC            = 0b0000_0000_0100;
        const UNDERLINED        = 0b0000_0000_1000;
        const SLOW_BLINK        = 0b0000_0001_0000;
        const RAPID_BLINK       = 0b0000_0010_0000;
        const REVERSED          = 0b0000_0100_0000;
        const HIDDEN            = 0b0000_1000_0000;
        const CROSSED_OUT       = 0b0001_0000_0000;
    }
}

/// Style lets you control the main characteristics of displayed elements.
///
/// Includes foreground color, background color, and text modifiers.
#[derive(Default, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Style {
    /// The foreground color.
    pub fg: Color,
    /// The background color.
    pub bg: Color,
    /// The text modifiers (bold, italic, etc.)
    pub modifiers: Modifier,
}

impl Style {
    /// Creates a new Style with default values.
    pub const fn new() -> Self {
        Self {
            fg: Color::Reset,
            bg: Color::Reset,
            modifiers: Modifier::empty(),
        }
    }

    /// Sets the foreground color.
    #[must_use]
    pub const fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    /// Sets the background color.
    #[must_use]
    pub const fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    /// Adds the BOLD modifier.
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.modifiers = self.modifiers.union(Modifier::BOLD);
        self
    }

    /// Adds the ITALIC modifier.
    #[must_use]
    pub fn italic(mut self) -> Self {
        self.modifiers = self.modifiers.union(Modifier::ITALIC);
        self
    }

    /// Adds the UNDERLINED modifier.
    #[must_use]
    pub fn underlined(mut self) -> Self {
        self.modifiers = self.modifiers.union(Modifier::UNDERLINED);
        self
    }

    /// Adds the DIM modifier.
    #[must_use]
    pub fn dim(mut self) -> Self {
        self.modifiers = self.modifiers.union(Modifier::DIM);
        self
    }

    /// Adds a modifier.
    #[must_use]
    pub fn add_modifier(mut self, modifier: Modifier) -> Self {
        self.modifiers = self.modifiers.union(modifier);
        self
    }

    /// Removes a modifier.
    #[must_use]
    pub fn remove_modifier(mut self, modifier: Modifier) -> Self {
        self.modifiers = self.modifiers.difference(modifier);
        self
    }

    /// Converts this style to an ANSI escape sequence string.
    ///
    /// Returns an empty string if the style has no changes (all defaults).
    /// The returned string includes the escape sequence prefix but not the reset.
    pub fn to_ansi_string(&self) -> String {
        let mut codes: Vec<String> = Vec::new();

        // Add modifier codes
        if self.modifiers.contains(Modifier::BOLD) {
            codes.push("1".to_string());
        }
        if self.modifiers.contains(Modifier::DIM) {
            codes.push("2".to_string());
        }
        if self.modifiers.contains(Modifier::ITALIC) {
            codes.push("3".to_string());
        }
        if self.modifiers.contains(Modifier::UNDERLINED) {
            codes.push("4".to_string());
        }
        if self.modifiers.contains(Modifier::SLOW_BLINK) {
            codes.push("5".to_string());
        }
        if self.modifiers.contains(Modifier::RAPID_BLINK) {
            codes.push("6".to_string());
        }
        if self.modifiers.contains(Modifier::REVERSED) {
            codes.push("7".to_string());
        }
        if self.modifiers.contains(Modifier::HIDDEN) {
            codes.push("8".to_string());
        }
        if self.modifiers.contains(Modifier::CROSSED_OUT) {
            codes.push("9".to_string());
        }

        // Add foreground color code
        if let Some(fg_code) = self.fg.to_ansi_fg() {
            codes.push(fg_code);
        }

        // Add background color code
        if let Some(bg_code) = self.bg.to_ansi_bg() {
            codes.push(bg_code);
        }

        if codes.is_empty() {
            String::new()
        } else {
            format!("\x1b[{}m", codes.join(";"))
        }
    }

    /// Returns the ANSI reset escape sequence.
    pub fn reset_ansi() -> String {
        "\x1b[0m".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_default_is_reset() {
        assert_eq!(Color::default(), Color::Reset);
    }

    #[test]
    fn test_color_rgb() {
        let c = Color::Rgb(255, 128, 0);
        match c {
            Color::Rgb(r, g, b) => {
                assert_eq!(r, 255);
                assert_eq!(g, 128);
                assert_eq!(b, 0);
            }
            _ => panic!("Expected Rgb variant"),
        }
    }

    #[test]
    fn test_modifier_bold() {
        let m = Modifier::BOLD;
        assert!(m.contains(Modifier::BOLD));
        assert!(!m.contains(Modifier::ITALIC));
    }

    #[test]
    fn test_modifier_combine() {
        let m = Modifier::BOLD | Modifier::ITALIC;
        assert!(m.contains(Modifier::BOLD));
        assert!(m.contains(Modifier::ITALIC));
    }

    #[test]
    fn test_style_default() {
        let s = Style::default();
        assert_eq!(s.fg, Color::Reset);
        assert_eq!(s.bg, Color::Reset);
        assert_eq!(s.modifiers, Modifier::empty());
    }

    #[test]
    fn test_style_builder() {
        let s = Style::default().fg(Color::Red).bg(Color::Blue).bold();
        assert_eq!(s.fg, Color::Red);
        assert_eq!(s.bg, Color::Blue);
        assert!(s.modifiers.contains(Modifier::BOLD));
    }

    #[test]
    fn test_style_to_ansi() {
        let s = Style::new().fg(Color::Red).bold();
        let ansi = s.to_ansi_string();
        assert!(ansi.contains("\x1b[")); // Has escape sequence
    }

    #[test]
    fn test_style_reset_ansi() {
        let reset = Style::reset_ansi();
        assert!(reset.contains("\x1b[0m")); // Contains reset sequence
    }

    #[test]
    fn test_style_to_ansi_fg_colors() {
        // Test basic colors
        assert!(Style::new().fg(Color::Red).to_ansi_string().contains("31"));
        assert!(Style::new()
            .fg(Color::Green)
            .to_ansi_string()
            .contains("32"));
        assert!(Style::new().fg(Color::Blue).to_ansi_string().contains("34"));
    }

    #[test]
    fn test_style_to_ansi_bg_colors() {
        // Test background colors (should be 40-47 range)
        assert!(Style::new().bg(Color::Red).to_ansi_string().contains("41"));
        assert!(Style::new()
            .bg(Color::Green)
            .to_ansi_string()
            .contains("42"));
    }

    #[test]
    fn test_style_to_ansi_modifiers() {
        assert!(Style::new().bold().to_ansi_string().contains("1"));
        assert!(Style::new().italic().to_ansi_string().contains("3"));
        assert!(Style::new().underlined().to_ansi_string().contains("4"));
    }

    #[test]
    fn test_style_to_ansi_rgb() {
        let s = Style::new().fg(Color::Rgb(255, 128, 64));
        let ansi = s.to_ansi_string();
        // RGB escape uses format: \x1b[38;2;R;G;Bm
        assert!(ansi.contains("38;2;255;128;64"));
    }

    #[test]
    fn test_style_to_ansi_indexed() {
        let s = Style::new().fg(Color::Indexed(196));
        let ansi = s.to_ansi_string();
        // Indexed escape uses format: \x1b[38;5;Nm
        assert!(ansi.contains("38;5;196"));
    }

    #[test]
    fn test_style_default_to_ansi_empty() {
        let s = Style::default();
        let ansi = s.to_ansi_string();
        // Default style should produce empty string (no changes needed)
        assert!(ansi.is_empty());
    }
}
