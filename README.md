# Blaeck

A terminal UI framework for Rust. Inline rendering, flexbox layout, 35+ components.

[![Crates.io](https://img.shields.io/crates/v/blaeck.svg)](https://crates.io/crates/blaeck)
[![Documentation](https://docs.rs/blaeck/badge.svg)](https://docs.rs/blaeck)
[![License](https://img.shields.io/crates/l/blaeck.svg)](LICENSE)

![Preview](assets/preview.gif)

## Install

```toml
[dependencies]
blaeck = "0.2"
```

---

## Inline Rendering

The killer feature: blaeck renders **within your terminal flow**. Output stays in scrollback. `println!()` works before and after.

![Inline Demo](assets/demo_inline.gif)

```rust
use blaeck::prelude::*;
use std::{thread, time::Duration};

fn main() -> std::io::Result<()> {
    println!("Starting build...\n");

    let mut blaeck = Blaeck::new(std::io::stdout())?;

    for i in 0..=10 {
        let bar = "█".repeat(i * 2) + &"░".repeat(20 - i * 2);
        blaeck.render(element! {
            Box(border_style: BorderStyle::Round, padding: 1.0) {
                Text(content: "Building...", bold: true)
                Text(content: bar, color: Color::Green)
            }
        })?;
        thread::sleep(Duration::from_millis(200));
    }

    blaeck.unmount()?;
    println!("Done! Terminal keeps working.");
    Ok(())
}
```

---

## Interactive Apps

Use `ReactiveApp` for keyboard-driven UIs. State changes trigger automatic re-renders.

![Form Demo](assets/demo_form.gif)

```rust
use blaeck::prelude::*;
use blaeck::reactive::*;
use crossterm::event::KeyCode;

fn form(cx: Scope) -> Element {
    let name = use_state(cx.clone(), || String::new());
    let confirmed = use_state(cx.clone(), || false);

    let name_h = name.clone();
    let confirmed_h = confirmed.clone();
    use_input(cx, move |key| match key.code {
        KeyCode::Char(c) => { name_h.update(|s| { s.push(c); s }); }
        KeyCode::Backspace => { name_h.update(|s| { s.pop(); s }); }
        KeyCode::Char(' ') => { confirmed_h.set(!confirmed_h.get()); }
        _ => {}
    });

    element! {
        Box(border_style: BorderStyle::Round, padding: 1.0) {
            Text(content: format!("Name: {}", name.get()))
            Text(content: format!("[{}] Confirm", if confirmed.get() { "x" } else { " " }))
        }
    }
}

fn main() -> std::io::Result<()> {
    ReactiveApp::run(form)
}
```

**Key concepts:** `use_state` creates reactive state. `use_input` registers keyboard handlers. When state changes, UI re-renders automatically.

---

## When to Use Blaeck

**Good for:**
- CLI tools with rich output (installers, build tools)
- Interactive prompts with layout control
- Progress displays, dashboards
- Any UI that shouldn't take over the screen

**Not for:**
- Fullscreen TUIs → use [Ratatui](https://github.com/ratatui-org/ratatui)
- Simple prompts → use [inquire](https://github.com/mikaelmello/inquire)

---

## Components

| Category | Components |
|----------|------------|
| **Layout** | `Box`, `Spacer`, `Newline`, `Indent` |
| **Text** | `Text`, `Gradient`, `Markdown`, `SyntaxHighlight` |
| **Input** | `TextInput`, `Select`, `MultiSelect`, `Checkbox`, `Confirm` |
| **Data** | `Table`, `Tabs`, `TreeView`, `BarChart`, `Sparkline` |
| **Feedback** | `Spinner`, `Progress`, `Timer`, `Modal`, `Diff` |

---

## Examples

```bash
cargo run --example demo_inline      # Inline rendering demo
cargo run --example demo_form        # Interactive form
cargo run --example reactive_counter # Counter with signals
cargo run --example reactive_list    # List navigation
cargo run --example dashboard        # Multi-panel layout
cargo run --example table            # Data tables
cargo run --example spinner_demo     # 15 spinner styles
```

See [all examples](blaeck/examples/) or [API docs](https://docs.rs/blaeck).

---

## License

MIT OR Apache-2.0
