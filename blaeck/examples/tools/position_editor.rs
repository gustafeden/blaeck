//! Position Editor - Interactive tool to find exact positions for showcase elements
//!
//! Controls:
//!   1-6    - Select element to move
//!   Arrows - Move selected element
//!   Shift+Arrows - Move by 5
//!   p      - Print all positions
//!   q/Esc  - Quit

use blaeck::prelude::*;
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use std::time::Duration;

const WIDTH: usize = 80;
const HEIGHT: usize = 24;

// Logo
const LOGO_FILL: &[&str] = &[
    "████  █      ███  ████  ███  █  █",
    "█  █  █     █  █  █     █    █ █ ",
    "████  █     ████  ███   █    ██  ",
    "█  █  █     █  █  █     █    █ █ ",
    "████  ████  █  █  ████  ███  █  █",
];
const LOGO_WIDTH: usize = 34;
const LOGO_HEIGHT: usize = 5;

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

#[derive(Clone, Copy)]
struct Pos {
    x: f32,
    y: f32,
}

struct EditorState {
    selected: usize,
    positions: [Pos; 6],
}

impl EditorState {
    fn new() -> Self {
        Self {
            selected: 0,
            positions: [
                // 0: Logo
                Pos { x: 23.0, y: 1.0 },
                // 1: RENDER (top-left)
                Pos { x: 2.0, y: 8.0 },
                // 2: LAYOUT (top-right)
                Pos { x: 64.0, y: 8.0 },
                // 3: BUFFER (bottom-left)
                Pos { x: 2.0, y: 13.0 },
                // 4: MEMORY (bottom-right)
                Pos { x: 64.0, y: 13.0 },
                // 5: Center showcase box
                Pos { x: 20.0, y: 12.0 },
            ],
        }
    }

    fn element_name(&self, idx: usize) -> &'static str {
        match idx {
            0 => "LOGO",
            1 => "RENDER",
            2 => "LAYOUT",
            3 => "BUFFER",
            4 => "MEMORY",
            5 => "CENTER BOX",
            _ => "???",
        }
    }

    fn move_selected(&mut self, dx: f32, dy: f32) {
        let pos = &mut self.positions[self.selected];
        pos.x = (pos.x + dx).max(0.0);
        pos.y = (pos.y + dy).max(0.0);
    }

    fn print_positions(&self) {
        print!("\r\n// Current positions:\r\n");
        print!("// Logo:       x={:.1}, y={:.1}\r\n", self.positions[0].x, self.positions[0].y);
        print!("// RENDER:     x={:.1}, y={:.1}\r\n", self.positions[1].x, self.positions[1].y);
        print!("// LAYOUT:     x={:.1}, y={:.1}\r\n", self.positions[2].x, self.positions[2].y);
        print!("// BUFFER:     x={:.1}, y={:.1}\r\n", self.positions[3].x, self.positions[3].y);
        print!("// MEMORY:     x={:.1}, y={:.1}\r\n", self.positions[4].x, self.positions[4].y);
        print!("// CENTER BOX: x={:.1}, y={:.1}\r\n", self.positions[5].x, self.positions[5].y);
    }
}

fn render_logo(x: f32, y: f32, selected: bool) -> Element {
    let color = if selected {
        Color::Rgb(THEME.accent.0, THEME.accent.1, THEME.accent.2)
    } else {
        Color::Rgb(THEME.text.0, THEME.text.1, THEME.text.2)
    };

    let rows: Vec<Element> = LOGO_FILL
        .iter()
        .map(|line| {
            element! { Text(content: line.to_string(), color: color) }
        })
        .collect();

    element! {
        Box(position: Position::Absolute, inset_top: y, inset_left: x) {
            #(Element::column(rows))
        }
    }
}

fn render_panel(title: &str, x: f32, y: f32, selected: bool) -> Element {
    let bg = Color::Rgb(THEME.panel.0, THEME.panel.1, THEME.panel.2);
    let border_color = if selected {
        Color::Rgb(THEME.accent.0, THEME.accent.1, THEME.accent.2)
    } else {
        bg
    };
    let title_color = Color::Rgb(THEME.text.0, THEME.text.1, THEME.text.2);
    let text_color = Color::Rgb(THEME.dim.0, THEME.dim.1, THEME.dim.2);

    element! {
        Box(position: Position::Absolute, inset_top: y, inset_left: x) {
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

fn render_center_box(x: f32, y: f32, selected: bool) -> Element {
    let box_bg = Color::Rgb(20, 20, 28);
    let border_color = if selected {
        Color::Rgb(THEME.accent.0, THEME.accent.1, THEME.accent.2)
    } else {
        Color::Rgb(THEME.dim.0, THEME.dim.1, THEME.dim.2)
    };
    let title_color = Color::Rgb(THEME.text.0, THEME.text.1, THEME.text.2);

    element! {
        Box(position: Position::Absolute, inset_top: y, inset_left: x) {
            Box(
                border_style: BorderStyle::Round,
                border_color: border_color,
                background_color: box_bg,
                padding: 1.0,
                width: 38.0,
            ) {
                Box(flex_direction: FlexDirection::Column, align_items: AlignItems::Center, background_color: box_bg) {
                    Text(content: " SHOWCASE ", color: title_color, bold: true, bg_color: box_bg)
                    Spacer(lines: 1u16)
                    Text(content: "Content here", color: title_color, bg_color: box_bg)
                    Spacer(lines: 1u16)
                }
            }
        }
    }
}

fn render_status(state: &EditorState) -> Element {
    let sel = state.selected;
    let pos = &state.positions[sel];
    let status = format!(
        " [{}] {} | x: {:.1}  y: {:.1} | 1-6: select  Arrows: move  p: print  q: quit ",
        sel + 1,
        state.element_name(sel),
        pos.x,
        pos.y
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
    let p = &state.positions;
    let sel = state.selected;

    element! {
        Box(position: Position::Relative, width: WIDTH as f32, height: (HEIGHT + 1) as f32) {
            // Background
            Box(position: Position::Absolute, inset_top: 0.0, inset_left: 0.0) {
                #(render_background())
            }
            // Logo
            #(render_logo(p[0].x, p[0].y, sel == 0))
            // Panels
            #(render_panel("RENDER", p[1].x, p[1].y, sel == 1))
            #(render_panel("LAYOUT", p[2].x, p[2].y, sel == 2))
            #(render_panel("BUFFER", p[3].x, p[3].y, sel == 3))
            #(render_panel("MEMORY", p[4].x, p[4].y, sel == 4))
            // Center box
            #(render_center_box(p[5].x, p[5].y, sel == 5))
            // Status bar
            #(render_status(state))
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut blaeck = Blaeck::new(std::io::stdout())?;
    let mut state = EditorState::new();

    crossterm::terminal::enable_raw_mode()?;

    loop {
        if poll(Duration::from_millis(16))? {
            match read()? {
                Event::Key(key) => {
                    let step = if key.modifiers.contains(KeyModifiers::SHIFT) { 5.0 } else { 1.0 };

                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('p') => state.print_positions(),
                        KeyCode::Char('1') => state.selected = 0,
                        KeyCode::Char('2') => state.selected = 1,
                        KeyCode::Char('3') => state.selected = 2,
                        KeyCode::Char('4') => state.selected = 3,
                        KeyCode::Char('5') => state.selected = 4,
                        KeyCode::Char('6') => state.selected = 5,
                        KeyCode::Up => state.move_selected(0.0, -step),
                        KeyCode::Down => state.move_selected(0.0, step),
                        KeyCode::Left => state.move_selected(-step, 0.0),
                        KeyCode::Right => state.move_selected(step, 0.0),
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

    crossterm::terminal::disable_raw_mode()?;
    blaeck.unmount()?;

    // Print final positions
    state.print_positions();

    Ok(())
}
