//! Blaeck Showcase Demo
//!
//! The definitive demo for blaeck's capabilities.
//! 12-second cinematic loop showcasing:
//! - Plasma background with bloom effects
//! - Animated logo with elastic physics
//! - Live system metrics
//! - Component showcase (progress, sparkline, gradient)
//! - Theme switching
//!
//! Controls:
//!   q/Esc  - Quit
//!   Space  - Pause/resume
//!   t      - Cycle themes
//!   r      - Restart animation

use blaeck::prelude::*;
use std::time::{Duration, Instant};
use sysinfo::{Pid, ProcessesToUpdate, System};

// =============================================================================
// Constants
// =============================================================================

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 24;
pub const CYCLE_LENGTH: f64 = 20.0;

// Logo - chunky block style (NOCTERM inspired)
pub const LOGO_FILL: &[&str] = &[
    "████  █      ███  ████  ███  █  █",
    "█  █  █     █  █  █     █    █ █ ",
    "████  █     ████  ███   █    ██  ",
    "█  █  █     █  █  █     █    █ █ ",
    "████  ████  █  █  ████  ███  █  █",
];
pub const LOGO_WIDTH: usize = 34;
pub const LOGO_HEIGHT: usize = 5;

// =============================================================================
// Themes (from theater.rs)
// =============================================================================

#[derive(Clone, Copy)]
pub struct Theme {
    #[allow(dead_code)]
    pub name: &'static str,
    pub bg: (u8, u8, u8),        // Background / field base
    pub panel: (u8, u8, u8),     // Panel background
    pub text: (u8, u8, u8),      // Primary text
    pub dim: (u8, u8, u8),       // Dim/secondary text
    pub accent: (u8, u8, u8),    // Accent color (OK status, highlights)
}

pub const THEMES: [Theme; 5] = [
    // Forest - deep greens with mint accent
    Theme {
        name: "forest",
        bg: (0x00, 0x1A, 0x23),      // #001A23 - deep blue-black
        panel: (0x31, 0x49, 0x3C),   // #31493C - dark forest
        text: (0xE8, 0xF1, 0xF2),    // #E8F1F2 - off-white
        dim: (0x7A, 0x9E, 0x7E),     // #7A9E7E - sage
        accent: (0xB3, 0xEF, 0xB2),  // #B3EFB2 - mint
    },
    // Steel - cool blues and grays
    Theme {
        name: "steel",
        bg: (0x55, 0x46, 0x40),      // #554640 - dark taupe
        panel: (0x70, 0x70, 0x78),   // #707078 - gray
        text: (0xCD, 0xE6, 0xF5),    // #CDE6F5 - light blue
        dim: (0x87, 0x91, 0x9E),     // #87919E - blue-gray
        accent: (0x8D, 0xA7, 0xBE),  // #8DA7BE - steel blue
    },
    // Terracotta - warm earth tones
    Theme {
        name: "terra",
        bg: (0x4F, 0x6D, 0x7A),      // #4F6D7A - dark teal
        panel: (0x5A, 0x78, 0x85),   // slightly lighter teal
        text: (0xEA, 0xEA, 0xEA),    // #EAEAEA - off-white
        dim: (0xE8, 0xDA, 0xB2),     // #E8DAB2 - tan
        accent: (0xDD, 0x6E, 0x42),  // #DD6E42 - terracotta
    },
    // Garden - fresh greens with peach
    Theme {
        name: "garden",
        bg: (0x3A, 0x5A, 0x40),      // darker sage (adjusted for contrast)
        panel: (0x6A, 0x8D, 0x73),   // #6A8D73 - sage green
        text: (0xF4, 0xFD, 0xD9),    // #F4FDD9 - cream
        dim: (0xE4, 0xFF, 0xE1),     // #E4FFE1 - light mint
        accent: (0xF0, 0xA8, 0x68),  // #F0A868 - peach
    },
    // Mauve - elegant purples and charcoal (harmonized palette)
    Theme {
        name: "mauve",
        bg: (0x2E, 0x2C, 0x2F),      // #2E2C2F - charcoal
        panel: (0x47, 0x5B, 0x63),   // #475B63 - dark slate
        text: (0xF3, 0xE8, 0xEE),    // #F3E8EE - light mauve
        dim: (0x7A, 0x6F, 0x80),     // #7A6F80 - dusty mauve shadow (warm, not blue)
        accent: (0xCF, 0xA9, 0xFF),  // #CFA9FF - soft electric lavender
    },
];

pub fn panel_bg(theme: &Theme, pulse: f64) -> Color {
    let p = theme.panel;
    let pulse_boost = (pulse * 30.0) as u8;
    Color::Rgb(
        p.0.saturating_add(pulse_boost),
        p.1.saturating_add(pulse_boost / 2),
        p.2.saturating_add(pulse_boost / 3),
    )
}

// =============================================================================
// Plasma Field
// =============================================================================

pub fn plasma_value(nx: f64, ny: f64, time: f64, zoom: f64) -> f64 {
    let x = nx * zoom;
    let y = ny * zoom;
    let t = time * 0.8;

    let v1 = (x * 3.0 + t).sin();
    let v2 = (y * 4.0 - t * 0.7).sin();
    let v3 = ((x + y) * 2.5 + t * 0.5).sin();
    let v4 = ((x * x + y * y).sqrt() * 3.0 - t).sin();

    (v1 + v2 + v3 + v4) / 4.0
}

pub fn plasma_color(v: f64, theme: &Theme, intensity: f64, pulse: f64) -> Color {
    let bg = theme.bg;
    let pulse_boost = 1.0 + pulse * 0.3;

    let t = (v + 1.0) / 2.0; // 0-1
    let blend = t * intensity;

    let r = (bg.0 as f64 + blend * 60.0 + (1.0 - blend) * 30.0) * pulse_boost;
    let g = (bg.1 as f64 + blend * 50.0 + (1.0 - blend) * 25.0) * pulse_boost;
    let b = (bg.2 as f64 + blend * 70.0 + (1.0 - blend) * 35.0) * pulse_boost;

    Color::Rgb(r.min(255.0) as u8, g.min(255.0) as u8, b.min(255.0) as u8)
}

pub fn plasma_char(v: f64) -> char {
    const CHARS: &[char] = &[' ', '░', '▒', '▓', '█'];
    let idx = ((v + 1.0) / 2.0 * (CHARS.len() - 1) as f64).round() as usize;
    CHARS[idx.min(CHARS.len() - 1)]
}

pub fn plasma_effective_bg(v: f64, theme: &Theme, intensity: f64, pulse: f64) -> Color {
    let fg = plasma_color(v, theme, intensity, pulse);
    let coverage = match plasma_char(v * intensity) {
        '█' => 1.0,
        '▓' => 0.75,
        '▒' => 0.50,
        '░' => 0.25,
        _ => 0.0,
    };
    // Terminal background color
    let bg = (0x19u8, 0x2Bu8, 0x48u8);
    if let Color::Rgb(fr, fg, fb) = fg {
        Color::Rgb(
            (fr as f64 * coverage + bg.0 as f64 * (1.0 - coverage)) as u8,
            (fg as f64 * coverage + bg.1 as f64 * (1.0 - coverage)) as u8,
            (fb as f64 * coverage + bg.2 as f64 * (1.0 - coverage)) as u8,
        )
    } else {
        fg
    }
}





// =============================================================================
// Animation State
// =============================================================================

pub struct ShowcaseState {
    pub start: Instant,
    pub paused: bool,
    pub pause_time: f64,
    pub theme_index: usize,
    // Live metrics
    pub fps: f32,
    pub fps_samples: Vec<f32>,
    pub render_ms: f32,
    pub frame_count: u64,
    // Memory tracking
    pub sys: System,
    pub pid: Pid,
    pub memory_mb: f64,
    pub memory_delta: f64,
    pub last_memory_mb: f64,
    // Sparkline data
    pub sparkline_data: Vec<f64>,
}

impl ShowcaseState {
    pub fn new() -> Self {
        let pid = Pid::from_u32(std::process::id());
        let mut sys = System::new();
        sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
        let initial_mem = sys
            .process(pid)
            .map(|p| p.memory() as f64 / 1024.0 / 1024.0)
            .unwrap_or(0.0);

        Self {
            start: Instant::now(),
            paused: false,
            pause_time: 0.0,
            theme_index: 0,
            fps: 0.0,
            fps_samples: Vec::with_capacity(30),
            render_ms: 0.0,
            frame_count: 0,
            sys,
            pid,
            memory_mb: initial_mem,
            memory_delta: 0.0,
            last_memory_mb: initial_mem,
            sparkline_data: vec![0.3, 0.5, 0.7, 0.4, 0.6, 0.8, 0.5, 0.3, 0.6, 0.9, 0.7, 0.4],
        }
    }

    /// Create a preview state without sysinfo (for static snapshots)
    pub fn new_preview() -> Self {
        Self {
            start: Instant::now(),
            paused: false,
            pause_time: 5.0, // Mid-cycle for interesting preview
            theme_index: 0,
            fps: 60.0,
            fps_samples: vec![60.0; 10],
            render_ms: 2.5,
            frame_count: 300,
            sys: System::new(),
            pid: Pid::from_u32(std::process::id()),
            memory_mb: 15.0,
            memory_delta: 0.1,
            last_memory_mb: 14.9,
            sparkline_data: vec![0.3, 0.5, 0.7, 0.4, 0.6, 0.8, 0.5, 0.3, 0.6, 0.9, 0.7, 0.4],
        }
    }

    pub fn theme(&self) -> &'static Theme {
        &THEMES[self.theme_index % THEMES.len()]
    }

    pub fn next_theme(&mut self) {
        self.theme_index = (self.theme_index + 1) % THEMES.len();
    }

    pub fn pulse(&self) -> f64 {
        let t = self.cycle_time();
        // Pulse at showcase transitions
        let pulse_times = [3.2, 5.2, 7.2, 9.2];
        for pt in pulse_times {
            if t >= pt && t < pt + 0.3 {
                let p = (t - pt) / 0.3;
                return (1.0 - p) * (p * std::f64::consts::PI).sin();
            }
        }
        0.0
    }

    pub fn time(&self) -> f64 {
        if self.paused {
            self.pause_time
        } else {
            self.start.elapsed().as_secs_f64()
        }
    }

    pub fn cycle_time(&self) -> f64 {
        self.time() % CYCLE_LENGTH
    }

    pub fn restart(&mut self) {
        self.start = Instant::now();
        self.pause_time = 0.0;
    }

    pub fn toggle_pause(&mut self) {
        if self.paused {
            let offset = self.pause_time;
            self.start = Instant::now() - Duration::from_secs_f64(offset);
            self.paused = false;
        } else {
            self.pause_time = self.time();
            self.paused = true;
        }
    }

    pub fn update(&mut self, dt: f64, render_ms: f32) {
        self.frame_count += 1;

        // FPS calculation
        let instant_fps = if dt > 0.0 { 1.0 / dt as f32 } else { 0.0 };
        self.fps_samples.push(instant_fps);
        if self.fps_samples.len() > 30 {
            self.fps_samples.remove(0);
        }
        self.fps = self.fps_samples.iter().sum::<f32>() / self.fps_samples.len() as f32;
        self.render_ms = render_ms;

        // Update memory stats every 30 frames (~0.5s at 60fps)
        if self.frame_count % 30 == 0 {
            self.sys.refresh_processes(ProcessesToUpdate::Some(&[self.pid]), true);
            if let Some(proc) = self.sys.process(self.pid) {
                let new_mem = proc.memory() as f64 / 1024.0 / 1024.0;
                self.memory_delta = new_mem - self.last_memory_mb;
                self.last_memory_mb = self.memory_mb;
                self.memory_mb = new_mem;
            }
        }

        // Update sparkline with slight variation
        if self.frame_count % 5 == 0 {
            self.sparkline_data.remove(0);
            let last = *self.sparkline_data.last().unwrap_or(&0.5);
            let new = (last + (self.time().sin() * 0.2) + 0.1).clamp(0.1, 1.0);
            self.sparkline_data.push(new);
        }
    }

    // Animation phases (20s total)
    // 0.0-0.4s   IGNITION
    // 0.4-0.7s   LOGO SLAM
    // 0.7-2.0s   LOGO HOLD
    // 2.0-2.6s   LOGO RISE
    // 2.6-3.2s   PANELS FLY IN
    // 3.2-5.2s   SHOWCASE: Progress
    // 5.2-7.2s   SHOWCASE: Spinner + LogBox
    // 7.2-9.2s   SHOWCASE: Sparkline + BarChart
    // 9.2-11.2s  SHOWCASE: Syntax
    // 11.2-13.2s SHOWCASE: Table
    // 13.2-15.2s SHOWCASE: Diff
    // 15.2-16.0s OUTRO (fade)
    // 16.0-20.0s COLLAPSE

    pub fn plasma_intensity(&self) -> f64 {
        let t = self.cycle_time();
        if t < 0.4 {
            ease_out_cubic(t / 0.4)
        } else if t < 16.0 {
            1.0
        } else if t < 20.0 {
            1.0 - ease_in_cubic((t - 16.0) / 4.0)
        } else {
            0.0
        }
    }

    pub fn logo_position(&self) -> (f64, f64) {
        let t = self.cycle_time();
        let cx = (WIDTH - LOGO_WIDTH) as f64 / 2.0;
        let cy = (HEIGHT - LOGO_HEIGHT) as f64 / 2.0 - 2.0;
        let top_y = 1.0;

        if t < 0.4 {
            (cx, -10.0)
        } else if t < 0.7 {
            let p = (t - 0.4) / 0.3;
            let y = lerp(-10.0, cy, ease_out_bounce_once(p));
            (cx, y)
        } else if t < 2.0 {
            (cx, cy)
        } else if t < 2.6 {
            let p = ease_in_out_cubic((t - 2.0) / 0.6);
            let y = lerp(cy, top_y, p);
            (cx, y)
        } else if t < 15.2 {
            (cx, top_y)
        } else if t < 16.0 {
            let p = ease_in_cubic((t - 15.2) / 0.8);
            let y = lerp(top_y, -15.0, p);
            (cx, y)
        } else {
            (cx, -10.0)
        }
    }

    pub fn logo_opacity(&self) -> f64 {
        let t = self.cycle_time();
        if t < 0.4 {
            0.0
        } else if t < 0.6 {
            (t - 0.4) / 0.2
        } else if t < 15.2 {
            1.0
        } else if t < 16.0 {
            1.0 - (t - 15.2) / 0.8
        } else {
            0.0
        }
    }

    pub fn panels_opacity(&self) -> f64 {
        let t = self.cycle_time();
        if t < 2.2 {
            0.0
        } else if t < 3.2 {
            ease_out_cubic((t - 2.2) / 1.0)
        } else if t < 15.2 {
            1.0
        } else if t < 16.0 {
            1.0 - ease_in_cubic((t - 15.2) / 0.8)
        } else {
            0.0
        }
    }

    pub fn panels_x_offset(&self) -> f64 {
        let t = self.cycle_time();
        if t < 2.2 {
            -20.0
        } else if t < 3.2 {
            -20.0 * (1.0 - ease_out_back((t - 2.2) / 1.0))
        } else if t < 15.2 {
            0.0
        } else if t < 16.0 {
            -20.0 * ease_in_cubic((t - 15.2) / 0.8)
        } else {
            -20.0
        }
    }

    pub fn showcase_phase(&self) -> usize {
        let t = self.cycle_time();
        if t < 3.2 { 0 }
        else if t < 5.2 { 1 }    // Progress
        else if t < 7.2 { 2 }    // Spinner + LogBox
        else if t < 9.2 { 3 }    // Sparkline + BarChart
        else if t < 11.2 { 4 }   // Syntax
        else if t < 13.2 { 5 }   // Table
        else if t < 15.2 { 6 }   // Diff
        else { 0 }
    }

    pub fn showcase_opacity(&self) -> f64 {
        let t = self.cycle_time();
        if t < 3.2 {
            0.0
        } else if t < 3.5 {
            (t - 3.2) / 0.3
        } else if t < 14.9 {
            1.0
        } else if t < 15.2 {
            1.0 - (t - 14.9) / 0.3
        } else {
            0.0
        }
    }

    pub fn progress_value(&self) -> f64 {
        let t = self.cycle_time();
        if t < 3.2 { 0.0 }
        else if t < 5.2 {
            ease_out_cubic((t - 3.2) / 2.0)
        } else { 1.0 }
    }

    pub fn spinner_frame(&self) -> usize {
        // ~10 frames per second for spinner
        (self.time() * 10.0) as usize
    }
}

// =============================================================================
// Easing Functions
// =============================================================================

pub fn ease_out_cubic(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(3)
}

pub fn ease_in_cubic(t: f64) -> f64 {
    t.powi(3)
}

pub fn ease_in_out_cubic(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t.powi(3)
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

pub fn ease_out_back(t: f64) -> f64 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

pub fn ease_out_bounce_once(t: f64) -> f64 {
    // Goes to ~1.2, then settles to 1.0 - single visible bounce
    // Logo slams down past center, then bounces back up
    if t < 0.5 {
        // Fast move to overshoot (1.2 = 20% past target)
        let p = t / 0.5;
        1.2 * ease_out_cubic(p)
    } else {
        // Settle back from 1.2 to 1.0
        let p = (t - 0.5) / 0.5;
        1.2 - 0.2 * ease_out_cubic(p)
    }
}

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}


// =============================================================================
// Rendering
// =============================================================================

pub const VERSION_TEXT: &str = "blaeck v0.2.0";

pub fn render_plasma_bg(state: &ShowcaseState) -> Element {
    let time = state.time();
    let intensity = state.plasma_intensity();
    let pulse = state.pulse();
    let theme = state.theme();
    let zoom = 2.5; // More zoomed out

    let ver_len = VERSION_TEXT.len();
    let ver_start = WIDTH - ver_len - 1;
    let ver_end = ver_start + ver_len;
    let ver_row = HEIGHT - 1;
    let ver_bytes = VERSION_TEXT.as_bytes();
    let text_color = Color::Rgb(theme.dim.0, theme.dim.1, theme.dim.2);

    let rows: Vec<Element> = (0..HEIGHT)
        .map(|y| {
            let ny = y as f64 / HEIGHT as f64;
            let cells: Vec<Element> = (0..WIDTH)
                .map(|x| {
                    let nx = x as f64 / WIDTH as f64;
                    let v = plasma_value(nx, ny, time, zoom);
                    let color = plasma_color(v, theme, intensity, pulse);

                    if y == ver_row && x >= ver_start && x < ver_end {
                        let ch = ver_bytes[x - ver_start] as char;
                        let eff = plasma_effective_bg(v, theme, intensity, pulse);
                        element! { Text(content: ch.to_string(), color: text_color, bg_color: eff) }
                    } else {
                        let ch = plasma_char(v * intensity);
                        element! { Text(content: ch.to_string(), color: color) }
                    }
                })
                .collect();
            Element::row(cells)
        })
        .collect();

    Element::column(rows)
}

/// Render the logo with stacked offset stroke effect (NOCTERM style)
/// Multiple layers with slight offsets create fake 3D depth
/// Transparent background - plasma shows through
pub fn render_logo(state: &ShowcaseState, logo_x: f64, logo_y: f64) -> Element {
    let opacity = state.logo_opacity();
    if opacity <= 0.0 {
        return Element::Empty;
    }

    let time = state.time();
    let intensity = state.plasma_intensity();
    let theme = state.theme();
    let pulse_anim = ((time * 1.5).sin() * 0.5 + 0.5) as f32;

    // Get theme colors for the logo layers (use panel/dim/accent)
    let (pr, pg, pb) = theme.panel;
    let (dr, dg, db) = theme.dim;     // Warm mauve shadow
    let (ar, ag, ab) = theme.accent;  // Lavender glow

    // Glow multiplier - less aggressive, more atmospheric
    let glow = 1.0 + pulse_anim * 0.6;

    // Stacked colors with depth separation (4 layers)
    let colors: [(f32, f32, f32); 4] = [
        // BACK SHADOW — deep, anchored to bg warmth
        (dr as f32 * 0.40, dg as f32 * 0.40, db as f32 * 0.45),
        // MID-BACK — volume structure
        (dr as f32 * 0.65, dg as f32 * 0.65, db as f32 * 0.70),
        // MID-FRONT — slightly lifted panel tone
        (pr as f32 * 1.08, pg as f32 * 1.08, pb as f32 * 1.12),
        // FRONT GLOW — luminous mauve (blue channel gets slight extra push)
        (
            (ar as f32 * glow).min(255.0),
            (ag as f32 * glow).min(255.0),
            (ab as f32 * (glow + 0.05)).min(255.0),
        ),
    ];

    // Offsets for each layer (creates the stacked extrusion)
    let offsets: [(i32, i32); 4] = [
        (2, 2),   // back layer - offset down-right
        (1, 1),   // middle-back
        (0, 1),   // middle-front
        (0, 0),   // front layer - no offset
    ];

    // Build a grid that composites all layers
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

    for (row_idx, row) in grid.iter().enumerate() {
        let mut row_elements: Vec<Element> = Vec::new();
        let ny = (logo_y as usize + row_idx) as f64 / HEIGHT as f64;

        for (col_idx, cell) in row.iter().enumerate() {
            let nx = (logo_x as usize + col_idx) as f64 / WIDTH as f64;

            // Get plasma color for background
            let v = plasma_value(nx, ny, time, 2.5);
            let field_bg = plasma_color(v, theme, intensity, state.pulse());
            let (fr, fg, fb) = match field_bg {
                Color::Rgb(r, g, b) => (r as f32, g as f32, b as f32),
                _ => (15.0, 16.0, 32.0),
            };

            match cell {
                Some((r, g, b)) => {
                    // Logo pixel - blend with field based on opacity
                    let blend = opacity as f32;
                    let final_r = (fr * (1.0 - blend) + r * blend) as u8;
                    let final_g = (fg * (1.0 - blend) + g * blend) as u8;
                    let final_b = (fb * (1.0 - blend) + b * blend) as u8;

                    let text_color = Color::Rgb(final_r, final_g, final_b);
                    row_elements.push(element! {
                        Text(content: "█", color: text_color)
                    });
                }
                None => {
                    // Empty - render exactly like the plasma (transparent)
                    let ch = plasma_char(v * intensity);
                    row_elements.push(element! {
                        Text(content: ch.to_string(), color: field_bg)
                    });
                }
            }
        }
        rows.push(Element::row(row_elements));
    }

    Element::column(rows)
}

pub fn render_panel(title: &str, lines: Vec<(&str, String)>, state: &ShowcaseState) -> Element {
    let opacity = state.panels_opacity();
    if opacity <= 0.0 {
        return Element::Empty;
    }

    let theme = state.theme();
    let pulse = state.pulse();

    // Panel background with pulse effect (like theater)
    let bg = panel_bg(theme, pulse);

    let title_color = Color::Rgb(
        (theme.text.0 as f64 * opacity) as u8,
        (theme.text.1 as f64 * opacity) as u8,
        (theme.text.2 as f64 * opacity) as u8,
    );
    let text_color = Color::Rgb(
        (theme.dim.0 as f64 * opacity) as u8,
        (theme.dim.1 as f64 * opacity) as u8,
        (theme.dim.2 as f64 * opacity) as u8,
    );

    let mut content: Vec<Element> = vec![
        element! { Text(content: format!(" {} ", title), color: title_color, bold: true, bg_color: bg) },
    ];

    for (label, value) in lines {
        content.push(element! {
            Text(content: format!(" {:>6} {:<6} ", label, value), color: text_color, bg_color: bg)
        });
    }

    element! {
        Box(
            flex_direction: FlexDirection::Column,
            background_color: bg,
            padding_left: 1.0,
            padding_right: 1.0,
        ) {
            #(Element::column(content))
        }
    }
}

pub fn render_showcase(state: &ShowcaseState) -> Element {
    let opacity = state.showcase_opacity();
    if opacity <= 0.0 {
        return Element::Empty;
    }

    let phase = state.showcase_phase();
    let theme = state.theme();

    // Box colors
    let box_bg = Color::Rgb(20, 20, 28);  // Dark terminal-like background
    let border_color = Color::Rgb(
        (theme.dim.0 as f64 * opacity) as u8,
        (theme.dim.1 as f64 * opacity) as u8,
        (theme.dim.2 as f64 * opacity) as u8,
    );

    let title_color = Color::Rgb(
        (theme.text.0 as f64 * opacity) as u8,
        (theme.text.1 as f64 * opacity) as u8,
        (theme.text.2 as f64 * opacity) as u8,
    );
    let dim_color = Color::Rgb(
        (theme.dim.0 as f64 * opacity) as u8,
        (theme.dim.1 as f64 * opacity) as u8,
        (theme.dim.2 as f64 * opacity) as u8,
    );
    let accent_color = Color::Rgb(
        (theme.accent.0 as f64 * opacity) as u8,
        (theme.accent.1 as f64 * opacity) as u8,
        (theme.accent.2 as f64 * opacity) as u8,
    );

    let content = match phase {
        1 => {
            // Progress - using real component
            let progress_val = state.progress_value();
            let progress = Element::node::<Progress>(
                ProgressProps::new(progress_val as f32)
                    .width(28)
                    .style(ProgressStyle::Block)
                    .color(accent_color)
                    .bg_color(box_bg)
                    .show_percentage(),
                vec![],
            );

            element! {
                Box(flex_direction: FlexDirection::Column, align_items: AlignItems::Center, background_color: box_bg) {
                    Text(content: " PROGRESS ", color: title_color, bold: true, bg_color: box_bg)
                    Spacer(lines: 1u16)
                    #(progress)
                    Spacer(lines: 1u16)
                }
            }
        }
        2 => {
            // Spinner + LogBox combo
            let spinner_frames = SpinnerStyle::Dots.frames();
            let frame_idx = state.spinner_frame() % spinner_frames.len();
            let spinner_char = spinner_frames[frame_idx];

            let logs = vec![
                LogLine::new("Server started").color(title_color),
                LogLine::new("Pool: 8 conns").color(dim_color).dim(),
                LogLine::new("Health: OK").color(Color::Rgb(107, 203, 119)),
                LogLine::new("Cache: 12%").color(Color::Rgb(255, 201, 60)),
            ];
            let logbox = Element::node::<LogBox>(
                LogBoxProps::with_lines(logs).max_lines(4).bg_color(box_bg),
                vec![],
            );

            element! {
                Box(flex_direction: FlexDirection::Column, align_items: AlignItems::Center, background_color: box_bg) {
                    Box(flex_direction: FlexDirection::Row, background_color: box_bg) {
                        Text(content: format!(" {} ", spinner_char), color: accent_color, bg_color: box_bg)
                        Text(content: "LOADING ", color: title_color, bold: true, bg_color: box_bg)
                    }
                    Spacer(lines: 1u16)
                    #(logbox)
                }
            }
        }
        3 => {
            // Sparkline + BarChart combo
            let sparkline = Element::node::<Sparkline>(
                SparklineProps {
                    data: state.sparkline_data.clone(),
                    color: Some(accent_color),
                    bg_color: Some(box_bg),
                    ..Default::default()
                },
                vec![],
            );

            let bar_data = vec![
                BarData::new("CPU", 65.0).color(accent_color),
                BarData::new("MEM", 42.0).color(dim_color),
                BarData::new("IO", 28.0).color(title_color),
            ];
            let barchart = Element::node::<BarChart>(
                BarChartProps::new(bar_data)
                    .max_value(100.0)
                    .bar_width(12)
                    .show_values()
                    .bg_color(box_bg),
                vec![],
            );

            element! {
                Box(flex_direction: FlexDirection::Column, align_items: AlignItems::Center, background_color: box_bg) {
                    Text(content: " METRICS ", color: title_color, bold: true, bg_color: box_bg)
                    Spacer(lines: 1u16)
                    #(sparkline)
                    Spacer(lines: 1u16)
                    #(barchart)
                }
            }
        }
        4 => {
            // Syntax highlighting
            let code = "fn main() {\n    println!(\"Hi\");\n}";
            let syntax = Element::node::<SyntaxHighlight>(
                SyntaxHighlightProps::new(code)
                    .language("rust")
                    .theme(SyntaxTheme::OceanDark)
                    .bg_color(box_bg),
                vec![],
            );

            element! {
                Box(flex_direction: FlexDirection::Column, align_items: AlignItems::Center, background_color: box_bg) {
                    Text(content: " SYNTAX ", color: title_color, bold: true, bg_color: box_bg)
                    Spacer(lines: 1u16)
                    #(syntax)
                }
            }
        }
        5 => {
            // Table
            let rows = vec![
                Row::new(vec!["FPS", &format!("{:.0}", state.fps)]),
                Row::new(vec!["Mem", &format!("{:.1}MB", state.memory_mb)]),
                Row::new(vec!["Frame", &format!("{}", state.frame_count)]),
            ];
            let table = Element::node::<Table>(
                TableProps::new(rows)
                    .header(Row::new(vec!["Name", "Value"]))
                    .fixed_widths([5, 7])
                    .column_spacing(1)
                    .border(BorderStyle::Single)
                    .bg_color(box_bg),
                vec![],
            );

            element! {
                Box(flex_direction: FlexDirection::Column, align_items: AlignItems::Center, background_color: box_bg) {
                    Text(content: " TABLE ", color: title_color, bold: true, bg_color: box_bg)
                    Spacer(lines: 1u16)
                    #(table)
                }
            }
        }
        6 => {
            // Diff - using real component
            let diff_lines = vec![
                DiffLine::context("fn render() {"),
                DiffLine::removed("    let x = old;"),
                DiffLine::added("    let x = new();"),
                DiffLine::context("}"),
            ];
            let diff = Element::node::<Diff>(
                DiffProps::with_lines(diff_lines).bg_color(box_bg),
                vec![],
            );

            element! {
                Box(flex_direction: FlexDirection::Column, align_items: AlignItems::Center, background_color: box_bg) {
                    Text(content: " DIFF ", color: title_color, bold: true, bg_color: box_bg)
                    Spacer(lines: 1u16)
                    #(diff)
                }
            }
        }
        _ => Element::Empty,
    };

    // Wrap content in a bordered box (terminal-like window)
    if matches!(content, Element::Empty) {
        return Element::Empty;
    }

    element! {
        Box(
            border_style: BorderStyle::Round,
            border_color: border_color,
            background_color: box_bg,
            padding: 1.0,
            width: 38.0,
        ) {
            #(content)
        }
    }
}

pub fn build_showcase(state: &ShowcaseState) -> Element {
    let bg = render_plasma_bg(state);
    let (logo_x, logo_y) = state.logo_position();
    let logo = render_logo(state, logo_x, logo_y);

    let x_off = state.panels_x_offset();

    // Metric panels
    let render_panel_left = render_panel("RENDER", vec![
        ("fps", format!("{:.0}", state.fps)),
        ("lat", format!("{:.1}ms", state.render_ms)),
    ], state);

    let buffer_panel = render_panel("BUFFER", vec![
        ("cells", format!("{}", WIDTH * HEIGHT)),
        ("writes", format!("{}", state.frame_count % 100)),
    ], state);

    let layout_panel = render_panel("LAYOUT", vec![
        ("nodes", format!("{}", 42 + (state.time() * 2.0) as u32 % 20)),
        ("depth", format!("{}", 4 + (state.time() * 0.5) as u32 % 3)),
    ], state);

    let memory_panel = render_panel("MEMORY", vec![
        ("rss", format!("{:.1}MB", state.memory_mb)),
        ("Δ", format!("{:+.2}MB", state.memory_delta)),
    ], state);

    let showcase = render_showcase(state);

    // Layout with absolute positioning
    element! {
        Box(position: Position::Relative, width: WIDTH as f32, height: HEIGHT as f32) {
            // Background layer
            Box(position: Position::Absolute, inset_top: 0.0, inset_left: 0.0) {
                #(bg)
            }
            // Logo layer
            Box(position: Position::Absolute, inset_top: logo_y as f32, inset_left: logo_x as f32) {
                #(logo)
            }
            // Left panels
            Box(position: Position::Absolute, inset_top: 8.0, inset_left: (2.0 + x_off) as f32) {
                Box(flex_direction: FlexDirection::Column) {
                    #(render_panel_left)
                    Spacer(lines: 1u16)
                    #(buffer_panel)
                }
            }
            // Right panels
            Box(position: Position::Absolute, inset_top: 8.0, inset_right: (2.0 - x_off) as f32) {
                Box(flex_direction: FlexDirection::Column) {
                    #(layout_panel)
                    Spacer(lines: 1u16)
                    #(memory_panel)
                }
            }
            // Center showcase
            Box(position: Position::Absolute, inset_top: 9.0, inset_left: 20.0, inset_right: 20.0) {
                Box(justify_content: JustifyContent::Center, width: 40.0) {
                    #(showcase)
                }
            }
        }
    }
}

/// Static preview with showcase at a fixed time
pub fn build_ui() -> Element {
    let mut state = ShowcaseState::new_preview();
    // Set paused so time() returns pause_time (5.0 = mid-animation)
    state.paused = true;
    build_showcase(&state)
}
