# Taffy 0.9 Migration Guide

This guide covers the changes when upgrading Blaeck's layout system to expose full Taffy 0.9 features.

## Breaking Changes

### 1. `LayoutStyle` has new required fields

If you construct `LayoutStyle` directly (not via `BoxProps::to_layout_style()`), you now **must** use `..Default::default()`:

```rust
// Before: worked with fewer fields
let style = LayoutStyle {
    width: Some(100.0),
    flex_grow: 1.0,
    // ... other fields
};

// After: use Default for new fields
let style = LayoutStyle {
    width: Some(100.0),
    flex_grow: 1.0,
    ..Default::default()  // Required!
};
```

### 2. `Display` is now `taffy::Display`

`Display` is re-exported from Taffy, not a custom enum. The variants are the same but it's a different type:

```rust
use blaeck::layout::Display;

// Variants available:
Display::Flex    // default
Display::Grid    // CSS Grid
Display::Block   // Block layout
Display::None    // Hidden from layout
```

## New Features

### Flexbox Enhancements

#### `FlexWrap` - Wrapping behavior

```rust
use blaeck::layout::FlexWrap;

let style = LayoutStyle {
    flex_wrap: FlexWrap::Wrap,        // Items wrap to next line
    // flex_wrap: FlexWrap::NoWrap,   // Single line (default)
    // flex_wrap: FlexWrap::WrapReverse, // Wrap in reverse
    ..Default::default()
};
```

#### `flex_basis` - Initial size before grow/shrink

```rust
let style = LayoutStyle {
    flex_basis: Some(200.0),  // Start at 200px before flex
    flex_grow: 1.0,
    ..Default::default()
};
```

### Positioning

#### `Position` - Relative vs Absolute

```rust
use blaeck::layout::Position;

// Absolute positioning (removed from flow)
let overlay = LayoutStyle {
    position: Position::Absolute,
    inset_top: Some(10.0),
    inset_left: Some(10.0),
    width: Some(200.0),
    height: Some(100.0),
    ..Default::default()
};

// Relative is default (normal flow with offset)
let nudged = LayoutStyle {
    position: Position::Relative,
    inset_top: Some(5.0),  // Offset 5 units down
    ..Default::default()
};
```

### CSS Grid Layout

#### Grid Container

```rust
use blaeck::layout::{Display, TrackSize, GridAutoFlow};

let grid_container = LayoutStyle {
    display: Display::Grid,

    // Define columns: 100px, 1fr, 2fr
    grid_template_columns: vec![
        TrackSize::Fixed(100.0),
        TrackSize::Flex(1.0),
        TrackSize::Flex(2.0),
    ],

    // Define rows: auto-sized
    grid_template_rows: vec![
        TrackSize::Auto,
        TrackSize::Auto,
    ],

    // Auto-placement direction
    grid_auto_flow: GridAutoFlow::Row,  // or Column, RowDense, ColumnDense

    // Size of implicitly created tracks
    grid_auto_rows: vec![TrackSize::Auto],

    gap: 10.0,  // Gap between cells
    ..Default::default()
};
```

#### Grid Item Placement

```rust
use blaeck::layout::GridPlacement;

let grid_item = LayoutStyle {
    // Place in column 1-3 (spans 2 columns)
    grid_column: GridPlacement::from_to(1, 3),

    // Place in row 2
    grid_row: GridPlacement::line(2),

    ..Default::default()
};

// Or use span
let spanning_item = LayoutStyle {
    grid_column: GridPlacement::span(2),  // Span 2 columns
    grid_row: GridPlacement::auto(),      // Auto-place row
    ..Default::default()
};
```

#### TrackSize Options

```rust
use blaeck::layout::TrackSize;

TrackSize::Auto           // Size based on content
TrackSize::Fixed(100.0)   // Exact size
TrackSize::Flex(1.0)      // Fractional unit (like CSS 'fr')
TrackSize::MinContent     // Minimum content size
TrackSize::MaxContent     // Maximum content size
TrackSize::FitContent(200.0)  // fit-content(200px)
TrackSize::Minmax(        // minmax(100px, 1fr)
    Box::new(TrackSize::Fixed(100.0)),
    Box::new(TrackSize::Flex(1.0)),
)
```

### Overflow Control

```rust
use blaeck::layout::Overflow;

let style = LayoutStyle {
    overflow_x: Overflow::Hidden,   // Clip horizontal overflow
    overflow_y: Overflow::Scroll,   // Scrollable vertical
    ..Default::default()
};

// Overflow variants:
// Overflow::Visible  - Content can overflow (default)
// Overflow::Hidden   - Content is clipped
// Overflow::Scroll   - Scrollable (affects layout)
// Overflow::Clip     - Clipped (no scroll)
```

### Aspect Ratio

```rust
let video = LayoutStyle {
    width: Some(320.0),
    aspect_ratio: Some(16.0 / 9.0),  // Height auto-calculated
    ..Default::default()
};
```

### Per-Axis Gap

```rust
let style = LayoutStyle {
    column_gap: Some(20.0),  // Horizontal gap
    row_gap: Some(10.0),     // Vertical gap
    // gap: 15.0,            // Or use 'gap' for both axes
    ..Default::default()
};
```

### Border in Layout

Border width now participates in Taffy's box model calculation:

```rust
let style = LayoutStyle {
    border_top: 1.0,
    border_bottom: 1.0,
    border_left: 1.0,
    border_right: 1.0,
    ..Default::default()
};
```

Note: `BoxProps` still handles visual borders separately. These fields are for layout calculation when you need explicit control.

## What You Should Do Differently

### 1. Use Grid for 2D layouts

Instead of nested flexbox:

```rust
// Before: Nested flexbox for grid-like layout
BoxProps::column().with_children(vec![
    BoxProps::row().with_children(vec![cell1, cell2, cell3]),
    BoxProps::row().with_children(vec![cell4, cell5, cell6]),
])

// After: Use CSS Grid
LayoutStyle {
    display: Display::Grid,
    grid_template_columns: vec![
        TrackSize::Flex(1.0),
        TrackSize::Flex(1.0),
        TrackSize::Flex(1.0),
    ],
    ..Default::default()
}
```

### 2. Use `flex_wrap` for responsive layouts

```rust
// Items wrap when container is too narrow
LayoutStyle {
    display: Display::Flex,
    flex_wrap: FlexWrap::Wrap,
    ..Default::default()
}
```

### 3. Use `Position::Absolute` for overlays

```rust
// Modal overlay positioned over content
LayoutStyle {
    position: Position::Absolute,
    inset_top: Some(0.0),
    inset_left: Some(0.0),
    inset_right: Some(0.0),
    inset_bottom: Some(0.0),
    ..Default::default()
}
```

### 4. Use `aspect_ratio` for media

```rust
// Maintain aspect ratio when width changes
LayoutStyle {
    aspect_ratio: Some(1.0),  // Square
    width: Some(50.0),        // Height auto = 50
    ..Default::default()
}
```

## BoxProps Compatibility

`BoxProps` continues to work as before. New Taffy features use default values. To access new features directly, either:

1. Use `LayoutStyle` directly for advanced layouts
2. We can add new fields to `BoxProps` as needed (future work)

## Summary of New Types

| Type | Purpose |
|------|---------|
| `FlexWrap` | NoWrap, Wrap, WrapReverse |
| `Position` | Relative, Absolute |
| `Overflow` | Visible, Hidden, Scroll, Clip |
| `GridAutoFlow` | Row, Column, RowDense, ColumnDense |
| `TrackSize` | Auto, Fixed, Flex, MinContent, MaxContent, FitContent, Minmax |
| `GridPlacement` | line(), span(), from_to(), auto() |

## Summary of New `LayoutStyle` Fields

| Field | Type | Default |
|-------|------|---------|
| `position` | `Position` | `Relative` |
| `overflow_x` | `Overflow` | `Visible` |
| `overflow_y` | `Overflow` | `Visible` |
| `aspect_ratio` | `Option<f32>` | `None` |
| `flex_wrap` | `FlexWrap` | `NoWrap` |
| `flex_basis` | `Option<f32>` | `None` (auto) |
| `border_top/bottom/left/right` | `f32` | `0.0` |
| `column_gap` | `Option<f32>` | `None` (uses `gap`) |
| `row_gap` | `Option<f32>` | `None` (uses `gap`) |
| `inset_top/bottom/left/right` | `Option<f32>` | `None` (auto) |
| `grid_template_columns` | `Vec<TrackSize>` | `[]` |
| `grid_template_rows` | `Vec<TrackSize>` | `[]` |
| `grid_auto_columns` | `Vec<TrackSize>` | `[]` |
| `grid_auto_rows` | `Vec<TrackSize>` | `[]` |
| `grid_auto_flow` | `GridAutoFlow` | `Row` |
| `grid_column` | `GridPlacement` | auto |
| `grid_row` | `GridPlacement` | auto |
