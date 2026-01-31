//! Table component - display data in rows and columns.
//!
//! The Table component renders structured data with headers, rows,
//! customizable column widths, borders, and optional styling features
//! like striped rows and cell alignment.
//!
//! ## When to use Table
//!
//! - Displaying structured/tabular data
//! - Comparing items with multiple attributes
//! - Status dashboards with multiple columns
//!
//! ## See also
//!
//! - [`TreeView`](super::TreeView) — Hierarchical data (files, nested structures)
//! - [`Select`](super::Select) — If you just need a selectable list
//! - [`BarChart`](super::BarChart) — Visual comparison of values

use crate::element::{Component, Element};
use crate::style::{Color, Style};

use super::BorderStyle;

/// Width specification for table columns.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ColumnWidth {
    /// Fixed character width.
    Fixed(u16),
    /// Percentage of total table width (0.0 to 1.0).
    Percent(f32),
    /// Auto-size based on content (uses flex-grow).
    #[default]
    Auto,
}

/// Text alignment within a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CellAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// A single cell in a table.
#[derive(Debug, Clone, Default)]
pub struct TableCell {
    /// Cell content.
    pub content: String,
    /// Text color.
    pub color: Option<Color>,
    /// Background color.
    pub bg_color: Option<Color>,
    /// Bold text.
    pub bold: bool,
    /// Dim text.
    pub dim: bool,
    /// Italic text.
    pub italic: bool,
    /// Cell alignment (overrides column default).
    pub align: Option<CellAlign>,
}

impl TableCell {
    /// Create a new cell with content.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    /// Set text color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set background color.
    #[must_use]
    pub fn bg_color(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Make text bold.
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Make text dim.
    #[must_use]
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Make text italic.
    #[must_use]
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Set cell alignment.
    #[must_use]
    pub fn align(mut self, align: CellAlign) -> Self {
        self.align = Some(align);
        self
    }

    /// Check if cell is empty.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

impl<S: Into<String>> From<S> for TableCell {
    fn from(s: S) -> Self {
        TableCell::new(s)
    }
}

/// A row in a table.
#[derive(Debug, Clone, Default)]
pub struct Row {
    /// Cells in this row.
    pub cells: Vec<TableCell>,
    /// Row background color.
    pub bg_color: Option<Color>,
    /// Row text style (applied to all cells unless overridden).
    pub style: Option<Style>,
}

impl Row {
    /// Create a new row with cells.
    pub fn new<I, C>(cells: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<TableCell>,
    {
        Self {
            cells: cells.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    /// Set background color for the row.
    #[must_use]
    pub fn bg_color(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set style for all cells in row.
    #[must_use]
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Get number of cells.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Check if row is empty.
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }
}

impl<I, C> From<I> for Row
where
    I: IntoIterator<Item = C>,
    C: Into<TableCell>,
{
    fn from(cells: I) -> Self {
        Row::new(cells)
    }
}

/// Styling options for table rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RowStyle {
    /// No special row styling.
    #[default]
    None,
    /// Alternate row colors (striped).
    Striped,
}

/// Properties for the Table component.
#[derive(Debug, Clone)]
pub struct TableProps {
    /// Optional header row.
    pub header: Option<Row>,
    /// Data rows.
    pub rows: Vec<Row>,
    /// Column widths.
    pub widths: Vec<ColumnWidth>,
    /// Default column alignment.
    pub column_aligns: Vec<CellAlign>,
    /// Border style around the table.
    pub border_style: BorderStyle,
    /// Border color.
    pub border_color: Option<Color>,
    /// Spacing between columns.
    pub column_spacing: u16,
    /// Header text color.
    pub header_color: Option<Color>,
    /// Header background color.
    pub header_bg_color: Option<Color>,
    /// Whether header is bold.
    pub header_bold: bool,
    /// Row styling (e.g., striped).
    pub row_style: RowStyle,
    /// Color for odd rows (when striped).
    pub stripe_color: Option<Color>,
    /// Currently selected row index (for highlighting).
    pub selected: Option<usize>,
    /// Selected row highlight color.
    pub selected_color: Option<Color>,
    /// Selected row background color.
    pub selected_bg_color: Option<Color>,
    /// Show row dividers.
    pub row_dividers: bool,
    /// Total table width (optional, for percentage calculations).
    pub width: Option<u16>,
    /// Background color for all cells (lowest priority).
    pub bg_color: Option<Color>,
}

impl Default for TableProps {
    fn default() -> Self {
        Self {
            header: None,
            rows: Vec::new(),
            widths: Vec::new(),
            column_aligns: Vec::new(),
            border_style: BorderStyle::None,
            border_color: None,
            column_spacing: 2,
            header_color: None,
            header_bg_color: None,
            header_bold: true,
            row_style: RowStyle::None,
            stripe_color: Some(Color::DarkGray),
            selected: None,
            selected_color: None,
            selected_bg_color: None,
            row_dividers: false,
            width: None,
            bg_color: None,
        }
    }
}

impl TableProps {
    /// Create a new table with rows.
    pub fn new<I, R>(rows: I) -> Self
    where
        I: IntoIterator<Item = R>,
        R: Into<Row>,
    {
        Self {
            rows: rows.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    /// Set the header row.
    #[must_use]
    pub fn header<R: Into<Row>>(mut self, header: R) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Set column widths.
    #[must_use]
    pub fn widths<I: IntoIterator<Item = ColumnWidth>>(mut self, widths: I) -> Self {
        self.widths = widths.into_iter().collect();
        self
    }

    /// Set all columns to fixed width.
    #[must_use]
    pub fn fixed_widths<I: IntoIterator<Item = u16>>(mut self, widths: I) -> Self {
        self.widths = widths.into_iter().map(ColumnWidth::Fixed).collect();
        self
    }

    /// Set column alignments.
    #[must_use]
    pub fn column_aligns<I: IntoIterator<Item = CellAlign>>(mut self, aligns: I) -> Self {
        self.column_aligns = aligns.into_iter().collect();
        self
    }

    /// Set border style.
    #[must_use]
    pub fn border(mut self, style: BorderStyle) -> Self {
        self.border_style = style;
        self
    }

    /// Set border color.
    #[must_use]
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Set column spacing.
    #[must_use]
    pub fn column_spacing(mut self, spacing: u16) -> Self {
        self.column_spacing = spacing;
        self
    }

    /// Set header color.
    #[must_use]
    pub fn header_color(mut self, color: Color) -> Self {
        self.header_color = Some(color);
        self
    }

    /// Set header background color.
    #[must_use]
    pub fn header_bg_color(mut self, color: Color) -> Self {
        self.header_bg_color = Some(color);
        self
    }

    /// Set whether header is bold.
    #[must_use]
    pub fn header_bold(mut self, bold: bool) -> Self {
        self.header_bold = bold;
        self
    }

    /// Enable striped rows.
    #[must_use]
    pub fn striped(mut self) -> Self {
        self.row_style = RowStyle::Striped;
        self
    }

    /// Set stripe color.
    #[must_use]
    pub fn stripe_color(mut self, color: Color) -> Self {
        self.stripe_color = Some(color);
        self
    }

    /// Set selected row index.
    #[must_use]
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }

    /// Set selected row colors.
    #[must_use]
    pub fn selected_style(mut self, color: Option<Color>, bg_color: Option<Color>) -> Self {
        self.selected_color = color;
        self.selected_bg_color = bg_color;
        self
    }

    /// Enable row dividers.
    #[must_use]
    pub fn row_dividers(mut self) -> Self {
        self.row_dividers = true;
        self
    }

    /// Set table width.
    #[must_use]
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Set background color for all cells.
    #[must_use]
    pub fn bg_color(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Get the number of columns (from widths, header, or first row).
    fn num_columns(&self) -> usize {
        if !self.widths.is_empty() {
            self.widths.len()
        } else if let Some(ref header) = self.header {
            header.len()
        } else if let Some(first_row) = self.rows.first() {
            first_row.len()
        } else {
            0
        }
    }

    /// Get alignment for a column.
    fn get_align(&self, col: usize) -> CellAlign {
        self.column_aligns.get(col).copied().unwrap_or_default()
    }

    /// Get width for a column.
    fn get_width(&self, col: usize) -> ColumnWidth {
        self.widths.get(col).copied().unwrap_or_default()
    }
}

/// A component that displays data in a table format.
///
/// # Examples
///
/// ```ignore
/// // Simple table with strings
/// Element::node::<Table>(
///     TableProps::new(vec![
///         vec!["Alice", "alice@example.com"],
///         vec!["Bob", "bob@example.com"],
///     ])
///     .header(vec!["Name", "Email"])
///     .border(BorderStyle::Single),
///     vec![]
/// )
///
/// // Table with styled cells
/// Element::node::<Table>(
///     TableProps::new(vec![
///         Row::new(vec![
///             TableCell::new("Active").color(Color::Green),
///             TableCell::new("Server 1"),
///         ]),
///         Row::new(vec![
///             TableCell::new("Down").color(Color::Red),
///             TableCell::new("Server 2"),
///         ]),
///     ])
///     .header(vec!["Status", "Server"])
///     .striped(),
///     vec![]
/// )
/// ```
pub struct Table;

impl Component for Table {
    type Props = TableProps;

    fn render(props: &Self::Props) -> Element {
        let num_cols = props.num_columns();
        if num_cols == 0 {
            return Element::text("");
        }

        let mut lines: Vec<String> = Vec::new();

        // Render header
        if let Some(ref header) = props.header {
            lines.push(render_row_string(header, props));

            // Add divider after header
            if props.row_dividers || props.border_style != BorderStyle::None {
                lines.push(render_divider_string(props));
            }
        }

        // Render data rows
        for (i, row) in props.rows.iter().enumerate() {
            lines.push(render_row_string(row, props));

            // Add row divider (except after last row)
            if props.row_dividers && i < props.rows.len() - 1 {
                lines.push(render_divider_string(props));
            }
        }

        let content = lines.join("\n");

        // Build style with bg_color
        let mut style = Style::new();
        if let Some(bg) = props.bg_color {
            style = style.bg(bg);
        }

        Element::styled_text(&content, style)
    }
}

/// Render a single row as a string.
fn render_row_string(row: &Row, props: &TableProps) -> String {
    let num_cols = props.num_columns();
    let spacing = " ".repeat(props.column_spacing as usize);
    let mut parts: Vec<String> = Vec::new();

    for col in 0..num_cols {
        let cell = row.cells.get(col);
        let cell_text = render_cell_content(cell, col, props);
        parts.push(cell_text);
    }

    parts.join(&spacing)
}

/// Render cell content (just the padded text, no styling).
fn render_cell_content(
    cell: Option<&TableCell>,
    col: usize,
    props: &TableProps,
) -> String {
    let content = cell.map(|c| c.content.as_str()).unwrap_or("");
    let align = cell
        .and_then(|c| c.align)
        .unwrap_or_else(|| props.get_align(col));

    // Get column width for padding
    let width = match props.get_width(col) {
        ColumnWidth::Fixed(w) => w as usize,
        ColumnWidth::Percent(p) => {
            if let Some(table_width) = props.width {
                (table_width as f32 * p).floor() as usize
            } else {
                content.chars().count()
            }
        }
        ColumnWidth::Auto => content.chars().count(),
    };

    // Pad content to width
    let content_len = content.chars().count();
    if content_len >= width {
        // Truncate if too long
        content.chars().take(width).collect::<String>()
    } else {
        let padding = width - content_len;
        match align {
            CellAlign::Left => format!("{}{}", content, " ".repeat(padding)),
            CellAlign::Right => format!("{}{}", " ".repeat(padding), content),
            CellAlign::Center => {
                let left_pad = padding / 2;
                let right_pad = padding - left_pad;
                format!(
                    "{}{}{}",
                    " ".repeat(left_pad),
                    content,
                    " ".repeat(right_pad)
                )
            }
        }
    }
}

/// Render a divider line as a string.
fn render_divider_string(props: &TableProps) -> String {
    // Calculate total width from column widths and spacing
    let num_cols = props.num_columns();
    let total_width: usize = (0..num_cols)
        .map(|col| match props.get_width(col) {
            ColumnWidth::Fixed(w) => w as usize,
            ColumnWidth::Percent(p) => {
                if let Some(table_width) = props.width {
                    (table_width as f32 * p).floor() as usize
                } else {
                    10 // Default width
                }
            }
            ColumnWidth::Auto => 10, // Default width
        })
        .sum::<usize>()
        + (num_cols.saturating_sub(1)) * props.column_spacing as usize;

    "─".repeat(total_width)
}

/// State for table selection.
#[derive(Debug, Clone, Default)]
pub struct TableState {
    /// Currently selected row index.
    pub selected: usize,
    /// Total number of rows.
    pub row_count: usize,
}

impl TableState {
    /// Create new state for a table with the given row count.
    pub fn new(row_count: usize) -> Self {
        Self {
            selected: 0,
            row_count,
        }
    }

    /// Move selection up.
    pub fn up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Move selection down.
    pub fn down(&mut self) {
        if self.selected < self.row_count.saturating_sub(1) {
            self.selected += 1;
        }
    }

    /// Move to first row.
    pub fn first(&mut self) {
        self.selected = 0;
    }

    /// Move to last row.
    pub fn last(&mut self) {
        self.selected = self.row_count.saturating_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_new() {
        let cell = TableCell::new("Hello");
        assert_eq!(cell.content, "Hello");
        assert!(cell.color.is_none());
        assert!(!cell.bold);
    }

    #[test]
    fn test_cell_builder() {
        let cell = TableCell::new("Test")
            .color(Color::Red)
            .bold()
            .italic()
            .align(CellAlign::Right);
        assert_eq!(cell.content, "Test");
        assert_eq!(cell.color, Some(Color::Red));
        assert!(cell.bold);
        assert!(cell.italic);
        assert_eq!(cell.align, Some(CellAlign::Right));
    }

    #[test]
    fn test_cell_from_string() {
        let cell: TableCell = "Hello".into();
        assert_eq!(cell.content, "Hello");
    }

    #[test]
    fn test_row_new() {
        let row = Row::new(vec!["A", "B", "C"]);
        assert_eq!(row.len(), 3);
        assert_eq!(row.cells[0].content, "A");
    }

    #[test]
    fn test_row_from_iter() {
        let row: Row = vec!["X", "Y"].into();
        assert_eq!(row.len(), 2);
    }

    #[test]
    fn test_row_bg_color() {
        let row = Row::new(vec!["A"]).bg_color(Color::Blue);
        assert_eq!(row.bg_color, Some(Color::Blue));
    }

    #[test]
    fn test_table_props_new() {
        let props = TableProps::new(vec![vec!["A", "B"], vec!["C", "D"]]);
        assert_eq!(props.rows.len(), 2);
        assert!(props.header.is_none());
    }

    #[test]
    fn test_table_props_header() {
        let props = TableProps::new(vec![vec!["A"]]).header(vec!["Header"]);
        assert!(props.header.is_some());
        assert_eq!(props.header.unwrap().cells[0].content, "Header");
    }

    #[test]
    fn test_table_props_widths() {
        let props = TableProps::new(vec![vec!["A", "B"]])
            .widths([ColumnWidth::Fixed(10), ColumnWidth::Percent(0.5)]);
        assert_eq!(props.widths.len(), 2);
        assert_eq!(props.widths[0], ColumnWidth::Fixed(10));
    }

    #[test]
    fn test_table_props_fixed_widths() {
        let props = TableProps::new(vec![vec!["A", "B"]]).fixed_widths([10, 20]);
        assert_eq!(props.widths[0], ColumnWidth::Fixed(10));
        assert_eq!(props.widths[1], ColumnWidth::Fixed(20));
    }

    #[test]
    fn test_table_props_striped() {
        let props = TableProps::new(vec![vec!["A"]]).striped();
        assert_eq!(props.row_style, RowStyle::Striped);
    }

    #[test]
    fn test_table_props_border() {
        let props = TableProps::new(vec![vec!["A"]])
            .border(BorderStyle::Round)
            .border_color(Color::Cyan);
        assert_eq!(props.border_style, BorderStyle::Round);
        assert_eq!(props.border_color, Some(Color::Cyan));
    }

    #[test]
    fn test_table_props_selected() {
        let props = TableProps::new(vec![vec!["A"], vec!["B"]])
            .selected(1)
            .selected_style(Some(Color::Yellow), Some(Color::Blue));
        assert_eq!(props.selected, Some(1));
        assert_eq!(props.selected_color, Some(Color::Yellow));
        assert_eq!(props.selected_bg_color, Some(Color::Blue));
    }

    #[test]
    fn test_table_state_navigation() {
        let mut state = TableState::new(5);
        assert_eq!(state.selected, 0);

        state.down();
        assert_eq!(state.selected, 1);

        state.down();
        state.down();
        assert_eq!(state.selected, 3);

        state.up();
        assert_eq!(state.selected, 2);

        state.last();
        assert_eq!(state.selected, 4);

        state.first();
        assert_eq!(state.selected, 0);

        // Test bounds
        state.up();
        assert_eq!(state.selected, 0); // Should not go negative
    }

    #[test]
    fn test_table_num_columns() {
        // From widths
        let props = TableProps::new(vec![vec!["A", "B"]]).widths([
            ColumnWidth::Fixed(10),
            ColumnWidth::Fixed(10),
            ColumnWidth::Fixed(10),
        ]);
        assert_eq!(props.num_columns(), 3);

        // From header
        let props = TableProps::new(Vec::<Vec<&str>>::new()).header(vec!["A", "B"]);
        assert_eq!(props.num_columns(), 2);

        // From first row
        let props = TableProps::new(vec![vec!["A", "B", "C", "D"]]);
        assert_eq!(props.num_columns(), 4);
    }

    #[test]
    fn test_table_render_empty() {
        let props = TableProps::new(Vec::<Vec<&str>>::new());
        let elem = Table::render(&props);
        assert!(elem.is_text());
    }

    #[test]
    fn test_table_render_basic() {
        let props = TableProps::new(vec![vec!["A", "B"]]).header(vec!["Col1", "Col2"]);
        let elem = Table::render(&props);
        // Table now renders to text with multiple lines
        assert!(elem.is_text());
    }

    #[test]
    fn test_column_width_default() {
        let width = ColumnWidth::default();
        assert_eq!(width, ColumnWidth::Auto);
    }

    #[test]
    fn test_cell_align_default() {
        let align = CellAlign::default();
        assert_eq!(align, CellAlign::Left);
    }
}
