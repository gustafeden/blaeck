# Blaeck

A component-based terminal UI framework for Rust — flexbox layout, async-first, inline rendering.

[![CI](https://github.com/gustafeden/blaeck/actions/workflows/ci.yml/badge.svg)](https://github.com/gustafeden/blaeck/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/blaeck.svg)](https://crates.io/crates/blaeck)
[![Documentation](https://docs.rs/blaeck/badge.svg)](https://docs.rs/blaeck)
[![License](https://img.shields.io/crates/l/blaeck.svg)](LICENSE)

---

## Preview

<!-- TODO: Replace with actual GIF/video recording (e.g., using vhs or asciinema) -->

```
╭─────────────────────────────────────────────────────────────╮
│                                                             │
│  ┌─ System Status ──────────────────────────────────────┐   │
│  │ ● API Server    running    cpu 2.3%    mem 128MB     │   │
│  │ ● Database      running    cpu 5.1%    mem 512MB     │   │
│  │ ○ Cache         stopped                               │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                             │
│  Tasks                               Progress               │
│  ├── Downloading assets              [████████░░] 80%      │
│  ├── Compiling                       [██████████] done     │
│  └── ⠋ Running tests...                                    │
│                                                             │
│  │ src/                                                    │
│  │ ├── main.rs                                             │
│  │ └── components/                                         │
│  │     ├── mod.rs                                          │
│  │     └── button.rs                                       │
│                                                             │
│  [Tab] Next  [Enter] Select  [q] Quit                      │
│                                                             │
╰─────────────────────────────────────────────────────────────╯
```

---

## Quickstart

```toml
[dependencies]
blaeck = "0.1"
```

```rust
use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let ui = element! {
        Box(border_style: BorderStyle::Round, padding: 1.0) {
            Box(flex_direction: FlexDirection::Row, gap: 2.0) {
                Text(content: "Status:", bold: true)
                Text(content: "Ready", color: Color::Green)
            }
            Box(flex_direction: FlexDirection::Row, gap: 1.0) {
                Text(content: "Progress:", dim: true)
                Text(content: "[████████░░] 80%", color: Color::Cyan)
            }
        }
    };

    blaeck.render(ui)?;
    blaeck.unmount()?;
    Ok(())
}
```

Output:
```
╭────────────────────────────╮
│ Status:  Ready             │
│ Progress: [████████░░] 80% │
╰────────────────────────────╯
```

### Interactive Example

```rust
use blaeck::prelude::*;
use blaeck::{App, Key, match_key};

fn main() -> std::io::Result<()> {
    let items = vec!["Build", "Test", "Deploy", "Rollback"];
    let mut selected = 0;

    App::new()?.run(
        |_| element! {
            Box(border_style: BorderStyle::Round, padding: 1.0) {
                Text(content: "Select action:", bold: true)
                // ... render items with selection indicator
            }
        },
        |app, key| {
            match_key(&key, &mut selected)
                .on_arrow(Arrow::Up, |s| *s = s.saturating_sub(1))
                .on_arrow(Arrow::Down, |s| *s = (*s + 1).min(items.len() - 1))
                .on_enter(|_| { /* handle selection */ })
                .on_char('q', |_| app.exit());
        },
    )
}
```

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

## Architecture

### Rendering Model

Blaeck uses **inline rendering**: it tracks how many lines were written and overwrites them on re-render. This means:

- Output stays in terminal scrollback
- `println!()` works after `unmount()`
- Multiple Blaeck instances can coexist
- No alternate screen buffer

Render throttling prevents excessive CPU usage:

```rust
blaeck.set_max_fps(30); // Limit to 30 renders/second
```

### Layout System

Uses [Taffy](https://github.com/DioxusLabs/taffy) for flexbox layout:

- `flex_direction`: Row or Column
- `justify_content`, `align_items`: Standard flexbox alignment
- `gap`, `padding`, `margin`: Spacing
- `flex_grow`, `flex_shrink`: Flexible sizing

### Focus Management

```rust
let mut focus = FocusManager::new();
focus.register(FocusId(0));
focus.register(FocusId(1));

focus.on_focus_change(|event| {
    // Handle focus/blur events
});

// In input handler
match_key(&key, &mut focus)
    .on_tab(|f| f.focus_next())
    .on_backtab(|f| f.focus_previous());
```

### Async Model

Enable with `features = ["async"]`:

```rust
use blaeck::{AsyncApp, AppEvent, channel};

#[tokio::main]
async fn main() -> io::Result<()> {
    let (tx, mut rx) = channel::<Message>(10);

    // Background task
    tokio::spawn(async move {
        tx.send(Message::Update).await.ok();
    });

    let mut app = AsyncApp::new(io::stdout())?;

    loop {
        tokio::select! {
            event = app.next_event() => { /* handle input */ }
            msg = rx.recv() => { /* handle background message */ }
        }
        app.render(build_ui())?;
    }
}
```

---

## Examples

```bash
# Basic
cargo run --example hello           # Hello world
cargo run --example interactive     # Keyboard input

# Components
cargo run --example spinner_demo    # 15 spinner styles
cargo run --example progress        # Progress bars
cargo run --example table           # Data tables
cargo run --example tree            # File tree view
cargo run --example syntax          # Syntax highlighting
cargo run --example modal           # Dialog boxes
cargo run --example barchart        # Charts
cargo run --example markdown        # Markdown rendering

# Advanced
cargo run --example dashboard       # Multi-panel layout
cargo run --example timer           # Stopwatch/countdown
cargo run --example logbox_command  # Command output viewer
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
