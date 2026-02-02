# Community Posts

Prepared posts for distribution. Adapt as needed before posting.

---

## Show HN (news.ycombinator.com)

**Title:**
Show HN: Blaeck – Inline terminal UI framework for Rust with flexbox layout and 35+ components

**Body:**
I built blaeck because I wanted rich terminal UIs for CLI tools without taking over the whole screen. Existing TUI frameworks (like Ratatui) use alternate screen mode — your output disappears when the program exits. blaeck renders inline: output stays in scrollback, and `println!()` works before and after.

It uses a log-update pattern (cursor up, erase, rewrite) similar to Node.js Ink, but in Rust with Taffy for flexbox/grid layout.

Two APIs:
- **Reactive** (recommended) — signals and hooks, like React. State changes trigger automatic re-renders.
- **Static** — one-shot `Blaeck::render()` for progress bars, build output, etc.

35+ components: tables, spinners, progress bars, text inputs, selects, tree views, modals, syntax highlighting, markdown rendering, and more.

Good for: installers, build tools, progress displays, interactive prompts — anything that shouldn't take over the screen.

Not for: fullscreen TUIs (use Ratatui for that).

Repo: https://github.com/gustafeden/blaeck
Docs: https://docs.rs/blaeck
Try the interactive example viewer (no Rust required):
```
curl -fsSL https://gustafeden.github.io/blaeck/run-example-viewer.sh | bash
```

Happy to answer questions about the rendering approach or component architecture.

---

## r/rust

**Title:**
blaeck: Terminal UI framework for Rust — inline rendering, flexbox layout, 35+ components

**Body:**
I've been working on blaeck, a terminal UI framework that renders **inline** rather than fullscreen. Think Ink (Node.js) but for Rust.

**The problem:** Most TUI frameworks use alternate screen mode. Great for editors and dashboards, but overkill for CLI tools that just need rich output. Your `println!()` disappears. Scrollback is gone.

**The approach:** blaeck uses a log-update pattern — cursor moves up, erases lines, rewrites content. Output stays in your terminal's scrollback. You can `println!()` before and after. No alternate screen.

**Technical details:**
- Layout engine: Taffy 0.9 (flexbox + CSS grid)
- Rendering: 2D virtual grid → ANSI diff → minimal terminal writes
- Two APIs: reactive (signals/hooks like React) and imperative
- Proc-macro `element!` for declarative component trees
- 35+ components (table, spinner, progress, text input, select, tree view, modal, syntax highlighting, markdown, etc.)
- Optional async support via tokio
- Animation system with timeline, spring physics, and stagger

**Example — inline progress bar:**
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

**When to use blaeck vs alternatives:**
- Need fullscreen TUI → Ratatui
- Need simple prompts → inquire
- Need rich inline output with layout control → blaeck

Repo: https://github.com/gustafeden/blaeck
crates.io: https://crates.io/crates/blaeck
Docs: https://docs.rs/blaeck

There's also an interactive example viewer you can run without Rust:
```
curl -fsSL https://gustafeden.github.io/blaeck/run-example-viewer.sh | bash
```

Feedback welcome — especially on the API design and component set.

---

## r/commandline

**Title:**
blaeck - Build rich terminal UIs that don't take over your screen (Rust)

**Body:**
Most terminal UI frameworks use alternate screen mode — they take over your terminal, and when the program exits, everything is gone.

blaeck is different: it renders inline. Output stays in scrollback. You can print before and after.

This makes it great for:
- CLI installers with progress tracking
- Build tools with rich output
- Interactive prompts with layout control
- Dashboards that don't need a dedicated screen

It comes with 35+ components (tables, spinners, progress bars, text inputs, selects, tree views, etc.) and uses flexbox for layout.

Written in Rust. Available on crates.io.

Try the interactive example viewer (downloads a prebuilt binary, no Rust needed):
```
curl -fsSL https://gustafeden.github.io/blaeck/run-example-viewer.sh | bash
```

Repo: https://github.com/gustafeden/blaeck

---

## X/Twitter

**Option 1 (inline rendering focus):**
Built blaeck — a terminal UI framework for Rust that renders inline instead of fullscreen.

Output stays in scrollback. println!() works before and after. 35+ components with flexbox layout.

Think Ink (Node.js), but for Rust.

[GIF: inline rendering demo]

https://github.com/gustafeden/blaeck

**Option 2 (component showcase):**
blaeck: 35+ terminal UI components for Rust

Tables, spinners, progress bars, text inputs, selects, tree views, modals, syntax highlighting, markdown rendering...

All with flexbox layout. All inline (no fullscreen takeover).

[GIF: example viewer]

https://github.com/gustafeden/blaeck

**Option 3 (reactive API):**
Built a React-like terminal UI framework for Rust.

Signals, hooks, declarative components, flexbox layout — but for the terminal. And it renders inline, not fullscreen.

[GIF: reactive form demo]

https://github.com/gustafeden/blaeck

---

## Future Posts

### Dev.to
**Title:** Why Your CLI Shouldn't Take Over the Screen

**Topics to cover:**
- The problem with alternate screen mode for CLI tools
- How inline rendering works (log-update pattern)
- Building a progress display with blaeck
- Component architecture and flexbox in the terminal
- When to use fullscreen vs inline

### r/programming
**Title:** Inline Terminal UI: A Different Approach to CLI Rendering

**Focus:** The concept of inline vs fullscreen terminal rendering. Language-agnostic framing with blaeck as the implementation.

### Hashnode
**Title:** Building a Terminal UI Framework Inspired by React and Ink

**Focus:** Technical deep-dive into the architecture — virtual grid rendering, Taffy layout engine, reactive signals, the element! macro.
