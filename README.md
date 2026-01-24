# Blaeck

A component-based terminal UI framework for Rust — flexbox layout, async-first, inline rendering.

[![CI](https://github.com/gustafeden/blaeck/actions/workflows/ci.yml/badge.svg)](https://github.com/gustafeden/blaeck/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/blaeck.svg)](https://crates.io/crates/blaeck)
[![Documentation](https://docs.rs/blaeck/badge.svg)](https://docs.rs/blaeck)
[![License](https://img.shields.io/crates/l/blaeck.svg)](LICENSE)

---

## Preview

![Preview](https://raw.githubusercontent.com/gustafeden/blaeck/main/preview.gif)

---

## Quickstart

```toml
[dependencies]
blaeck = "0.2"
```

### 1. Print Something

The simplest thing you can do — print styled text:

```rust
use blaeck::prelude::*;

fn main() -> std::io::Result<()> {
    blaeck::print(element! {
        Text(content: "Hello from Blaeck!", color: Color::Green, bold: true)
    })
}
```

Output:
```
Hello from Blaeck!   (in bold green)
```

### 2. Add Layout

Use `Box` to arrange multiple elements:

```rust
use blaeck::prelude::*;

fn main() -> std::io::Result<()> {
    blaeck::print(element! {
        Box(border_style: BorderStyle::Round, padding: 1.0) {
            Text(content: "Status:", bold: true)
            Text(content: "Ready", color: Color::Green)
        }
    })
}
```

Output:
```
╭──────────────╮
│ Status:      │
│ Ready        │
╰──────────────╯
```

### 3. Make It Interactive

For apps that respond to keyboard input, use `ReactiveApp`:

```rust
use blaeck::prelude::*;
use blaeck::reactive::*;

fn counter(cx: Scope) -> Element {
    // Create a piece of state (starts at 0)
    let count = use_state(cx.clone(), || 0);

    // Handle keyboard input
    let count_handler = count.clone();
    use_input(cx, move |key| {
        if key.is_char(' ') {
            count_handler.set(count_handler.get() + 1);
        }
    });

    // Return the UI (re-renders automatically when count changes)
    element! {
        Box(border_style: BorderStyle::Round, padding: 1.0) {
            Text(content: format!("Count: {}", count.get()), color: Color::Green)
            Text(content: "SPACE to increment, Ctrl+C to exit", dim: true)
        }
    }
}

fn main() -> std::io::Result<()> {
    ReactiveApp::run(counter)
}
```

**Key concepts:**
- `Scope` — context passed to your component (like React's component instance)
- `use_state` — creates reactive state; UI re-renders when it changes
- `use_input` — registers a keyboard handler (only runs once, not every render)
- `Signal` — a container for state; use `.get()` to read, `.set()` to write

The `.clone()` calls are Rust's way of sharing data between the render function and the input handler — both need access to `count`.

---

## Why Blaeck

### Blaeck is...

- **Component-based** — Build UIs from composable, reusable components
- **Flexbox layout** — Powered by [Taffy](https://github.com/DioxusLabs/taffy), the same engine used by Dioxus
- **Inline rendering** — Renders within terminal flow, coexists with stdout
- **Async-first** — Native tokio integration for background tasks
- **JSX-like syntax** — `element!` macro for declarative UI trees
- **Focus management** — Tab navigation with hooks for accessibility
- **35+ components** — Tables, trees, modals, syntax highlighting, charts, and more

### Blaeck is NOT...

- A fullscreen TUI framework (use [Ratatui](https://github.com/ratatui-org/ratatui) for that)
- A prompt-only library (use [inquire](https://github.com/mikaelmello/inquire) or [dialoguer](https://github.com/console-rs/dialoguer) for simple prompts)
- An immediate-mode renderer

### When to use Blaeck

- CLI tools with rich output (installers, build tools, dev servers)
- Interactive prompts that need layout control
- Dashboards that update in-place
- Progress displays with multiple concurrent tasks
- Any terminal UI that shouldn't take over the whole screen

---

## Feature Highlights

### Layout System

```rust
element! {
    Box(flex_direction: FlexDirection::Row, gap: 2.0) {
        Box(flex_direction: FlexDirection::Column, border_style: BorderStyle::Single) {
            Text(content: "Left Panel")
        }
        Spacer  // Fills available space
        Box(flex_direction: FlexDirection::Column, border_style: BorderStyle::Single) {
            Text(content: "Right Panel")
        }
    }
}
```

### Interactive Forms

```rust
// Text input with state
let mut input_state = TextInputState::new();

element! {
    Box(flex_direction: FlexDirection::Column, gap: 1.0) {
        Text(content: "Username:")
        TextInput(state: input_state.clone(), placeholder: "Enter name...")
        Confirm(prompt: "Save changes?", selected: true)
    }
}
```

### Data Display

```rust
// Table with selection
element! {
    Table(
        headers: vec!["Name", "Status", "CPU"],
        rows: vec![
            Row::new(vec!["nginx", "running", "2.3%"]),
            Row::new(vec!["postgres", "running", "5.1%"]),
        ],
        border_style: BorderStyle::Round,
        selected_row: Some(0)
    )
}

// Tree view
element! {
    TreeView(
        root: TreeNode::new("src")
            .child(TreeNode::leaf("main.rs"))
            .child(TreeNode::new("components")
                .child(TreeNode::leaf("mod.rs"))),
        state: tree_state
    )
}
```

---

## Components

| Category | Components | Notes |
|----------|------------|-------|
| **Layout** | `Box`, `Spacer`, `Newline`, `Indent`, `Transform` | Flexbox-based |
| **Text** | `Text`, `Gradient`, `Markdown`, `SyntaxHighlight` | Full styling |
| **Input** | `TextInput`, `Checkbox`, `Select`, `MultiSelect`, `Confirm`, `Autocomplete` | Interactive, focus-aware |
| **Data** | `Table`, `Tabs`, `TreeView`, `BarChart`, `Sparkline` | Scrollable, selectable |
| **Feedback** | `Spinner`, `Progress`, `Timer`, `LogBox`, `Diff` | Animated |
| **Navigation** | `Breadcrumbs`, `StatusBar`, `KeyHints` | Context display |
| **Overlay** | `Modal`, `Badge`, `Link`, `Divider` | Dialogs, alerts |
| **Animation** | `AnimationTimer`, `blink`, `Easing` | 10+ easing functions |

---

## How It Works

Blaeck uses **inline rendering** — it renders within the normal terminal flow (not fullscreen). Your output stays in scrollback, and `println!()` works after you're done.

For interactive apps, `ReactiveApp` handles the event loop: poll for input → dispatch to handlers → re-render if state changed.

For details on layout, focus management, and async, see [ARCHITECTURE.md](ARCHITECTURE.md).

---

## Examples

```bash
# Reactive API (recommended for interactive apps)
cargo run --example reactive_counter  # Counter with use_state
cargo run --example reactive_list     # List with multiple signals

# Static rendering
cargo run --example hello             # Hello world

# Components
cargo run --example spinner_demo      # 15 spinner styles
cargo run --example progress          # Progress bars
cargo run --example table             # Data tables
cargo run --example tree              # File tree view
cargo run --example syntax            # Syntax highlighting
cargo run --example modal             # Dialog boxes
cargo run --example barchart          # Charts
cargo run --example markdown          # Markdown rendering

# Advanced
cargo run --example dashboard         # Multi-panel layout
cargo run --example timer             # Stopwatch/countdown
cargo run --example interactive       # Imperative API (advanced)
cargo run --example async_demo --features async  # Async app
```

---

## Comparison

| | Blaeck | Ratatui | inquire | dialoguer |
|---|:---:|:---:|:---:|:---:|
| **Rendering** | Inline | Fullscreen | Inline | Inline |
| **Layout** | Flexbox | Constraint-based | None | None |
| **Component model** | Yes | No (immediate) | No | No |
| **Async support** | Native | Manual | No | No |
| **Focus management** | Built-in | Manual | Per-prompt | Per-prompt |
| **Use case** | Rich CLIs | Full TUIs | Prompts | Prompts |

---

## Roadmap

- [ ] Mouse support
- [ ] Virtualized scrolling for large datasets
- [ ] Theming system
- [ ] More chart types (line, pie)
- [ ] Terminal capability detection
- [ ] Accessibility improvements

---

## Contributing

```bash
# Build
cargo build --all-features

# Test (660 tests)
cargo test --all

# Lint
cargo clippy --all-targets --all-features

# Format
cargo fmt --all
```

PRs and issues welcome. For larger changes, open an issue first to discuss.

---

## License

MIT OR Apache-2.0
