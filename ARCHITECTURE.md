# Blaeck Architecture

This document explains how Blaeck works internally. Read this before diving into the code.

---

## Mental Model (5-minute version)

Blaeck renders terminal UI **inline** — it doesn't take over the screen like Ratatui. Instead, it tracks how many lines it wrote and overwrites them on re-render.

```
User code (element! macro)
        │
        ▼
┌───────────────────┐
│  Element Tree     │  ← Tree of components (Box, Text, etc.)
└───────────────────┘
        │
        ▼
┌───────────────────┐
│  Layout (Taffy)   │  ← Flexbox computes x, y, width, height
└───────────────────┘
        │
        ▼
┌───────────────────┐
│  Output (2D grid) │  ← Virtual grid of styled characters
└───────────────────┘
        │
        ▼
┌───────────────────┐
│  LogUpdate        │  ← The clever bit: cursor up, erase, rewrite
└───────────────────┘
        │
        ▼
    Terminal
```

**The key insight**: `LogUpdate` (stolen from Ink) makes inline re-rendering possible. It remembers how many lines were written, moves the cursor up that many lines, erases them, and writes the new content. This creates the illusion of in-place updates without alternate screen mode.

---

## Core Types

| Type | File | What it does |
|------|------|--------------|
| `Element` | `element.rs` | Tree node — can be `Text`, `Node` (component), `Fragment`, or `Empty` |
| `Component` | `element.rs` | Trait for defining UI pieces. Has `Props` type and `render()` function |
| `Blaeck<W>` | `renderer.rs` | Main renderer. Owns the output writer, orchestrates rendering |
| `Output` | `output.rs` | Virtual 2D grid. Write styled characters at (x, y), then convert to string |
| `LogUpdate` | `log_update.rs` | Tracks line count, handles cursor movement and line erasure |
| `LayoutStyle` | `layout.rs` | Flexbox properties. Thin wrapper around Taffy |
| `Style` | `style.rs` | Text styling: color (16/256/RGB), bold, italic, dim, etc. |

---

## Where to Start Reading

If you want to understand Blaeck's internals, read these files in order:

### 1. `quill/src/renderer.rs` — The Engine

This is where everything comes together. `Blaeck::render()` does:
1. Build layout tree from element tree
2. Call Taffy to compute positions
3. Render each element to the Output grid
4. Convert Output to string
5. Call LogUpdate to write to terminal

Start here. Everything else exists to serve this file.

### 2. `quill/src/element.rs` — The Component Model

Defines `Element` (the tree node) and `Component` (the trait). Components are type-erased via `TypeId` + `Box<dyn Any>` so the element tree can be heterogeneous.

Key pattern:
```rust
pub trait Component {
    type Props: Default + 'static;
    fn render(props: &Self::Props) -> Element;
}
```

### 3. `quill/src/log_update.rs` — The Clever Bit

This is what makes Blaeck different from fullscreen TUIs. It implements Ink's `log-update` pattern:

```rust
// Simplified logic:
fn render(&mut self, content: &str) {
    // 1. Move cursor up N lines (where N = previous line count)
    // 2. Erase each line
    // 3. Write new content
    // 4. Remember new line count for next render
}
```

Uses ANSI escapes: `ESC[nA` (cursor up), `ESC[2K` (erase line), `ESC[0G` (cursor to column 0).

### 4. `quill/src/output.rs` — The Virtual Grid

A 2D array of `StyledChar`. Components write to it at (x, y) coordinates, then it's converted to a string with embedded ANSI codes.

Handles:
- Multi-line text (split and write each line)
- Wide characters (CJK takes 2 columns)
- Style transitions (only emit ANSI codes when style changes)
- Trailing whitespace trimming

### 5. `quill/src/components/box_component.rs` — Example Component

Read this to understand how components work. `Box` is the most complex built-in component:
- Borders (10+ styles)
- Padding, margin
- Flexbox properties (direction, gap, justify, align)
- Visibility control

---

## Data Flow (Detailed)

### Step 1: User writes element tree

```rust
element! {
    Box(border_style: BorderStyle::Round, padding: 1.0) {
        Text(content: "Hello", bold: true)
    }
}
```

The `element!` macro expands to:
```rust
Element::node::<Box>(
    BoxProps { border_style: BorderStyle::Round, padding: 1.0, .. },
    vec![
        Element::node::<Text>(
            TextProps { content: "Hello".into(), bold: true, .. },
            vec![]
        )
    ]
)
```

### Step 2: Renderer builds layout tree

`Blaeck::render()` walks the element tree and creates Taffy nodes:
- Each `Element::Node` becomes a Taffy node
- Props are converted to `taffy::Style` (flex direction, padding, etc.)
- Children are recursively processed

### Step 3: Taffy computes layout

```rust
taffy.compute_layout(root, available_size);
```

Now every node has `x`, `y`, `width`, `height`.

### Step 4: Render to Output grid

Walk the tree again, this time rendering:
- `Text` → write characters to Output at computed position
- `Box` → draw border characters, then render children inside

### Step 5: Output → String

`Output::get()` converts the 2D grid to a string:
- Iterate rows
- Track current style, emit ANSI codes on change
- Trim trailing whitespace
- Join with newlines

### Step 6: LogUpdate writes to terminal

```rust
log_update.print(&output_string);
```

This erases previous output and writes new content.

---

## Design Decisions (Why?)

### Why inline rendering instead of fullscreen?

Blaeck targets CLI tools, not full TUIs. CLI tools should:
- Coexist with normal terminal output
- Leave scrollback intact
- Allow `println!()` after the UI exits

Fullscreen (alternate buffer) is the wrong model for installers, build tools, progress displays.

### Why Taffy for layout?

Flexbox is familiar to web developers and handles the common cases well:
- Row/column layouts
- Spacing (gap, padding, margin)
- Flexible sizing (grow/shrink)
- Alignment

Taffy is battle-tested (used by Dioxus, Bevy) and well-maintained.

### Why type-erased components?

The element tree needs to hold any component type. Options:
1. Enum with all component variants — doesn't scale, can't extend
2. Trait objects — loses type info for props
3. Type erasure with `TypeId` + `Box<dyn Any>` — works

We went with (3). Props are stored as `Box<dyn Any>` and downcast at render time. The `TypeId` identifies which component to render.

### Why copy from Ink/Ratatui/Iocraft?

These libraries solved the same problems. Copying proven patterns:
- Reduces bugs (patterns are already debugged)
- Provides familiar APIs
- Lets us focus on integration, not invention

Specifically:
- **Ink**: LogUpdate pattern, Output grid concept
- **Ratatui**: Style/Color/Modifier design, cell representation
- **Iocraft**: Component trait, element macro, Taffy integration

---

## File Map

```
quill/src/
├── lib.rs              # Public exports, prelude
├── renderer.rs         # Main render engine (START HERE)
├── element.rs          # Element enum, Component trait
├── log_update.rs       # Inline rendering magic
├── output.rs           # Virtual 2D grid
├── layout.rs           # Taffy wrapper
├── style.rs            # Colors, modifiers, ANSI codes
├── app.rs              # Synchronous app runtime
├── async_runtime.rs    # Async app runtime (tokio)
├── input.rs            # Keyboard input handling
├── focus.rs            # Focus management
├── animation.rs        # Animation utilities
├── buffer.rs           # Terminal buffer utilities
└── components/         # Built-in components
    ├── mod.rs
    ├── box_component.rs    # Container with borders/padding
    ├── text.rs             # Styled text
    ├── spacer.rs           # Flexible space filler
    ├── table.rs            # Data tables
    ├── spinner.rs          # Loading indicators
    ├── progress.rs         # Progress bars
    ├── text_input.rs       # Text editing
    ├── select.rs           # Single selection
    ├── multi_select.rs     # Multiple selection
    ├── tree_view.rs        # Hierarchical display
    ├── modal.rs            # Dialog boxes
    └── ... (20+ more)
```

---

## Common Tasks

### Adding a new component

1. Create `quill/src/components/my_component.rs`
2. Define props struct with builder methods
3. Implement `Component` trait
4. Add to `components/mod.rs` exports
5. Add tests

See `spacer.rs` for a minimal example, `table.rs` for a complex one.

### Understanding a render bug

1. Check `Output::get()` — is the grid correct?
2. Check layout computation — are positions/sizes right?
3. Check component render — is it writing to correct coordinates?
4. Check `LogUpdate` — is line count tracked correctly?

### Adding a new style feature

1. Add to `Modifier` bitflags or `Color` enum in `style.rs`
2. Update `Style::to_ansi_string()` to emit correct escape codes
3. Add builder method to `Style`
4. Update component props to expose the feature

---

## Glossary

| Term | Meaning |
|------|---------|
| **Inline rendering** | Rendering within terminal flow (vs fullscreen/alternate buffer) |
| **LogUpdate** | Pattern from Ink: track lines, cursor up, erase, rewrite |
| **Element tree** | Nested structure of UI components before layout |
| **Layout tree** | Taffy tree with computed positions/sizes |
| **Output grid** | 2D array of styled characters |
| **Props** | Component configuration (immutable per render) |
| **Type erasure** | Storing any type via `Box<dyn Any>` + `TypeId` |

---

## Further Reading

- `QUILL.md` — Development guide with TDD phases
- `ROADMAP.md` — Feature roadmap and comparisons
- `refs/ink/src/log-update.ts` — Original Ink implementation
- `refs/iocraft/packages/iocraft/src/` — Component model inspiration
