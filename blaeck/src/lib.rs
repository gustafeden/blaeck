//! Blaeck - A declarative, component-based terminal UI library for Rust.
//!
//! Blaeck provides an Ink-like API for building terminal user interfaces
//! with flexbox layout, inline rendering (not fullscreen), and 35+ components.
//!
//! # Quick Start
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
//! - [`renderer`] — Main render engine, orchestrates everything
//! - [`element`] — Element tree and Component trait
//! - [`log_update`] — Ink-style inline rendering (the clever bit)
//! - [`output`] — Virtual 2D grid for styled characters
//! - [`layout`] — Flexbox via Taffy
//!
//! # Where to Start
//!
//! - **Using Blaeck**: Start with `examples/hello.rs`, then `examples/interactive.rs`
//! - **Understanding internals**: Read `ARCHITECTURE.md`, then `renderer.rs`
//! - **Adding components**: Study `components/box_component.rs` (marked as example)
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
pub mod renderer;
pub mod input;
pub mod layout;
pub mod log_update;
pub mod output;
pub mod style;

#[cfg(feature = "async")]
pub mod async_runtime;

pub use animation::{AnimationTimer, BlinkPattern, Easing, IndicatorStyle, lerp_rgb, lerp_u8};
pub use app::{App, AppConfig, AppResult, ExitReason};
pub use buffer::{Buffer, Cell};
pub use components::{
    animated_indicator, animated_indicator_colored, badge, badge_bracket, bar_chart,
    bar_chart_with_values, blink, blink_or, blink_pattern, blinking_dot, breadcrumbs,
    breadcrumbs_path, checkbox, confirm_prompt, divider, BarChart, BarChartProps, BarData, BarStyle, ValueFormat,
    divider_with_label, gradient, gradient_preset, link, link_url, markdown_block, progress_bar,
    progress_bar_bracketed, pulsing_dot, spinner_frame, spinner_frame_interval, transforms, Autocomplete,
    AutocompleteItem, AutocompleteProps, AutocompleteState, Badge, BadgeProps, BadgeStyle,
    BorderChars, BorderColors, BorderSides, BorderStyle, Box, BoxProps, Breadcrumbs,
    BreadcrumbsProps, BreadcrumbSeparator, Checkbox, CheckboxProps, CheckboxStyle, ColorStop,
    Confirm, ConfirmProps, ConfirmStyle, Crumb, Diff, DiffLine, DiffLineType, DiffProps, DiffStyle,
    diff_lines, Divider, DividerProps, DividerStyle, FilterMode, Gradient, GradientPreset,
    GradientProps, Indent, IndentProps, KeyHint, KeyHints, KeyHintsProps,
    KeyHintSeparator, KeyHintStyle, key_hints, Link, LinkProps, log_box, LogBox, LogBoxProps,
    LogLine, Markdown, MarkdownProps, Modal, ModalButton, ModalProps, ModalStyle, alert, confirm_modal,
    error_modal, success_modal, MultiSelect, MultiSelectItem, MultiSelectProps, TreeStyle,
    MultiSelectState, MultiSelectStyle, Newline,
    NewlineProps, Progress, ProgressChars, ProgressProps, ProgressStyle, Select, SelectIndicator,
    SelectItem, SelectProps, SelectState, spacer, flex_spacer, Spacer, SpacerProps, Sparkline, SparklineProps,
    SparklineStyle, sparkline, sparkline_labeled, Spinner, SpinnerProps, SpinnerStyle, Static,
    SyntaxHighlight, SyntaxHighlightProps, SyntaxTheme, LineNumberStyle, syntax_highlight, syntax_highlight_with_lines,
    StaticItem, StaticProps, StatusBar, StatusBarProps, StatusSegment, StatusSeparator, git_branch,
    icons, status_error, status_ok, status_warning, Table, TableCell, TableProps, TableState,
    CellAlign, ColumnWidth, Row, RowStyle, Tab, TabDivider, Tabs, TabsProps, TabsState, TabStyle,
    Text, TextInput, TextInputProps, TextInputState, TextProps, TextWrap, TimeFormat, Timer,
    TimerMode, TimerProps, countdown, countdown_with_thresholds, stopwatch, timer_display,
    Transform, TransformFn, TransformProps, TreeConnectors, TreeNode, TreeState, TreeView,
    TreeViewProps, tree_view,
};
pub use element::{Component, Element};
pub use focus::{FocusCallback, FocusEvent, FocusId, FocusManager, FocusState};
pub use renderer::Blaeck;
pub use input::{match_key, poll_key, read_key, Arrow, InputHandler, Key, KeyMatcher};
pub use layout::{
    AlignContent, AlignItems, AlignSelf, FlexDirection, JustifyContent, LayoutResult, LayoutStyle, LayoutTree,
};
pub use log_update::LogUpdate;
pub use output::{Output, OutputResult};
pub use style::{Color, Modifier, Style};

#[cfg(feature = "async")]
pub use async_runtime::{
    channel, poll_key_async, read_key_async, AppEvent, AsyncApp, AsyncAppConfig, Receiver, Sender,
};

/// Re-export the element! macro from blaeck-macros.
pub use blaeck_macros::element;

/// Prelude module with commonly used types.
pub mod prelude {
    pub use crate::animation::{AnimationTimer, BlinkPattern, Easing, IndicatorStyle};
    pub use crate::components::{
        animated_indicator, animated_indicator_colored, badge, badge_bracket, bar_chart,
        bar_chart_with_values, blink, blink_or, blink_pattern, blinking_dot, breadcrumbs,
        breadcrumbs_path, checkbox, confirm_prompt, divider, BarChart, BarChartProps, BarData, BarStyle, ValueFormat,
        divider_with_label, gradient, gradient_preset, link, link_url, markdown_block, progress_bar,
        progress_bar_bracketed, pulsing_dot, spinner_frame, spinner_frame_interval, transforms, Autocomplete,
        AutocompleteItem, AutocompleteProps, AutocompleteState, Badge, BadgeProps, BadgeStyle,
        BorderChars, BorderColors, BorderSides, BorderStyle, Box, BoxProps, Breadcrumbs,
        BreadcrumbsProps, BreadcrumbSeparator, Checkbox, CheckboxProps, CheckboxStyle, ColorStop,
        Confirm, ConfirmProps, ConfirmStyle, Crumb, Diff, DiffLine, DiffLineType, DiffProps,
        DiffStyle, diff_lines, Divider, DividerProps, DividerStyle, FilterMode, Gradient,
        GradientPreset, GradientProps, Indent, IndentProps, KeyHint, KeyHints, KeyHintsProps,
        KeyHintSeparator, KeyHintStyle, key_hints, Link, LinkProps, log_box, LogBox, LogBoxProps,
        LogLine, Markdown, MarkdownProps, Modal, ModalButton, ModalProps, ModalStyle, alert,
        confirm_modal, error_modal, success_modal, MultiSelect, MultiSelectItem, MultiSelectProps, TreeStyle,
        MultiSelectState, MultiSelectStyle, Newline,
        NewlineProps, Progress, ProgressChars, ProgressProps, ProgressStyle, Select,
        SelectIndicator, SelectItem, SelectProps, SelectState, spacer, flex_spacer, Spacer, SpacerProps, Sparkline,
        SparklineProps, SparklineStyle, sparkline, sparkline_labeled, Spinner, SpinnerProps,
        SpinnerStyle, Static, StaticItem, StaticProps, StatusBar, StatusBarProps, StatusSegment,
        SyntaxHighlight, SyntaxHighlightProps, SyntaxTheme, LineNumberStyle, syntax_highlight, syntax_highlight_with_lines,
        StatusSeparator, git_branch, icons, status_error, status_ok, status_warning, Table,
        TableCell, TableProps, TableState, CellAlign, ColumnWidth, Row, RowStyle, Tab, TabDivider,
        Tabs, TabsProps, TabsState, TabStyle, Text, TextInput, TextInputProps, TextInputState,
        TextProps, TextWrap, TimeFormat, Timer, TimerMode, TimerProps, countdown,
        countdown_with_thresholds, stopwatch, timer_display, Transform, TransformFn, TransformProps,
        TreeConnectors, TreeNode, TreeState, TreeView, TreeViewProps, tree_view,
    };
    pub use crate::element::{Component, Element};
    pub use crate::renderer::Blaeck;
    pub use crate::layout::{
        AlignContent, AlignItems, AlignSelf, FlexDirection, JustifyContent, LayoutResult, LayoutStyle,
    };
    pub use crate::style::{Color, Modifier, Style};
    pub use blaeck_macros::element;

    #[cfg(feature = "async")]
    pub use crate::async_runtime::{
        channel, AppEvent, AsyncApp, AsyncAppConfig, Receiver, Sender,
    };
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
