//! Virtual output grid for terminal rendering.
//!
//! This module provides the Output class that manages a virtual 2D grid
//! where text can be written at x,y positions with styles. The grid is then
//! converted to a string with ANSI escape codes for terminal rendering.
//!
//! Based on Ink's output.ts pattern.

use crate::style::Style;
use unicode_width::UnicodeWidthChar;

/// Result of getting the rendered output from the Output grid.
#[derive(Debug, Clone)]
pub struct OutputResult {
    /// The rendered string output with ANSI escape codes.
    pub output: String,
    /// The height (number of lines) in the output.
    pub height: usize,
}

/// A styled character in the output grid.
#[derive(Debug, Clone)]
struct StyledChar {
    /// The character value (may be empty for wide char continuation).
    char: char,
    /// The style applied to this character.
    style: Style,
    /// Whether this is a placeholder for a wide character.
    is_wide_continuation: bool,
}

impl Default for StyledChar {
    fn default() -> Self {
        Self {
            char: ' ',
            style: Style::default(),
            is_wide_continuation: false,
        }
    }
}

/// Virtual output grid for building terminal UI.
///
/// Write text at arbitrary x,y positions with styles, then call `get()` to
/// render the entire grid to a string with ANSI escape codes.
#[derive(Debug)]
pub struct Output {
    /// Width of the output grid in columns.
    pub width: u16,
    /// Height of the output grid in rows.
    pub height: u16,
    /// The 2D grid of styled characters.
    grid: Vec<Vec<StyledChar>>,
}

impl Output {
    /// Creates a new Output grid with the specified dimensions.
    ///
    /// The grid is initialized with space characters and default style.
    pub fn new(width: u16, height: u16) -> Self {
        let grid = (0..height)
            .map(|_| (0..width).map(|_| StyledChar::default()).collect())
            .collect();

        Self {
            width,
            height,
            grid,
        }
    }

    /// Writes text at the specified position with the given style.
    ///
    /// Multi-line text (containing '\n') is split and written line by line.
    /// Text that extends beyond the grid boundaries is clipped.
    /// Wide characters (like CJK) are handled properly.
    /// Embedded ANSI escape codes are stripped (use the style parameter instead).
    pub fn write(&mut self, x: u16, y: u16, text: &str, style: Style) {
        if text.is_empty() {
            return;
        }

        for (line_offset, line) in text.split('\n').enumerate() {
            let current_y = y as usize + line_offset;
            if current_y >= self.height as usize {
                break;
            }

            let mut current_x = x as usize;
            let mut chars = line.chars().peekable();

            while let Some(ch) = chars.next() {
                if current_x >= self.width as usize {
                    break;
                }

                // Skip ANSI escape sequences
                if ch == '\x1b' {
                    match chars.peek() {
                        Some('[') => {
                            // Standard ANSI escape: \x1b[...m
                            chars.next(); // consume '['
                            while let Some(&c) = chars.peek() {
                                chars.next();
                                if c == 'm' {
                                    break;
                                }
                            }
                            continue;
                        }
                        Some(']') => {
                            // OSC escape: \x1b]...\x07 or \x1b]...\x1b\\
                            chars.next(); // consume ']'
                            while let Some(c) = chars.next() {
                                if c == '\x07' {
                                    break;
                                }
                                if c == '\x1b'
                                    && chars.peek() == Some(&'\\') {
                                        chars.next();
                                        break;
                                    }
                            }
                            continue;
                        }
                        _ => {
                            // Unknown escape, skip just the ESC
                            continue;
                        }
                    }
                }

                // Get the display width of the character
                let char_width = ch.width().unwrap_or(1);

                // Write the character
                self.grid[current_y][current_x] = StyledChar {
                    char: ch,
                    style,
                    is_wide_continuation: false,
                };

                current_x += 1;

                // ## Why wide character handling?
                //
                // CJK characters (Chinese, Japanese, Korean) and some emoji are
                // "full-width" — they occupy 2 terminal columns but are 1 char.
                // If we don't account for this, text after wide chars is misaligned.
                //
                // Solution: mark the "extra" column as a continuation cell.
                // When rendering, we skip continuation cells (they're just placeholders).
                // This keeps our grid coordinates aligned with terminal columns.
                for _ in 1..char_width {
                    if current_x >= self.width as usize {
                        break;
                    }
                    self.grid[current_y][current_x] = StyledChar {
                        char: '\0',
                        style,
                        is_wide_continuation: true,
                    };
                    current_x += 1;
                }
            }
        }
    }

    /// Renders the grid to a string with ANSI escape codes.
    ///
    /// Each line has trailing whitespace trimmed (like Ink does).
    /// Returns both the output string and the height.
    ///
    /// ## Why style tracking optimization?
    ///
    /// ANSI escape codes are verbose (~10 bytes each). Naively emitting a style
    /// code for every character would bloat output and slow down rendering.
    ///
    /// Instead, we track `current_style` and only emit codes when the style changes.
    /// For a line like "Hello World" where both words are red, we emit:
    ///   `\x1b[31mHello World\x1b[0m`  (one style code)
    /// Instead of:
    ///   `\x1b[31mH\x1b[31me\x1b[31ml...`  (11 style codes)
    ///
    /// This is a ~10x reduction in escape code overhead for typical UIs.
    pub fn get(&self) -> OutputResult {
        let mut lines: Vec<String> = Vec::with_capacity(self.height as usize);

        for row in &self.grid {
            let mut line = String::new();
            let mut current_style: Option<Style> = None;

            for styled_char in row {
                // Skip wide character continuations (see write() for why these exist)
                if styled_char.is_wide_continuation {
                    continue;
                }

                // Only emit ANSI codes when style changes (optimization)
                if Some(styled_char.style) != current_style {
                    // Reset previous style if any
                    if current_style.is_some() {
                        line.push_str(&Style::reset_ansi());
                    }
                    // Apply new style if it has any attributes
                    let ansi = styled_char.style.to_ansi_string();
                    if !ansi.is_empty() {
                        line.push_str(&ansi);
                    }
                    current_style = Some(styled_char.style);
                }

                line.push(styled_char.char);
            }

            // Reset at end of line if we have an active style
            if let Some(style) = current_style {
                if !style.to_ansi_string().is_empty() {
                    line.push_str(&Style::reset_ansi());
                }
            }

            // Trim trailing whitespace (but preserve ANSI sequences)
            let line = Self::trim_trailing_whitespace(&line);
            lines.push(line);
        }

        // Use \r\n for line endings to work correctly in raw terminal mode
        // In raw mode, \n alone moves down but doesn't reset to column 0
        OutputResult {
            height: self.height as usize,
            output: lines.join("\r\n"),
        }
    }

    /// Trims trailing whitespace from a line while preserving ANSI escape sequences.
    fn trim_trailing_whitespace(line: &str) -> String {
        // Simple approach: trim trailing spaces, ANSI reset sequences will be preserved
        // because they don't end with spaces
        line.trim_end_matches(' ').to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    #[test]
    fn test_output_new() {
        let out = Output::new(80, 24);
        assert_eq!(out.width, 80);
        assert_eq!(out.height, 24);
    }

    #[test]
    fn test_output_write_simple() {
        let mut out = Output::new(80, 5);
        out.write(0, 0, "Hello", Style::default());
        let result = out.get();
        assert!(result.output.contains("Hello"));
        assert_eq!(result.height, 5);
    }

    #[test]
    fn test_output_write_position() {
        let mut out = Output::new(80, 5);
        out.write(10, 2, "Test", Style::default());
        let result = out.get();
        let lines: Vec<&str> = result.output.lines().collect();
        // Line 2 should have "Test" starting at column 10
        assert!(lines.len() > 2);
        // Check that there are spaces before "Test"
        assert!(lines[2].contains("Test"));
        // The line should have at least 10 spaces + "Test" (14 chars)
        // But we trim trailing whitespace, so check that Test appears
        let test_pos = lines[2].find("Test").unwrap();
        assert_eq!(test_pos, 10);
    }

    #[test]
    fn test_output_write_styled() {
        let mut out = Output::new(80, 5);
        let style = Style::new().fg(Color::Red).bold();
        out.write(0, 0, "Red", style);
        let result = out.get();
        // Should contain ANSI escape codes
        assert!(result.output.contains("\x1b["));
    }

    #[test]
    fn test_output_write_multiline() {
        let mut out = Output::new(80, 5);
        out.write(0, 0, "Line1\nLine2\nLine3", Style::default());
        let result = out.get();
        let lines: Vec<&str> = result.output.lines().collect();
        assert!(lines[0].contains("Line1"));
        assert!(lines[1].contains("Line2"));
        assert!(lines[2].contains("Line3"));
    }

    #[test]
    fn test_output_write_overlap() {
        let mut out = Output::new(80, 5);
        out.write(0, 0, "AAAA", Style::default());
        out.write(2, 0, "BB", Style::default()); // Overwrites middle
        let result = out.get();
        let lines: Vec<&str> = result.output.lines().collect();
        // Should be "AABBA" - wait, "AAAA" then "BB" at position 2 -> "AABBA"
        // Actually: positions 0,1,2,3 have "AAAA", then we write "BB" at 2,3 -> "AABB"
        assert!(lines[0].starts_with("AABB"));
    }

    #[test]
    fn test_output_width_unicode() {
        let mut out = Output::new(80, 5);
        out.write(0, 0, "日本語", Style::default()); // Full-width chars
        let result = out.get();
        assert!(result.output.contains("日本語"));
    }

    #[test]
    fn test_output_trims_trailing_whitespace() {
        let mut out = Output::new(80, 3);
        out.write(0, 0, "Hi", Style::default());
        let result = out.get();
        let lines: Vec<&str> = result.output.lines().collect();
        // First line should be "Hi" not "Hi" followed by 78 spaces
        assert_eq!(lines[0], "Hi");
    }

    #[test]
    fn test_output_preserves_height() {
        let mut out = Output::new(80, 10);
        out.write(0, 0, "Top", Style::default());
        out.write(0, 9, "Bottom", Style::default());
        let result = out.get();
        assert_eq!(result.height, 10);
        let lines: Vec<&str> = result.output.lines().collect();
        assert_eq!(lines.len(), 10);
    }

    #[test]
    fn test_output_clips_beyond_width() {
        let mut out = Output::new(5, 1);
        out.write(0, 0, "Hello World", Style::default());
        let result = out.get();
        // Should only contain "Hello" (5 chars)
        assert_eq!(result.output.trim(), "Hello");
    }

    #[test]
    fn test_output_clips_beyond_height() {
        let mut out = Output::new(80, 2);
        out.write(0, 0, "Line1\nLine2\nLine3\nLine4", Style::default());
        let result = out.get();
        let lines: Vec<&str> = result.output.lines().collect();
        // Should only have 2 lines
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("Line1"));
        assert!(lines[1].contains("Line2"));
    }

    #[test]
    fn test_output_different_styles() {
        let mut out = Output::new(80, 1);
        out.write(0, 0, "Red", Style::new().fg(Color::Red));
        out.write(3, 0, "Blue", Style::new().fg(Color::Blue));
        let result = out.get();
        // Should have both colors' escape codes
        assert!(result.output.contains("31")); // Red fg
        assert!(result.output.contains("34")); // Blue fg
    }

    #[test]
    fn test_output_empty_write() {
        let mut out = Output::new(80, 5);
        out.write(0, 0, "", Style::default());
        let result = out.get();
        // Should not panic, output should be empty lines
        assert_eq!(result.height, 5);
    }
}
