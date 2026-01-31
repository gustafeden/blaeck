//! Inline rendering with line tracking and erasure.
//!
//! **This is the clever bit that makes Blaeck different from fullscreen TUIs.**
//!
//! `LogUpdate` enables Ink-style inline rendering by:
//! 1. Tracking how many lines were written in the previous render
//! 2. On re-render: move cursor up N lines, erase each line, write new content
//! 3. Remembering the new line count for next time
//!
//! ANSI sequences used:
//! - `ESC[nA` — cursor up n lines
//! - `ESC[2K` — erase entire line
//! - `ESC[0G` — cursor to column 0
//!
//! This creates the illusion of in-place updates without alternate screen mode.
//!
//! Based on Ink's `log-update.ts`. See `refs/ink/src/log-update.ts` for the original.

use std::io::Write;

/// Result of a LogUpdate operation.
pub type Result<T> = std::io::Result<T>;

/// LogUpdate manages inline terminal rendering by tracking line counts
/// and erasing previous output before writing new content.
///
/// This is the core of Ink-style rendering - instead of taking over the whole screen,
/// it renders inline and re-renders in place by moving the cursor up and erasing lines.
pub struct LogUpdate<W: Write> {
    writer: W,
    previous_line_count: usize,
    previous_output: String,
    cursor_visible: bool,
}

impl<W: Write> LogUpdate<W> {
    /// Creates a new LogUpdate instance wrapping the given writer.
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            previous_line_count: 0,
            previous_output: String::new(),
            cursor_visible: true,
        }
    }

    /// Returns the number of lines from the previous render.
    pub fn previous_line_count(&self) -> usize {
        self.previous_line_count
    }

    /// Sets whether the cursor should be visible after each render.
    ///
    /// When set to `false`, the cursor remains hidden after rendering,
    /// which is useful for fullscreen-style apps or animations where
    /// a blinking cursor is distracting. The cursor is always hidden
    /// during rendering regardless of this setting.
    ///
    /// Default is `true` (cursor shown after render).
    pub fn set_cursor_visible(&mut self, visible: bool) {
        self.cursor_visible = visible;
    }

    /// Renders new content, erasing the previous output first.
    ///
    /// If the content is the same as the previous render, this is a no-op.
    /// The content should NOT include a trailing newline - one will be added.
    ///
    /// Uses synchronized output (DEC private mode 2026) to prevent flicker
    /// by buffering all updates until complete.
    pub fn render(&mut self, content: &str) -> Result<()> {
        // Use \r\n to work correctly in raw terminal mode
        let output = format!("{}\r\n", content);

        // Skip if content unchanged
        if output == self.previous_output {
            return Ok(());
        }

        // Buffer everything to minimize flicker
        let mut buffer = String::new();

        // Begin synchronized output (terminal buffers until we end)
        buffer.push_str("\x1b[?2026h");

        // Hide cursor during render
        buffer.push_str("\x1b[?25l");

        // Build erase sequence
        if self.previous_line_count > 0 {
            // Move cursor up
            buffer.push_str(&format!("\x1b[{}A", self.previous_line_count));

            // Clear each line
            for i in 0..self.previous_line_count {
                buffer.push_str("\x1b[2K");
                if i < self.previous_line_count - 1 {
                    buffer.push_str("\x1b[1B");
                }
            }

            // Move back to start
            if self.previous_line_count > 1 {
                buffer.push_str(&format!("\x1b[{}A", self.previous_line_count - 1));
            }
            buffer.push_str("\x1b[0G");
        }

        // Add new content
        buffer.push_str(&output);

        // Restore cursor visibility
        if self.cursor_visible {
            buffer.push_str("\x1b[?25h");
        }

        // End synchronized output (terminal flushes buffer)
        buffer.push_str("\x1b[?2026l");

        // Single write for entire frame
        write!(self.writer, "{}", buffer)?;
        self.writer.flush()?;

        // Update tracking
        self.previous_output = output;
        self.previous_line_count = self.previous_output.matches('\n').count().max(1);

        Ok(())
    }

    /// Clears the current output without rendering new content.
    ///
    /// After calling clear(), the next render() will write from scratch.
    pub fn clear(&mut self) -> Result<()> {
        self.erase_lines(self.previous_line_count)?;
        self.writer.flush()?;
        self.previous_output.clear();
        self.previous_line_count = 0;
        Ok(())
    }

    /// Handle terminal resize by clearing our content area only.
    ///
    /// Moves cursor to start of our content (based on tracked line count),
    /// clears from there to end of screen, preserving scrollback above.
    pub fn handle_resize(&mut self) -> Result<()> {
        if self.previous_line_count > 0 {
            // Move cursor up to start of our content
            write!(self.writer, "\x1b[{}A", self.previous_line_count)?;
            // Move to column 0
            write!(self.writer, "\x1b[0G")?;
            // Clear from cursor to end of screen (preserves everything above)
            write!(self.writer, "\x1b[J")?;
            self.writer.flush()?;
        }
        self.previous_output.clear();
        self.previous_line_count = 0;
        Ok(())
    }

    /// Finalizes the output, leaving it visible.
    ///
    /// After calling done(), subsequent render() calls will write below
    /// the current content instead of replacing it.
    pub fn done(&mut self) -> Result<()> {
        self.previous_output.clear();
        self.previous_line_count = 0;
        Ok(())
    }

    /// Erases the specified number of lines.
    ///
    /// Moves cursor up and clears each line. This implements the core
    /// of Ink's eraseLines functionality:
    /// - For each line: move cursor up, clear line, move to column 0
    ///
    /// ## Why this algorithm?
    ///
    /// The naive approach would be to just move up and clear each line going down.
    /// But terminals require careful cursor management:
    ///
    /// 1. We first jump up to the top of the previous output
    /// 2. Clear each line while moving down (so we erase from top to bottom)
    /// 3. Return to the starting position (top-left of cleared area)
    ///
    /// This ensures the cursor ends up where the next write should begin,
    /// and all previously rendered content is erased before new content appears.
    /// Without this precise positioning, you get visual artifacts and flicker.
    fn erase_lines(&mut self, count: usize) -> Result<()> {
        if count == 0 {
            return Ok(());
        }

        // ANSI escape sequences:
        // ESC[{n}A - Move cursor up n lines
        // ESC[2K   - Clear entire line
        // ESC[0G   - Move cursor to column 0

        // Move cursor up by count lines
        if count > 0 {
            write!(self.writer, "\x1b[{}A", count)?;
        }

        // Clear each line (moving down through them)
        for i in 0..count {
            // Clear the current line
            write!(self.writer, "\x1b[2K")?;
            // If not the last line, move down to clear the next one
            if i < count - 1 {
                write!(self.writer, "\x1b[1B")?; // Move down one line
            }
        }

        // Move back up to the starting position and to column 0
        // (We're currently at the bottom of what we cleared; go back to top)
        if count > 1 {
            write!(self.writer, "\x1b[{}A", count - 1)?;
        }
        write!(self.writer, "\x1b[0G")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_update_new() {
        let buf = Vec::new();
        let lu = LogUpdate::new(buf);
        assert_eq!(lu.previous_line_count(), 0);
    }

    #[test]
    fn test_log_update_render() {
        let mut buf = Vec::new();
        {
            let mut lu = LogUpdate::new(&mut buf);
            lu.render("Hello\nWorld").unwrap();
            assert_eq!(lu.previous_line_count(), 2);
        }

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Hello"));
        assert!(output.contains("World"));
    }

    #[test]
    fn test_log_update_rerender_erases() {
        let mut buf = Vec::new();
        {
            let mut lu = LogUpdate::new(&mut buf);
            lu.render("First").unwrap();
            lu.render("Second").unwrap();
        }

        let output = String::from_utf8(buf).unwrap();
        // Should contain ANSI escape sequences for cursor movement/erase
        assert!(output.contains("\x1b["));
        assert!(output.contains("Second"));
    }

    #[test]
    fn test_log_update_clear() {
        let mut buf = Vec::new();
        {
            let mut lu = LogUpdate::new(&mut buf);
            lu.render("Content").unwrap();
            lu.clear().unwrap();
            assert_eq!(lu.previous_line_count(), 0);
        }
    }

    #[test]
    fn test_log_update_done() {
        let mut buf = Vec::new();
        {
            let mut lu = LogUpdate::new(&mut buf);
            lu.render("Final").unwrap();
            lu.done().unwrap();
            assert_eq!(lu.previous_line_count(), 0);
        }

        // Content should remain visible (no erase after done)
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Final"));
    }

    #[test]
    fn test_log_update_same_content_no_rerender() {
        // Use a wrapper that tracks write counts
        use std::cell::RefCell;
        use std::rc::Rc;

        struct CountingWriter {
            data: Rc<RefCell<Vec<u8>>>,
            write_count: Rc<RefCell<usize>>,
        }

        impl Write for CountingWriter {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                *self.write_count.borrow_mut() += 1;
                self.data.borrow_mut().extend_from_slice(buf);
                Ok(buf.len())
            }

            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        let data = Rc::new(RefCell::new(Vec::new()));
        let write_count = Rc::new(RefCell::new(0usize));

        let writer = CountingWriter {
            data: data.clone(),
            write_count: write_count.clone(),
        };

        let mut lu = LogUpdate::new(writer);
        lu.render("Same").unwrap();
        let count_after_first = *write_count.borrow();

        lu.render("Same").unwrap();
        let count_after_second = *write_count.borrow();

        // Should not write more data if content unchanged
        assert_eq!(
            count_after_first, count_after_second,
            "Should not write more data if content unchanged"
        );
    }

    #[test]
    fn test_log_update_height_decrease() {
        let mut buf = Vec::new();
        {
            let mut lu = LogUpdate::new(&mut buf);
            lu.render("Line1\nLine2\nLine3").unwrap();
            assert_eq!(lu.previous_line_count(), 3);

            lu.render("Line1").unwrap();
            assert_eq!(lu.previous_line_count(), 1);
        }

        // Output should contain erase sequences for the old lines
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("\x1b["));
    }

    #[test]
    fn test_log_update_empty_render() {
        let mut buf = Vec::new();
        {
            let mut lu = LogUpdate::new(&mut buf);
            lu.render("").unwrap();
            // Empty string + newline = 1 line
            assert_eq!(lu.previous_line_count(), 1);
        }
    }

    #[test]
    fn test_log_update_multiple_renders() {
        let mut buf = Vec::new();
        {
            let mut lu = LogUpdate::new(&mut buf);
            lu.render("A").unwrap();
            lu.render("B").unwrap();
            lu.render("C").unwrap();
            assert_eq!(lu.previous_line_count(), 1);
        }

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("C"));
    }

    #[test]
    fn test_log_update_after_done() {
        let mut buf = Vec::new();
        {
            let mut lu = LogUpdate::new(&mut buf);
            lu.render("First").unwrap();
            lu.done().unwrap();
            lu.render("Second").unwrap();
            // After done(), render writes below, not replacing
        }

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("First"));
        assert!(output.contains("Second"));
    }

    #[test]
    fn test_log_update_clear_then_render() {
        let mut buf = Vec::new();
        {
            let mut lu = LogUpdate::new(&mut buf);
            lu.render("First").unwrap();
            lu.clear().unwrap();
            lu.render("After clear").unwrap();
        }

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("After clear"));
    }

    #[test]
    fn test_log_update_multiline_with_styles() {
        let mut buf = Vec::new();
        {
            let mut lu = LogUpdate::new(&mut buf);
            // Simulate styled output with ANSI codes
            lu.render("\x1b[31mRed\x1b[0m\n\x1b[32mGreen\x1b[0m")
                .unwrap();
            assert_eq!(lu.previous_line_count(), 2);
        }

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Red"));
        assert!(output.contains("Green"));
    }
}
