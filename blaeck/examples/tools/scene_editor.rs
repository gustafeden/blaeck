//! Scene Editor - Keyframe animation editor for terminal UIs
//!
//! Controls:
//!   1-6        - Select element to move
//!   Arrows     - Move selected element
//!   Shift+Arr  - Move by 5
//!
//!   n          - New keyframe (copy current)
//!   d          - Delete current keyframe
//!   [/]        - Previous/Next keyframe
//!
//!   Space      - Play/pause animation preview
//!   s          - Save to scene.json
//!   l          - Load from scene.json
//!
//!   c          - Calibrate: set canvas origin to current mouse position
//!   q/Esc      - Quit

use blaeck::prelude::*;
use crossterm::event::{
    poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers,
    MouseButton, MouseEventKind,
};
use crossterm::cursor;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{Duration, Instant};

const WIDTH: usize = 80;
const HEIGHT: usize = 24;
const SAVE_FILE: &str = "scene.json";

// Logo
const LOGO_FILL: &[&str] = &[
    "████  █      ███  ████  ███  █  █",
    "█  █  █     █  █  █     █    █ █ ",
    "████  █     ████  ███   █    ██  ",
    "█  █  █     █  █  █     █    █ █ ",
    "████  ████  █  █  ████  ███  █  █",
];

struct Theme {
    bg: (u8, u8, u8),
    panel: (u8, u8, u8),
    text: (u8, u8, u8),
    dim: (u8, u8, u8),
    accent: (u8, u8, u8),
}

const THEME: Theme = Theme {
    bg: (0x2E, 0x2C, 0x2F),
    panel: (0x47, 0x5B, 0x63),
    text: (0xF3, 0xE8, 0xEE),
    dim: (0x7A, 0x6F, 0x80),
    accent: (0xCF, 0xA9, 0xFF),
};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
struct ElementPos {
    x: f32,
    y: f32,
    visible: bool,
}

impl Default for ElementPos {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, visible: true }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Keyframe {
    name: String,
    duration: f32, // seconds to animate TO this frame
    elements: [ElementPos; 6],
}

impl Default for Keyframe {
    fn default() -> Self {
        Self {
            name: "Frame".to_string(),
            duration: 0.5,
            elements: [
                // 0: Logo (centered: (80-34)/2 = 23)
                ElementPos { x: 23.0, y: 1.0, visible: true },
                // 1: RENDER (top-left)
                ElementPos { x: 2.0, y: 8.0, visible: true },
                // 2: LAYOUT (top-right)
                ElementPos { x: 60.0, y: 8.0, visible: true },
                // 3: BUFFER (bottom-left)
                ElementPos { x: 2.0, y: 15.0, visible: true },
                // 4: MEMORY (bottom-right)
                ElementPos { x: 60.0, y: 15.0, visible: true },
                // 5: Center showcase box (centered: (80-40)/2 = 20)
                ElementPos { x: 20.0, y: 9.0, visible: true },
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Scene {
    keyframes: Vec<Keyframe>,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            keyframes: vec![Keyframe::default()],
        }
    }
}

struct EditorState {
    scene: Scene,
    current_frame: usize,
    selected_element: usize,
    playing: bool,
    play_start: Instant,
    play_frame: usize,
    message: Option<(String, Instant)>,
    // Mouse dragging
    dragging: Option<usize>,
    drag_offset: (f32, f32),
    // Canvas offset (where the UI starts in terminal coordinates)
    canvas_offset: (u16, u16),
    // Debug - raw terminal mouse position
    mouse_pos: (u16, u16),
}

impl EditorState {
    fn new(canvas_offset: (u16, u16)) -> Self {
        Self {
            scene: Scene::default(),
            current_frame: 0,
            selected_element: 0,
            playing: false,
            play_start: Instant::now(),
            play_frame: 0,
            message: None,
            dragging: None,
            drag_offset: (0.0, 0.0),
            canvas_offset,
            mouse_pos: (0, 0),
        }
    }

    // Convert terminal mouse coords to canvas coords
    fn to_canvas_coords(&self, mx: u16, my: u16) -> (u16, u16) {
        (
            mx.saturating_sub(self.canvas_offset.0),
            my.saturating_sub(self.canvas_offset.1),
        )
    }

    // Calibrate: set canvas origin to current mouse position
    fn calibrate(&mut self) {
        self.canvas_offset = self.mouse_pos;
        self.show_message(&format!("Calibrated: offset ({}, {})", self.mouse_pos.0, self.mouse_pos.1));
    }

    // Hit test - returns element index if mouse is over it (uses canvas coords)
    fn hit_test(&self, cx: u16, cy: u16) -> Option<usize> {
        let positions = &self.current_keyframe().elements;

        // Element bounding boxes (idx, width, height) - generous sizes for easier clicking
        let bounds: [(usize, f32, f32); 6] = [
            (0, 37.0, 8.0),  // Logo
            (1, 18.0, 5.0),  // RENDER panel
            (2, 18.0, 5.0),  // LAYOUT panel
            (3, 18.0, 5.0),  // BUFFER panel
            (4, 18.0, 5.0),  // MEMORY panel
            (5, 40.0, 6.0),  // Center box
        ];

        let cxf = cx as f32;
        let cyf = cy as f32;

        // Check in reverse order (front elements first)
        for &(idx, w, h) in bounds.iter().rev() {
            let pos = &positions[idx];
            if !pos.visible {
                continue;
            }
            let x1 = pos.x;
            let y1 = pos.y;
            let x2 = x1 + w;
            let y2 = y1 + h;

            if cxf >= x1 && cxf < x2 && cyf >= y1 && cyf < y2 {
                return Some(idx);
            }
        }
        None
    }

    fn start_drag(&mut self, mx: u16, my: u16) {
        let (cx, cy) = self.to_canvas_coords(mx, my);
        if let Some(idx) = self.hit_test(cx, cy) {
            let pos = self.current_keyframe().elements[idx];
            self.dragging = Some(idx);
            self.selected_element = idx;
            self.drag_offset = (cx as f32 - pos.x, cy as f32 - pos.y);
        }
    }

    fn update_drag(&mut self, mx: u16, my: u16) {
        let (cx, cy) = self.to_canvas_coords(mx, my);
        if let Some(idx) = self.dragging {
            let frame = self.current_frame;
            let elem = &mut self.scene.keyframes[frame].elements[idx];
            elem.x = (cx as f32 - self.drag_offset.0).max(0.0);
            elem.y = (cy as f32 - self.drag_offset.1).max(0.0);
        }
    }

    fn stop_drag(&mut self) {
        self.dragging = None;
    }

    fn element_name(&self, idx: usize) -> &'static str {
        match idx {
            0 => "LOGO",
            1 => "RENDER",
            2 => "LAYOUT",
            3 => "BUFFER",
            4 => "MEMORY",
            5 => "CENTER",
            _ => "???",
        }
    }

    fn current_keyframe(&self) -> &Keyframe {
        &self.scene.keyframes[self.current_frame]
    }

    fn current_keyframe_mut(&mut self) -> &mut Keyframe {
        &mut self.scene.keyframes[self.current_frame]
    }

    fn move_selected(&mut self, dx: f32, dy: f32) {
        let idx = self.selected_element;
        let elem = &mut self.current_keyframe_mut().elements[idx];
        elem.x = (elem.x + dx).max(0.0);
        elem.y = (elem.y + dy).max(0.0);
    }

    fn toggle_visibility(&mut self) {
        let idx = self.selected_element;
        let elem = &mut self.current_keyframe_mut().elements[idx];
        elem.visible = !elem.visible;
    }

    fn new_keyframe(&mut self) {
        let copy = self.current_keyframe().clone();
        let mut new_frame = copy;
        new_frame.name = format!("Frame {}", self.scene.keyframes.len() + 1);
        self.scene.keyframes.insert(self.current_frame + 1, new_frame);
        self.current_frame += 1;
        self.show_message("New keyframe created");
    }

    fn delete_keyframe(&mut self) {
        if self.scene.keyframes.len() > 1 {
            self.scene.keyframes.remove(self.current_frame);
            if self.current_frame >= self.scene.keyframes.len() {
                self.current_frame = self.scene.keyframes.len() - 1;
            }
            self.show_message("Keyframe deleted");
        } else {
            self.show_message("Can't delete last keyframe");
        }
    }

    fn prev_frame(&mut self) {
        if self.current_frame > 0 {
            self.current_frame -= 1;
        }
    }

    fn next_frame(&mut self) {
        if self.current_frame < self.scene.keyframes.len() - 1 {
            self.current_frame += 1;
        }
    }

    fn toggle_play(&mut self) {
        self.playing = !self.playing;
        if self.playing {
            self.play_start = Instant::now();
            self.play_frame = 0;
            self.show_message("Playing...");
        } else {
            self.show_message("Stopped");
        }
    }

    fn save(&self) {
        match serde_json::to_string_pretty(&self.scene) {
            Ok(json) => {
                if let Err(e) = fs::write(SAVE_FILE, json) {
                    eprintln!("Save error: {}", e);
                }
            }
            Err(e) => eprintln!("Serialize error: {}", e),
        }
    }

    fn load(&mut self) {
        match fs::read_to_string(SAVE_FILE) {
            Ok(json) => match serde_json::from_str(&json) {
                Ok(scene) => {
                    self.scene = scene;
                    self.current_frame = 0;
                    self.show_message(&format!("Loaded {} keyframes", self.scene.keyframes.len()));
                }
                Err(e) => self.show_message(&format!("Parse error: {}", e)),
            },
            Err(_) => self.show_message("No scene.json found"),
        }
    }

    fn show_message(&mut self, msg: &str) {
        self.message = Some((msg.to_string(), Instant::now()));
    }

    // Get interpolated positions for animation playback
    fn get_animated_positions(&self) -> [ElementPos; 6] {
        if !self.playing || self.scene.keyframes.len() < 2 {
            return self.current_keyframe().elements;
        }

        let elapsed = self.play_start.elapsed().as_secs_f32();

        // Calculate which frame we're animating to and progress
        let mut time_acc = 0.0;

        for (i, kf) in self.scene.keyframes.iter().enumerate().skip(1) {
            if elapsed < time_acc + kf.duration {
                let from_frame = i - 1;
                let progress = (elapsed - time_acc) / kf.duration;
                return Self::lerp_frames(
                    &self.scene.keyframes[from_frame],
                    &self.scene.keyframes[i],
                    progress,
                );
            }
            time_acc += kf.duration;
        }

        // Loop back
        self.scene.keyframes.last().unwrap().elements
    }

    fn lerp_frames(from: &Keyframe, to: &Keyframe, t: f32) -> [ElementPos; 6] {
        let mut result = [ElementPos::default(); 6];
        for i in 0..6 {
            result[i] = ElementPos {
                x: from.elements[i].x + (to.elements[i].x - from.elements[i].x) * t,
                y: from.elements[i].y + (to.elements[i].y - from.elements[i].y) * t,
                visible: if t < 0.5 { from.elements[i].visible } else { to.elements[i].visible },
            };
        }
        result
    }
}

const LOGO_WIDTH: usize = 34;
const LOGO_HEIGHT: usize = 5;

fn render_logo(pos: &ElementPos, selected: bool) -> Element {
    if !pos.visible {
        return Element::Empty;
    }

    let (pr, pg, pb) = THEME.panel;
    let (dr, dg, db) = THEME.dim;
    let (ar, ag, ab) = THEME.accent;

    // Highlight multiplier when selected
    let glow = if selected { 1.4 } else { 1.0 };

    // Stacked colors with depth separation (4 layers)
    let colors: [(f32, f32, f32); 4] = [
        // BACK SHADOW
        (dr as f32 * 0.40, dg as f32 * 0.40, db as f32 * 0.45),
        // MID-BACK
        (dr as f32 * 0.65, dg as f32 * 0.65, db as f32 * 0.70),
        // MID-FRONT
        (pr as f32 * 1.08, pg as f32 * 1.08, pb as f32 * 1.12),
        // FRONT GLOW
        (
            (ar as f32 * glow).min(255.0),
            (ag as f32 * glow).min(255.0),
            (ab as f32 * glow).min(255.0),
        ),
    ];

    // Offsets for each layer (creates the stacked extrusion)
    let offsets: [(i32, i32); 4] = [
        (2, 2),   // back layer
        (1, 1),   // middle-back
        (0, 1),   // middle-front
        (0, 0),   // front layer
    ];

    let grid_h = LOGO_HEIGHT + 3;
    let grid_w = LOGO_WIDTH + 3;

    // Initialize grid with None (transparent)
    let mut grid: Vec<Vec<Option<(f32, f32, f32)>>> = vec![vec![None; grid_w]; grid_h];

    // Render each layer (back to front)
    for ((ox, oy), color) in offsets.iter().zip(colors.iter()) {
        for (row_idx, line) in LOGO_FILL.iter().enumerate() {
            for (col_idx, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    let gx = col_idx as i32 + ox;
                    let gy = row_idx as i32 + oy;
                    if gx >= 0 && gy >= 0 && (gx as usize) < grid_w && (gy as usize) < grid_h {
                        grid[gy as usize][gx as usize] = Some(*color);
                    }
                }
            }
        }
    }

    // Convert grid to elements
    let mut rows: Vec<Element> = Vec::new();

    for row in grid.iter() {
        let mut row_elements: Vec<Element> = Vec::new();

        for cell in row.iter() {
            match cell {
                Some((r, g, b)) => {
                    let text_color = Color::Rgb(*r as u8, *g as u8, *b as u8);
                    row_elements.push(element! {
                        Text(content: "█", color: text_color)
                    });
                }
                None => {
                    // Transparent - just a space
                    row_elements.push(element! { Text(content: " ") });
                }
            }
        }
        rows.push(Element::row(row_elements));
    }

    element! {
        Box(position: Position::Absolute, inset_top: pos.y, inset_left: pos.x) {
            #(Element::column(rows))
        }
    }
}

fn render_panel(title: &str, pos: &ElementPos, selected: bool) -> Element {
    if !pos.visible {
        return Element::Empty;
    }

    let bg = Color::Rgb(THEME.panel.0, THEME.panel.1, THEME.panel.2);
    let border_color = if selected {
        Color::Rgb(THEME.accent.0, THEME.accent.1, THEME.accent.2)
    } else {
        bg
    };
    let title_color = Color::Rgb(THEME.text.0, THEME.text.1, THEME.text.2);
    let text_color = Color::Rgb(THEME.dim.0, THEME.dim.1, THEME.dim.2);

    element! {
        Box(position: Position::Absolute, inset_top: pos.y, inset_left: pos.x) {
            Box(
                flex_direction: FlexDirection::Column,
                background_color: bg,
                border_style: BorderStyle::Single,
                border_color: border_color,
                padding_left: 1.0,
                padding_right: 1.0,
            ) {
                Text(content: format!(" {} ", title), color: title_color, bold: true, bg_color: bg)
                Text(content: "  value 123  ", color: text_color, bg_color: bg)
                Text(content: "  value 456  ", color: text_color, bg_color: bg)
            }
        }
    }
}

// Helper to center text within a given width
fn center_text(text: &str, width: usize) -> String {
    let text_len = text.chars().count();
    if text_len >= width {
        return text.to_string();
    }
    let total_pad = width - text_len;
    let left_pad = total_pad / 2;
    let right_pad = total_pad - left_pad;
    format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
}

fn render_center_box(pos: &ElementPos, selected: bool) -> Element {
    if !pos.visible {
        return Element::Empty;
    }

    let box_bg = Color::Rgb(20, 20, 28);
    let border_color = if selected {
        Color::Rgb(THEME.accent.0, THEME.accent.1, THEME.accent.2)
    } else {
        Color::Rgb(THEME.dim.0, THEME.dim.1, THEME.dim.2)
    };
    let title_color = Color::Rgb(THEME.text.0, THEME.text.1, THEME.text.2);

    // Box width 40, border 2, padding 2 = content width 36
    let content_width = 36;

    element! {
        Box(position: Position::Absolute, inset_top: pos.y, inset_left: pos.x) {
            Box(
                border_style: BorderStyle::Round,
                border_color: border_color,
                background_color: box_bg,
                padding: 1.0,
                width: 40.0,
            ) {
                Box(flex_direction: FlexDirection::Column, background_color: box_bg) {
                    Text(content: center_text("SHOWCASE", content_width), color: title_color, bold: true, bg_color: box_bg)
                    Text(content: center_text("", content_width), color: title_color, bg_color: box_bg)
                    Text(content: center_text("Content here", content_width), color: title_color, bg_color: box_bg)
                    Text(content: center_text("", content_width), color: title_color, bg_color: box_bg)
                }
            }
        }
    }
}

fn render_status(state: &EditorState) -> Element {
    let sel = state.selected_element;
    let pos = &state.current_keyframe().elements[sel];
    let frame_num = state.current_frame + 1;
    let total_frames = state.scene.keyframes.len();

    let drag_info = if state.dragging.is_some() { " [DRAG]" } else { "" };
    let play_info = if state.playing { " [PLAY]" } else { "" };

    let status = format!(
        " [{}] {} ({:.0},{:.0}) | Frame {}/{} | 1-6:sel Arrows:move n:new []:nav Space:play s:save c:calibrate{}{}",
        sel + 1,
        state.element_name(sel),
        pos.x, pos.y,
        frame_num, total_frames,
        drag_info,
        play_info,
    );

    element! {
        Box(position: Position::Absolute, inset_bottom: 0.0, inset_left: 0.0) {
            Text(content: status, color: Color::Black, bg_color: Color::White)
        }
    }
}

fn render_background() -> Element {
    let bg = Color::Rgb(THEME.bg.0, THEME.bg.1, THEME.bg.2);
    let rows: Vec<Element> = (0..HEIGHT)
        .map(|_| {
            let line = " ".repeat(WIDTH);
            element! { Text(content: line, bg_color: bg) }
        })
        .collect();
    Element::column(rows)
}

fn build_ui(state: &EditorState) -> Element {
    let positions = if state.playing {
        state.get_animated_positions()
    } else {
        state.current_keyframe().elements
    };

    let sel = if state.playing { usize::MAX } else { state.selected_element };

    element! {
        Box(position: Position::Relative, width: WIDTH as f32, height: (HEIGHT + 1) as f32) {
            Box(position: Position::Absolute, inset_top: 0.0, inset_left: 0.0) {
                #(render_background())
            }
            #(render_logo(&positions[0], sel == 0))
            #(render_panel("RENDER", &positions[1], sel == 1))
            #(render_panel("LAYOUT", &positions[2], sel == 2))
            #(render_panel("BUFFER", &positions[3], sel == 3))
            #(render_panel("MEMORY", &positions[4], sel == 4))
            #(render_center_box(&positions[5], sel == 5))
            #(render_status(state))
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut blaeck = Blaeck::new(std::io::stdout())?;

    // Get cursor position before enabling raw mode - this is where canvas starts
    let canvas_offset = cursor::position().unwrap_or((0, 0));

    let mut state = EditorState::new(canvas_offset);

    // Try to load existing scene
    if fs::metadata(SAVE_FILE).is_ok() {
        state.load();
    }

    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), EnableMouseCapture)?;

    loop {
        if poll(Duration::from_millis(16))? {
            match read()? {
                Event::Key(key) => {
                    let step = if key.modifiers.contains(KeyModifiers::SHIFT) { 5.0 } else { 1.0 };

                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,

                        // Element selection
                        KeyCode::Char('1') => state.selected_element = 0,
                        KeyCode::Char('2') => state.selected_element = 1,
                        KeyCode::Char('3') => state.selected_element = 2,
                        KeyCode::Char('4') => state.selected_element = 3,
                        KeyCode::Char('5') => state.selected_element = 4,
                        KeyCode::Char('6') => state.selected_element = 5,

                        // Movement
                        KeyCode::Up => state.move_selected(0.0, -step),
                        KeyCode::Down => state.move_selected(0.0, step),
                        KeyCode::Left => state.move_selected(-step, 0.0),
                        KeyCode::Right => state.move_selected(step, 0.0),

                        // Visibility toggle
                        KeyCode::Char('v') => state.toggle_visibility(),

                        // Keyframe management
                        KeyCode::Char('n') => state.new_keyframe(),
                        KeyCode::Char('d') => state.delete_keyframe(),
                        KeyCode::Char('[') => state.prev_frame(),
                        KeyCode::Char(']') => state.next_frame(),

                        // Playback
                        KeyCode::Char(' ') => state.toggle_play(),

                        // Save/Load
                        KeyCode::Char('s') => {
                            state.save();
                            state.show_message("Saved to scene.json");
                        }
                        KeyCode::Char('l') => state.load(),

                        // Calibrate mouse offset
                        KeyCode::Char('c') => state.calibrate(),

                        _ => {}
                    }
                }
                Event::Mouse(mouse) => {
                    state.mouse_pos = (mouse.column, mouse.row);
                    match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            state.start_drag(mouse.column, mouse.row);
                        }
                        MouseEventKind::Drag(MouseButton::Left) => {
                            state.update_drag(mouse.column, mouse.row);
                        }
                        MouseEventKind::Up(MouseButton::Left) => {
                            state.stop_drag();
                        }
                        _ => {}
                    }
                }
                Event::Resize(w, h) => {
                    let _ = blaeck.handle_resize(w, h);
                }
                _ => {}
            }
        }

        let ui = build_ui(&state);
        blaeck.render(ui)?;
    }

    crossterm::execute!(std::io::stdout(), DisableMouseCapture)?;
    crossterm::terminal::disable_raw_mode()?;
    blaeck.unmount()?;

    Ok(())
}
