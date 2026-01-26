//! Blaeck renderer for terminal UIs.
//!
//! **Start here when reading the codebase.** This is the main entry point.
//!
//! The `Blaeck` struct orchestrates the entire render pipeline:
//! 1. Takes an Element tree (from `element!` macro)
//! 2. Builds a layout tree and computes positions via Taffy
//! 3. Renders each element to a virtual 2D grid (`Output`)
//! 4. Converts the grid to an ANSI string
//! 5. Writes to terminal via `LogUpdate` (which handles cursor movement and line erasure)
//!
//! Key methods:
//! - `Blaeck::new(writer)` — create a renderer
//! - `blaeck.render(element)` — render an element tree
//! - `blaeck.unmount()` — finalize and leave output visible
//!
//! See `ARCHITECTURE.md` for the full mental model.

use crate::components::{
    Autocomplete, Badge, BarChart, BoxProps, Breadcrumbs, Checkbox, Confirm, Diff, Divider,
    Gradient, KeyHints, Link, LogBox, Markdown, Modal, MultiSelect, Progress, Select, Sparkline,
    Spinner, StatusBar, SyntaxHighlight, Table, Tabs, TextInput, Timer, TreeView,
};
use crate::element::Element;
use crate::layout::{LayoutStyle, LayoutTree};
use crate::log_update::LogUpdate;
use crate::output::Output;
use crate::style::{Color, Style};
use std::any::TypeId;
use std::collections::HashMap;
use std::io::Write;
use std::time::{Duration, Instant};
use taffy::NodeId;

/// Result type for Blaeck operations.
pub type Result<T> = std::io::Result<T>;

/// Strip ANSI and OSC escape sequences from a string for width calculation.
/// This handles both standard ANSI escapes (\x1b[...m) and OSC 8 hyperlinks (\x1b]8;;...\x07).
fn strip_ansi_escapes(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Check next char
            match chars.peek() {
                Some('[') => {
                    // Standard ANSI escape: \x1b[...m
                    chars.next(); // consume '['
                    while let Some(&ch) = chars.peek() {
                        chars.next();
                        if ch == 'm' {
                            break;
                        }
                    }
                }
                Some(']') => {
                    // OSC escape: \x1b]...\x07 or \x1b]...\x1b\\
                    chars.next(); // consume ']'
                    while let Some(ch) = chars.next() {
                        if ch == '\x07' {
                            break;
                        }
                        if ch == '\x1b' && chars.peek() == Some(&'\\') {
                            chars.next();
                            break;
                        }
                    }
                }
                _ => {
                    // Unknown escape, skip just the ESC
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// The main Blaeck renderer that manages terminal output.
///
/// Blaeck provides inline terminal rendering - it tracks what was previously rendered,
/// erases it, and redraws. This is different from fullscreen TUI libraries.
///
/// # Render Throttling
///
/// By default, every call to `render()` updates the terminal. For animated UIs,
/// you can enable throttling to limit the frame rate:
///
/// ```ignore
/// let mut blaeck = Blaeck::new(io::stdout())?;
/// blaeck.set_max_fps(30);  // Limit to 30 FPS
///
/// loop {
///     blaeck.render(ui)?;  // Skipped if called too soon
/// }
/// ```
pub struct Blaeck<W: Write> {
    log_update: LogUpdate<W>,
    width: u16,
    height: u16,
    static_output: String,
    /// Minimum duration between renders (for throttling)
    min_render_interval: Option<Duration>,
    /// Last time a render was performed
    last_render: Option<Instant>,
    /// Reusable layout tree to avoid memory growth from Taffy allocations
    layout_tree: LayoutTree,
}

impl<W: Write> Blaeck<W> {
    /// Creates a new Blaeck instance with the given writer.
    ///
    /// The terminal width is queried, falling back to 80 columns if not available.
    pub fn new(writer: W) -> Result<Self> {
        // Try to get terminal size, fall back to 80x24
        let (width, height) = crossterm::terminal::size().unwrap_or((80, 24));
        Self::with_size(writer, width, height)
    }

    /// Creates a new Blaeck instance with explicit dimensions.
    pub fn with_size(writer: W, width: u16, height: u16) -> Result<Self> {
        Ok(Self {
            log_update: LogUpdate::new(writer),
            width,
            height,
            static_output: String::new(),
            min_render_interval: None,
            last_render: None,
            layout_tree: LayoutTree::new(),
        })
    }

    /// Sets the maximum frames per second for rendering.
    ///
    /// When set, calls to `render()` that occur faster than this rate
    /// will be skipped. This prevents excessive CPU usage and terminal
    /// flicker for animated UIs.
    ///
    /// Common values:
    /// - 60 FPS: Smooth animations (16.6ms between frames)
    /// - 30 FPS: Good balance of smoothness and efficiency (33.3ms)
    /// - 15 FPS: Low CPU usage, still responsive (66.6ms)
    ///
    /// Pass `None` or `0` to disable throttling.
    pub fn set_max_fps(&mut self, fps: u32) {
        if fps == 0 {
            self.min_render_interval = None;
        } else {
            self.min_render_interval = Some(Duration::from_nanos(1_000_000_000 / fps as u64));
        }
    }

    /// Sets the minimum interval between renders.
    ///
    /// This is an alternative to `set_max_fps()` for direct control
    /// over the throttle duration.
    pub fn set_throttle(&mut self, interval: Option<Duration>) {
        self.min_render_interval = interval;
    }

    /// Returns whether this render would be throttled (skipped).
    ///
    /// Useful if you want to skip expensive state updates when
    /// the render would be throttled anyway.
    pub fn would_throttle(&self) -> bool {
        if let (Some(interval), Some(last)) = (self.min_render_interval, self.last_render) {
            last.elapsed() < interval
        } else {
            false
        }
    }

    /// Gets the current terminal width.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Gets the current terminal height.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Renders an element tree to the terminal.
    ///
    /// This computes the layout, renders to a virtual output buffer,
    /// and then updates the terminal using LogUpdate.
    ///
    /// If throttling is enabled via `set_max_fps()` or `set_throttle()`,
    /// this method will skip rendering if called too soon after the last
    /// render. Use `render_force()` to bypass throttling.
    pub fn render(&mut self, element: Element) -> Result<()> {
        // Check throttling
        if let (Some(interval), Some(last)) = (self.min_render_interval, self.last_render) {
            if last.elapsed() < interval {
                return Ok(()); // Skip this render
            }
        }

        self.render_force(element)
    }

    /// Renders an element tree, bypassing any throttling.
    ///
    /// Use this when you need to force a render regardless of throttling,
    /// such as for the final render before unmounting.
    pub fn render_force(&mut self, element: Element) -> Result<()> {
        // Update last render time
        self.last_render = Some(Instant::now());

        // Check for Static content
        let (static_content, has_static) = self.check_for_static(&element);

        // If there's new static content, append it
        if has_static && !static_content.is_empty() {
            // Clear current output, write static, then continue
            self.log_update.clear()?;
            self.static_output.push_str(&static_content);
            self.static_output.push('\n');
            // Write static output directly (it scrolls up)
            self.log_update.render(&self.static_output)?;
            self.log_update.done()?;
        }

        // Render the element
        let rendered = self.render_element(&element)?;
        self.log_update.render(&rendered)?;

        Ok(())
    }

    /// Renders an element tree and returns the string output.
    fn render_element(&mut self, element: &Element) -> Result<String> {
        // Reuse layout tree's memory. If tree has grown very large, recreate it
        // to release memory (prevents unbounded growth from varying tree sizes)
        let mut layout_tree = std::mem::take(&mut self.layout_tree);
        layout_tree.clear();

        let mut node_elements: HashMap<NodeId, &Element> = HashMap::new();

        // Create layout tree recursively
        let root_node = self.build_layout_tree(&mut layout_tree, element, &mut node_elements)?;

        // Compute layout
        layout_tree.compute(root_node, self.width as f32, self.height as f32);

        // Calculate total height needed
        let root_layout = layout_tree.get_layout(root_node);
        let output_height = (root_layout.height.ceil() as u16).max(1);

        // Create output buffer
        let mut output = Output::new(self.width, output_height);

        // Render each element to the output buffer using Taffy's computed layout
        self.render_node(
            &mut output,
            &layout_tree,
            root_node,
            0.0,
            0.0,
            &node_elements,
        )?;

        // Put the layout tree back for reuse
        self.layout_tree = layout_tree;

        let result = output.get();
        Ok(result.output)
    }

    /// Builds a Taffy layout tree from an element tree.
    ///
    /// ## Why a separate layout tree?
    ///
    /// Taffy (the flexbox engine) has its own tree structure with nodes identified
    /// by `NodeId`. Our `Element` tree has a different structure (enum variants,
    /// type-erased components). We can't use Elements directly with Taffy.
    ///
    /// So we walk the Element tree and build a parallel Taffy tree:
    /// - Each Element becomes a Taffy node with computed `LayoutStyle`
    /// - We store a mapping `NodeId -> &Element` to render later
    /// - Components are "expanded" (their render() called) to get actual content
    ///
    /// After Taffy computes layout, we walk both trees together:
    /// the Taffy tree gives us positions, the Element tree gives us content.
    #[allow(clippy::only_used_in_recursion)]
    fn build_layout_tree<'a>(
        &self,
        tree: &mut LayoutTree,
        element: &'a Element,
        node_elements: &mut HashMap<NodeId, &'a Element>,
    ) -> Result<NodeId> {
        match element {
            Element::Empty => {
                let node = tree.new_leaf(LayoutStyle::default()).map_err(to_io_error)?;
                node_elements.insert(node, element);
                Ok(node)
            }
            Element::Text { content, .. } => {
                // Text elements take up width based on the widest line
                let text_width = content
                    .lines()
                    .map(|line| unicode_width::UnicodeWidthStr::width(line) as f32)
                    .fold(0.0_f32, |a, b| a.max(b));
                let lines = content.lines().count().max(1);
                let style = LayoutStyle {
                    width: Some(text_width),
                    height: Some(lines as f32),
                    ..Default::default()
                };
                let node = tree.new_leaf(style).map_err(to_io_error)?;
                node_elements.insert(node, element);
                Ok(node)
            }
            Element::Fragment(children) => {
                // Fragment: create a container node with all children laid out horizontally
                let mut child_nodes = Vec::new();
                for child in children {
                    let child_node = self.build_layout_tree(tree, child, node_elements)?;
                    child_nodes.push(child_node);
                }
                let style = LayoutStyle {
                    flex_direction: crate::layout::FlexDirection::Row,
                    ..Default::default()
                };
                let node = tree
                    .new_with_children(style, &child_nodes)
                    .map_err(to_io_error)?;
                node_elements.insert(node, element);
                Ok(node)
            }
            Element::Node {
                type_id,
                props,
                children,
                layout_style,
                render_fn,
                ..
            } => {
                // Handle leaf components that render to Text
                if *type_id == TypeId::of::<crate::components::Text>()
                    || *type_id == TypeId::of::<Spinner>()
                    || *type_id == TypeId::of::<Progress>()
                    || *type_id == TypeId::of::<TextInput>()
                    || *type_id == TypeId::of::<Checkbox>()
                    || *type_id == TypeId::of::<Select>()
                    || *type_id == TypeId::of::<Confirm>()
                    || *type_id == TypeId::of::<Divider>()
                    || *type_id == TypeId::of::<Badge>()
                    || *type_id == TypeId::of::<Link>()
                    || *type_id == TypeId::of::<Table>()
                    || *type_id == TypeId::of::<Tabs>()
                    || *type_id == TypeId::of::<Autocomplete>()
                    || *type_id == TypeId::of::<MultiSelect>()
                    || *type_id == TypeId::of::<Sparkline>()
                    || *type_id == TypeId::of::<KeyHints>()
                    || *type_id == TypeId::of::<Gradient>()
                    || *type_id == TypeId::of::<Breadcrumbs>()
                    || *type_id == TypeId::of::<StatusBar>()
                    || *type_id == TypeId::of::<Diff>()
                    || *type_id == TypeId::of::<Markdown>()
                    || *type_id == TypeId::of::<LogBox>()
                    || *type_id == TypeId::of::<Timer>()
                    || *type_id == TypeId::of::<TreeView>()
                    || *type_id == TypeId::of::<BarChart>()
                    || *type_id == TypeId::of::<SyntaxHighlight>()
                    || *type_id == TypeId::of::<Modal>()
                    || *type_id == TypeId::of::<crate::components::Spacer>()
                {
                    let rendered = render_fn(props.as_ref());
                    // Handle Fragment (for Gradient/Breadcrumbs/StatusBar/Diff/Markdown/LogBox/TreeView/BarChart/SyntaxHighlight/Modal/Spacer component)
                    if let Element::Fragment(children) = &rendered {
                        // Diff, Markdown, LogBox, TreeView, BarChart, SyntaxHighlight, Modal, Spacer render vertically - each child is a separate line
                        if *type_id == TypeId::of::<Diff>()
                            || *type_id == TypeId::of::<Markdown>()
                            || *type_id == TypeId::of::<LogBox>()
                            || *type_id == TypeId::of::<TreeView>()
                            || *type_id == TypeId::of::<BarChart>()
                            || *type_id == TypeId::of::<SyntaxHighlight>()
                            || *type_id == TypeId::of::<Modal>()
                            || *type_id == TypeId::of::<crate::components::Spacer>()
                        {
                            let mut max_width: f32 = 0.0;
                            for child in children {
                                match child {
                                    Element::Text { content, .. } => {
                                        let stripped = strip_ansi_escapes(content);
                                        let w = unicode_width::UnicodeWidthStr::width(
                                            stripped.as_str(),
                                        ) as f32;
                                        max_width = max_width.max(w);
                                    }
                                    Element::Fragment(inline_children) => {
                                        // Inline fragment - calculate combined width
                                        let mut line_width: f32 = 0.0;
                                        for inline_child in inline_children {
                                            if let Element::Text { content, .. } = inline_child {
                                                let stripped = strip_ansi_escapes(content);
                                                line_width += unicode_width::UnicodeWidthStr::width(
                                                    stripped.as_str(),
                                                )
                                                    as f32;
                                            }
                                        }
                                        max_width = max_width.max(line_width);
                                    }
                                    _ => {}
                                }
                            }
                            let style = LayoutStyle {
                                width: Some(max_width),
                                height: Some(children.len() as f32),
                                ..Default::default()
                            };
                            let node = tree.new_leaf(style).map_err(to_io_error)?;
                            node_elements.insert(node, element);
                            return Ok(node);
                        }
                        // Other fragments render horizontally - concatenate width
                        let mut total_content = String::new();
                        for child in children {
                            if let Element::Text { content, .. } = child {
                                total_content.push_str(content);
                            }
                        }
                        let text_width = total_content
                            .lines()
                            .map(|line| {
                                let stripped = strip_ansi_escapes(line);
                                unicode_width::UnicodeWidthStr::width(stripped.as_str()) as f32
                            })
                            .fold(0.0_f32, |a, b| a.max(b));
                        let lines = total_content.lines().count().max(1);
                        let style = LayoutStyle {
                            width: Some(text_width),
                            height: Some(lines as f32),
                            ..Default::default()
                        };
                        let node = tree.new_leaf(style).map_err(to_io_error)?;
                        node_elements.insert(node, element);
                        return Ok(node);
                    }
                    if let Element::Text { content, .. } = &rendered {
                        // Width is the widest line, not total string width
                        // Strip escape sequences for accurate width calculation
                        let text_width = content
                            .lines()
                            .map(|line| {
                                let stripped = strip_ansi_escapes(line);
                                unicode_width::UnicodeWidthStr::width(stripped.as_str()) as f32
                            })
                            .fold(0.0_f32, |a, b| a.max(b));
                        let lines = content.lines().count().max(1);
                        let style = LayoutStyle {
                            width: Some(text_width),
                            height: Some(lines as f32),
                            ..Default::default()
                        };
                        let node = tree.new_leaf(style).map_err(to_io_error)?;
                        node_elements.insert(node, element);
                        return Ok(node);
                    }
                }

                // Build child nodes first
                let mut child_nodes = Vec::new();
                for child in children {
                    let child_node = self.build_layout_tree(tree, child, node_elements)?;
                    child_nodes.push(child_node);
                }

                // Get layout style from props if it's a Box
                let style = if *type_id == TypeId::of::<crate::components::Box>() {
                    if let Some(box_props) = props.downcast_ref::<BoxProps>() {
                        box_props.to_layout_style()
                    } else {
                        layout_style.clone()
                    }
                } else if *type_id == TypeId::of::<crate::components::Spacer>() {
                    if let Some(spacer_props) =
                        props.downcast_ref::<crate::components::SpacerProps>()
                    {
                        crate::components::Spacer::layout_style(spacer_props)
                    } else {
                        crate::components::Spacer::layout_style(
                            &crate::components::SpacerProps::default(),
                        )
                    }
                } else {
                    layout_style.clone()
                };

                let node = if child_nodes.is_empty() {
                    tree.new_leaf(style).map_err(to_io_error)?
                } else {
                    tree.new_with_children(style, &child_nodes)
                        .map_err(to_io_error)?
                };

                node_elements.insert(node, element);
                Ok(node)
            }
        }
    }

    /// Renders a node and its children using Taffy's computed layout.
    fn render_node(
        &self,
        output: &mut Output,
        layout_tree: &LayoutTree,
        node: NodeId,
        parent_x: f32,
        parent_y: f32,
        node_elements: &HashMap<NodeId, &Element>,
    ) -> Result<()> {
        let element = match node_elements.get(&node) {
            Some(e) => *e,
            None => return Ok(()),
        };

        let layout = layout_tree.get_layout(node);
        let x = parent_x + layout.x;
        let y = parent_y + layout.y;

        match element {
            Element::Empty => {}
            Element::Text { content, style } => {
                output.write(x as u16, y as u16, content, *style);
            }
            Element::Fragment(_) => {
                // Fragment children are rendered through the layout tree
                let child_nodes = layout_tree.children(node);
                for child_node in child_nodes {
                    self.render_node(output, layout_tree, child_node, x, y, node_elements)?;
                }
            }
            Element::Node {
                type_id,
                props,
                render_fn,
                ..
            } => {
                // Handle leaf components that render to Text
                if *type_id == TypeId::of::<crate::components::Text>()
                    || *type_id == TypeId::of::<Spinner>()
                    || *type_id == TypeId::of::<Progress>()
                    || *type_id == TypeId::of::<TextInput>()
                    || *type_id == TypeId::of::<Checkbox>()
                    || *type_id == TypeId::of::<Select>()
                    || *type_id == TypeId::of::<Confirm>()
                    || *type_id == TypeId::of::<Divider>()
                    || *type_id == TypeId::of::<Badge>()
                    || *type_id == TypeId::of::<Link>()
                    || *type_id == TypeId::of::<Table>()
                    || *type_id == TypeId::of::<Tabs>()
                    || *type_id == TypeId::of::<Autocomplete>()
                    || *type_id == TypeId::of::<MultiSelect>()
                    || *type_id == TypeId::of::<Sparkline>()
                    || *type_id == TypeId::of::<KeyHints>()
                    || *type_id == TypeId::of::<Gradient>()
                    || *type_id == TypeId::of::<Breadcrumbs>()
                    || *type_id == TypeId::of::<StatusBar>()
                    || *type_id == TypeId::of::<Diff>()
                    || *type_id == TypeId::of::<Markdown>()
                    || *type_id == TypeId::of::<LogBox>()
                    || *type_id == TypeId::of::<Timer>()
                    || *type_id == TypeId::of::<TreeView>()
                    || *type_id == TypeId::of::<BarChart>()
                    || *type_id == TypeId::of::<SyntaxHighlight>()
                    || *type_id == TypeId::of::<Modal>()
                    || *type_id == TypeId::of::<crate::components::Spacer>()
                {
                    let rendered = render_fn(props.as_ref());
                    // Handle Fragment (for Gradient/Breadcrumbs/StatusBar/Diff/Markdown/LogBox/TreeView/BarChart/SyntaxHighlight/Modal/Spacer component)
                    if let Element::Fragment(children) = &rendered {
                        // Diff, Markdown, LogBox, TreeView, BarChart, SyntaxHighlight, Modal, Spacer render vertically (each line on new row)
                        if *type_id == TypeId::of::<Diff>()
                            || *type_id == TypeId::of::<Markdown>()
                            || *type_id == TypeId::of::<LogBox>()
                            || *type_id == TypeId::of::<TreeView>()
                            || *type_id == TypeId::of::<BarChart>()
                            || *type_id == TypeId::of::<SyntaxHighlight>()
                            || *type_id == TypeId::of::<Modal>()
                            || *type_id == TypeId::of::<crate::components::Spacer>()
                        {
                            let mut line_y = y as u16;
                            for child in children {
                                match child {
                                    Element::Text { content, style } => {
                                        output.write(x as u16, line_y, content, *style);
                                        line_y += 1;
                                    }
                                    Element::Fragment(inline_children) => {
                                        // Inline fragment within a line - render horizontally
                                        let mut char_x = x as u16;
                                        for inline_child in inline_children {
                                            if let Element::Text { content, style } = inline_child {
                                                let stripped = strip_ansi_escapes(content);
                                                output.write(char_x, line_y, content, *style);
                                                let char_width =
                                                    unicode_width::UnicodeWidthStr::width(
                                                        stripped.as_str(),
                                                    );
                                                char_x += char_width as u16;
                                            }
                                        }
                                        line_y += 1;
                                    }
                                    _ => {}
                                }
                            }
                            return Ok(());
                        }
                        // Other components render horizontally
                        let mut char_x = x as u16;
                        for child in children {
                            if let Element::Text { content, style } = child {
                                output.write(char_x, y as u16, content, *style);
                                let char_width =
                                    unicode_width::UnicodeWidthStr::width(content.as_str());
                                char_x += char_width as u16;
                            }
                        }
                        return Ok(());
                    }
                    if let Element::Text { content, style } = &rendered {
                        output.write(x as u16, y as u16, content, *style);
                    }
                    return Ok(());
                }

                // Handle Box with border - use Taffy's computed size
                if *type_id == TypeId::of::<crate::components::Box>() {
                    if let Some(box_props) = props.downcast_ref::<BoxProps>() {
                        // If box is hidden, skip rendering but preserve layout space
                        if !box_props.visible {
                            return Ok(());
                        }
                        self.render_box(output, box_props, x, y, layout.width, layout.height);
                    }
                }

                // Render children using Taffy's computed layout
                let child_nodes = layout_tree.children(node);
                for child_node in child_nodes {
                    self.render_node(output, layout_tree, child_node, x, y, node_elements)?;
                }
            }
        }

        Ok(())
    }

    /// Renders a box border with per-side colors and visibility.
    fn render_box(
        &self,
        output: &mut Output,
        props: &BoxProps,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        // If box is hidden, skip rendering (but layout space is preserved)
        if !props.visible {
            return;
        }

        if !props.border_style.has_border() {
            return;
        }

        let chars = props.border_style.chars();
        let sides = props.effective_border_sides();

        // Get per-side colors, applying dim modifier if requested
        let make_style = |color: Option<Color>| {
            let mut style = color.map(|c| Style::new().fg(c)).unwrap_or_default();
            if props.border_dim {
                style = style.dim();
            }
            style
        };

        let top_style = make_style(props.top_border_color());
        let bottom_style = make_style(props.bottom_border_color());
        let left_style = make_style(props.left_border_color());
        let right_style = make_style(props.right_border_color());

        let x = x as u16;
        let y = y as u16;
        let width = width as u16;
        let height = height as u16;

        if width < 2 || height < 2 {
            return;
        }

        // Determine corner characters based on which sides are visible
        let top_left_char = if sides.top && sides.left {
            chars.top_left
        } else if sides.top {
            chars.horizontal
        } else if sides.left {
            chars.vertical
        } else {
            ' '
        };

        let top_right_char = if sides.top && sides.right {
            chars.top_right
        } else if sides.top {
            chars.horizontal
        } else if sides.right {
            chars.vertical
        } else {
            ' '
        };

        let bottom_left_char = if sides.bottom && sides.left {
            chars.bottom_left
        } else if sides.bottom {
            chars.horizontal
        } else if sides.left {
            chars.vertical
        } else {
            ' '
        };

        let bottom_right_char = if sides.bottom && sides.right {
            chars.bottom_right
        } else if sides.bottom {
            chars.horizontal
        } else if sides.right {
            chars.vertical
        } else {
            ' '
        };

        // Top border
        if sides.top {
            // Top-left corner (use top color for corners when top is visible)
            if sides.left || sides.top {
                output.write(x, y, &top_left_char.to_string(), top_style);
            }

            // Top horizontal line
            let top_line = chars.horizontal.to_string().repeat((width - 2) as usize);
            output.write(x + 1, y, &top_line, top_style);

            // Top-right corner
            if sides.right || sides.top {
                output.write(x + width - 1, y, &top_right_char.to_string(), top_style);
            }
        }

        // Side borders
        for row in 1..(height - 1) {
            if sides.left {
                output.write(x, y + row, &chars.vertical.to_string(), left_style);
            }
            if sides.right {
                output.write(
                    x + width - 1,
                    y + row,
                    &chars.vertical.to_string(),
                    right_style,
                );
            }
        }

        // Bottom border
        if sides.bottom {
            // Bottom-left corner
            if sides.left || sides.bottom {
                output.write(
                    x,
                    y + height - 1,
                    &bottom_left_char.to_string(),
                    bottom_style,
                );
            }

            // Bottom horizontal line
            let bottom_line = chars.horizontal.to_string().repeat((width - 2) as usize);
            output.write(x + 1, y + height - 1, &bottom_line, bottom_style);

            // Bottom-right corner
            if sides.right || sides.bottom {
                output.write(
                    x + width - 1,
                    y + height - 1,
                    &bottom_right_char.to_string(),
                    bottom_style,
                );
            }
        }

        // Draw left side corners when only left is visible (no top/bottom)
        if sides.left && !sides.top {
            output.write(x, y, &top_left_char.to_string(), left_style);
        }
        if sides.left && !sides.bottom {
            output.write(x, y + height - 1, &bottom_left_char.to_string(), left_style);
        }

        // Draw right side corners when only right is visible (no top/bottom)
        if sides.right && !sides.top {
            output.write(x + width - 1, y, &top_right_char.to_string(), right_style);
        }
        if sides.right && !sides.bottom {
            output.write(
                x + width - 1,
                y + height - 1,
                &bottom_right_char.to_string(),
                right_style,
            );
        }
    }

    /// Checks if the element is a Static component and returns its rendered content.
    /// Returns (static_content, has_static).
    fn check_for_static(&mut self, element: &Element) -> (String, bool) {
        match element {
            Element::Node {
                type_id, children, ..
            } => {
                // Check if this is a Static component
                if *type_id == TypeId::of::<Static>() {
                    let static_content = self.render_element(element).unwrap_or_default();
                    return (static_content, true);
                }

                // Check children for Static components
                let mut static_parts = Vec::new();

                for child in children {
                    if let Element::Node { type_id, .. } = child {
                        if *type_id == TypeId::of::<Static>() {
                            if let Ok(content) = self.render_element(child) {
                                static_parts.push(content);
                            }
                        }
                    }
                }

                if static_parts.is_empty() {
                    (String::new(), false)
                } else {
                    (static_parts.join("\n"), true)
                }
            }
            _ => (String::new(), false),
        }
    }

    /// Finalizes rendering, leaving the current output visible.
    ///
    /// After calling unmount(), subsequent renders will write below the current
    /// content instead of replacing it.
    pub fn unmount(&mut self) -> Result<()> {
        self.log_update.done()
    }

    /// Clears the current output.
    pub fn clear(&mut self) -> Result<()> {
        self.log_update.clear()
    }

    /// Handle terminal resize event.
    ///
    /// Call this when you receive a resize event from crossterm/termion.
    /// It clears the display and updates the internal dimensions so the
    /// next render uses the new size.
    ///
    /// ```no_run
    /// use crossterm::event::{Event, read};
    ///
    /// // In your event loop:
    /// // if let Event::Resize(w, h) = read()? {
    /// //     blaeck.handle_resize(w, h)?;
    /// // }
    /// ```
    pub fn handle_resize(&mut self, width: u16, height: u16) -> Result<()> {
        self.width = width;
        self.height = height;
        // Clear our content area only, preserving scrollback above
        self.log_update.handle_resize()
    }
}

/// Convert a Taffy error to an io::Error.
fn to_io_error(e: taffy::TaffyError) -> std::io::Error {
    std::io::Error::other(format!("Layout error: {:?}", e))
}

// We need to import Static for the type checking
use crate::components::r#static::Static;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Box, BoxProps, Spacer, Text, TextProps};
    use crate::layout::FlexDirection;
    use crate::style::Color;

    #[test]
    fn test_blaeck_new() {
        let buf = Vec::new();
        let blaeck = Blaeck::with_size(buf, 80, 24);
        assert!(blaeck.is_ok());
    }

    #[test]
    fn test_blaeck_dimensions() {
        let buf = Vec::new();
        let blaeck = Blaeck::with_size(buf, 80, 24).unwrap();
        assert_eq!(blaeck.width(), 80);
        assert_eq!(blaeck.height(), 24);
    }

    #[test]
    fn test_blaeck_render_text() {
        let mut buf = Vec::new();
        {
            let mut blaeck = Blaeck::with_size(&mut buf, 80, 24).unwrap();
            let elem = Element::node::<Text>(
                TextProps {
                    content: "Hello".into(),
                    ..Default::default()
                },
                vec![],
            );
            blaeck.render(elem).unwrap();
        }

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Hello"));
    }

    #[test]
    fn test_blaeck_render_styled_text() {
        let mut buf = Vec::new();
        {
            let mut blaeck = Blaeck::with_size(&mut buf, 80, 24).unwrap();
            let elem = Element::node::<Text>(
                TextProps {
                    content: "Styled".into(),
                    color: Some(Color::Red),
                    bold: true,
                    ..Default::default()
                },
                vec![],
            );
            blaeck.render(elem).unwrap();
        }

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Styled"));
        // Should have ANSI codes for color
        assert!(output.contains("\x1b["));
    }

    #[test]
    fn test_blaeck_render_box_with_children() {
        let mut buf = Vec::new();
        {
            let mut blaeck = Blaeck::with_size(&mut buf, 80, 24).unwrap();
            let child1 = Element::node::<Text>(
                TextProps {
                    content: "Child1".into(),
                    ..Default::default()
                },
                vec![],
            );
            let child2 = Element::node::<Text>(
                TextProps {
                    content: "Child2".into(),
                    ..Default::default()
                },
                vec![],
            );
            let elem = Element::node::<Box>(BoxProps::default(), vec![child1, child2]);
            blaeck.render(elem).unwrap();
        }

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Child1"));
        assert!(output.contains("Child2"));
    }

    #[test]
    fn test_blaeck_render_box_with_border() {
        let mut buf = Vec::new();
        {
            let mut blaeck = Blaeck::with_size(&mut buf, 80, 24).unwrap();
            let child = Element::node::<Text>(
                TextProps {
                    content: "Bordered".into(),
                    ..Default::default()
                },
                vec![],
            );
            let elem = Element::node::<Box>(
                BoxProps {
                    border_style: crate::components::BorderStyle::Single,
                    width: Some(20.0),
                    height: Some(5.0),
                    ..Default::default()
                },
                vec![child],
            );
            blaeck.render(elem).unwrap();
        }

        let output = String::from_utf8(buf).unwrap();
        // Should have border characters
        assert!(output.contains('┌') || output.contains('─') || output.contains('│'));
        assert!(output.contains("Bordered"));
    }

    #[test]
    fn test_blaeck_rerender() {
        let mut buf = Vec::new();
        {
            let mut blaeck = Blaeck::with_size(&mut buf, 80, 24).unwrap();

            let elem1 = Element::node::<Text>(
                TextProps {
                    content: "First".into(),
                    ..Default::default()
                },
                vec![],
            );
            blaeck.render(elem1).unwrap();

            let elem2 = Element::node::<Text>(
                TextProps {
                    content: "Second".into(),
                    ..Default::default()
                },
                vec![],
            );
            blaeck.render(elem2).unwrap();
        }

        let output = String::from_utf8(buf).unwrap();
        // Should contain erase sequences and second content
        assert!(output.contains("\x1b["));
        assert!(output.contains("Second"));
    }

    #[test]
    fn test_blaeck_unmount() {
        let mut buf = Vec::new();
        {
            let mut blaeck = Blaeck::with_size(&mut buf, 80, 24).unwrap();

            let elem = Element::node::<Text>(
                TextProps {
                    content: "Final".into(),
                    ..Default::default()
                },
                vec![],
            );
            blaeck.render(elem).unwrap();
            blaeck.unmount().unwrap();
        }

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Final"));
    }

    #[test]
    fn test_blaeck_clear() {
        let mut buf = Vec::new();
        {
            let mut blaeck = Blaeck::with_size(&mut buf, 80, 24).unwrap();

            let elem = Element::node::<Text>(
                TextProps {
                    content: "ToBeCleared".into(),
                    ..Default::default()
                },
                vec![],
            );
            blaeck.render(elem).unwrap();
            blaeck.clear().unwrap();
        }

        let output = String::from_utf8(buf).unwrap();
        // Should have erase sequences
        assert!(output.contains("\x1b["));
    }

    #[test]
    fn test_blaeck_render_empty() {
        let mut buf = Vec::new();
        {
            let mut blaeck = Blaeck::with_size(&mut buf, 80, 24).unwrap();
            blaeck.render(Element::Empty).unwrap();
        }

        // Should not panic
        let output = String::from_utf8(buf).unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_blaeck_render_nested_boxes() {
        let mut buf = Vec::new();
        {
            let mut blaeck = Blaeck::with_size(&mut buf, 80, 24).unwrap();

            let inner = Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Left".into(),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: "Right".into(),
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            );

            let outer = Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Top".into(),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    inner,
                    Element::node::<Text>(
                        TextProps {
                            content: "Bottom".into(),
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            );

            blaeck.render(outer).unwrap();
        }

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Top"));
        assert!(output.contains("Left"));
        assert!(output.contains("Right"));
        assert!(output.contains("Bottom"));
    }

    #[test]
    fn test_blaeck_render_with_spacer() {
        let mut buf = Vec::new();
        {
            let mut blaeck = Blaeck::with_size(&mut buf, 80, 24).unwrap();

            let elem = Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: "Left".into(),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Spacer>(crate::components::SpacerProps::default(), vec![]),
                    Element::node::<Text>(
                        TextProps {
                            content: "Right".into(),
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            );

            blaeck.render(elem).unwrap();
        }

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Left"));
        assert!(output.contains("Right"));
    }

    #[test]
    fn test_blaeck_set_max_fps() {
        let buf = Vec::new();
        let mut blaeck = Blaeck::with_size(buf, 80, 24).unwrap();

        // Initially no throttling
        assert!(blaeck.min_render_interval.is_none());

        // Set to 30 FPS
        blaeck.set_max_fps(30);
        assert!(blaeck.min_render_interval.is_some());
        let interval = blaeck.min_render_interval.unwrap();
        assert!(interval.as_millis() >= 33 && interval.as_millis() <= 34);

        // Disable with 0
        blaeck.set_max_fps(0);
        assert!(blaeck.min_render_interval.is_none());
    }

    #[test]
    fn test_blaeck_set_throttle() {
        let buf = Vec::new();
        let mut blaeck = Blaeck::with_size(buf, 80, 24).unwrap();

        blaeck.set_throttle(Some(Duration::from_millis(100)));
        assert_eq!(blaeck.min_render_interval, Some(Duration::from_millis(100)));

        blaeck.set_throttle(None);
        assert!(blaeck.min_render_interval.is_none());
    }

    #[test]
    fn test_blaeck_would_throttle() {
        let buf = Vec::new();
        let mut blaeck = Blaeck::with_size(buf, 80, 24).unwrap();

        // No throttling configured - never throttles
        assert!(!blaeck.would_throttle());

        // Configure throttling
        blaeck.set_max_fps(10); // 100ms between frames

        // No render yet - won't throttle
        assert!(!blaeck.would_throttle());

        // Render once
        let elem = Element::text("test");
        blaeck.render(elem).unwrap();

        // Immediately after render - should throttle
        assert!(blaeck.would_throttle());
    }

    #[test]
    fn test_blaeck_throttle_skips_render() {
        let mut buf = Vec::new();
        {
            let mut blaeck = Blaeck::with_size(&mut buf, 80, 24).unwrap();
            blaeck.set_max_fps(10); // 100ms between frames

            // First render goes through
            let elem1 = Element::node::<Text>(
                TextProps {
                    content: "First".into(),
                    ..Default::default()
                },
                vec![],
            );
            blaeck.render(elem1).unwrap();

            // Second render immediately after should be skipped
            let elem2 = Element::node::<Text>(
                TextProps {
                    content: "Second".into(),
                    ..Default::default()
                },
                vec![],
            );
            blaeck.render(elem2).unwrap();
        }

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("First"));
        // Second should be skipped due to throttling
        assert!(!output.contains("Second"));
    }

    #[test]
    fn test_blaeck_render_force_bypasses_throttle() {
        let mut buf = Vec::new();
        {
            let mut blaeck = Blaeck::with_size(&mut buf, 80, 24).unwrap();
            blaeck.set_max_fps(10);

            // First render
            let elem1 = Element::node::<Text>(
                TextProps {
                    content: "First".into(),
                    ..Default::default()
                },
                vec![],
            );
            blaeck.render(elem1).unwrap();

            // Force render bypasses throttle
            let elem2 = Element::node::<Text>(
                TextProps {
                    content: "Forced".into(),
                    ..Default::default()
                },
                vec![],
            );
            blaeck.render_force(elem2).unwrap();
        }

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Forced"));
    }
}
