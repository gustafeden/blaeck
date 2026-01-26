//! Scalar Field Dashboard - A living system monitor where UI emerges from chaos.
//!
//! The plasma field isn't decoration—it's the system's energy substrate.
//! UI panels react to the underlying field, creating a terminal that feels sentient.
//!
//! Controls:
//! - q/Esc: Quit
//! - r: Restart boot sequence
//! - Space: Pause/resume field animation

use blaeck::prelude::*;
use crossterm::event::{poll, read, Event, KeyCode};
use std::time::{Duration, Instant};

// ============================================================================
// Constants
// ============================================================================

const WIDTH: usize = 82;
const HEIGHT: usize = 24;

const SHADES: [char; 6] = ['█', '▓', '▒', '░', '·', ' '];

// ============================================================================
// Color Palette (restrained, serious - NOT neon cyberpunk)
// ============================================================================

mod palette {
    use blaeck::prelude::Color;

    // Base colors (when field is calm)
    pub const PANEL_BORDER: Color = Color::Rgb(58, 80, 107);   // #3a506b
    pub const TEXT: Color = Color::Rgb(201, 209, 217);         // #c9d1d9
    pub const TEXT_DIM: Color = Color::Rgb(100, 100, 120);     // dimmed

    // Status colors
    pub const OK: Color = Color::Rgb(107, 203, 119);           // #6bcb77
    pub const DORMANT: Color = Color::Rgb(74, 74, 90);         // #4a4a5a
    pub const WARNING: Color = Color::Rgb(255, 201, 60);       // #ffc93c
}

// ============================================================================
// Field Parameters (slow, meditative movement)
// ============================================================================

struct FieldParams {
    freq1: f64,
    freq2: f64,
    freq3: f64,
    freq4: f64,
}

impl Default for FieldParams {
    fn default() -> Self {
        // "Slow Flow" style - very slow, meditative
        Self {
            freq1: 6.0,
            freq2: 5.0,
            freq3: 4.0,
            freq4: 8.0,
        }
    }
}

// ============================================================================
// Boot Sequence
// ============================================================================

#[derive(Clone, Copy, PartialEq)]
enum BootPhase {
    Black,           // T+0.0 - 0.3s
    FieldIgniting,   // T+0.3 - 0.8s
    FieldFull,       // T+0.8 - 1.2s
    InitText,        // T+1.2 - 1.5s
    SystemChecks,    // T+1.5 - 2.5s
    ChecksComplete,  // T+2.5 - 3.0s
    PanelsFadeIn,    // T+3.0 - 3.8s
    LogoAppear,      // T+3.8 - 4.5s
    Running,         // T+4.5+ steady state
}

impl BootPhase {
    fn from_time(t: f64) -> Self {
        if t < 0.3 { BootPhase::Black }
        else if t < 0.8 { BootPhase::FieldIgniting }
        else if t < 1.2 { BootPhase::FieldFull }
        else if t < 1.5 { BootPhase::InitText }
        else if t < 2.5 { BootPhase::SystemChecks }
        else if t < 3.0 { BootPhase::ChecksComplete }
        else if t < 3.8 { BootPhase::PanelsFadeIn }
        else if t < 4.5 { BootPhase::LogoAppear }
        else { BootPhase::Running }
    }

    fn field_intensity(&self, boot_time: f64) -> f64 {
        match self {
            BootPhase::Black => 0.0,
            BootPhase::FieldIgniting => ((boot_time - 0.3) / 0.5).clamp(0.0, 1.0) * 0.5,
            BootPhase::FieldFull => 0.5 + ((boot_time - 0.8) / 0.4).clamp(0.0, 1.0) * 0.5,
            _ => 1.0,
        }
    }

    fn panel_opacity(&self, boot_time: f64, panel_index: usize) -> f64 {
        match self {
            BootPhase::PanelsFadeIn | BootPhase::LogoAppear | BootPhase::Running => {
                let panel_start = 3.0 + (panel_index as f64) * 0.15;
                ((boot_time - panel_start) / 0.3).clamp(0.0, 1.0)
            }
            _ => 0.0,
        }
    }

    fn logo_opacity(&self, boot_time: f64) -> f64 {
        match self {
            BootPhase::LogoAppear | BootPhase::Running => {
                ((boot_time - 4.0) / 0.5).clamp(0.0, 1.0)
            }
            _ => 0.0,
        }
    }
}

// ============================================================================
// System Check Display
// ============================================================================

struct SystemCheck {
    name: &'static str,
    start_time: f64,
    dots_duration: f64,
}

const SYSTEM_CHECKS: &[SystemCheck] = &[
    SystemCheck { name: "renderer", start_time: 1.5, dots_duration: 0.15 },
    SystemCheck { name: "scheduler", start_time: 1.7, dots_duration: 0.12 },
    SystemCheck { name: "entropy pool", start_time: 1.9, dots_duration: 0.18 },
    SystemCheck { name: "field stable", start_time: 2.1, dots_duration: 0.20 },
];

fn render_system_checks(boot_time: f64) -> Vec<Element> {
    let mut lines = vec![];

    if (1.2..2.8).contains(&boot_time) {
        lines.push(element! {
            Text(content: "initializing...", color: palette::TEXT_DIM, dim: true)
        });
        lines.push(element! { Newline });
    }

    for check in SYSTEM_CHECKS {
        if boot_time < check.start_time {
            continue;
        }

        let elapsed = boot_time - check.start_time;
        let dots_done = elapsed >= check.dots_duration;

        let target_dots = 16 - check.name.len();
        let dots = if dots_done {
            ".".repeat(target_dots)
        } else {
            let dot_count = ((elapsed / check.dots_duration) * target_dots as f64) as usize;
            ".".repeat(dot_count)
        };

        let status = if dots_done && boot_time >= check.start_time + check.dots_duration + 0.1 {
            element! { Text(content: " OK", color: palette::OK) }
        } else {
            Element::Empty
        };

        lines.push(element! {
            Box(flex_direction: FlexDirection::Row) {
                Text(content: format!("{} {}", check.name, dots), color: palette::TEXT_DIM, dim: true)
                #(status)
            }
        });
    }

    lines
}

// ============================================================================
// Live Stats (Real Metrics)
// ============================================================================

/// Get current process memory usage in bytes (macOS/Linux)
fn get_memory_usage() -> Option<u64> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let pid = std::process::id();
        let output = Command::new("ps")
            .args(["-o", "rss=", "-p", &pid.to_string()])
            .output()
            .ok()?;
        let rss_kb: u64 = String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse()
            .ok()?;
        Some(rss_kb * 1024) // Convert KB to bytes
    }
    #[cfg(target_os = "linux")]
    {
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
    {
        None
    }
}

#[derive(Clone)]
struct RendererStats {
    frames: u64,
    fps: f32,
    fps_samples: Vec<f32>,  // Rolling window for FPS smoothing
    latency_ms: f32,
    latency_samples: Vec<f32>,  // Rolling window for latency smoothing
}

impl RendererStats {
    fn new() -> Self {
        Self {
            frames: 0,
            fps: 0.0,
            fps_samples: Vec::with_capacity(30),
            latency_ms: 0.0,
            latency_samples: Vec::with_capacity(30),
        }
    }

    fn update(&mut self, dt: f64, render_time_ms: f32) {
        self.frames += 1;

        // Calculate instantaneous FPS from delta time
        let instant_fps = if dt > 0.0 { 1.0 / dt as f32 } else { 0.0 };

        // Rolling average for smooth FPS display
        self.fps_samples.push(instant_fps);
        if self.fps_samples.len() > 30 {
            self.fps_samples.remove(0);
        }
        self.fps = self.fps_samples.iter().sum::<f32>() / self.fps_samples.len() as f32;

        // Rolling average for smooth latency display
        self.latency_samples.push(render_time_ms);
        if self.latency_samples.len() > 30 {
            self.latency_samples.remove(0);
        }
        self.latency_ms = self.latency_samples.iter().sum::<f32>() / self.latency_samples.len() as f32;
    }
}

#[derive(Clone)]
struct LayoutStats {
    nodes: u32,
    depth: u8,
    renders: u64,  // Total render count
}

impl LayoutStats {
    fn new() -> Self {
        Self { nodes: 0, depth: 0, renders: 0 }
    }

    fn update(&mut self, node_count: u32, tree_depth: u8) {
        self.nodes = node_count;
        self.depth = tree_depth;
        self.renders += 1;
    }
}

#[derive(Clone)]
struct MemoryStats {
    baseline_mb: Option<f32>,  // Memory after first render (lazy init)
    used_mb: f32,
    peak_mb: f32,
}

impl MemoryStats {
    fn new() -> Self {
        Self {
            baseline_mb: None,
            used_mb: 0.0,
            peak_mb: 0.0,
        }
    }

    fn update(&mut self) {
        if let Some(bytes) = get_memory_usage() {
            self.used_mb = bytes as f32 / (1024.0 * 1024.0);
            // Capture baseline on first successful read (after libs are loaded)
            if self.baseline_mb.is_none() {
                self.baseline_mb = Some(self.used_mb);
            }
            self.peak_mb = self.peak_mb.max(self.used_mb);
        }
    }

    fn delta(&self) -> f32 {
        self.baseline_mb.map(|b| self.used_mb - b).unwrap_or(0.0)
    }
}

/// Field energy stats - represents the scalar field's current state
#[derive(Clone)]
struct FieldStats {
    energy_pct: u8,      // Current field energy as percentage
    avg_intensity: f32,  // Rolling average intensity
    drift: f32,          // Rate of change
    last_energy: f32,
}

impl FieldStats {
    fn new() -> Self {
        Self {
            energy_pct: 50,
            avg_intensity: 0.5,
            drift: 0.0,
            last_energy: 0.5,
        }
    }

    fn update(&mut self, field_energy: f64) {
        let energy = field_energy as f32;

        // Calculate drift (rate of change)
        self.drift = energy - self.last_energy;
        self.last_energy = energy;

        // Smooth average
        self.avg_intensity = self.avg_intensity * 0.95 + energy * 0.05;

        // Percentage
        self.energy_pct = (energy * 100.0).clamp(0.0, 100.0) as u8;
    }
}

/// Count nodes and max depth in an Element tree
fn count_element_tree(element: &Element) -> (u32, u8) {
    fn count_recursive(el: &Element, depth: u8) -> (u32, u8) {
        match el {
            Element::Empty => (0, depth),
            Element::Text { .. } => (1, depth),
            Element::Node { children, .. } => {
                let mut total = 1u32;
                let mut max_depth = depth;
                for child in children {
                    let (child_count, child_depth) = count_recursive(child, depth + 1);
                    total += child_count;
                    max_depth = max_depth.max(child_depth);
                }
                (total, max_depth)
            }
            Element::Fragment(children) => {
                let mut total = 0u32;
                let mut max_depth = depth;
                for child in children {
                    let (child_count, child_depth) = count_recursive(child, depth);
                    total += child_count;
                    max_depth = max_depth.max(child_depth);
                }
                (total, max_depth)
            }
        }
    }
    count_recursive(element, 0)
}

#[derive(Clone)]
struct DashboardState {
    renderer: RendererStats,
    layout: LayoutStats,
    memory: MemoryStats,
    field: FieldStats,
    paused: bool,
    boot_start: Instant,
    field_time: f64,
    memory_update_counter: u32,  // Only update memory every N frames
}

impl DashboardState {
    fn new() -> Self {
        Self {
            renderer: RendererStats::new(),
            layout: LayoutStats::new(),
            memory: MemoryStats::new(),
            field: FieldStats::new(),
            paused: false,
            boot_start: Instant::now(),
            field_time: 0.0,
            memory_update_counter: 0,
        }
    }

    fn boot_time(&self) -> f64 {
        self.boot_start.elapsed().as_secs_f64()
    }

    fn restart_boot(&mut self) {
        self.boot_start = Instant::now();
        self.field_time = 0.0;
        self.renderer = RendererStats::new();
        self.layout = LayoutStats::new();
    }

    fn update(&mut self, dt: f64, render_time_ms: f32, field_energy: f64, node_count: u32, tree_depth: u8) {
        if !self.paused {
            self.field_time += dt * 0.15; // Slow field animation
        }
        self.renderer.update(dt, render_time_ms);
        self.layout.update(node_count, tree_depth);
        self.field.update(field_energy);

        // Update memory less frequently (every 30 frames) to avoid overhead
        self.memory_update_counter += 1;
        if self.memory_update_counter >= 30 {
            self.memory.update();
            self.memory_update_counter = 0;
        }
    }
}

// ============================================================================
// Field Calculations
// ============================================================================

fn plasma_value(nx: f64, ny: f64, time: f64, params: &FieldParams) -> f64 {
    let v1 = (nx * params.freq1 + time).sin();
    let v2 = (ny * params.freq2 + time).cos();
    let v3 = ((nx + ny) * params.freq3 + time).sin();
    let v4 = ((nx * nx + ny * ny).sqrt() * params.freq4 - time).cos();
    (v1 + v2 + v3 + v4) / 4.0
}

fn panel_field_energy(panel_x: f64, panel_y: f64, time: f64, params: &FieldParams) -> f64 {
    let value = plasma_value(panel_x, panel_y, time, params);
    (value + 1.0) / 2.0 // Convert -1..1 to 0..1
}

fn value_to_char(v: f64) -> char {
    if v > 0.6 { SHADES[0] }
    else if v > 0.3 { SHADES[1] }
    else if v > 0.0 { SHADES[2] }
    else if v > -0.3 { SHADES[3] }
    else if v > -0.6 { SHADES[4] }
    else { SHADES[5] }
}

fn field_color(v: f64, time: f64, intensity: f64) -> Color {
    // Restrained blue-gray base with warm amber highlights
    let base_r = 26.0 + (v + 1.0) * 15.0;
    let base_g = 26.0 + (v + 1.0) * 10.0;
    let base_b = 46.0 + (v + 1.0) * 20.0;

    // Add warm tones when field value is high
    let warmth = ((v + 1.0) / 2.0).powf(2.0);
    let pulse = (time * 0.5).sin() * 0.1 + 0.9;

    let r = (base_r + warmth * 80.0 * pulse) * intensity;
    let g = (base_g + warmth * 40.0 * pulse) * intensity;
    let b = (base_b - warmth * 20.0) * intensity;

    Color::Rgb(
        r.clamp(0.0, 255.0) as u8,
        g.clamp(0.0, 255.0) as u8,
        b.clamp(0.0, 255.0) as u8,
    )
}

fn border_color_with_energy(field_energy: f64, opacity: f64) -> Color {
    let base = palette::PANEL_BORDER;
    if let Color::Rgb(r, g, b) = base {
        let boost = (field_energy * 60.0) as u8;
        Color::Rgb(
            ((r.saturating_add(boost)) as f64 * opacity) as u8,
            ((g.saturating_add(boost / 2)) as f64 * opacity) as u8,
            ((b.saturating_add(boost / 3)) as f64 * opacity) as u8,
        )
    } else {
        base
    }
}

fn text_color_with_glow(field_energy: f64) -> Color {
    if field_energy > 0.6 {
        let warmth = ((field_energy - 0.6) * 100.0) as u8;
        Color::Rgb(200 + warmth / 2, 180, 160)
    } else {
        palette::TEXT
    }
}

// ============================================================================
// Entropy Bar
// ============================================================================

fn entropy_bar_with_bg(width: usize, y_pos: f64, time: f64, params: &FieldParams) -> Element {
    let cells: Vec<Element> = (0..width)
        .map(|x| {
            let nx = x as f64 / width as f64;
            let value = plasma_value(nx, y_pos, time, params);
            let fg = field_color(value, time, 1.0);
            let bg = field_bg_at(nx, y_pos, time, params);
            element! { Text(content: value_to_char(value).to_string(), color: fg, bg_color: bg) }
        })
        .collect();

    Element::row(cells)
}

// ============================================================================
// Signal Strength Bar
// ============================================================================

fn signal_bar_with_bg(strength_pct: u8, width: usize, time: f64, params: &FieldParams, y_pos: f64) -> Element {
    let filled = (width as f32 * strength_pct as f32 / 100.0) as usize;
    let cells: Vec<Element> = (0..width)
        .map(|x| {
            let nx = x as f64 / width as f64;
            let bg = field_bg_at(nx * 0.3 + 0.1, y_pos, time, params); // scale to panel width
            if x < filled {
                let value = plasma_value(nx, 0.5, time, params);
                let intensity = (value + 1.0) / 2.0;
                let color = Color::Rgb(
                    (80.0 + intensity * 40.0) as u8,
                    (160.0 + intensity * 40.0) as u8,
                    (140.0 + intensity * 30.0) as u8,
                );
                element! { Text(content: "▕", color: color, bg_color: bg) }
            } else {
                element! { Text(content: "░", color: palette::TEXT_DIM, bg_color: bg) }
            }
        })
        .collect();

    Element::row(cells)
}

// ============================================================================
// Panel Rendering with Field Background
// ============================================================================

/// Get field background color at a given normalized position
fn field_bg_at(nx: f64, ny: f64, time: f64, params: &FieldParams) -> Color {
    let v = plasma_value(nx, ny, time, params);
    // Darker version of field color for background (so text is readable)
    let base_r = 18.0 + (v + 1.0) * 12.0;
    let base_g = 18.0 + (v + 1.0) * 8.0;
    let base_b = 32.0 + (v + 1.0) * 16.0;

    let warmth = ((v + 1.0) / 2.0).powf(2.0);
    let pulse = (time * 0.5).sin() * 0.1 + 0.9;

    let r = base_r + warmth * 50.0 * pulse;
    let g = base_g + warmth * 25.0 * pulse;
    let b = base_b - warmth * 10.0;

    Color::Rgb(
        r.clamp(0.0, 255.0) as u8,
        g.clamp(0.0, 255.0) as u8,
        b.clamp(0.0, 255.0) as u8,
    )
}

fn render_panel(
    title: &str,
    content: Element,
    field_energy: f64,
    opacity: f64,
    panel_x: f64,
    panel_y: f64,
    time: f64,
    params: &FieldParams,
) -> Element {
    if opacity <= 0.0 {
        return Element::Empty;
    }

    let border_color = border_color_with_energy(field_energy, opacity);
    let title_color = text_color_with_glow(field_energy);
    let bg = field_bg_at(panel_x, panel_y, time, params);

    element! {
        Box(
            flex_direction: FlexDirection::Column,
            border_style: BorderStyle::Single,
            border_color: border_color,
            padding_left: 1.0,
            padding_right: 1.0,
            background_color: bg,
        ) {
            Text(content: format!(" {} ", title), color: title_color, bold: true, bg_color: bg)
            #(content)
        }
    }
}

fn render_renderer_panel(stats: &RendererStats, field_energy: f64, opacity: f64, time: f64, panel_x: f64, panel_y: f64, params: &FieldParams) -> Element {
    let jitter = if field_energy > 0.5 {
        ((time * 10.0).sin() * field_energy * 3.0) as i64
    } else {
        0
    };

    let bg = field_bg_at(panel_x, panel_y, time, params);
    let bg2 = field_bg_at(panel_x, panel_y + 0.05, time, params);
    let bg3 = field_bg_at(panel_x, panel_y + 0.1, time, params);

    let content = element! {
        Box(flex_direction: FlexDirection::Column, background_color: bg) {
            Text(content: format!("frames {:>7}", stats.frames as i64 + jitter), color: palette::TEXT, bg_color: bg)
            Text(content: format!("fps    {:>7.0}", stats.fps), color: palette::TEXT, bg_color: bg2)
            Text(content: format!("latency {:>5.1}ms", stats.latency_ms), color: palette::TEXT, bg_color: bg3)
        }
    };

    render_panel("RENDERER", content, field_energy, opacity, panel_x, panel_y, time, params)
}

fn render_layout_panel(stats: &LayoutStats, field_energy: f64, opacity: f64, panel_x: f64, panel_y: f64, time: f64, params: &FieldParams) -> Element {
    let bg = field_bg_at(panel_x, panel_y, time, params);
    let bg2 = field_bg_at(panel_x, panel_y + 0.05, time, params);
    let bg3 = field_bg_at(panel_x, panel_y + 0.1, time, params);

    let content = element! {
        Box(flex_direction: FlexDirection::Column, background_color: bg) {
            Text(content: format!("nodes  {:>7}", stats.nodes), color: palette::TEXT, bg_color: bg)
            Text(content: format!("depth  {:>7}", stats.depth), color: palette::TEXT, bg_color: bg2)
            Text(content: format!("renders{:>7}", stats.renders), color: palette::TEXT, bg_color: bg3)
        }
    };

    render_panel("LAYOUT", content, field_energy, opacity, panel_x, panel_y, time, params)
}

fn render_memory_panel(stats: &MemoryStats, field_energy: f64, opacity: f64, panel_x: f64, panel_y: f64, time: f64, params: &FieldParams) -> Element {
    let delta = stats.delta();
    let delta_color = if delta > 1.0 { palette::WARNING } else { palette::TEXT };
    let bg = field_bg_at(panel_x, panel_y, time, params);
    let bg2 = field_bg_at(panel_x, panel_y + 0.05, time, params);
    let bg3 = field_bg_at(panel_x, panel_y + 0.1, time, params);

    let content = element! {
        Box(flex_direction: FlexDirection::Column, background_color: bg) {
            Text(content: format!("rss   {:>6.1} MB", stats.used_mb), color: palette::TEXT, bg_color: bg)
            Text(content: format!("peak  {:>6.1} MB", stats.peak_mb), color: palette::TEXT, bg_color: bg2)
            Text(content: format!("alloc {:>+5.1} MB", delta), color: delta_color, bg_color: bg3)
        }
    };

    render_panel("PROCESS", content, field_energy, opacity, panel_x, panel_y, time, params)
}

fn render_entropy_panel(time: f64, params: &FieldParams, field_energy: f64, opacity: f64, panel_x: f64, panel_y: f64) -> Element {
    if opacity <= 0.0 {
        return Element::Empty;
    }

    let border_color = border_color_with_energy(field_energy, opacity);
    let bg = field_bg_at(panel_x, panel_y, time, params);
    // Entropy bar already has field colors as foreground, so we use field bg behind it too
    let bar = entropy_bar_with_bg(68, panel_y + 0.05, time, params);

    element! {
        Box(
            flex_direction: FlexDirection::Column,
            border_style: BorderStyle::Single,
            border_color: border_color,
            padding_left: 1.0,
            padding_right: 1.0,
            background_color: bg,
        ) {
            Text(content: " ENTROPY ", color: text_color_with_glow(field_energy), bold: true, bg_color: bg)
            #(bar)
        }
    }
}

fn render_field_panel(stats: &FieldStats, time: f64, params: &FieldParams, field_energy: f64, opacity: f64, panel_x: f64, panel_y: f64) -> Element {
    let bar = signal_bar_with_bg(stats.energy_pct, 18, time, params, panel_y + 0.03);
    let drift_sign = if stats.drift >= 0.0 { "+" } else { "" };
    let bg = field_bg_at(panel_x, panel_y, time, params);
    let bg2 = field_bg_at(panel_x, panel_y + 0.06, time, params);
    let bg3 = field_bg_at(panel_x, panel_y + 0.09, time, params);

    let content = element! {
        Box(flex_direction: FlexDirection::Column, background_color: bg) {
            #(bar)
            Text(content: format!("energy  {:>5}%", stats.energy_pct), color: palette::TEXT, bg_color: bg2)
            Text(content: format!("drift  {}{:.4}", drift_sign, stats.drift), color: palette::TEXT, bg_color: bg3)
        }
    };

    render_panel("FIELD", content, field_energy, opacity, panel_x, panel_y, time, params)
}

fn render_status_panel(field_energy: f64, opacity: f64, time: f64, panel_x: f64, panel_y: f64, params: &FieldParams) -> Element {
    // Dormant module occasionally flickers
    let panic_color = if ((time * 3.0).sin() > 0.95) && field_energy > 0.7 {
        palette::WARNING
    } else {
        palette::DORMANT
    };

    let bg1 = field_bg_at(panel_x, panel_y, time, params);
    let bg2 = field_bg_at(panel_x, panel_y + 0.04, time, params);
    let bg3 = field_bg_at(panel_x, panel_y + 0.08, time, params);
    let bg4 = field_bg_at(panel_x, panel_y + 0.12, time, params);

    let content = element! {
        Box(flex_direction: FlexDirection::Column, background_color: bg1) {
            Box(flex_direction: FlexDirection::Row, background_color: bg1) {
                Text(content: "[module::core]   ", color: palette::TEXT_DIM, bg_color: bg1)
                Text(content: "operational", color: palette::OK, bg_color: bg1)
            }
            Box(flex_direction: FlexDirection::Row, background_color: bg2) {
                Text(content: "[module::render] ", color: palette::TEXT_DIM, bg_color: bg2)
                Text(content: "operational", color: palette::OK, bg_color: bg2)
            }
            Box(flex_direction: FlexDirection::Row, background_color: bg3) {
                Text(content: "[module::input]  ", color: palette::TEXT_DIM, bg_color: bg3)
                Text(content: "operational", color: palette::OK, bg_color: bg3)
            }
            Box(flex_direction: FlexDirection::Row, background_color: bg4) {
                Text(content: "[module::panic]  ", color: palette::TEXT_DIM, bg_color: bg4)
                Text(content: "dormant", color: panic_color, bg_color: bg4)
            }
        }
    };

    render_panel("STATUS", content, field_energy, opacity, panel_x, panel_y, time, params)
}

// ============================================================================
// Field Background Rendering
// ============================================================================

fn build_field_row(y: usize, width: usize, height: usize, time: f64, params: &FieldParams, intensity: f64) -> Element {
    let ny = y as f64 / height as f64;

    let cells: Vec<Element> = (0..width)
        .map(|x| {
            let nx = x as f64 / width as f64;
            let v = plasma_value(nx, ny, time, params);
            let color = field_color(v, time, intensity);
            element! { Text(content: value_to_char(v).to_string(), color: color) }
        })
        .collect();

    Element::row(cells)
}

fn build_field_background(width: usize, height: usize, time: f64, params: &FieldParams, intensity: f64) -> Element {
    if intensity <= 0.0 {
        // Black screen
        let rows: Vec<Element> = (0..height)
            .map(|_| {
                let cells: Vec<Element> = (0..width)
                    .map(|_| element! { Text(content: " ") })
                    .collect();
                Element::row(cells)
            })
            .collect();
        return Element::column(rows);
    }

    Element::column(
        (0..height)
            .map(|y| build_field_row(y, width, height, time, params, intensity))
            .collect()
    )
}

/// Build a horizontal strip of field (for gaps between panel rows)
fn field_strip(width: usize, row_y: f64, time: f64, params: &FieldParams) -> Element {
    let cells: Vec<Element> = (0..width)
        .map(|x| {
            let nx = x as f64 / width as f64;
            let v = plasma_value(nx, row_y, time, params);
            let color = field_color(v, time, 1.0);
            element! { Text(content: value_to_char(v).to_string(), color: color) }
        })
        .collect();
    Element::row(cells)
}

/// Build a vertical gutter of field (for gaps between panels)
fn field_gutter(char_count: usize, x_pos: f64, y_pos: f64, time: f64, params: &FieldParams) -> Element {
    let cells: Vec<Element> = (0..char_count)
        .map(|i| {
            let nx = x_pos + (i as f64 * 0.01); // slight variation across gutter
            let v = plasma_value(nx, y_pos, time, params);
            let color = field_color(v, time, 1.0);
            element! { Text(content: value_to_char(v).to_string(), color: color) }
        })
        .collect();
    Element::row(cells)
}

// ============================================================================
// Main Dashboard Layout
// ============================================================================

fn build_dashboard(state: &DashboardState, params: &FieldParams) -> Element {
    let boot_time = state.boot_time();
    let phase = BootPhase::from_time(boot_time);
    let field_intensity = phase.field_intensity(boot_time);
    let time = state.field_time;

    // Calculate field energy at various panel positions for coupling
    let renderer_energy = panel_field_energy(0.15, 0.2, time, params);
    let layout_energy = panel_field_energy(0.45, 0.2, time, params);
    let memory_energy = panel_field_energy(0.75, 0.2, time, params);
    let entropy_energy = panel_field_energy(0.5, 0.45, time, params);
    let signal_energy = panel_field_energy(0.15, 0.7, time, params);
    let status_energy = panel_field_energy(0.6, 0.7, time, params);

    // Panel opacities based on boot sequence
    let renderer_opacity = phase.panel_opacity(boot_time, 0);
    let layout_opacity = phase.panel_opacity(boot_time, 1);
    let memory_opacity = phase.panel_opacity(boot_time, 2);
    let entropy_opacity = phase.panel_opacity(boot_time, 3);
    let signal_opacity = phase.panel_opacity(boot_time, 4);
    let status_opacity = phase.panel_opacity(boot_time, 5);
    let logo_opacity = phase.logo_opacity(boot_time);

    // During early boot, just show field
    if phase == BootPhase::Black || phase == BootPhase::FieldIgniting || phase == BootPhase::FieldFull {
        return build_field_background(WIDTH, HEIGHT, time, params, field_intensity);
    }

    // During boot sequence, show checks over field
    if phase == BootPhase::InitText || phase == BootPhase::SystemChecks || phase == BootPhase::ChecksComplete {
        let field = build_field_background(WIDTH, HEIGHT - 8, time, params, field_intensity);
        let checks = render_system_checks(boot_time);

        return element! {
            Box(flex_direction: FlexDirection::Column) {
                #(field)
                Box(flex_direction: FlexDirection::Column, padding: 1.0) {
                    #(Element::column(checks))
                }
            }
        };
    }

    // Panel positions in normalized coordinates (for field sampling)
    let renderer_pos = (0.05, 0.15);
    let layout_pos = (0.35, 0.15);
    let memory_pos = (0.65, 0.15);
    let entropy_pos = (0.05, 0.45);
    let field_pos = (0.05, 0.65);
    let status_pos = (0.45, 0.65);

    // Main dashboard layout with field flowing through
    // Field strip at top
    let top_field = field_strip(WIDTH, 0.05, time, params);

    // Top row: field gutter | panels with field between | field gutter
    let top_row = element! {
        Box(flex_direction: FlexDirection::Row) {
            #(field_gutter(2, 0.0, 0.15, time, params))
            #(render_renderer_panel(&state.renderer, renderer_energy, renderer_opacity, time, renderer_pos.0, renderer_pos.1, params))
            #(field_gutter(1, 0.28, 0.15, time, params))
            #(render_layout_panel(&state.layout, layout_energy, layout_opacity, layout_pos.0, layout_pos.1, time, params))
            #(field_gutter(1, 0.56, 0.15, time, params))
            #(render_memory_panel(&state.memory, memory_energy, memory_opacity, memory_pos.0, memory_pos.1, time, params))
            #(field_gutter(3, 0.85, 0.15, time, params))
        }
    };

    // Field strip between top row and entropy
    let mid_field1 = field_strip(WIDTH, 0.35, time, params);

    // Entropy row with field gutters
    let entropy_row = element! {
        Box(flex_direction: FlexDirection::Row) {
            #(field_gutter(2, 0.0, 0.45, time, params))
            #(render_entropy_panel(time, params, entropy_energy, entropy_opacity, entropy_pos.0, entropy_pos.1))
            #(field_gutter(6, 0.9, 0.45, time, params))
        }
    };

    // Field strip between entropy and bottom row
    let mid_field2 = field_strip(WIDTH, 0.55, time, params);

    // Bottom row with field gutters
    let bottom_row = element! {
        Box(flex_direction: FlexDirection::Row) {
            #(field_gutter(2, 0.0, 0.7, time, params))
            #(render_field_panel(&state.field, time, params, signal_energy, signal_opacity, field_pos.0, field_pos.1))
            #(field_gutter(1, 0.35, 0.7, time, params))
            #(render_status_panel(status_energy, status_opacity, time, status_pos.0, status_pos.1, params))
            #(field_gutter(3, 0.85, 0.7, time, params))
        }
    };

    // Field strips at bottom with logo
    let bottom_field1 = field_strip(WIDTH, 0.85, time, params);
    let bottom_field2 = field_strip(WIDTH, 0.9, time, params);

    let logo = if logo_opacity > 0.0 {
        let logo_color = Color::Rgb(
            (100.0 * logo_opacity) as u8,
            (100.0 * logo_opacity) as u8,
            (120.0 * logo_opacity) as u8,
        );
        let logo_bg = field_bg_at(0.85, 0.95, time, params);
        // Logo row with field flowing around it
        element! {
            Box(flex_direction: FlexDirection::Row) {
                #(field_gutter(WIDTH - 12, 0.0, 0.95, time, params))
                Text(content: "blaeck 0.2", color: logo_color, dim: true, bg_color: logo_bg)
                #(field_gutter(2, 0.98, 0.95, time, params))
            }
        }
    } else {
        field_strip(WIDTH, 0.95, time, params)
    };

    // Combine everything with field flowing through
    element! {
        Box(flex_direction: FlexDirection::Column) {
            #(top_field)
            #(top_row)
            #(mid_field1)
            #(entropy_row)
            #(mid_field2)
            #(bottom_row)
            #(bottom_field1)
            #(bottom_field2)
            #(logo)
        }
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() -> std::io::Result<()> {
    let mut blaeck = Blaeck::new(std::io::stdout())?;
    let params = FieldParams::default();
    let mut state = DashboardState::new();
    let mut last_time = Instant::now();

    crossterm::terminal::enable_raw_mode()?;

    loop {
        // Handle input (16ms ≈ 60 FPS target)
        if poll(Duration::from_millis(16))? {
            match read()? {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => break,
                        KeyCode::Char('r') => state.restart_boot(),
                        KeyCode::Char(' ') => state.paused = !state.paused,
                        _ => {}
                    }
                }
                Event::Resize(w, h) => {
                    let _ = blaeck.handle_resize(w, h);
                }
                _ => {}
            }
        }

        // Update timing
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f64();
        last_time = now;

        // Build the dashboard UI
        let dashboard = build_dashboard(&state, &params);

        // Count elements in the tree for layout stats
        let (node_count, tree_depth) = count_element_tree(&dashboard);

        // Measure render time
        let render_start = Instant::now();
        blaeck.render(dashboard)?;
        let render_time_ms = render_start.elapsed().as_secs_f32() * 1000.0;

        // Calculate field energy for stats
        let avg_energy = panel_field_energy(0.5, 0.5, state.field_time, &params);

        // Update all stats with real metrics
        state.update(dt, render_time_ms, avg_energy, node_count, tree_depth);
    }

    crossterm::terminal::disable_raw_mode()?;
    blaeck.unmount()?;

    Ok(())
}
