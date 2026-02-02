# Changelog

All notable changes to Blaeck will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-02-02

### Added

**Developer Tools**
- `example_viewer` - Interactive tool for exploring all 44 examples with live previews
- Three-panel interface: example list, source code viewer, and live preview/output
- Focus mode for interactive testing of examples
- Resizable panels with `[` and `]` keys
- Background example execution with build/run/stop controls
- UTF-8 safe code truncation for proper display

**GitHub Pages Distribution**
- `run-example-viewer.sh` script for instant demo without Rust installation
- One-line curl installation: `curl -fsSL https://gustafeden.github.io/blaeck/run-example-viewer.sh | bash`
- Landing page at https://gustafeden.github.io/blaeck/
- Release workflow builds binaries for Linux x86_64, macOS x86_64, and macOS ARM64

**Animation System Enhancements**
- Timeline animation system with declarative API
- Spring physics for natural animations with presets (snappy, bouncy, gentle, overdamped)
- Stagger animations for coordinated multi-item sequences
- Timeline callbacks (`on_enter`, `on_exit`) for act transitions
- Timeline visualization and debug tools
- `use_timeline` hook for reactive integration

**New Examples**
- `showcase` - Full-featured demo with memory monitoring, FPS display, and animations
- `dashboard` - Complex multi-panel layout with choreographed animations
- `plasma` - Interactive lava lamp effect with color presets
- `stagger_demo` - Demonstrates stagger animation patterns
- `timeline_demo` - Interactive timeline animation showcase
- `timeline_debug` - Timeline state visualization tools

**Layout Improvements**
- Upgraded to Taffy 0.9 with full feature support
- Grid layout with `grid_template_columns` and `grid_template_rows`
- Grid placement with `grid_row` and `grid_column`
- Grid gaps: `row_gap`, `column_gap`, `gap`
- Aspect ratio control with `aspect_ratio`
- Grid auto-flow and alignment features

**API Additions**
- `Blaeck::set_cursor_visible()` for cursor visibility control
- `Blaeck::handle_resize()` for terminal resize events

### Changed

- Preview system: All examples migrated to new `previews/` module pattern
- Example wrapper pattern: Examples now separate main logic from UI rendering
- Documentation improvements with clearer example organization

### Removed

- 11 redundant/obsolete examples consolidated into better alternatives:
  - `async_demo` → superseded by `async_app`
  - `components_demo` → consolidated into other examples
  - `countdown` → merged into `timer`
  - `cube3d` → superseded by `cube3d_braille`
  - `demo_form` → renamed to `form_demo`
  - `nested_test` → internal testing only
  - `progress` → merged into `timer` and `task_runner`
  - `quickstart_static` → superseded by `quickstart_interactive`
  - `theater` + spec files → incomplete feature removed

### Fixed

- CI: Resolved all clippy warnings (large enum variants, type complexity, needless borrows)
- Formatting: Applied consistent formatting across all files
- Performance: Reduced `Element::Node` size by boxing large layout style

## [0.2.0] - 2025-01-24

### Added

**Reactive API** (new recommended way to build UIs)
- `reactive` module with signals-based state management
- `ReactiveApp` - Entry point for reactive applications
- `use_state` hook - Create reactive state that triggers re-render on change
- `use_input` hook - Register keyboard input handlers
- `Signal<T>` - Reactive state container with `get()`, `set()`, and `update()`
- `Scope` - Context passed to reactive component functions
- Explicit `RuntimeHandle` (no thread-local state) for testability

**Examples**
- `reactive_counter.rs` - Basic counter demonstrating use_state and use_input
- `reactive_list.rs` - List management with multiple signals

### Changed

- Documentation now positions ReactiveApp as the preferred/recommended approach
- Updated "Quick Start" to show reactive API first
- Added "Two Paradigms" section explaining reactive vs imperative approaches

### Dependencies

- Added `slotmap = "1.0"` for efficient arena-based storage

## [0.1.0] - 2025-01-22

Initial release.

### Added

**Core**
- Inline rendering engine with log-update pattern
- Flexbox layout via Taffy
- `element!` macro for JSX-like syntax
- Focus management with hooks
- Async support (tokio)
- Render throttling

**Layout Components**
- `Box` - Flexbox container with borders
- `Spacer` - Flexible space
- `Newline` - Line break
- `Indent` - Indentation
- `Transform` - Text transformations

**Text Components**
- `Text` - Styled text
- `Gradient` - Color gradients (10 presets)
- `Markdown` - Inline markdown rendering
- `SyntaxHighlight` - Code highlighting (7 themes)

**Input Components**
- `TextInput` - Text entry with cursor
- `Checkbox` - Toggle (5 styles)
- `Select` - Single selection list
- `MultiSelect` - Multiple selection
- `Confirm` - Yes/No prompt
- `Autocomplete` - Filtered suggestions

**Data Components**
- `Table` - Data tables with selection
- `Tabs` - Tab navigation
- `TreeView` - Hierarchical display
- `BarChart` - Horizontal bar charts
- `Sparkline` - Inline mini charts

**Feedback Components**
- `Spinner` - Animated spinners (15 styles)
- `Progress` - Progress bars (6 styles)
- `Timer` - Stopwatch/countdown
- `LogBox` - Scrolling log viewer
- `Diff` - Git-style diff display

**Navigation Components**
- `Breadcrumbs` - Path navigation
- `StatusBar` - Status display
- `KeyHints` - Keyboard shortcuts

**Overlay Components**
- `Modal` - Dialog boxes (5 styles)
- `Badge` - Status badges
- `Link` - Terminal hyperlinks
- `Divider` - Horizontal dividers

**Animation**
- `AnimationTimer` - Frame timing
- Blink/pulse helpers
- 10+ easing functions
