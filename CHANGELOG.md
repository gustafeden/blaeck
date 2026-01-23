# Changelog

All notable changes to Blaeck will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
