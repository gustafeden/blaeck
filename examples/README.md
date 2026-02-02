# Blaeck Examples

Run examples with:

```bash
cargo run --example <name>
cargo run --example <name> --features async  # for async examples
```

**Or explore all examples interactively:**

```bash
cargo run --example example_viewer
```

The example viewer provides a three-panel interface with live previews, source code viewing, and interactive testing. Press `Enter` on any example to interact with it in focus mode. See the main [README](../README.md#example-viewer) for details.

---

## Learning Path

### Start Here

| Example | What it teaches |
|---------|-----------------|
| `hello` | Minimal Blaeck app — render and exit |
| `interactive` | Keyboard input, event loop, App runtime |
| `borders` | Box borders and padding |

### Core Patterns

| Example | What it teaches |
|---------|-----------------|
| `spinner_demo` | Animated spinners, render loop timing |
| `timer` | Stopwatch and countdown, state updates |
| `select_demo` | Arrow key navigation, SelectState |
| `form_demo` | Text input, checkboxes, form state |
| `table` | Data tables, column alignment |

### Advanced

| Example | What it teaches |
|---------|-----------------|
| `dashboard` | Multi-panel layouts, flexbox composition |
| `async_app` | Tokio integration, background tasks |
| `focus_demo` | Tab navigation, focus management |
| `task_runner` | Real-world pattern: progress + status |

---

## All Examples

### Basic

- **hello** — "Hello World" — simplest possible Blaeck app
- **interactive** — Keyboard input with App event loop
- **borders** — Box border styles (Single, Double, Round, etc.)

### Input Components

- **select_demo** — Single-selection list with arrow keys
- **multiselect** — Checkbox list for multiple selections
- **autocomplete** — Text input with filtered suggestions
- **form_demo** — Complete form with text input, checkbox, confirm

### Data Display

- **table** — Tables with headers, rows, alignment
- **tree** — Hierarchical tree view with expand/collapse
- **tabs** — Tab bar navigation
- **barchart** — Horizontal bar charts
- **sparkline** — Mini inline charts

### Feedback

- **spinner_demo** — All 15 spinner styles
- **timer** — Stopwatch and countdown timers
- **logbox** — Scrolling log viewer
- **logbox_command** — Log viewer with live command output

### Rich Text

- **gradient** — Color gradient text (10 presets)
- **markdown** — Markdown rendering
- **syntax** — Syntax highlighted code
- **diff** — Git-style diff display

### Navigation

- **breadcrumbs** — Path navigation display
- **statusbar** — Git-style status segments
- **keyhints** — Keyboard shortcut display
- **menu** — Menu selection pattern

### Dialogs

- **modal** — Modal dialogs (alert, confirm, error)
- **banner** — Large text banners

### Layout

- **dashboard** — Multi-panel flexbox layout
- **polish_demo** — Polished UI patterns

### Animation

- **animation** — Animation timer, easing functions
- **cube3d_braille** — 3D cube with braille rendering

### Async

- **async_app** — AsyncApp with background tasks and tokio integration

---

## Code Patterns

### Minimal App

```rust
use quill::prelude::*;
use quill::Blaeck;

fn main() -> std::io::Result<()> {
    let mut quill = Blaeck::new(std::io::stdout())?;
    quill.render(element! {
        Text(content: "Hello!")
    })?;
    quill.unmount()
}
```

### Interactive App

```rust
use quill::prelude::*;
use quill::{App, Key};

fn main() -> std::io::Result<()> {
    let mut count = 0;

    App::new()?.run(
        |_| element! {
            Box(padding: 1.0) {
                Text(content: format!("Count: {}", count))
            }
        },
        |app, key| {
            if key.is_char(' ') { count += 1; }
            if key.is_char('q') { app.exit(); }
        },
    )
}
```

### State Management

```rust
// Use companion State structs for interactive components
let mut select_state = SelectState::new(items.len());
let mut input_state = TextInputState::new();
let mut multi_state = MultiSelectState::new(items.len());

// Handle input
select_state.down();  // Arrow down
select_state.up();    // Arrow up
input_state.insert('a');  // Type character
multi_state.toggle(select_state.selected());  // Toggle selection
```

---

## Tips

1. **Start simple**: `hello.rs` → `interactive.rs` → add components
2. **Stable layouts**: Keep element tree structure consistent across states
3. **State structs**: Use `SelectState`, `TextInputState`, etc. for interactive components
4. **Throttle renders**: Use `quill.set_max_fps(30)` for animations
5. **Check ARCHITECTURE.md**: Understand the render pipeline before diving deep
