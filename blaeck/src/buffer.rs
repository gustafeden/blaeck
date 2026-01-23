//! Buffer module for terminal rendering.
//!
//! Provides Cell and Buffer types for building a grid of styled characters
//! that can be diffed for efficient terminal updates.

use crate::style::{Color, Modifier, Style};

/// A single cell in the terminal buffer.
///
/// Each cell contains a symbol (grapheme), foreground color, background color,
/// and text modifiers.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Cell {
    /// The string/symbol displayed in this cell
    pub symbol: String,
    /// Foreground color
    pub fg: Color,
    /// Background color
    pub bg: Color,
    /// Text modifiers (bold, italic, etc.)
    pub modifiers: Modifier,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            symbol: " ".to_string(),
            fg: Color::Reset,
            bg: Color::Reset,
            modifiers: Modifier::empty(),
        }
    }
}

impl Cell {
    /// Creates a new Cell with the given symbol.
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            ..Default::default()
        }
    }

    /// Sets the foreground color.
    #[must_use]
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    /// Sets the background color.
    #[must_use]
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    /// Sets the style (fg, bg, modifiers).
    pub fn set_style(&mut self, style: Style) {
        self.fg = style.fg;
        self.bg = style.bg;
        self.modifiers = style.modifiers;
    }

    /// Sets the symbol.
    pub fn set_symbol(&mut self, symbol: &str) {
        self.symbol = symbol.to_string();
    }

    /// Resets the cell to default state.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// A buffer representing a grid of cells.
///
/// The buffer stores cells in row-major order and provides methods
/// for reading, writing, and diffing cells.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Buffer {
    cells: Vec<Cell>,
    width: u16,
    height: u16,
}

impl Buffer {
    /// Creates a new buffer filled with default (empty) cells.
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            cells: vec![Cell::default(); size],
            width,
            height,
        }
    }

    /// Returns the width of the buffer.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Returns the height of the buffer.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Returns the index in the cells vec for the given (x, y) coordinates.
    fn index_of(&self, x: u16, y: u16) -> usize {
        (y as usize) * (self.width as usize) + (x as usize)
    }

    /// Gets a reference to the cell at (x, y).
    ///
    /// # Panics
    /// Panics if coordinates are out of bounds.
    pub fn get(&self, x: u16, y: u16) -> &Cell {
        let idx = self.index_of(x, y);
        &self.cells[idx]
    }

    /// Gets a mutable reference to the cell at (x, y).
    ///
    /// # Panics
    /// Panics if coordinates are out of bounds.
    pub fn get_mut(&mut self, x: u16, y: u16) -> &mut Cell {
        let idx = self.index_of(x, y);
        &mut self.cells[idx]
    }

    /// Sets the cell at (x, y).
    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        let idx = self.index_of(x, y);
        self.cells[idx] = cell;
    }

    /// Writes a string starting at (x, y) with the given style.
    ///
    /// Each character in the string occupies one cell.
    /// Writing stops at the edge of the buffer.
    pub fn set_string(&mut self, x: u16, y: u16, text: &str, style: Style) {
        let mut current_x = x;
        for ch in text.chars() {
            if current_x >= self.width {
                break;
            }
            let cell = self.get_mut(current_x, y);
            cell.set_symbol(&ch.to_string());
            cell.set_style(style);
            current_x += 1;
        }
    }

    /// Computes the difference between two buffers.
    ///
    /// Returns a list of (x, y, Cell) tuples for cells that differ.
    pub fn diff(old: &Buffer, new: &Buffer) -> Vec<(u16, u16, Cell)> {
        let mut changes = Vec::new();

        // Buffers must have same dimensions for meaningful diff
        assert_eq!(old.width, new.width);
        assert_eq!(old.height, new.height);

        for y in 0..new.height {
            for x in 0..new.width {
                let old_cell = old.get(x, y);
                let new_cell = new.get(x, y);
                if old_cell != new_cell {
                    changes.push((x, y, new_cell.clone()));
                }
            }
        }

        changes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_default() {
        let cell = Cell::default();
        assert_eq!(cell.symbol, " ");
        assert_eq!(cell.fg, Color::Reset);
        assert_eq!(cell.bg, Color::Reset);
    }

    #[test]
    fn test_cell_new() {
        let cell = Cell::new("X");
        assert_eq!(cell.symbol, "X");
    }

    #[test]
    fn test_cell_styled() {
        let cell = Cell::new("A").fg(Color::Red).bg(Color::Blue);
        assert_eq!(cell.symbol, "A");
        assert_eq!(cell.fg, Color::Red);
        assert_eq!(cell.bg, Color::Blue);
    }

    #[test]
    fn test_buffer_new() {
        let buf = Buffer::new(80, 24);
        assert_eq!(buf.width(), 80);
        assert_eq!(buf.height(), 24);
    }

    #[test]
    fn test_buffer_filled_with_default() {
        let buf = Buffer::new(10, 5);
        for y in 0..5 {
            for x in 0..10 {
                assert_eq!(buf.get(x, y).symbol, " ");
            }
        }
    }

    #[test]
    fn test_buffer_set_get() {
        let mut buf = Buffer::new(10, 5);
        buf.set(3, 2, Cell::new("X"));
        assert_eq!(buf.get(3, 2).symbol, "X");
        assert_eq!(buf.get(0, 0).symbol, " "); // unchanged
    }

    #[test]
    fn test_buffer_set_string() {
        let mut buf = Buffer::new(20, 5);
        buf.set_string(2, 1, "Hello", Style::default());
        assert_eq!(buf.get(2, 1).symbol, "H");
        assert_eq!(buf.get(3, 1).symbol, "e");
        assert_eq!(buf.get(4, 1).symbol, "l");
        assert_eq!(buf.get(5, 1).symbol, "l");
        assert_eq!(buf.get(6, 1).symbol, "o");
    }

    #[test]
    fn test_buffer_diff_empty() {
        let a = Buffer::new(10, 5);
        let b = Buffer::new(10, 5);
        let diff = Buffer::diff(&a, &b);
        assert!(diff.is_empty());
    }

    #[test]
    fn test_buffer_diff_single_change() {
        let old = Buffer::new(10, 5);
        let mut new = Buffer::new(10, 5);
        new.set(3, 2, Cell::new("X"));
        let diff = Buffer::diff(&old, &new);
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].0, 3); // x
        assert_eq!(diff[0].1, 2); // y
        assert_eq!(diff[0].2.symbol, "X");
    }

    #[test]
    fn test_buffer_diff_multiple_changes() {
        let old = Buffer::new(10, 5);
        let mut new = Buffer::new(10, 5);
        new.set(0, 0, Cell::new("A"));
        new.set(5, 3, Cell::new("B"));
        new.set(9, 4, Cell::new("C"));
        let diff = Buffer::diff(&old, &new);
        assert_eq!(diff.len(), 3);
    }
}
