# Distribution & Visibility

How to get blaeck in front of people.

## Demo GIFs

Record with [VHS](https://github.com/charmbracelet/vhs) or [asciinema](https://asciinema.org).

Priority recordings:

1. **Inline rendering** — the killer feature. Show `println!()` before and after blaeck output.
2. **Interactive form** — reactive app with keyboard navigation.
3. **Example viewer** — the full component showcase.
4. **Animation system** — timeline/spring physics demos.

Embed in README. GIFs are the single most important conversion tool for a TUI library.

## crates.io

- [x] Published as [`blaeck`](https://crates.io/crates/blaeck) and [`blaeck-macros`](https://crates.io/crates/blaeck-macros)
- [x] Metadata complete (license, repository, keywords, categories, description)
- [x] docs.rs auto-generated from published crate
- [x] README renders on crates.io page
- [ ] Verify docs.rs feature flags render correctly (especially `async` feature)

## Example Viewer Binary

The example_viewer is distributed as a prebuilt binary — lets people explore blaeck without installing Rust.

- [x] GitHub Release workflow builds for 3 targets (linux-x86_64, macos-x86_64, macos-arm64)
- [x] `run-example-viewer.sh` — curl-pipe install script
- [x] Linked from README
- [x] Add `aarch64-unknown-linux-gnu` target (Linux ARM — Raspberry Pi, cloud instances)
- [ ] Host `run-example-viewer.sh` on GitHub Pages for stable URL
- [ ] Consider Homebrew tap for `example_viewer` if adoption warrants it

## GitHub Discoverability

- [x] Add repo topics: `terminal`, `tui`, `cli`, `rust`, `flexbox`, `inline`, `components`, `ui-framework`
- [ ] Submit PR to [awesome-rust](https://github.com/rust-unofficial/awesome-rust) under Command-line > TUI
- [ ] Submit PR to [awesome-tui](https://github.com/rothgar/awesome-tuis) (or similar TUI list)
- [ ] Add comparison section to README (vs Ratatui, vs Ink, vs inquire — positioning, not bashing)

## Community Posts

**Ready to post** (prepared in `POSTS.md`):

- [ ] Show HN — "Blaeck: Inline terminal UI framework for Rust (not fullscreen)"
- [ ] r/rust — technical post about inline rendering approach + component architecture
- [ ] r/commandline — practical "build rich CLI output" angle
- [ ] X/Twitter — short demo with GIF

**Future posts:**

- [ ] Dev.to — "Why Your CLI Shouldn't Take Over the Screen"
- [ ] r/programming — broader audience, focus on the inline rendering concept
- [ ] Hashnode — "Building a Terminal UI Framework Inspired by Ink"

## Positioning

blaeck occupies a unique niche. Key messages:

1. **Inline, not fullscreen** — output stays in scrollback, `println!()` works before and after
2. **Component-based** — 35+ ready-to-use components with flexbox layout
3. **Two APIs** — reactive (like React) for interactive apps, static for one-shot rendering
4. **Rust-native Ink** — familiar model for anyone who knows Ink (Node.js)

Competitors and positioning:
- **Ratatui** — fullscreen TUI. blaeck is for when you don't want to take over the screen.
- **Ink (Node.js)** — same model, but blaeck is Rust-native with better performance.
- **inquire/dialoguer** — simple prompts. blaeck is for when you need layout control.
- **indicatif** — progress bars only. blaeck is a full component framework.
