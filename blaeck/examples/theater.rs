//! Capability Theater - A choreographed demonstration of blaeck's capabilities.
//!
//! A 20 second looping demo that shows off flexbox layout, dynamic reflow,
//! stateful updates, theme transitions, and animation coupling through motion.
//!
//! Controls:
//! - q/Esc: Quit
//! - Space: Pause/resume
//! - r: Restart from Act I

use blaeck::prelude::*;
use crossterm::event::{poll, read, Event, KeyCode};
use std::time::{Duration, Instant};

// ============================================================================
// Constants
// ============================================================================

const DEFAULT_WIDTH: usize = 120;
const DEFAULT_HEIGHT: usize = 30;

const SHADES: [char; 6] = ['█', '▓', '▒', '░', '·', ' '];

// ASCII art logo for Act 1 - clean block style
const LOGO: &[&str] = &[
    "▐█▄▄▄    █        ▄▄▄▄    ▐█▄▄▄   ▄▄▄▄   █   █",
    "▐█   █  █ █      █    █   ▐█     █       █  █ ",
    "▐█▄▄▄▀  █  █     █▄▄▄▄█   ▐█▄▄▄  █       █▄█  ",
    "▐█   █  █   █    █    █   ▐█     █       █  █ ",
    "▐█▄▄▄   █▄▄▄▄█   █    █   ▐█▄▄▄  ▀▄▄▄▄▀  █   █",
];

const LOGO_TAGLINE: &str = "terminal ui for rust";

// Timing (20 second loop)
const ACT1_END: f64 = 3.0;
const ACT2_END: f64 = 12.0;
const ACT3_END: f64 = 20.0;
const LOOP_DURATION: f64 = 20.0;

// Panel appearance times within Act II
const PANEL_INITIAL_START: f64 = 3.0;      // Single "RENDER PIPELINE" panel
const PANEL_SPLIT_START: f64 = 5.0;        // Split into RENDERER + LAYOUT
const PANEL_THIRD_START: f64 = 7.0;        // PROCESS panel slides in
const PANELS_ENTROPY_START: f64 = 8.5;     // ENTROPY bar
const PANELS_BOTTOM_START: f64 = 10.0;     // FIELD + STATUS
const THEME_TRANSITION_START: f64 = 9.0;

// Act III timing
const BRANDING_START: f64 = 13.0;
const FADEOUT_START: f64 = 18.0;

// ============================================================================
// Color Themes
// ============================================================================

#[derive(Clone, Copy)]
struct ThemeColors {
    field_base: (u8, u8, u8),
    field_blob: (u8, u8, u8),
    panel_border: (u8, u8, u8),
    panel_bg: (u8, u8, u8),
    text_primary: (u8, u8, u8),
    text_dim: (u8, u8, u8),
}

const THEME_COOL: ThemeColors = ThemeColors {
    field_base: (10, 10, 18),
    field_blob: (26, 26, 46),
    panel_border: (58, 80, 107),
    panel_bg: (12, 12, 20),
    text_primary: (201, 209, 217),
    text_dim: (100, 100, 120),
};

const THEME_WARM: ThemeColors = ThemeColors {
    field_base: (18, 12, 10),
    field_blob: (46, 32, 26),
    panel_border: (180, 100, 80),
    panel_bg: (20, 14, 12),
    text_primary: (240, 230, 211),
    text_dim: (140, 120, 100),
};

// ============================================================================
// Field Parameters
// ============================================================================

struct FieldParams {
    freq1: f64,
    freq2: f64,
    freq3: f64,
    freq4: f64,
}

impl Default for FieldParams {
    fn default() -> Self {
        Self { freq1: 6.0, freq2: 5.0, freq3: 4.0, freq4: 8.0 }
    }
}

// ============================================================================
// Act State Machine
// ============================================================================

#[derive(Clone, Copy, PartialEq, Debug)]
enum Act {
    One,
    Two,
    Three,
}

impl Act {
    fn from_time(t: f64) -> Self {
        let t = t % LOOP_DURATION;
        if t < ACT1_END { Act::One }
        else if t < ACT2_END { Act::Two }
        else { Act::Three }
    }

    fn field_brightness(&self, loop_time: f64) -> f64 {
        match self {
            Act::One => 0.4 + (loop_time / ACT1_END) * 0.1,
            Act::Two => 1.0,
            Act::Three => 1.0 - ((loop_time - ACT2_END) / (ACT3_END - ACT2_END)) * 0.3,
        }
    }
}

// ============================================================================
// Stress Events (Field-UI Coupling)
// ============================================================================

#[derive(Clone)]
struct StressEvent {
    x: f64,
    y: f64,
    intensity: f64,
    start_time: f64,
    duration: f64,
}

impl StressEvent {
    fn influence_at(&self, x: f64, y: f64, current_time: f64) -> f64 {
        let age = current_time - self.start_time;
        if age < 0.0 || age >= self.duration { return 0.0; }
        let decay = 1.0 - (age / self.duration);
        let dx = x - self.x;
        let dy = y - self.y;
        let dist = (dx * dx + dy * dy).sqrt();
        let falloff = 1.0 / (1.0 + dist * 5.0);
        self.intensity * falloff * decay
    }
}

// ============================================================================
// Real Metrics
// ============================================================================

#[cfg(target_os = "macos")]
fn get_memory_usage() -> Option<u64> {
    use std::process::Command;
    let pid = std::process::id();
    let output = Command::new("ps")
        .args(["-o", "rss=", "-p", &pid.to_string()])
        .output().ok()?;
    let rss_kb: u64 = String::from_utf8_lossy(&output.stdout).trim().parse().ok()?;
    Some(rss_kb * 1024)
}

#[cfg(target_os = "linux")]
fn get_memory_usage() -> Option<u64> {
    use std::fs;
    let status = fs::read_to_string("/proc/self/status").ok()?;
    for line in status.lines() {
        if line.starts_with("VmRSS:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let kb: u64 = parts[1].parse().ok()?;
                return Some(kb * 1024);
            }
        }
    }
    None
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn get_memory_usage() -> Option<u64> { None }

#[derive(Clone)]
struct RendererStats {
    frames: u64,
    fps: f32,
    fps_samples: Vec<f32>,
    latency_ms: f32,
    latency_samples: Vec<f32>,
}

impl RendererStats {
    fn new() -> Self {
        Self {
            frames: 0, fps: 0.0, fps_samples: Vec::with_capacity(30),
            latency_ms: 0.0, latency_samples: Vec::with_capacity(30),
        }
    }

    fn update(&mut self, dt: f64, render_time_ms: f32) {
        self.frames += 1;
        let instant_fps = if dt > 0.0 { 1.0 / dt as f32 } else { 0.0 };
        self.fps_samples.push(instant_fps);
        if self.fps_samples.len() > 30 { self.fps_samples.remove(0); }
        self.fps = self.fps_samples.iter().sum::<f32>() / self.fps_samples.len() as f32;
        self.latency_samples.push(render_time_ms);
        if self.latency_samples.len() > 30 { self.latency_samples.remove(0); }
        self.latency_ms = self.latency_samples.iter().sum::<f32>() / self.latency_samples.len() as f32;
    }
}

#[derive(Clone)]
struct LayoutStats {
    nodes: u32,
    depth: u8,
    renders: u64,
}

impl LayoutStats {
    fn new() -> Self { Self { nodes: 0, depth: 0, renders: 0 } }
    fn update(&mut self, node_count: u32, tree_depth: u8) {
        self.nodes = node_count;
        self.depth = tree_depth;
        self.renders += 1;
    }
}

#[derive(Clone)]
struct MemoryStats {
    baseline_mb: f32,
    used_mb: f32,
    peak_mb: f32,
}

impl MemoryStats {
    fn new() -> Self {
        let initial = get_memory_usage().unwrap_or(0) as f32 / (1024.0 * 1024.0);
        Self { baseline_mb: initial, used_mb: initial, peak_mb: initial }
    }
    fn update(&mut self) {
        if let Some(bytes) = get_memory_usage() {
            self.used_mb = bytes as f32 / (1024.0 * 1024.0);
            self.peak_mb = self.peak_mb.max(self.used_mb);
        }
    }
    fn delta(&self) -> f32 { self.used_mb - self.baseline_mb }
}

#[derive(Clone)]
struct FieldStats {
    energy_pct: u8,
    drift: f32,
    last_energy: f32,
}

impl FieldStats {
    fn new() -> Self { Self { energy_pct: 50, drift: 0.0, last_energy: 0.5 } }
    fn update(&mut self, field_energy: f64) {
        let energy = field_energy as f32;
        self.drift = energy - self.last_energy;
        self.last_energy = energy;
        self.energy_pct = (energy * 100.0).clamp(0.0, 100.0) as u8;
    }
}

fn count_element_tree(element: &Element) -> (u32, u8) {
    fn count_recursive(el: &Element, depth: u8) -> (u32, u8) {
        match el {
            Element::Empty => (0, depth),
            Element::Text { .. } => (1, depth),
            Element::Node { children, .. } => {
                let mut total = 1u32;
                let mut max_depth = depth;
                for child in children {
                    let (c, d) = count_recursive(child, depth + 1);
                    total += c;
                    max_depth = max_depth.max(d);
                }
                (total, max_depth)
            }
            Element::Fragment(children) => {
                let mut total = 0u32;
                let mut max_depth = depth;
                for child in children {
                    let (c, d) = count_recursive(child, depth);
                    total += c;
                    max_depth = max_depth.max(d);
                }
                (total, max_depth)
            }
        }
    }
    count_recursive(element, 0)
}

// ============================================================================
// Panel Definition
// ============================================================================

#[derive(Clone)]
struct Panel {
    // Target dimensions (what we're animating toward)
    target_x: usize,
    target_y: usize,
    target_width: usize,
    target_height: usize,
    // Current animated dimensions
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    // Animation state
    opacity: f64,
    border_progress: f64,
    scale: f64, // 0.98 → 1.0 for scale-in effect
    // Content
    title: String,
    lines: Vec<(String, String, Option<Color>)>,
    // Progress bars: (current_value, target_value, color)
    progress_bars: Vec<(f64, f64, (u8, u8, u8))>,
}

impl Panel {
    fn new(x: usize, y: usize, width: usize, height: usize, title: &str) -> Self {
        Self {
            target_x: x,
            target_y: y,
            target_width: width,
            target_height: height,
            x: x as f64,
            y: y as f64,
            width: width as f64,
            height: height as f64,
            opacity: 0.0,
            border_progress: 0.0,
            scale: 0.90,
            title: title.to_string(),
            lines: Vec::new(),
            progress_bars: Vec::new(),
        }
    }

    fn set_lines(&mut self, lines: Vec<(String, String, Option<Color>)>) {
        self.lines = lines;
    }

    fn set_progress_bar(&mut self, index: usize, target: f64, color: (u8, u8, u8)) {
        while self.progress_bars.len() <= index {
            self.progress_bars.push((0.0, 0.0, (100, 160, 100)));
        }
        self.progress_bars[index].1 = target;
        self.progress_bars[index].2 = color;
    }

    fn animate_toward_target(&mut self, dt: f64) {
        let lerp_speed = 4.0 * dt;
        self.x = lerp(self.x, self.target_x as f64, lerp_speed);
        self.y = lerp(self.y, self.target_y as f64, lerp_speed);
        self.width = lerp(self.width, self.target_width as f64, lerp_speed);
        self.height = lerp(self.height, self.target_height as f64, lerp_speed);

        // Animate progress bars toward their targets
        for (current, target, _) in &mut self.progress_bars {
            *current = lerp(*current, *target, lerp_speed * 0.5);
        }
    }

    // Get effective dimensions with scale applied
    fn effective_dims(&self) -> (usize, usize, usize, usize) {
        let ew = (self.width * self.scale).round() as usize;
        let eh = (self.height * self.scale).round() as usize;
        // Center the scaled panel
        let offset_x = ((self.width - self.width * self.scale) / 2.0).round() as usize;
        let offset_y = ((self.height - self.height * self.scale) / 2.0).round() as usize;
        let ex = self.x.round() as usize + offset_x;
        let ey = self.y.round() as usize + offset_y;
        (ex, ey, ew.max(4), eh.max(3))
    }
}

// ============================================================================
// Theater State
// ============================================================================

struct TheaterState {
    // Real metrics
    renderer: RendererStats,
    layout: LayoutStats,
    memory: MemoryStats,
    field_stats: FieldStats,

    // Panels
    panels: Vec<Panel>,

    // Animation state
    field_time: f64,
    stress_events: Vec<StressEvent>,
    theme_progress: f64,
    branding_opacity: f64,
    logo_opacity: f64,

    // Split animation state
    split_phase: SplitPhase,
    split_progress: f64, // 0 = single panel, 1 = fully split

    // Control
    paused: bool,
    loop_start: Instant,
    memory_update_counter: u32,

    // Track stress events
    added_initial_stress: bool,
    added_split_stress: bool,
    added_third_stress: bool,
    added_entropy_stress: bool,
    added_bottom_stress: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum SplitPhase {
    None,           // No panels yet
    SinglePanel,    // One "RENDER PIPELINE" panel
    Splitting,      // Animating the split
    Split,          // Two panels: RENDERER + LAYOUT
    ThirdAdded,     // PROCESS added
}

impl TheaterState {
    fn new() -> Self {
        Self {
            renderer: RendererStats::new(),
            layout: LayoutStats::new(),
            memory: MemoryStats::new(),
            field_stats: FieldStats::new(),
            panels: Vec::new(),
            field_time: 0.0,
            stress_events: Vec::new(),
            theme_progress: 0.0,
            branding_opacity: 0.0,
            logo_opacity: 0.0,
            split_phase: SplitPhase::None,
            split_progress: 0.0,
            paused: false,
            loop_start: Instant::now(),
            memory_update_counter: 0,
            added_initial_stress: false,
            added_split_stress: false,
            added_third_stress: false,
            added_entropy_stress: false,
            added_bottom_stress: false,
        }
    }

    fn loop_time(&self) -> f64 {
        self.loop_start.elapsed().as_secs_f64() % LOOP_DURATION
    }

    fn restart(&mut self) {
        self.loop_start = Instant::now();
        self.panels.clear();
        self.stress_events.clear();
        self.theme_progress = 0.0;
        self.branding_opacity = 0.0;
        self.logo_opacity = 0.0;
        self.split_phase = SplitPhase::None;
        self.split_progress = 0.0;
        self.added_initial_stress = false;
        self.added_split_stress = false;
        self.added_third_stress = false;
        self.added_entropy_stress = false;
        self.added_bottom_stress = false;
    }

    fn add_stress(&mut self, x: f64, y: f64, intensity: f64, duration: f64) {
        self.stress_events.push(StressEvent {
            x, y, intensity, start_time: self.field_time, duration,
        });
    }

    fn total_stress_at(&self, x: f64, y: f64) -> f64 {
        self.stress_events.iter()
            .map(|e| e.influence_at(x, y, self.field_time))
            .sum::<f64>().min(1.0)
    }

    fn cleanup_old_events(&mut self) {
        self.stress_events.retain(|e| self.field_time < e.start_time + e.duration);
    }

    fn update_metrics(&mut self, dt: f64, render_time_ms: f32, field_energy: f64, node_count: u32, tree_depth: u8) {
        if !self.paused {
            self.field_time += dt * 0.15;
        }
        self.renderer.update(dt, render_time_ms);
        self.layout.update(node_count, tree_depth);
        self.field_stats.update(field_energy);
        self.memory_update_counter += 1;
        if self.memory_update_counter >= 30 {
            self.memory.update();
            self.memory_update_counter = 0;
        }
        self.cleanup_old_events();

        // Animate panels toward their targets
        for panel in &mut self.panels {
            panel.animate_toward_target(dt);
        }
    }
}

// ============================================================================
// Easing & Color Interpolation
// ============================================================================

fn ease_out_cubic(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(3)
}

fn ease_in_out_cubic(t: f64) -> f64 {
    if t < 0.5 { 4.0 * t * t * t }
    else { 1.0 - (-2.0 * t + 2.0).powi(3) / 2.0 }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

fn lerp_u8(a: u8, b: u8, t: f64) -> u8 {
    lerp(a as f64, b as f64, t) as u8
}

fn lerp_color(a: (u8, u8, u8), b: (u8, u8, u8), t: f64) -> (u8, u8, u8) {
    (lerp_u8(a.0, b.0, t), lerp_u8(a.1, b.1, t), lerp_u8(a.2, b.2, t))
}

// ============================================================================
// Field Rendering
// ============================================================================

fn plasma_value(nx: f64, ny: f64, time: f64, params: &FieldParams) -> f64 {
    let v1 = (nx * params.freq1 + time).sin();
    let v2 = (ny * params.freq2 + time).cos();
    let v3 = ((nx + ny) * params.freq3 + time).sin();
    let v4 = ((nx * nx + ny * ny).sqrt() * params.freq4 - time).cos();
    (v1 + v2 + v3 + v4) / 4.0
}

fn value_to_char(v: f64) -> char {
    if v > 0.6 { SHADES[0] }
    else if v > 0.3 { SHADES[1] }
    else if v > 0.0 { SHADES[2] }
    else if v > -0.3 { SHADES[3] }
    else if v > -0.6 { SHADES[4] }
    else { SHADES[5] }
}

fn field_color(value: f64, stress: f64, theme_progress: f64, brightness: f64, x: f64, y: f64) -> Color {
    // Spatial diffusion for theme transition
    let edge_dist = ((x - 0.5).abs() + (y - 0.5).abs()) / 1.0;
    let local_progress = if theme_progress > 0.0 {
        (theme_progress * 1.5 - (1.0 - edge_dist) * 0.5).clamp(0.0, 1.0)
    } else { 0.0 };

    let base = lerp_color(THEME_COOL.field_base, THEME_WARM.field_base, local_progress);
    let blob = lerp_color(THEME_COOL.field_blob, THEME_WARM.field_blob, local_progress);

    let intensity = (value + 1.0) / 2.0;
    let stress_boost = (stress * 60.0) as u8;

    let r = lerp_u8(base.0, blob.0, intensity);
    let g = lerp_u8(base.1, blob.1, intensity);
    let b = lerp_u8(base.2, blob.2, intensity);

    let r = ((r as f64 * brightness) as u8).saturating_add(stress_boost);
    let g = ((g as f64 * brightness) as u8).saturating_add(stress_boost / 2);
    let b = ((b as f64 * brightness) as u8).saturating_add(stress_boost / 3);

    Color::Rgb(r, g, b)
}

// ============================================================================
// Grid-Based Rendering (Panels overlay on field)
// ============================================================================

fn build_grid(
    width: usize,
    height: usize,
    state: &TheaterState,
    params: &FieldParams,
    frame_ms: f64,
) -> Vec<Vec<(char, Color, Option<Color>)>> {
    let loop_time = state.loop_time();
    let act = Act::from_time(loop_time);
    let brightness = act.field_brightness(loop_time);

    // Initialize grid with field
    let mut grid: Vec<Vec<(char, Color, Option<Color>)>> = (0..height)
        .map(|y| {
            let ny = y as f64 / height as f64;
            (0..width)
                .map(|x| {
                    let nx = x as f64 / width as f64;
                    let value = plasma_value(nx, ny, state.field_time, params);
                    let stress = state.total_stress_at(nx, ny);
                    let color = field_color(value, stress, state.theme_progress, brightness, nx, ny);
                    (value_to_char(value), color, None)
                })
                .collect()
        })
        .collect();

    // Draw panels on top
    for panel in &state.panels {
        if panel.opacity < 0.01 { continue; }

        let (px, py, pw, ph) = panel.effective_dims();
        if pw < 4 || ph < 3 { continue; }

        let border_color = lerp_color(THEME_COOL.panel_border, THEME_WARM.panel_border, state.theme_progress);
        let bg_color = lerp_color(THEME_COOL.panel_bg, THEME_WARM.panel_bg, state.theme_progress);
        let text_color = lerp_color(THEME_COOL.text_primary, THEME_WARM.text_primary, state.theme_progress);
        let dim_color = lerp_color(THEME_COOL.text_dim, THEME_WARM.text_dim, state.theme_progress);

        let op = panel.opacity;
        let bc = Color::Rgb(
            (border_color.0 as f64 * op) as u8,
            (border_color.1 as f64 * op) as u8,
            (border_color.2 as f64 * op) as u8,
        );
        let bg = Color::Rgb(bg_color.0, bg_color.1, bg_color.2);
        let tc = Color::Rgb(
            (text_color.0 as f64 * op) as u8,
            (text_color.1 as f64 * op) as u8,
            (text_color.2 as f64 * op) as u8,
        );
        let dc = Color::Rgb(
            (dim_color.0 as f64 * op) as u8,
            (dim_color.1 as f64 * op) as u8,
            (dim_color.2 as f64 * op) as u8,
        );

        // Draw border with typewriter effect
        let perimeter = 2 * pw + 2 * ph;
        let drawn = (perimeter as f64 * panel.border_progress) as usize;
        let mut count = 0;

        // Top edge
        for x in px..=(px + pw).min(width - 1) {
            if count >= drawn { break; }
            let ch = if x == px { '┌' } else if x == px + pw { '┐' } else { '─' };
            if py < height { grid[py][x] = (ch, bc, None); }
            count += 1;
        }
        // Right edge
        for y in (py + 1)..=(py + ph).min(height - 1) {
            if count >= drawn { break; }
            let ch = if y == py + ph { '┘' } else { '│' };
            let edge_x = (px + pw).min(width - 1);
            grid[y][edge_x] = (ch, bc, None);
            count += 1;
        }
        // Bottom edge
        for x in (px..(px + pw)).rev() {
            if count >= drawn { break; }
            let ch = if x == px { '└' } else { '─' };
            let edge_y = (py + ph).min(height - 1);
            if x < width { grid[edge_y][x] = (ch, bc, None); }
            count += 1;
        }
        // Left edge
        for y in ((py + 1)..(py + ph)).rev() {
            if count >= drawn { break; }
            if y < height { grid[y][px] = ('│', bc, None); }
            count += 1;
        }

        // Fill interior with background
        for y in (py + 1)..(py + ph).min(height) {
            for x in (px + 1)..(px + pw).min(width) {
                grid[y][x] = (' ', tc, Some(bg));
            }
        }

        // Draw title
        let title_x = px + 2;
        let title_y = py + 1;
        if title_y < height {
            for (i, ch) in panel.title.chars().enumerate() {
                let char_x = title_x + i;
                if char_x < width && char_x < px + pw {
                    grid[title_y][char_x] = (ch, tc, Some(bg));
                }
            }
        }

        // Draw content lines
        for (li, (label, value, color_override)) in panel.lines.iter().enumerate() {
            let ly = py + 2 + li;
            if ly >= height || ly >= py + ph { break; }

            // Label
            for (i, ch) in label.chars().enumerate() {
                let char_x = px + 2 + i;
                if char_x < width && char_x < px + pw - 1 {
                    grid[ly][char_x] = (ch, dc, Some(bg));
                }
            }
            // Value (after label)
            let value_x = px + 2 + label.len();
            let vc = color_override.unwrap_or(tc);
            for (i, ch) in value.chars().enumerate() {
                let char_x = value_x + i;
                if char_x < width && char_x < px + pw - 1 {
                    grid[ly][char_x] = (ch, vc, Some(bg));
                }
            }
        }

        // Draw progress bars
        for (bar_idx, (current, _target, color)) in panel.progress_bars.iter().enumerate() {
            let bar_y = py + 2 + panel.lines.len() + bar_idx;
            if bar_y >= height || bar_y >= py + ph - 1 { break; }

            let bar_start = px + 2;
            let bar_width = pw.saturating_sub(4);
            let filled = (bar_width as f64 * current).round() as usize;

            for i in 0..bar_width {
                let bar_x = bar_start + i;
                if bar_x >= width || bar_x >= px + pw - 1 { break; }

                if i < filled {
                    let bar_color = Color::Rgb(
                        (color.0 as f64 * op) as u8,
                        (color.1 as f64 * op) as u8,
                        (color.2 as f64 * op) as u8,
                    );
                    grid[bar_y][bar_x] = ('▓', bar_color, Some(bg));
                } else {
                    grid[bar_y][bar_x] = ('░', dc, Some(bg));
                }
            }
        }
    }

    // Frame time indicator (top right, equal padding from top and right = 3)
    if act != Act::One {
        let stress_sum: f64 = state.stress_events.iter()
            .filter(|e| state.field_time >= e.start_time && state.field_time < e.start_time + e.duration)
            .map(|e| e.intensity * 0.2)
            .sum();
        let display_ms = frame_ms + stress_sum * 4.0;
        let bar_fill = ((display_ms / 20.0) * 5.0) as usize;
        let bar = format!("FRAME {}{} {:>4.1}ms",
            "▓".repeat(bar_fill.min(5)),
            "░".repeat(5 - bar_fill.min(5)),
            display_ms
        );
        let fc = if display_ms < 10.0 {
            Color::Rgb(100, 160, 100)
        } else {
            Color::Rgb(180, 160, 100)
        };
        let padding = 3;
        let start_x = width.saturating_sub(bar.len() + padding);
        let start_y = padding;
        for (i, ch) in bar.chars().enumerate() {
            if start_x + i < width && start_y < height {
                grid[start_y][start_x + i] = (ch, fc, None);
            }
        }
    }

    // Branding (bottom right, Act III)
    if state.branding_opacity > 0.01 {
        let gray = (100.0 * state.branding_opacity) as u8;
        let bc = Color::Rgb(gray, gray, (120.0 * state.branding_opacity) as u8);
        let brand = "blaeck 0.2";
        let start_x = width.saturating_sub(brand.len() + 2);
        let start_y = height.saturating_sub(2);
        for (i, ch) in brand.chars().enumerate() {
            if start_x + i < width && start_y < height {
                grid[start_y][start_x + i] = (ch, bc, None);
            }
        }
    }

    // Big ASCII logo (Act I)
    if state.logo_opacity > 0.01 {
        let logo_width = LOGO.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        let logo_height = LOGO.len();
        let logo_x = width.saturating_sub(logo_width) / 2;
        let logo_y = height.saturating_sub(logo_height + 3) / 2; // Center vertically, slightly up

        // Calculate logo color based on opacity and field energy at center
        let center_value = plasma_value(0.5, 0.5, state.field_time, params);
        let intensity = (center_value + 1.0) / 2.0;
        let base_brightness = 80.0 + intensity * 60.0;

        let logo_color = Color::Rgb(
            (base_brightness * state.logo_opacity * 0.9) as u8,
            (base_brightness * state.logo_opacity * 0.95) as u8,
            ((base_brightness + 20.0) * state.logo_opacity) as u8,
        );

        // Draw logo
        for (row_idx, row) in LOGO.iter().enumerate() {
            let y = logo_y + row_idx;
            if y >= height { continue; }
            for (col_idx, ch) in row.chars().enumerate() {
                let x = logo_x + col_idx;
                if x >= width { continue; }
                if ch != ' ' {
                    grid[y][x] = (ch, logo_color, None);
                }
            }
        }

        // Draw tagline below logo
        let tagline_x = width.saturating_sub(LOGO_TAGLINE.len()) / 2;
        let tagline_y = logo_y + logo_height + 1;
        if tagline_y < height {
            let tagline_color = Color::Rgb(
                (60.0 * state.logo_opacity) as u8,
                (60.0 * state.logo_opacity) as u8,
                (80.0 * state.logo_opacity) as u8,
            );
            for (i, ch) in LOGO_TAGLINE.chars().enumerate() {
                let x = tagline_x + i;
                if x < width {
                    grid[tagline_y][x] = (ch, tagline_color, None);
                }
            }
        }
    }

    grid
}

fn grid_to_element(grid: Vec<Vec<(char, Color, Option<Color>)>>) -> Element {
    let rows: Vec<Element> = grid.into_iter()
        .map(|row| {
            let cells: Vec<Element> = row.into_iter()
                .map(|(ch, fg, bg)| {
                    if let Some(bg_color) = bg {
                        element! { Text(content: ch.to_string(), color: fg, bg_color: bg_color) }
                    } else {
                        element! { Text(content: ch.to_string(), color: fg) }
                    }
                })
                .collect();
            Element::row(cells)
        })
        .collect();
    Element::column(rows)
}

// ============================================================================
// Choreography
// ============================================================================

fn update_choreography(state: &mut TheaterState, _width: usize, _height: usize) {
    let loop_time = state.loop_time();
    let act = Act::from_time(loop_time);

    // Reset on loop
    if loop_time < 0.1 && !state.panels.is_empty() {
        state.restart();
    }

    match act {
        Act::One => {
            state.panels.clear();
            state.theme_progress = 0.0;
            state.branding_opacity = 0.0;
            state.split_phase = SplitPhase::None;
            state.split_progress = 0.0;

            // Logo fades in during Act 1, then fades out near the end
            if loop_time < 1.0 {
                // Fade in over first second
                state.logo_opacity = ease_out_cubic(loop_time / 1.0);
            } else if loop_time > ACT1_END - 0.8 {
                // Fade out in last 0.8 seconds of Act 1
                let fade_progress = (loop_time - (ACT1_END - 0.8)) / 0.8;
                state.logo_opacity = 1.0 - ease_in_out_cubic(fade_progress);
            } else {
                state.logo_opacity = 1.0;
            }
        }
        Act::Two => {
            // ============================================================
            // CAPABILITY 1: Single "RENDER PIPELINE" panel appears (T+3s)
            // ============================================================
            if loop_time >= PANEL_INITIAL_START && state.split_phase == SplitPhase::None {
                state.split_phase = SplitPhase::SinglePanel;

                // Create ONE wide panel centered
                // Width: 44 chars, centered at (120-44)/2 = 38
                let mut p = Panel::new(38, 5, 44, 7, "RENDER PIPELINE");
                p.x = 38.0;
                p.y = 5.0;
                p.width = 44.0;
                p.height = 7.0;
                state.panels.push(p);

                if !state.added_initial_stress {
                    state.added_initial_stress = true;
                    state.add_stress(0.5, 0.22, 1.0, 1.0);
                }
            }

            // Animate the single panel appearing
            if state.split_phase == SplitPhase::SinglePanel && state.panels.len() == 1 {
                let progress = ((loop_time - PANEL_INITIAL_START) / 1.0).clamp(0.0, 1.0);
                let eased = ease_out_cubic(progress);
                state.panels[0].opacity = eased;
                state.panels[0].border_progress = ease_out_cubic((progress * 1.2).min(1.0));
                state.panels[0].scale = lerp(0.90, 1.0, eased);
            }

            // ============================================================
            // CAPABILITY 2: Panel SPLITS into RENDERER + LAYOUT (T+5s)
            // ============================================================
            if loop_time >= PANEL_SPLIT_START && state.split_phase == SplitPhase::SinglePanel {
                state.split_phase = SplitPhase::Splitting;

                // Add second panel (starts at same position, will animate apart)
                let mut p2 = Panel::new(60, 5, 22, 7, "LAYOUT");
                p2.x = 38.0; // Start at same position as first panel
                p2.y = 5.0;
                p2.width = 44.0; // Start at same width
                p2.height = 7.0;
                p2.opacity = 0.0;
                p2.border_progress = 1.0; // Already has border
                p2.scale = 1.0;
                state.panels.push(p2);

                // Update first panel's target to become RENDERER
                state.panels[0].title = "RENDERER".to_string();
                state.panels[0].target_x = 26;
                state.panels[0].target_width = 22;

                if !state.added_split_stress {
                    state.added_split_stress = true;
                    state.add_stress(0.35, 0.22, 1.2, 0.8);
                    state.add_stress(0.65, 0.22, 1.2, 0.8);
                }
            }

            // Animate the split
            if state.split_phase == SplitPhase::Splitting && state.panels.len() >= 2 {
                let progress = ((loop_time - PANEL_SPLIT_START) / 1.0).clamp(0.0, 1.0);
                let eased = ease_out_cubic(progress);
                state.split_progress = eased;

                // First panel shrinks and moves left
                // (animate_toward_target handles the smooth movement)

                // Second panel fades in and moves right
                state.panels[1].opacity = ease_out_cubic((progress * 1.5 - 0.3).clamp(0.0, 1.0));
                state.panels[1].target_x = 50;
                state.panels[1].target_width = 22;

                if progress >= 0.95 {
                    state.split_phase = SplitPhase::Split;
                }
            }

            // ============================================================
            // CAPABILITY 3: PROCESS panel slides in (T+7s)
            // ============================================================
            if loop_time >= PANEL_THIRD_START && state.split_phase == SplitPhase::Split {
                state.split_phase = SplitPhase::ThirdAdded;

                // Third panel (PROCESS) slides in from right
                let mut p3 = Panel::new(74, 5, 20, 7, "PROCESS");
                p3.x = 100.0; // Start off-screen right
                p3.y = 5.0;
                p3.width = 20.0;
                p3.height = 7.0;
                p3.opacity = 1.0;
                p3.border_progress = 1.0;
                p3.scale = 1.0;
                state.panels.push(p3);

                if !state.added_third_stress {
                    state.added_third_stress = true;
                    state.add_stress(0.75, 0.22, 0.8, 0.8);
                }
            }

            // Keep animating third panel into position
            if state.split_phase == SplitPhase::ThirdAdded && state.panels.len() >= 3 {
                let progress = ((loop_time - PANEL_THIRD_START) / 0.8).clamp(0.0, 1.0);
                let _eased = ease_out_cubic(progress);
                // animate_toward_target handles smooth movement to target_x=74
            }

            // ============================================================
            // ENTROPY panel (T+8.5s)
            // ============================================================
            if loop_time >= PANELS_ENTROPY_START {
                let progress = ((loop_time - PANELS_ENTROPY_START) / 0.8).clamp(0.0, 1.0);
                let eased = ease_out_cubic(progress);

                // Find or create entropy panel (index 3 after the three top panels)
                let entropy_idx = 3;
                if state.panels.len() == entropy_idx {
                    let mut p = Panel::new(26, 13, 68, 4, "ENTROPY");
                    p.scale = 0.90;
                    state.panels.push(p);

                    if !state.added_entropy_stress {
                        state.added_entropy_stress = true;
                        state.add_stress(0.5, 0.48, 1.0, 0.6);
                    }
                }

                if state.panels.len() > entropy_idx {
                    state.panels[entropy_idx].opacity = eased;
                    state.panels[entropy_idx].border_progress = ease_out_cubic((progress * 1.2).min(1.0));
                    state.panels[entropy_idx].scale = lerp(0.90, 1.0, eased);
                }
            }

            // ============================================================
            // Bottom row: FIELD + STATUS (T+10s)
            // ============================================================
            if loop_time >= PANELS_BOTTOM_START {
                let progress = ((loop_time - PANELS_BOTTOM_START) / 1.0).clamp(0.0, 1.0);
                let eased = ease_out_cubic(progress);

                // FIELD panel (index 4)
                if state.panels.len() == 4 {
                    let mut p1 = Panel::new(26, 18, 24, 6, "FIELD");
                    p1.scale = 0.96;
                    state.panels.push(p1);
                }

                // STATUS panel (index 5)
                if state.panels.len() == 5 {
                    let mut p2 = Panel::new(52, 18, 42, 6, "STATUS");
                    p2.scale = 0.96;
                    state.panels.push(p2);

                    if !state.added_bottom_stress {
                        state.added_bottom_stress = true;
                        state.add_stress(0.32, 0.65, 0.8, 0.8);
                        state.add_stress(0.60, 0.65, 0.8, 0.8);
                    }
                }

                if state.panels.len() > 4 {
                    state.panels[4].opacity = eased;
                    state.panels[4].border_progress = ease_out_cubic((progress * 1.2).min(1.0));
                    state.panels[4].scale = lerp(0.90, 1.0, eased);
                }
                if state.panels.len() > 5 {
                    state.panels[5].opacity = eased;
                    state.panels[5].border_progress = ease_out_cubic((progress * 1.2).min(1.0));
                    state.panels[5].scale = lerp(0.90, 1.0, eased);
                }
            }

            // ============================================================
            // Theme transition (T+9s)
            // ============================================================
            if loop_time >= THEME_TRANSITION_START {
                let progress = ((loop_time - THEME_TRANSITION_START) / 2.5).clamp(0.0, 1.0);
                state.theme_progress = ease_in_out_cubic(progress);
            }
        }
        Act::Three => {
            // Keep panels visible but slightly dimmed
            for panel in &mut state.panels {
                panel.opacity = 0.85;
                panel.scale = 1.0;
            }

            // Branding fades in
            if loop_time >= BRANDING_START {
                let progress = ((loop_time - BRANDING_START) / 1.0).clamp(0.0, 1.0);
                state.branding_opacity = ease_out_cubic(progress);
            }

            // Theme transitions back
            let theme_decay = ((loop_time - ACT2_END) / (ACT3_END - ACT2_END)).clamp(0.0, 1.0);
            state.theme_progress = lerp(1.0, 0.3, theme_decay);

            // Fadeout
            if loop_time >= FADEOUT_START {
                let fade_progress = ((loop_time - FADEOUT_START) / 2.0).clamp(0.0, 1.0);
                let fade = 1.0 - ease_in_out_cubic(fade_progress);
                for panel in &mut state.panels {
                    panel.opacity = 0.85 * fade;
                }
                state.branding_opacity *= fade;
            }
        }
    }

    // Update panel content with real metrics
    update_panel_content(state);
}

fn update_panel_content(state: &mut TheaterState) {
    match state.split_phase {
        SplitPhase::None => {}

        SplitPhase::SinglePanel => {
            // Single "RENDER PIPELINE" panel shows combined stats
            if state.panels.len() > 0 {
                state.panels[0].set_lines(vec![
                    (format!("frames  "), format!("{:>8}", state.renderer.frames), None),
                    (format!("nodes   "), format!("{:>8}", state.layout.nodes), None),
                    (format!("latency "), format!("{:>6.1}ms", state.renderer.latency_ms), None),
                ]);
                // Progress bar for FPS (target 60)
                let fps_pct = (state.renderer.fps / 60.0).clamp(0.0, 1.0) as f64;
                state.panels[0].set_progress_bar(0, fps_pct, (100, 180, 140));
            }
        }

        SplitPhase::Splitting | SplitPhase::Split | SplitPhase::ThirdAdded => {
            // RENDERER panel (index 0)
            if state.panels.len() > 0 {
                state.panels[0].set_lines(vec![
                    (format!("frames "), format!("{:>6}", state.renderer.frames), None),
                    (format!("fps    "), format!("{:>6.0}", state.renderer.fps), None),
                    (format!("latency"), format!("{:>5.1}ms", state.renderer.latency_ms), None),
                ]);
                // FPS progress bar
                let fps_pct = (state.renderer.fps / 60.0).clamp(0.0, 1.0) as f64;
                state.panels[0].set_progress_bar(0, fps_pct, (100, 180, 140));
            }

            // LAYOUT panel (index 1)
            if state.panels.len() > 1 {
                state.panels[1].set_lines(vec![
                    (format!("nodes  "), format!("{:>5}", state.layout.nodes), None),
                    (format!("depth  "), format!("{:>5}", state.layout.depth), None),
                    (format!("renders"), format!("{:>5}", state.layout.renders), None),
                ]);
                // Layout complexity bar (based on nodes, max ~500)
                let complexity = (state.layout.nodes as f64 / 500.0).clamp(0.0, 1.0);
                state.panels[1].set_progress_bar(0, complexity, (140, 140, 180));
            }

            // PROCESS panel (index 2)
            if state.panels.len() > 2 {
                let delta = state.memory.delta();
                let delta_color = if delta > 1.0 {
                    Some(Color::Rgb(255, 201, 60))
                } else { None };
                state.panels[2].set_lines(vec![
                    (format!("rss  "), format!("{:>6.1}MB", state.memory.used_mb), None),
                    (format!("peak "), format!("{:>6.1}MB", state.memory.peak_mb), None),
                    (format!("alloc"), format!("{:>+5.1}MB", delta), delta_color),
                ]);
                // Memory usage bar (percentage of peak)
                let mem_pct = if state.memory.peak_mb > 0.0 {
                    (state.memory.used_mb / state.memory.peak_mb).clamp(0.0, 1.0) as f64
                } else { 0.0 };
                state.panels[2].set_progress_bar(0, mem_pct, (180, 140, 100));
            }

            // ENTROPY panel (index 3) - special rendering handled separately
            if state.panels.len() > 3 {
                state.panels[3].lines.clear();
            }

            // FIELD panel (index 4)
            if state.panels.len() > 4 {
                let drift_sign = if state.field_stats.drift >= 0.0 { "+" } else { "" };
                state.panels[4].set_lines(vec![
                    (format!("energy "), format!("{:>5}%", state.field_stats.energy_pct), None),
                    (format!("drift  "), format!("{}{:.4}", drift_sign, state.field_stats.drift), None),
                ]);
                // Energy bar
                let energy_pct = state.field_stats.energy_pct as f64 / 100.0;
                state.panels[4].set_progress_bar(0, energy_pct, (100, 160, 150));
            }

            // STATUS panel (index 5)
            if state.panels.len() > 5 {
                let ok = Color::Rgb(107, 203, 119);
                let dormant = Color::Rgb(74, 74, 90);
                state.panels[5].set_lines(vec![
                    (format!("[core]   "), format!("operational"), Some(ok)),
                    (format!("[render] "), format!("operational"), Some(ok)),
                    (format!("[input]  "), format!("operational"), Some(ok)),
                    (format!("[panic]  "), format!("dormant"), Some(dormant)),
                ]);
            }
        }
    }
}

// ============================================================================
// Special Panel Rendering (Entropy bar, Signal bar)
// ============================================================================

fn draw_special_panels(
    grid: &mut Vec<Vec<(char, Color, Option<Color>)>>,
    state: &TheaterState,
    params: &FieldParams,
    width: usize,
    height: usize,
) {
    // ENTROPY bar (panel index 3 when we have all panels)
    let entropy_idx = 3;
    if state.panels.len() > entropy_idx && state.panels[entropy_idx].opacity > 0.01 {
        let panel = &state.panels[entropy_idx];
        let (px, py, pw, _ph) = panel.effective_dims();
        let bar_y = py + 2;
        let bar_start = px + 2;
        let bar_width = pw.saturating_sub(4);

        if bar_y < height && bar_width > 0 {
            for i in 0..bar_width {
                let char_x = bar_start + i;
                if char_x >= width { break; }
                let nx = i as f64 / bar_width as f64;
                let value = plasma_value(nx, 0.5, state.field_time, params);
                let base = lerp_color(THEME_COOL.field_blob, THEME_WARM.field_blob, state.theme_progress);
                let intensity = (value + 1.0) / 2.0;
                let r = (base.0 as f64 * (0.5 + intensity * 0.5) * panel.opacity) as u8;
                let g = (base.1 as f64 * (0.5 + intensity * 0.5) * panel.opacity) as u8;
                let b = (base.2 as f64 * (0.5 + intensity * 0.8) * panel.opacity) as u8;
                let bg = lerp_color(THEME_COOL.panel_bg, THEME_WARM.panel_bg, state.theme_progress);
                grid[bar_y][char_x] = (value_to_char(value), Color::Rgb(r, g, b), Some(Color::Rgb(bg.0, bg.1, bg.2)));
            }
        }
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() -> std::io::Result<()> {
    let mut blaeck = Blaeck::new(std::io::stdout())?;
    blaeck.set_max_fps(60);
    let params = FieldParams::default();
    let mut state = TheaterState::new();
    let mut last_time = Instant::now();

    // Get terminal size
    let (mut width, mut height) = crossterm::terminal::size()
        .map(|(w, h)| (w as usize, h as usize))
        .unwrap_or((DEFAULT_WIDTH + 4, DEFAULT_HEIGHT + 4));
    width = width.saturating_sub(2).min(DEFAULT_WIDTH).max(40);
    height = height.saturating_sub(2).min(DEFAULT_HEIGHT).max(10);

    crossterm::terminal::enable_raw_mode()?;

    loop {
        if poll(Duration::from_millis(16))? {
            match read()? {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => break,
                        KeyCode::Char(' ') => state.paused = !state.paused,
                        KeyCode::Char('r') => state.restart(),
                        _ => {}
                    }
                }
                Event::Resize(w, h) => {
                    width = (w as usize).saturating_sub(2).min(DEFAULT_WIDTH).max(40);
                    height = (h as usize).saturating_sub(2).min(DEFAULT_HEIGHT).max(10);
                    let _ = blaeck.handle_resize(w, h);
                }
                _ => {}
            }
        }

        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f64();
        last_time = now;

        // Update choreography
        update_choreography(&mut state, width, height);

        // Build grid with field background and panels
        let mut grid = build_grid(width, height, &state, &params, state.renderer.latency_ms as f64);

        // Draw special panels (entropy bar, signal bar)
        draw_special_panels(&mut grid, &state, &params, width, height);

        // Convert to element
        let frame = grid_to_element(grid);

        // Count elements
        let (node_count, tree_depth) = count_element_tree(&frame);

        // Render
        let render_start = Instant::now();
        blaeck.render(element! {
            Box(flex_direction: FlexDirection::Column) {
                #(frame)
                Text(content: format!("Act {:?} | Space:pause  r:restart  q:quit", Act::from_time(state.loop_time())), dim: true)
            }
        })?;
        let render_time_ms = render_start.elapsed().as_secs_f32() * 1000.0;

        // Calculate field energy
        let avg_energy = (plasma_value(0.5, 0.5, state.field_time, &params) + 1.0) / 2.0;

        // Update metrics
        state.update_metrics(dt, render_time_ms, avg_energy, node_count, tree_depth);
    }

    crossterm::terminal::disable_raw_mode()?;
    blaeck.unmount()?;

    Ok(())
}
