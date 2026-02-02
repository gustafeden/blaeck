//! Example Viewer - Interactive tool for reviewing and cleaning up blaeck examples
//!
//! Run with: cargo run --example example_viewer
//!
//! Three-panel layout: file list | source code | live output
//!
//! Controls:
//!   ↑/k, ↓/j     - Navigate example list
//!   PageUp/K      - Scroll code up
//!   PageDown/J    - Scroll code down
//!   [, ]          - Resize preview panel (shrink/grow)
//!   Enter         - Enter live interactive preview (focus mode)
//!   Esc           - Exit focus mode / Quit
//!   r             - Run selected example (output appears in right panel)
//!   s             - Stop running example
//!   b             - Build selected example
//!   d             - Toggle mark for deletion
//!   x             - Execute deletions
//!   q/Esc         - Quit

#[path = "previews/mod.rs"]
mod previews;

use std::boxed::Box as StdBox;

use blaeck::prelude::*;
use blaeck::input::Key;
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use previews::live::{self, LivePreview};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const ASYNC_EXAMPLES: &[&str] = &["cube3d", "cube3d_braille"];
const LIST_WIDTH: f32 = 26.0;
const RUN_TIMEOUT_SECS: u64 = 10;
const MIN_PREVIEW_RATIO: f32 = 0.15;
const MAX_PREVIEW_RATIO: f32 = 0.70;
const DEFAULT_PREVIEW_RATIO: f32 = 0.35;

struct ExampleInfo {
    name: String,
    path: PathBuf,
    requires_async: bool,
    marked: bool,
}

enum Action {
    None,
    Quit,
}

// Background process capture shared between threads
struct Capture {
    output: Mutex<String>,
    done: AtomicBool,
    pid: Mutex<Option<u32>>,
}

impl Capture {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            output: Mutex::new(String::new()),
            done: AtomicBool::new(true),
            pid: Mutex::new(None),
        })
    }

    fn append(&self, text: &str) {
        self.output.lock().unwrap().push_str(text);
    }

    fn get_output(&self) -> String {
        self.output.lock().unwrap().clone()
    }

    fn is_done(&self) -> bool {
        self.done.load(Ordering::SeqCst)
    }

    fn is_running(&self) -> bool {
        !self.is_done()
    }

    fn reset(&self) {
        *self.output.lock().unwrap() = String::new();
        self.done.store(false, Ordering::SeqCst);
        *self.pid.lock().unwrap() = None;
    }

    fn kill(&self) {
        if let Some(pid) = *self.pid.lock().unwrap() {
            unsafe {
                libc::kill(pid as i32, libc::SIGTERM);
            }
        }
    }
}

/// Strip ANSI escape sequences from text
fn strip_ansi(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Skip ESC[...X sequences
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['
                // Consume until we hit a letter (the final byte)
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c.is_ascii_alphabetic() || c == '~' || c == 'H' || c == 'J' {
                        break;
                    }
                }
            } else if chars.peek() == Some(&']') {
                // OSC sequence: ESC]...ST or ESC]...BEL
                chars.next();
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c == '\x07' {
                        break;
                    }
                    if c == '\x1b' {
                        chars.next(); // consume '\'
                        break;
                    }
                }
            }
        } else if ch == '\r' {
            // Skip carriage returns
        } else {
            result.push(ch);
        }
    }
    result
}

fn spawn_example(capture: &Arc<Capture>, name: &str, is_async: bool) {
    capture.reset();
    capture.append(&format!("Building {}...\n", name));

    let cap = capture.clone();
    let name = name.to_string();
    let is_async = is_async;

    std::thread::spawn(move || {
        // Build first
        let mut build_cmd = Command::new("cargo");
        build_cmd
            .arg("build")
            .arg("--example")
            .arg(&name)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if is_async {
            build_cmd.arg("--features").arg("async");
        }

        match build_cmd.output() {
            Ok(output) if !output.status.success() => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                cap.append(&format!("Build failed:\n{}", strip_ansi(&stderr)));
                cap.done.store(true, Ordering::SeqCst);
                return;
            }
            Err(e) => {
                cap.append(&format!("Build error: {}\n", e));
                cap.done.store(true, Ordering::SeqCst);
                return;
            }
            _ => {}
        }

        cap.append("Running...\n\n");

        // Run the binary directly (skip cargo overhead)
        let binary = format!("./target/debug/examples/{}", name);
        let mut child = match Command::new(&binary)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                cap.append(&format!("Run error: {}\n", e));
                cap.done.store(true, Ordering::SeqCst);
                return;
            }
        };

        *cap.pid.lock().unwrap() = Some(child.id());

        // Read stdout in a separate thread
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let cap_out = cap.clone();
        let out_thread = std::thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let clean = strip_ansi(&line);
                    if !clean.trim().is_empty() {
                        cap_out.append(&clean);
                        cap_out.append("\n");
                    }
                }
            }
        });

        let cap_err = cap.clone();
        let err_thread = std::thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let clean = strip_ansi(&line);
                    if !clean.trim().is_empty() {
                        cap_err.append(&clean);
                        cap_err.append("\n");
                    }
                }
            }
        });

        // Wait with timeout
        let start = std::time::Instant::now();
        loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    cap.append(&format!(
                        "\n--- Exited ({}) ---\n",
                        if status.success() { "OK" } else { "error" }
                    ));
                    break;
                }
                Ok(None) => {
                    if start.elapsed() > Duration::from_secs(RUN_TIMEOUT_SECS) {
                        let _ = child.kill();
                        cap.append(&format!("\n--- Killed after {}s ---\n", RUN_TIMEOUT_SECS));
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
                Err(e) => {
                    cap.append(&format!("\nWait error: {}\n", e));
                    break;
                }
            }
        }

        let _ = out_thread.join();
        let _ = err_thread.join();
        cap.done.store(true, Ordering::SeqCst);
    });
}

struct ViewerState {
    examples: Vec<ExampleInfo>,
    selected: usize,
    code_scroll: usize,
    current_code: String,
    status_msg: String,
    term_width: u16,
    term_height: u16,
    capture: Arc<Capture>,
    output_scroll: usize,
    show_preview: bool,
    animation_timer: AnimationTimer,
    // Focus mode: live interactive preview
    focused_preview: Option<StdBox<dyn LivePreview>>,
    focus_mode: bool,
    last_frame_time: Instant,
    preview_ratio: f32,
}

impl ViewerState {
    fn new() -> Self {
        let (w, h) = crossterm::terminal::size().unwrap_or((120, 30));
        let mut state = Self {
            examples: Vec::new(),
            selected: 0,
            code_scroll: 0,
            current_code: String::new(),
            status_msg: String::new(),
            term_width: w,
            term_height: h,
            capture: Capture::new(),
            output_scroll: 0,
            show_preview: true,
            animation_timer: AnimationTimer::new(),
            focused_preview: None,
            focus_mode: false,
            last_frame_time: Instant::now(),
            preview_ratio: DEFAULT_PREVIEW_RATIO,
        };
        state.discover_examples();
        state.load_code();
        state
    }

    fn enter_focus_mode(&mut self) {
        if let Some(ex) = self.examples.get(self.selected) {
            if let Some(preview) = live::create_live_preview(&ex.name) {
                self.focused_preview = Some(preview);
                self.focus_mode = true;
                self.status_msg = format!("LIVE: {}", ex.name);
            } else {
                self.status_msg = format!("No live preview for {}", ex.name);
            }
        }
    }

    fn exit_focus_mode(&mut self) {
        self.focused_preview = None;
        self.focus_mode = false;
        self.status_msg.clear();
    }

    fn discover_examples(&mut self) {
        let examples_dir = PathBuf::from("blaeck/examples");
        if let Ok(entries) = std::fs::read_dir(&examples_dir) {
            let mut examples: Vec<ExampleInfo> = entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    let name = e.file_name().to_string_lossy().to_string();
                    name.ends_with(".rs") && name != "example_viewer.rs"
                })
                .map(|e| {
                    let filename = e.file_name().to_string_lossy().to_string();
                    let name = filename.trim_end_matches(".rs").to_string();
                    let requires_async = ASYNC_EXAMPLES.contains(&name.as_str());
                    ExampleInfo {
                        name,
                        path: e.path(),
                        requires_async,
                        marked: false,
                    }
                })
                .collect();
            examples.sort_by(|a, b| a.name.cmp(&b.name));
            self.examples = examples;
        }
    }

    fn load_code(&mut self) {
        if let Some(ex) = self.examples.get(self.selected) {
            // If there's a preview module with the real UI code, show that
            let preview_path = ex.path.parent().unwrap().join("previews").join(&format!("{}.rs", ex.name));
            if preview_path.exists() {
                let main_code = std::fs::read_to_string(&ex.path).unwrap_or_default();
                let preview_code = std::fs::read_to_string(&preview_path).unwrap_or_default();
                self.current_code = format!(
                    "// === {} (main) ===\n{}\n\n// === previews/{}.rs (UI code) ===\n{}",
                    ex.name, main_code.trim(), ex.name, preview_code.trim()
                );
            } else {
                self.current_code =
                    std::fs::read_to_string(&ex.path).unwrap_or_else(|_| "Error reading file".into());
            }
        }
        self.code_scroll = 0;
    }

    fn visible_code(&self) -> (String, usize) {
        let max_lines = (self.term_height as usize).saturating_sub(4);
        let lines: Vec<&str> = self.current_code.lines().collect();
        let start = self.code_scroll.min(lines.len().saturating_sub(1));
        let end = (start + max_lines).min(lines.len());

        // Calculate available width for code panel (accounting for borders, padding, line numbers)
        let remaining = self.term_width as f32 - LIST_WIDTH;
        let code_width = remaining - (remaining * self.preview_ratio).round();
        // Account for: border (2), padding (1), line number column (~6), separator (~2)
        let max_chars = (code_width as usize).saturating_sub(11);

        // Truncate each line to fit within the panel (character-aware)
        let truncated_lines: Vec<String> = lines[start..end]
            .iter()
            .map(|line| {
                let char_count = line.chars().count();
                if char_count > max_chars {
                    let truncated: String = line.chars().take(max_chars.saturating_sub(1)).collect();
                    format!("{}…", truncated)
                } else {
                    line.to_string()
                }
            })
            .collect();

        let visible = truncated_lines.join("\n");
        (visible, start + 1)
    }

    fn code_line_count(&self) -> usize {
        self.current_code.lines().count()
    }

    fn max_scroll(&self) -> usize {
        let max_lines = (self.term_height as usize).saturating_sub(4);
        self.code_line_count().saturating_sub(max_lines)
    }

    fn visible_output(&self) -> String {
        let output = self.capture.get_output();
        let max_lines = (self.term_height as usize).saturating_sub(4);
        let lines: Vec<&str> = output.lines().collect();

        if lines.len() <= max_lines {
            output
        } else {
            // Auto-scroll to bottom, or use manual scroll
            let start = if self.output_scroll == 0 {
                // Auto: show last N lines
                lines.len().saturating_sub(max_lines)
            } else {
                self.output_scroll.min(lines.len().saturating_sub(max_lines))
            };
            let end = (start + max_lines).min(lines.len());
            lines[start..end].join("\n")
        }
    }

    fn handle_input(&mut self, key: crossterm::event::KeyEvent) -> Action {
        // Resize keys work in both normal and focus mode
        match key.code {
            KeyCode::Char(']') => {
                self.preview_ratio = (self.preview_ratio + 0.05).min(MAX_PREVIEW_RATIO);
                return Action::None;
            }
            KeyCode::Char('[') => {
                self.preview_ratio = (self.preview_ratio - 0.05).max(MIN_PREVIEW_RATIO);
                return Action::None;
            }
            _ => {}
        }

        // Focus mode: forward keys to the live preview
        if self.focus_mode {
            return self.handle_focus_input(key);
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                if self.capture.is_running() {
                    self.capture.kill();
                }
                return Action::Quit;
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.capture.is_running() {
                    self.capture.kill();
                }
                return Action::Quit;
            }

            // Enter focus mode with live preview
            KeyCode::Enter => {
                self.enter_focus_mode();
            }

            // Navigate list
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected > 0 {
                    self.selected -= 1;
                    self.load_code();
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected + 1 < self.examples.len() {
                    self.selected += 1;
                    self.load_code();
                }
            }

            // Scroll code
            KeyCode::PageUp | KeyCode::Char('K') => {
                self.code_scroll = self.code_scroll.saturating_sub(10);
            }
            KeyCode::PageDown | KeyCode::Char('J') => {
                self.code_scroll = (self.code_scroll + 10).min(self.max_scroll());
            }

            // Run example in background
            KeyCode::Char('r') => {
                if let Some(ex) = self.examples.get(self.selected) {
                    if self.capture.is_running() {
                        self.capture.kill();
                        // Small delay for cleanup
                        std::thread::sleep(Duration::from_millis(200));
                    }
                    self.output_scroll = 0;
                    spawn_example(&self.capture, &ex.name, ex.requires_async);
                    self.status_msg = format!("Running {}...", ex.name);
                }
            }

            // Toggle preview
            KeyCode::Char('p') => {
                self.show_preview = !self.show_preview;
                self.status_msg = if self.show_preview {
                    "Preview mode".into()
                } else {
                    "Output mode".into()
                };
            }

            // Stop running example
            KeyCode::Char('s') => {
                if self.capture.is_running() {
                    self.capture.kill();
                    self.status_msg = "Stopped".into();
                }
            }

            // Build
            KeyCode::Char('b') => {
                if let Some(ex) = self.examples.get(self.selected) {
                    let name = ex.name.clone();
                    let is_async = ex.requires_async;
                    self.status_msg = format!("Building {}...", name);
                    let mut cmd = Command::new("cargo");
                    cmd.arg("build").arg("--example").arg(&name);
                    if is_async {
                        cmd.arg("--features").arg("async");
                    }
                    match cmd.output() {
                        Ok(output) if output.status.success() => {
                            self.status_msg = format!("✓ {} built OK", name);
                        }
                        Ok(output) => {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            let last = stderr.lines().last().unwrap_or("unknown error");
                            self.status_msg = format!("✗ {}: {}", name, last);
                        }
                        Err(e) => {
                            self.status_msg = format!("✗ {}: {}", name, e);
                        }
                    }
                }
            }

            // Mark for deletion
            KeyCode::Char('d') => {
                if let Some(ex) = self.examples.get_mut(self.selected) {
                    ex.marked = !ex.marked;
                    let label = if ex.marked { "marked" } else { "unmarked" };
                    self.status_msg = format!("{} {} for deletion", label, ex.name);
                }
            }

            // Execute deletions
            KeyCode::Char('x') => {
                let count = self.examples.iter().filter(|e| e.marked).count();
                if count == 0 {
                    self.status_msg = "No files marked for deletion".into();
                } else {
                    self.status_msg =
                        format!("Delete {} files? Press 'y' to confirm", count);
                }
            }
            KeyCode::Char('y') => {
                if self.status_msg.contains("Press 'y' to confirm") {
                    let mut deleted = 0;
                    let to_delete: Vec<PathBuf> = self
                        .examples
                        .iter()
                        .filter(|e| e.marked)
                        .map(|e| e.path.clone())
                        .collect();
                    for path in &to_delete {
                        if std::fs::remove_file(path).is_ok() {
                            deleted += 1;
                        }
                    }
                    self.examples.retain(|e| !e.marked);
                    if self.selected >= self.examples.len() && !self.examples.is_empty() {
                        self.selected = self.examples.len() - 1;
                    }
                    self.load_code();
                    self.status_msg = format!("Deleted {} files", deleted);
                }
            }

            _ => {}
        }
        Action::None
    }

    fn handle_focus_input(&mut self, key: crossterm::event::KeyEvent) -> Action {
        // Ctrl+C always quits
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            return Action::Quit;
        }

        // Esc exits focus mode
        if key.code == KeyCode::Esc {
            self.exit_focus_mode();
            return Action::None;
        }

        // Forward all other keys to the live preview
        if let Some(ref mut preview) = self.focused_preview {
            let blaeck_key = Key::from(key);
            let should_exit = preview.handle_key(&blaeck_key);
            if should_exit {
                self.exit_focus_mode();
            }
        }

        Action::None
    }
}

fn render_list(state: &ViewerState) -> Element {
    let max_visible = (state.term_height as usize).saturating_sub(5);
    let list_scroll = if state.selected >= max_visible {
        state.selected - max_visible + 1
    } else {
        0
    };

    let mut items: Vec<Element> = Vec::new();

    items.push(element! {
        Text(
            content: format!(" Examples ({}) ", state.examples.len()),
            bold: true,
            color: Color::Cyan
        )
    });

    for (i, ex) in state.examples.iter().enumerate().skip(list_scroll) {
        if items.len() >= max_visible {
            break;
        }

        let is_selected = i == state.selected;
        let prefix = if ex.marked {
            "✗ "
        } else if is_selected {
            "▸ "
        } else {
            "  "
        };

        let suffix = if ex.requires_async { " ⚡" } else { "" };

        let color = if ex.marked {
            Color::Red
        } else if is_selected {
            Color::Yellow
        } else {
            Color::White
        };

        let name = format!("{}{}{}", prefix, ex.name, suffix);
        let max_name_len = (LIST_WIDTH as usize).saturating_sub(3);
        let display = if name.len() > max_name_len {
            format!("{}…", &name[..max_name_len - 1])
        } else {
            name
        };

        items.push(element! {
            Text(content: display, color: color, bold: is_selected)
        });
    }

    Element::column(items)
}

fn render_code(state: &ViewerState) -> Element {
    let (visible_code, start_line) = state.visible_code();

    if visible_code.is_empty() {
        return element! {
            Text(content: "  (empty file)", dim: true)
        };
    }

    Element::node::<SyntaxHighlight>(
        SyntaxHighlightProps::new(&visible_code)
            .language("rust")
            .theme(SyntaxTheme::OceanDark)
            .line_numbers(LineNumberStyle::WithSeparator)
            .start_line(start_line),
        vec![],
    )
}

fn try_render_preview(name: &str, timer: &AnimationTimer) -> Option<Element> {
    match name {
        "animation" => Some(previews::animation::build_ui_with_timer(timer)),
        "banner" => Some(previews::banner::build_ui()),
        "barchart" => Some(previews::barchart::build_ui()),
        "borders" => Some(previews::borders::build_ui()),
        "breadcrumbs" => Some(previews::breadcrumbs::build_ui()),
        "diff" => Some(previews::diff::build_ui()),
        "gradient" => Some(previews::gradient::build_ui()),
        "hello" => Some(previews::hello::build_ui()),
        "keyhints" => Some(previews::keyhints::build_ui()),
        "markdown" => Some(previews::markdown::build_ui()),
        "modal" => Some(previews::modal::build_ui()),
        "statusbar" => Some(previews::statusbar::build_ui()),
        "syntax" => Some(previews::syntax::build_ui()),
        "table" => Some(previews::table::build_ui()),
        "tree" => Some(previews::tree::build_ui()),
        "demo_inline" => Some(previews::demo_inline::build_ui_with_timer(timer)),
        "logbox" => Some(previews::logbox::build_ui_with_timer(timer)),
        "logbox_command" => Some(previews::logbox_command::build_ui_with_timer(timer)),
        "task_runner" => Some(previews::task_runner::build_ui_with_timer(timer)),
        "reactive_counter" => Some(previews::reactive_counter::build_ui()),
        "reactive_list" => Some(previews::reactive_list::build_ui()),
        "quickstart_interactive" => Some(previews::quickstart_interactive::build_ui()),
        "spinner_demo" => Some(previews::spinner_demo::build_ui_with_timer(timer)),
        "timer" => Some(previews::timer::build_ui_with_timer(timer)),
        "preview" => Some(previews::preview::build_ui_with_timer(timer)),
        "reactive_timeline" => Some(previews::reactive_timeline::build_ui()),
        "stagger_demo" => Some(previews::stagger_demo::build_ui()),
        "timeline_debug" => Some(previews::timeline_debug::build_ui()),
        "timeline_demo" => Some(previews::timeline_demo::build_ui_with_timer(timer)),
        "focus_demo" => Some(previews::focus_demo::build_ui()),
        "form_demo" => Some(previews::form_demo::build_ui()),
        "select_demo" => Some(previews::select_demo::build_ui()),
        "interactive" => Some(previews::interactive::build_ui()),
        "menu" => Some(previews::menu::build_ui()),
        "multiselect" => Some(previews::multiselect::build_ui()),
        "autocomplete" => Some(previews::autocomplete::build_ui()),
        "tabs" => Some(previews::tabs::build_ui()),
        "sparkline" => Some(previews::sparkline::build_ui()),
        "polish_demo" => Some(previews::polish_demo::build_ui()),
        "async_app" => Some(previews::async_app::build_ui()),
        "cube3d_braille" => Some(previews::cube3d_braille::build_ui()),
        "dashboard" => Some(previews::dashboard::build_ui()),
        "plasma" => Some(previews::plasma::build_ui()),
        "showcase" => Some(previews::showcase::build_ui()),
        _ => None,
    }
}

fn render_output(state: &mut ViewerState) -> Element {
    let name = state.examples.get(state.selected).map(|e| e.name.as_str()).unwrap_or("");

    // Focus mode: render the live interactive preview
    if state.focus_mode {
        if let Some(ref mut preview) = state.focused_preview {
            let preview_element = preview.render();
            return element! {
                Box(flex_direction: FlexDirection::Column) {
                    Text(content: " LIVE ", bold: true, color: Color::Yellow)
                    #(preview_element)
                }
            };
        }
    }

    // Try direct element render first
    if state.show_preview {
        if let Some(preview) = try_render_preview(name, &state.animation_timer) {
            return element! {
                Box(flex_direction: FlexDirection::Column) {
                    Text(content: " Preview ", bold: true, color: Color::Green)
                    #(preview)
                }
            };
        }
    }

    // Fall back to captured subprocess output
    let output = state.visible_output();

    if output.is_empty() && !state.show_preview {
        return element! {
            Box(flex_direction: FlexDirection::Column) {
                Text(content: " Output ", bold: true, color: Color::Cyan)
                Text(content: "", dim: true)
                Text(content: " Press 'r' to run", dim: true)
                Text(content: " Press 'p' to preview", dim: true)
            }
        };
    }

    if output.is_empty() {
        return element! {
            Box(flex_direction: FlexDirection::Column) {
                Text(content: " Output ", bold: true, color: Color::Cyan)
                Text(content: "", dim: true)
                Text(content: " No preview available", dim: true)
                Text(content: " Press 'r' to run", dim: true)
            }
        };
    }

    let running = state.capture.is_running();
    let header_text = if running { " Output (running) " } else { " Output " };
    let header_color = if running { Color::Green } else { Color::Cyan };

    let lines: Vec<Element> = output
        .lines()
        .map(|line| {
            element! {
                Text(content: line.to_string(), color: Color::White)
            }
        })
        .collect();

    let mut children = vec![element! {
        Text(content: header_text.to_string(), bold: true, color: header_color)
    }];
    children.extend(lines);

    Element::column(children)
}

fn render_status(state: &ViewerState) -> Element {
    if state.focus_mode {
        let name = state.examples.get(state.selected).map(|e| e.name.as_str()).unwrap_or("");
        let status = format!(
            " LIVE: {}  │  [Esc] Exit  [Keys] Interact  [/] Resize",
            name
        );
        return element! {
            Text(content: status, color: Color::Black, bg_color: Color::Yellow)
        };
    }

    let marked_count = state.examples.iter().filter(|e| e.marked).count();
    let scroll_info = format!(
        " L{}/{}",
        state.code_scroll + 1,
        state.code_line_count()
    );

    let marked_info = if marked_count > 0 {
        format!(" | {}✗", marked_count)
    } else {
        String::new()
    };

    let running_info = if state.capture.is_running() {
        " | [s] Stop"
    } else {
        ""
    };

    let controls = format!(
        " [↑↓] Nav  [Enter] Live  [PgUp/Dn] Scroll  [/] Resize  [r] Run{}  [b] Build  [d] Mark  [x] Del  [q] Quit{}{}",
        running_info, marked_info, scroll_info
    );

    let status = if state.status_msg.is_empty() {
        controls
    } else {
        format!(" {} │{}", state.status_msg, controls)
    };

    element! {
        Text(content: status, color: Color::Black, bg_color: Color::White)
    }
}

fn build_ui(state: &mut ViewerState) -> Element {
    let list = render_list(state);
    let code = render_code(state);
    let output = render_output(state);
    let status = render_status(state);

    let right_border_color = if state.focus_mode { Color::Yellow } else { Color::DarkGray };
    let right_border_style = if state.focus_mode { BorderStyle::Double } else { BorderStyle::Single };

    // Compute fixed widths based on preview ratio
    let remaining = state.term_width as f32 - LIST_WIDTH;
    let preview_width = (remaining * state.preview_ratio).round();
    let code_width = remaining - preview_width;

    element! {
        Box(flex_direction: FlexDirection::Column, width: state.term_width as f32, height: state.term_height as f32) {
            Box(flex_direction: FlexDirection::Row, flex_grow: 1.0) {
                // Left: example list
                Box(
                    width: LIST_WIDTH,
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single,
                    border_color: Color::DarkGray,
                    padding_left: 1.0,
                ) {
                    #(list)
                }
                // Center: code view
                Box(
                    width: code_width,
                    flex_direction: FlexDirection::Column,
                    border_style: BorderStyle::Single,
                    border_color: Color::DarkGray,
                    padding_left: 1.0,
                    overflow_x: Overflow::Hidden,
                    overflow_y: Overflow::Hidden,
                ) {
                    #(code)
                }
                // Right: output panel
                Box(
                    width: preview_width,
                    flex_direction: FlexDirection::Column,
                    border_style: right_border_style,
                    border_color: right_border_color,
                    padding_left: 1.0,
                    overflow_x: Overflow::Hidden,
                    overflow_y: Overflow::Hidden,
                ) {
                    #(output)
                }
            }
            // Status bar
            #(status)
        }
    }
}

// libc for process kill
mod libc {
    extern "C" {
        pub fn kill(pid: i32, sig: i32) -> i32;
    }
    pub const SIGTERM: i32 = 15;
}

fn main() -> std::io::Result<()> {
    let mut blaeck = Blaeck::new(std::io::stdout())?;
    let mut state = ViewerState::new();

    crossterm::terminal::enable_raw_mode()?;
    blaeck.set_cursor_visible(false);

    loop {
        // Calculate delta time
        let now = Instant::now();
        let dt = now.duration_since(state.last_frame_time).as_secs_f64();
        state.last_frame_time = now;

        // Tick the focused preview if any
        let needs_continuous = if let Some(ref mut preview) = state.focused_preview {
            preview.poll_events();
            preview.tick(dt);
            preview.needs_tick()
        } else {
            false
        };

        let ui = build_ui(&mut state);
        blaeck.render(ui)?;

        // Update status when capture finishes
        if state.capture.is_done() && state.status_msg.contains("Running") {
            let name = state.status_msg.replace("Running ", "").replace("...", "");
            state.status_msg = format!("✓ {} done", name.trim());
        }

        // Use shorter poll timeout when in continuous rendering mode (animations)
        let poll_timeout = if needs_continuous {
            Duration::from_millis(16) // ~60fps
        } else {
            Duration::from_millis(50) // Normal mode
        };

        if poll(poll_timeout)? {
            match read()? {
                Event::Key(key) => {
                    if let Action::Quit = state.handle_input(key) {
                        break;
                    }
                }
                Event::Resize(w, h) => {
                    state.term_width = w;
                    state.term_height = h;
                    let _ = blaeck.handle_resize(w, h);
                }
                _ => {}
            }
        }
    }

    blaeck.set_cursor_visible(true);
    crossterm::terminal::disable_raw_mode()?;
    blaeck.unmount()?;

    let marked: Vec<&str> = state
        .examples
        .iter()
        .filter(|e| e.marked)
        .map(|e| e.name.as_str())
        .collect();
    if !marked.is_empty() {
        println!("Warning: {} files still marked for deletion:", marked.len());
        for name in &marked {
            println!("  - {}", name);
        }
    }

    Ok(())
}
