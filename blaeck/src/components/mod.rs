//! Built-in components for Blaeck.
//!
//! This module provides the standard components: Box, Text, Spacer, Static, Transform,
//! Newline, Indent, Spinner, Progress, TextInput, Checkbox, Select, Confirm, and Autocomplete.

pub mod autocomplete;
pub mod badge;
pub mod barchart;
pub mod blink;
pub mod box_component;
pub mod breadcrumbs;
pub mod checkbox;
pub mod confirm;
pub mod diff;
pub mod divider;
pub mod gradient;
pub mod indent;
pub mod keyhints;
pub mod link;
pub mod logbox;
pub mod markdown;
pub mod modal;
pub mod multiselect;
pub mod newline;
pub mod progress;
pub mod select;
pub mod spacer;
pub mod sparkline;
pub mod spinner;
pub mod r#static;
pub mod statusbar;
pub mod syntax;
pub mod table;
pub mod tabs;
pub mod text;
pub mod text_input;
pub mod timer;
pub mod transform;
pub mod tree;

pub use autocomplete::{
    Autocomplete, AutocompleteItem, AutocompleteProps, AutocompleteState, FilterMode,
};
pub use badge::{badge, badge_bracket, Badge, BadgeProps, BadgeStyle};
pub use barchart::{
    bar_chart, bar_chart_with_values, BarChart, BarChartProps, BarData, BarStyle, ValueFormat,
};
pub use blink::{
    animated_indicator, animated_indicator_colored, blink, blink_or, blink_pattern, blinking_dot,
    pulsing_dot,
};
pub use box_component::{BorderChars, BorderColors, BorderSides, BorderStyle, Box, BoxProps};
pub use breadcrumbs::{
    breadcrumbs, breadcrumbs_path, BreadcrumbSeparator, Breadcrumbs, BreadcrumbsProps, Crumb,
};
pub use checkbox::{checkbox, Checkbox, CheckboxProps, CheckboxStyle};
pub use confirm::{confirm_prompt, Confirm, ConfirmProps, ConfirmStyle};
pub use diff::{diff_lines, Diff, DiffLine, DiffLineType, DiffProps, DiffStyle};
pub use divider::{divider, divider_with_label, Divider, DividerProps, DividerStyle};
pub use gradient::{gradient, gradient_preset, ColorStop, Gradient, GradientPreset, GradientProps};
pub use indent::{Indent, IndentProps};
pub use keyhints::{key_hints, KeyHint, KeyHintSeparator, KeyHintStyle, KeyHints, KeyHintsProps};
pub use link::{link, link_url, Link, LinkProps};
pub use logbox::{log_box, LogBox, LogBoxProps, LogLine, TreeStyle};
pub use markdown::{markdown_block, Markdown, MarkdownProps};
pub use modal::{
    alert, confirm_modal, error_modal, success_modal, Modal, ModalButton, ModalProps, ModalStyle,
};
pub use multiselect::{
    MultiSelect, MultiSelectItem, MultiSelectProps, MultiSelectState, MultiSelectStyle,
};
pub use newline::{Newline, NewlineProps};
pub use progress::{
    progress_bar, progress_bar_bracketed, Progress, ProgressChars, ProgressProps, ProgressStyle,
};
pub use r#static::{Static, StaticItem, StaticProps};
pub use select::{Select, SelectIndicator, SelectItem, SelectProps, SelectState};
pub use spacer::{flex_spacer, spacer, Spacer, SpacerProps};
pub use sparkline::{sparkline, sparkline_labeled, Sparkline, SparklineProps, SparklineStyle};
pub use spinner::{spinner_frame, spinner_frame_interval, Spinner, SpinnerProps, SpinnerStyle};
pub use statusbar::{
    git_branch, icons, status_error, status_ok, status_warning, StatusBar, StatusBarProps,
    StatusSegment, StatusSeparator,
};
pub use syntax::{
    syntax_highlight, syntax_highlight_with_lines, LineNumberStyle, SyntaxHighlight,
    SyntaxHighlightProps, SyntaxTheme,
};
pub use table::{CellAlign, ColumnWidth, Row, RowStyle, Table, TableCell, TableProps, TableState};
pub use tabs::{Tab, TabDivider, TabStyle, Tabs, TabsProps, TabsState};
pub use text::{Text, TextProps, TextWrap};
pub use text_input::{TextInput, TextInputProps, TextInputState};
pub use timer::{
    countdown, countdown_with_thresholds, stopwatch, timer_display, TimeFormat, Timer, TimerMode,
    TimerProps,
};
pub use transform::{transforms, Transform, TransformFn, TransformProps};
pub use tree::{tree_view, TreeConnectors, TreeNode, TreeState, TreeView, TreeViewProps};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::{Component, Element};
    use crate::layout::FlexDirection;
    use crate::style::Color;

    #[test]
    fn test_box_default_props() {
        let props = BoxProps::default();
        assert_eq!(props.flex_direction, FlexDirection::Column);
        assert_eq!(props.border_style, BorderStyle::None);
    }

    #[test]
    fn test_box_props_with_layout() {
        let props = BoxProps {
            flex_direction: FlexDirection::Row,
            padding: 2.0,
            ..Default::default()
        };
        assert_eq!(props.flex_direction, FlexDirection::Row);
        assert_eq!(props.padding, 2.0);
    }

    #[test]
    fn test_text_props() {
        let props = TextProps {
            content: "Hello".into(),
            color: Some(Color::Red),
            bold: true,
            ..Default::default()
        };
        assert_eq!(props.content, "Hello");
        assert!(props.bold);
        assert_eq!(props.color, Some(Color::Red));
    }

    #[test]
    fn test_text_default_props() {
        let props = TextProps::default();
        assert_eq!(props.content, "");
        assert!(!props.bold);
        assert!(!props.italic);
        assert!(!props.underline);
        assert!(!props.dim);
        assert!(props.color.is_none());
        assert!(props.bg_color.is_none());
    }

    #[test]
    fn test_spacer_default() {
        let props = SpacerProps::default();
        // Spacer default has no lines (flex mode)
        assert_eq!(props.lines, 0);
    }

    #[test]
    fn test_border_style_single_chars() {
        let chars = BorderStyle::Single.chars();
        assert_eq!(chars.top_left, '┌');
        assert_eq!(chars.top_right, '┐');
        assert_eq!(chars.bottom_left, '└');
        assert_eq!(chars.bottom_right, '┘');
        assert_eq!(chars.horizontal, '─');
        assert_eq!(chars.vertical, '│');
    }

    #[test]
    fn test_border_style_double_chars() {
        let chars = BorderStyle::Double.chars();
        assert_eq!(chars.top_left, '╔');
        assert_eq!(chars.top_right, '╗');
        assert_eq!(chars.bottom_left, '╚');
        assert_eq!(chars.bottom_right, '╝');
        assert_eq!(chars.horizontal, '═');
        assert_eq!(chars.vertical, '║');
    }

    #[test]
    fn test_border_style_round_chars() {
        let chars = BorderStyle::Round.chars();
        assert_eq!(chars.top_left, '╭');
        assert_eq!(chars.top_right, '╮');
        assert_eq!(chars.bottom_left, '╰');
        assert_eq!(chars.bottom_right, '╯');
        assert_eq!(chars.horizontal, '─');
        assert_eq!(chars.vertical, '│');
    }

    #[test]
    fn test_border_style_bold_chars() {
        let chars = BorderStyle::Bold.chars();
        assert_eq!(chars.top_left, '┏');
        assert_eq!(chars.top_right, '┓');
        assert_eq!(chars.bottom_left, '┗');
        assert_eq!(chars.bottom_right, '┛');
        assert_eq!(chars.horizontal, '━');
        assert_eq!(chars.vertical, '┃');
    }

    #[test]
    fn test_border_style_classic_chars() {
        let chars = BorderStyle::Classic.chars();
        assert_eq!(chars.top_left, '+');
        assert_eq!(chars.top_right, '+');
        assert_eq!(chars.bottom_left, '+');
        assert_eq!(chars.bottom_right, '+');
        assert_eq!(chars.horizontal, '-');
        assert_eq!(chars.vertical, '|');
    }

    #[test]
    fn test_border_style_custom() {
        let custom = BorderChars {
            top_left: 'A',
            top_right: 'B',
            bottom_left: 'C',
            bottom_right: 'D',
            horizontal: 'E',
            vertical: 'F',
        };
        let style = BorderStyle::Custom(custom);
        let chars = style.chars();
        assert_eq!(chars.top_left, 'A');
        assert_eq!(chars.horizontal, 'E');
    }

    #[test]
    fn test_border_style_none_has_no_chars() {
        // BorderStyle::None should not be used to get chars in practice
        // but chars() still returns something (empty chars or a default)
        let style = BorderStyle::None;
        let chars = style.chars();
        assert_eq!(chars.top_left, ' ');
    }

    #[test]
    fn test_box_component_render() {
        let elem = Element::node::<Box>(BoxProps::default(), vec![]);
        assert!(elem.is_node());
    }

    #[test]
    fn test_text_component_render() {
        let props = TextProps {
            content: "Hello".into(),
            ..Default::default()
        };
        let elem = Text::render(&props);
        assert!(elem.is_text());
    }

    #[test]
    fn test_spacer_has_flex_grow() {
        // Spacer render should return an element with flex_grow: 1.0
        let elem = Element::node::<Spacer>(SpacerProps::default(), vec![]);
        // The actual flex_grow is set in the layout_style of the Node
        // We verify the element is created correctly
        assert!(elem.is_node());
        // Verify Spacer's flex layout_style has the expected flex_grow
        let layout = Spacer::layout_style(&SpacerProps::default());
        assert_eq!(layout.flex_grow, 1.0);
    }

    #[test]
    fn test_text_with_style() {
        let props = TextProps {
            content: "Styled".into(),
            color: Some(Color::Green),
            bold: true,
            italic: true,
            ..Default::default()
        };
        let elem = Text::render(&props);
        match &elem {
            Element::Text { content, style } => {
                assert_eq!(content, "Styled");
                assert_eq!(style.fg, Color::Green);
                assert!(style.modifiers.contains(crate::style::Modifier::BOLD));
                assert!(style.modifiers.contains(crate::style::Modifier::ITALIC));
            }
            _ => panic!("Expected Text element"),
        }
    }

    #[test]
    fn test_text_wrap_default() {
        let props = TextProps::default();
        assert_eq!(props.wrap, TextWrap::Wrap);
    }
}
