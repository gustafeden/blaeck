//! Blaeck - A declarative, component-based terminal UI library for Rust.
//!
//! Blaeck provides an Ink-like API for building terminal user interfaces
//! with flexbox layout, inline rendering (not fullscreen), and 35+ components.
//!
//! # Quick Start (Reactive API)
//!
//! The recommended way to build interactive UIs is with the **reactive API**,
//! which uses signals and hooks similar to React:
//!
//! ```ignore
//! use blaeck::prelude::*;
//! use blaeck::reactive::*;
//!
//! fn counter(cx: Scope) -> Element {
//!     let count = use_state(cx.clone(), || 0);
//!
//!     let count_handler = count.clone();
//!     use_input(cx, move |key| {
//!         if key.is_char(' ') {
//!             count_handler.set(count_handler.get() + 1);
//!         }
//!     });
//!
//!     element! {
//!         Box(border_style: BorderStyle::Round, padding: 1.0) {
//!             Text(content: format!("Count: {}", count.get()), color: Color::Green)
//!         }
//!     }
//! }
//!
//! fn main() -> std::io::Result<()> {
//!     ReactiveApp::run(counter)
//! }
//! ```
//!
//! See [`reactive`] module for the full API and more examples.
//!
//! # Two Paradigms
//!
//! Blaeck offers two ways to build UIs:
//!
//! ## 1. Reactive/Declarative (Recommended)
//!
//! Use [`reactive::ReactiveApp`] with hooks like `use_state` and `use_input`.
//! State changes automatically trigger re-renders. This is the **preferred approach**
//! for most applications.
//!
//! **Pros:**
//! - Cleaner mental model - state and UI are declaratively connected
//! - Automatic re-rendering when state changes
//! - No manual state management boilerplate
//! - Familiar to React/Solid.js developers
//!
//! **Examples:** `reactive_counter.rs`, `reactive_list.rs`
//!
//! ## 2. Imperative (Advanced)
//!
//! Use [`App`] with explicit state management (typically `RefCell` or similar).
//! You manually control when and how state changes and re-renders occur.
//!
//! **When to use:**
//! - Integration with existing imperative codebases
//! - Fine-grained control over render timing
//! - Cases where the reactive model doesn't fit
//!
//! **Examples:** `interactive.rs`, `form_demo.rs`
//!
//! ```ignore
//! use blaeck::prelude::*;
//! use blaeck::App;
//! use std::cell::RefCell;
//!
//! let state = RefCell::new(0);
//! App::new()?.run(
//!     |_app| element! { Text(content: format!("Count: {}", state.borrow())) },
//!     |_app, key| {
//!         if key.is_char(' ') {
//!             *state.borrow_mut() += 1;
//!         }
//!     },
//! )?;
//! ```
//!
//! # Static Rendering
//!
//! For one-shot rendering without interactivity, use [`Blaeck`] directly:
//!
//! ```ignore
//! use blaeck::prelude::*;
//! use blaeck::Blaeck;
//!
//! let mut blaeck = Blaeck::new(std::io::stdout())?;
//! blaeck.render(element! {
//!     Box(border_style: BorderStyle::Round, padding: 1.0) {
//!         Text(content: "Hello, Blaeck!", color: Color::Green)
//!     }
//! })?;
//! blaeck.unmount()?;
//! ```
//!
//! # Architecture
//!
//! See `ARCHITECTURE.md` in the repository root for the full mental model.
//!
//! **Data flow**: `element! macro` → Element tree → Layout (Taffy) → Output grid → Terminal
//!
//! **Key modules** (read in this order):
//! - [`reactive`] — Signals-based reactive system (start here for new apps)
//! - [`renderer`] — Main render engine, orchestrates everything
//! - [`element`] — Element tree and Component trait
//! - [`log_update`] — Ink-style inline rendering (the clever bit)
//! - [`output`] — Virtual 2D grid for styled characters
//! - [`layout`] — Flexbox, CSS Grid, and positioning via Taffy
//!
//! # Where to Start
//!
//! - **New to Blaeck**: Start with `examples/reactive_counter.rs`
//! - **Interactive apps**: See `examples/reactive_list.rs` for multiple signals
//! - **Understanding internals**: Read `ARCHITECTURE.md`, then `renderer.rs`
//! - **Adding components**: Study `components/box_component.rs`
//!
//! # Async Support
//!
//! Enable the `async` feature for tokio-based async runtime:
//!
//! ```toml
//! blaeck = { version = "0.1", features = ["async"] }
//! ```
//!
//! This provides [`async_runtime::AsyncApp`] for apps that need to integrate
//! with async operations like D-Bus, HTTP requests, or other I/O.

pub mod animation;
pub mod app;
pub mod buffer;
pub mod components;
pub mod element;
pub mod focus;
pub mod input;
pub mod layout;
pub mod log_update;
pub mod output;
pub mod reactive;
pub mod renderer;
pub mod style;

#[cfg(feature = "async")]
pub mod async_runtime;

pub use animation::{lerp_rgb, lerp_u8, AnimationTimer, BlinkPattern, Easing, IndicatorStyle};
pub use app::{App, AppConfig, AppResult, ExitReason};
pub use buffer::{Buffer, Cell};
pub use components::{
    alert, animated_indicator, animated_indicator_colored, badge, badge_bracket, bar_chart,
    bar_chart_with_values, blink, blink_or, blink_pattern, blinking_dot, breadcrumbs,
    breadcrumbs_path, checkbox, confirm_modal, confirm_prompt, countdown,
    countdown_with_thresholds, diff_lines, divider, divider_with_label, error_modal, flex_spacer,
    git_branch, gradient, gradient_preset, icons, key_hints, link, link_url, log_box,
    markdown_block, progress_bar, progress_bar_bracketed, pulsing_dot, spacer, sparkline,
    sparkline_labeled, spinner_frame, spinner_frame_interval, status_error, status_ok,
    status_warning, stopwatch, success_modal, syntax_highlight, syntax_highlight_with_lines,
    timer_display, transforms, tree_view, Autocomplete, AutocompleteItem, AutocompleteProps,
    AutocompleteState, Badge, BadgeProps, BadgeStyle, BarChart, BarChartProps, BarData, BarStyle,
    BorderChars, BorderColors, BorderSides, BorderStyle, Box, BoxProps, BreadcrumbSeparator,
    Breadcrumbs, BreadcrumbsProps, CellAlign, Checkbox, CheckboxProps, CheckboxStyle, ColorStop,
    ColumnWidth, Confirm, ConfirmProps, ConfirmStyle, Crumb, Diff, DiffLine, DiffLineType,
    DiffProps, DiffStyle, Divider, DividerProps, DividerStyle, FilterMode, Gradient,
    GradientPreset, GradientProps, Indent, IndentProps, KeyHint, KeyHintSeparator, KeyHintStyle,
    KeyHints, KeyHintsProps, LineNumberStyle, Link, LinkProps, LogBox, LogBoxProps, LogLine,
    Markdown, MarkdownProps, Modal, ModalButton, ModalProps, ModalStyle, MultiSelect,
    MultiSelectItem, MultiSelectProps, MultiSelectState, MultiSelectStyle, Newline, NewlineProps,
    Progress, ProgressChars, ProgressProps, ProgressStyle, Row, RowStyle, Select, SelectIndicator,
    SelectItem, SelectProps, SelectState, Spacer, SpacerProps, Sparkline, SparklineProps,
    SparklineStyle, Spinner, SpinnerProps, SpinnerStyle, Static, StaticItem, StaticProps,
    StatusBar, StatusBarProps, StatusSegment, StatusSeparator, SyntaxHighlight,
    SyntaxHighlightProps, SyntaxTheme, Tab, TabDivider, TabStyle, Table, TableCell, TableProps,
    TableState, Tabs, TabsProps, TabsState, Text, TextInput, TextInputProps, TextInputState,
    TextProps, TextWrap, TimeFormat, Timer, TimerMode, TimerProps, Transform, TransformFn,
    TransformProps, TreeConnectors, TreeNode, TreeState, TreeStyle, TreeView, TreeViewProps,
    ValueFormat,
};
pub use element::{Component, Element};
pub use focus::{FocusCallback, FocusEvent, FocusId, FocusManager, FocusState};
pub use input::{match_key, poll_key, read_key, Arrow, InputHandler, Key, KeyMatcher};
pub use layout::{
    AlignContent, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, GridAutoFlow,
    GridPlacement, JustifyContent, LayoutResult, LayoutStyle, LayoutTree, Overflow, Position,
    TrackSize,
};
pub use log_update::LogUpdate;
pub use output::{Output, OutputResult};
pub use renderer::Blaeck;
pub use style::{Color, Modifier, Style};

#[cfg(feature = "async")]
pub use async_runtime::{
    channel, poll_key_async, read_key_async, AppEvent, AsyncApp, AsyncAppConfig, Receiver, Sender,
};

/// Re-export the element! macro from blaeck-macros.
pub use blaeck_macros::element;

/// Print an element to stdout and return.
///
/// This is the simplest way to render something with Blaeck — great for
/// one-shot output like status messages or formatted results.
///
/// # Example
///
/// ```ignore
/// use blaeck::prelude::*;
///
/// fn main() -> std::io::Result<()> {
///     blaeck::print(element! {
///         Text(content: "Hello!", color: Color::Green, bold: true)
///     })
/// }
/// ```
///
/// For interactive apps that respond to keyboard input, use [`reactive::ReactiveApp`].
pub fn print(element: Element) -> std::io::Result<()> {
    let mut blaeck = Blaeck::new(std::io::stdout())?;
    blaeck.render(element)?;
    blaeck.unmount()
}

/// Prelude module with commonly used types.
pub mod prelude {
    pub use crate::animation::{AnimationTimer, BlinkPattern, Easing, IndicatorStyle};
    pub use crate::components::{
        alert, animated_indicator, animated_indicator_colored, badge, badge_bracket, bar_chart,
        bar_chart_with_values, blink, blink_or, blink_pattern, blinking_dot, breadcrumbs,
        breadcrumbs_path, checkbox, confirm_modal, confirm_prompt, countdown,
        countdown_with_thresholds, diff_lines, divider, divider_with_label, error_modal,
        flex_spacer, git_branch, gradient, gradient_preset, icons, key_hints, link, link_url,
        log_box, markdown_block, progress_bar, progress_bar_bracketed, pulsing_dot, spacer,
        sparkline, sparkline_labeled, spinner_frame, spinner_frame_interval, status_error,
        status_ok, status_warning, stopwatch, success_modal, syntax_highlight,
        syntax_highlight_with_lines, timer_display, transforms, tree_view, Autocomplete,
        AutocompleteItem, AutocompleteProps, AutocompleteState, Badge, BadgeProps, BadgeStyle,
        BarChart, BarChartProps, BarData, BarStyle, BorderChars, BorderColors, BorderSides,
        BorderStyle, Box, BoxProps, BreadcrumbSeparator, Breadcrumbs, BreadcrumbsProps, CellAlign,
        Checkbox, CheckboxProps, CheckboxStyle, ColorStop, ColumnWidth, Confirm, ConfirmProps,
        ConfirmStyle, Crumb, Diff, DiffLine, DiffLineType, DiffProps, DiffStyle, Divider,
        DividerProps, DividerStyle, FilterMode, Gradient, GradientPreset, GradientProps, Indent,
        IndentProps, KeyHint, KeyHintSeparator, KeyHintStyle, KeyHints, KeyHintsProps,
        LineNumberStyle, Link, LinkProps, LogBox, LogBoxProps, LogLine, Markdown, MarkdownProps,
        Modal, ModalButton, ModalProps, ModalStyle, MultiSelect, MultiSelectItem, MultiSelectProps,
        MultiSelectState, MultiSelectStyle, Newline, NewlineProps, Progress, ProgressChars,
        ProgressProps, ProgressStyle, Row, RowStyle, Select, SelectIndicator, SelectItem,
        SelectProps, SelectState, Spacer, SpacerProps, Sparkline, SparklineProps, SparklineStyle,
        Spinner, SpinnerProps, SpinnerStyle, Static, StaticItem, StaticProps, StatusBar,
        StatusBarProps, StatusSegment, StatusSeparator, SyntaxHighlight, SyntaxHighlightProps,
        SyntaxTheme, Tab, TabDivider, TabStyle, Table, TableCell, TableProps, TableState, Tabs,
        TabsProps, TabsState, Text, TextInput, TextInputProps, TextInputState, TextProps, TextWrap,
        TimeFormat, Timer, TimerMode, TimerProps, Transform, TransformFn, TransformProps,
        TreeConnectors, TreeNode, TreeState, TreeStyle, TreeView, TreeViewProps, ValueFormat,
    };
    pub use crate::element::{Component, Element};
    pub use crate::layout::{
        AlignContent, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, GridAutoFlow,
        GridPlacement, JustifyContent, LayoutResult, LayoutStyle, Overflow, Position, TrackSize,
    };
    pub use crate::renderer::Blaeck;
    pub use crate::style::{Color, Modifier, Style};
    pub use blaeck_macros::element;

    #[cfg(feature = "async")]
    pub use crate::async_runtime::{channel, AppEvent, AsyncApp, AsyncAppConfig, Receiver, Sender};
}

#[cfg(test)]
mod macro_tests {
    use crate::prelude::*;

    #[test]
    fn test_macro_simple_text() {
        let elem = element! {
            Text(content: "Hello")
        };
        assert!(elem.is_node());
    }

    #[test]
    fn test_macro_text_with_props() {
        let elem = element! {
            Text(content: "Styled", color: Color::Red, bold: true)
        };
        assert!(elem.is_node());
    }

    #[test]
    fn test_macro_box_empty() {
        let elem = element! {
            Box
        };
        assert!(elem.is_node());
    }

    #[test]
    fn test_macro_box_with_props() {
        let elem = element! {
            Box(flex_direction: FlexDirection::Row, padding: 2.0)
        };
        assert!(elem.is_node());
    }

    #[test]
    fn test_macro_nested() {
        let elem = element! {
            Box {
                Text(content: "A")
                Text(content: "B")
            }
        };
        assert!(elem.is_node());
        assert_eq!(elem.children().len(), 2);
    }

    #[test]
    fn test_macro_deeply_nested() {
        let elem = element! {
            Box(flex_direction: FlexDirection::Column) {
                Box(flex_direction: FlexDirection::Row) {
                    Text(content: "Left")
                    Spacer
                    Text(content: "Right")
                }
                Text(content: "Bottom")
            }
        };
        assert!(elem.is_node());
        assert_eq!(elem.children().len(), 2);
    }

    #[test]
    fn test_macro_spacer() {
        let elem = element! {
            Spacer
        };
        assert!(elem.is_node());
    }

    #[test]
    fn test_macro_box_with_border() {
        let elem = element! {
            Box(border_style: BorderStyle::Single, padding: 1.0) {
                Text(content: "Bordered")
            }
        };
        assert!(elem.is_node());
        assert_eq!(elem.children().len(), 1);
    }

    #[test]
    fn test_macro_with_string_expression() {
        let message = "Dynamic".to_string();
        let elem = element! {
            Text(content: message)
        };
        assert!(elem.is_node());
    }

    #[test]
    fn test_macro_with_format() {
        let count = 42;
        let elem = element! {
            Text(content: format!("Count: {}", count))
        };
        assert!(elem.is_node());
    }
}
