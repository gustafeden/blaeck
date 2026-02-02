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
use std::time::Instant;

// ============================================================================
// Constants
// ============================================================================

pub const WIDTH: usize = 82;
pub const HEIGHT: usize = 24;

pub const SHADES: [char; 6] = ['█', '▓', '▒', '░', '·', ' '];

// ============================================================================
// Color Palette (restrained, serious - NOT neon cyberpunk)
// ============================================================================

mod palette {
    use blaeck::prelude::Color;

    pub const TEXT: Color = Color::Rgb(201, 209, 217); // #c9d1d9

    // Status colors
    pub const OK: Color = Color::Rgb(107, 203, 119); // #6bcb77
    pub const WARNING: Color = Color::Rgb(255, 201, 60); // #ffc93c
}

// ============================================================================
// Field Parameters (slow, meditative movement)
// ============================================================================

pub struct FieldParams {
    pub freq1: f64,
    pub freq2: f64,
    pub freq3: f64,
    pub freq4: f64,
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
// Boot Sequence (Cinematic Logo Reveal)
// ============================================================================

// Animation is now driven by layout positions
// Position 0 = "around logo" (logo visible, panels arranged around it)
// Positions 1-3 = normal dashboard layouts
// This creates a seamless loop

// ASCII art logo - chunky block style (like NOCTERM)
// Clean full blocks for clear rendering
const LOGO_FILL: &[&str] = &[
    "████  █      ███  ████  ███  █  █",
    "█  █  █     █  █  █     █    █ █ ",
    "████  █     ████  ███   █    ██  ",
    "█  █  █     █  █  █     █    █ █ ",
    "████  ████  █  █  ████  ███  █  █",
];

const LOGO_WIDTH: usize = 34;
const LOGO_HEIGHT: usize = 5;

/// Render the logo with stacked offset stroke effect (NOCTERM style)
/// Multiple layers with slight offsets create fake 3D depth
fn render_logo_stacked(
    opacity: f64, // 0.0 to 1.0 - overall visibility
    time: f64,    // for pulse animation
    params: &FieldParams,
) -> Element {
    if opacity <= 0.0 {
        return Element::Empty;
    }

    let logo_x = (WIDTH - LOGO_WIDTH) / 2;
    let logo_y = (HEIGHT - LOGO_HEIGHT) / 2;

    // Pulse for subtle animation
    let pulse = ((time * 1.5).sin() * 0.5 + 0.5) as f32;

    // Color palette - stacked purples from dark to light
    // Back layers (darker) -> Front layer (bright)
    let colors: [(f32, f32, f32); 4] = [
        (95.0, 75.0, 168.0),                                 // #5F4BA8 - darkest (back)
        (124.0, 102.0, 201.0),                               // #7C66C9
        (155.0, 132.0, 232.0),                               // #9B84E8
        (183.0 + pulse * 20.0, 156.0 + pulse * 15.0, 255.0), // #B79CFF - brightest (front)
    ];

    // Offsets for each layer (creates the stacked extrusion)
    let offsets: [(i32, i32); 4] = [
        (2, 2), // back layer - offset down-right
        (1, 1), // middle-back
        (0, 1), // middle-front
        (0, 0), // front layer - no offset
    ];

    // Build a grid that composites all layers
    let grid_h = LOGO_HEIGHT + 3; // extra space for offsets
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
                        // Front layers overwrite back layers
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
        let ny = (logo_y + row_idx) as f64 / HEIGHT as f64;

        for (col_idx, cell) in row.iter().enumerate() {
            let nx = (logo_x + col_idx) as f64 / WIDTH as f64;

            // Get field color for background
            let field_v = plasma_value(nx, ny, time, params);
            let field_bg = field_color(field_v, time, 1.0);
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
                    // Empty - render exactly like the field does (same char + color)
                    let field_char = value_to_char(field_v);
                    row_elements.push(element! {
                        Text(content: field_char.to_string(), color: field_bg)
                    });
                }
            }
        }
        rows.push(Element::row(row_elements));
    }

    Element::column(rows)
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
pub struct RendererStats {
    pub frames: u64,
    pub fps: f32,
    pub fps_samples: Vec<f32>, // Rolling window for FPS smoothing
    pub fps_history: Vec<f32>, // Longer history for sparkline
    pub latency_ms: f32,
    pub latency_samples: Vec<f32>, // Rolling window for latency smoothing
}

impl RendererStats {
    fn new() -> Self {
        Self {
            frames: 0,
            fps: 0.0,
            fps_samples: Vec::with_capacity(30),
            fps_history: Vec::with_capacity(20),
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

        // FPS history for sparkline (sample every ~10 frames)
        if self.frames.is_multiple_of(10) {
            self.fps_history.push(self.fps);
            if self.fps_history.len() > 20 {
                self.fps_history.remove(0);
            }
        }

        // Rolling average for smooth latency display
        self.latency_samples.push(render_time_ms);
        if self.latency_samples.len() > 30 {
            self.latency_samples.remove(0);
        }
        self.latency_ms =
            self.latency_samples.iter().sum::<f32>() / self.latency_samples.len() as f32;
    }
}

#[derive(Clone)]
pub struct LayoutStats {
    pub nodes: u32,
    pub depth: u8,
    pub renders: u64, // Total render count
}

#[derive(Clone)]
pub struct BufferStats {
    pub total_cells: u32,      // WIDTH * HEIGHT
    pub field_cells: u32,      // Cells used by field background
    pub panel_cells: u32,      // Cells used by panels
    pub fill_pct: f32,         // Percentage of cells with content
    pub writes_per_frame: u32, // Estimated write operations
}

impl BufferStats {
    fn new() -> Self {
        Self {
            total_cells: (WIDTH * HEIGHT) as u32,
            field_cells: 0,
            panel_cells: 0,
            fill_pct: 0.0,
            writes_per_frame: 0,
        }
    }

    fn update(&mut self, node_count: u32) {
        self.total_cells = (WIDTH * HEIGHT) as u32;
        // Field background fills most of the screen
        self.field_cells = self.total_cells;
        // Panels overlay some cells (estimate from node count)
        self.panel_cells = node_count * 15; // ~15 chars per node average
                                            // Calculate fill percentage (field + panels can overlap)
        self.fill_pct = ((self.field_cells + self.panel_cells) as f32 / self.total_cells as f32
            * 100.0)
            .min(100.0);
        // Each node roughly equals one write operation
        self.writes_per_frame = node_count + (HEIGHT as u32); // nodes + field rows
    }
}

impl LayoutStats {
    fn new() -> Self {
        Self {
            nodes: 0,
            depth: 0,
            renders: 0,
        }
    }

    fn update(&mut self, node_count: u32, tree_depth: u8) {
        self.nodes = node_count;
        self.depth = tree_depth;
        self.renders += 1;
    }
}

#[derive(Clone)]
pub struct MemoryStats {
    pub baseline_mb: Option<f32>, // Memory after first render (lazy init)
    pub used_mb: f32,
    pub peak_mb: f32,
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
pub struct FieldStats {
    pub energy_pct: u8,     // Current field energy as percentage
    pub avg_intensity: f32, // Rolling average intensity
    pub drift: f32,         // Rate of change
    pub last_energy: f32,
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
#[allow(dead_code)]
pub fn count_element_tree(element: &Element) -> (u32, u8) {
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

// ============================================================================
// Layout Animation System
// ============================================================================

/// Panel positions for a single layout configuration
#[derive(Clone, Copy)]
struct LayoutPositions {
    buffer: (f32, f32),
    layout_panel: (f32, f32),
    process: (f32, f32),
    // field: (f32, f32),  // Commented out for now
    status: (f32, f32),
    anim_stats: (f32, f32),
}

impl LayoutPositions {
    /// Panels positioned around the logo (not overlapping!)
    /// Logo is ~34 wide, ~8 tall, centered at (24, 8) to (58, 16)
    fn near_logo() -> Self {
        // These positions match exactly where orbit(0) starts
        // so there's no jump when transitioning to orbit
        let cx = WIDTH as f32 / 2.0;
        let cy = HEIGHT as f32 / 2.0;
        let orbit_w = 28.0;
        let orbit_h = 7.0;
        let ox = -8.0; // panel offset
        let oy = -2.0;

        Self {
            // Top-left corner (orbit phase 0.0)
            buffer: (cx - orbit_w + ox, cy - orbit_h + oy),
            // Top-right corner (orbit phase 0.25)
            layout_panel: (cx + orbit_w + ox, cy - orbit_h + oy),
            // Bottom-left corner (orbit phase 0.75)
            process: (cx - orbit_w + ox, cy + orbit_h + oy),
            // Bottom-right corner (orbit phase 0.5)
            status: (cx + orbit_w + ox, cy + orbit_h + oy),
            // Bottom center (orbit phase 0.625)
            anim_stats: (cx + ox, cy + orbit_h + oy),
        }
    }

    /// Orbit position around logo - t is 0.0 to 1.0 for one full orbit
    /// Panels start from their corner positions (near_logo) and orbit from there
    fn orbit(t: f32) -> Self {
        let cx = WIDTH as f32 / 2.0;
        let cy = HEIGHT as f32 / 2.0;
        let orbit_w = 28.0;
        let orbit_h = 7.0;

        // Helper to get position on square path given an angle (0-1)
        let square_pos = |angle: f32| -> (f32, f32) {
            let a = angle % 1.0;
            let (x, y) = if a < 0.25 {
                // Top edge: left to right
                let seg_t = a / 0.25;
                (-orbit_w + seg_t * 2.0 * orbit_w, -orbit_h)
            } else if a < 0.5 {
                // Right edge: top to bottom
                let seg_t = (a - 0.25) / 0.25;
                (orbit_w, -orbit_h + seg_t * 2.0 * orbit_h)
            } else if a < 0.75 {
                // Bottom edge: right to left
                let seg_t = (a - 0.5) / 0.25;
                (orbit_w - seg_t * 2.0 * orbit_w, orbit_h)
            } else {
                // Left edge: bottom to top
                let seg_t = (a - 0.75) / 0.25;
                (-orbit_w, orbit_h - seg_t * 2.0 * orbit_h)
            };
            (cx + x - 8.0, cy + y - 2.0)
        };

        // Each panel starts at a phase matching its corner position in near_logo:
        // - buffer: top-left corner → phase 0.0 (start of top edge)
        // - layout_panel: top-right corner → phase 0.25 (end of top edge)
        // - status: bottom-right corner → phase 0.5 (end of right edge)
        // - process: bottom-left corner → phase 0.75 (end of bottom edge)
        // - anim_stats: bottom-center → phase 0.625 (middle of bottom edge)
        Self {
            buffer: square_pos(t + 0.0),
            layout_panel: square_pos(t + 0.25),
            process: square_pos(t + 0.75),
            status: square_pos(t + 0.5),
            anim_stats: square_pos(t + 0.625),
        }
    }

    /// Position 1: Left-aligned (standard terminal UI)
    fn left_aligned() -> Self {
        Self {
            buffer: (2.0, 2.0),
            layout_panel: (2.0, 7.0),
            process: (2.0, 12.0),
            status: (25.0, 2.0),
            anim_stats: (25.0, 8.0),
        }
    }

    /// Starting positions - just 3 chars away from near_logo (fade in while moving)
    fn offscreen() -> Self {
        let near = Self::near_logo();
        Self {
            // Each panel starts a few chars further out from its corner
            buffer: (near.buffer.0 - 3.0, near.buffer.1 - 2.0), // nudge up-left
            layout_panel: (near.layout_panel.0 + 3.0, near.layout_panel.1 - 2.0), // nudge up-right
            process: (near.process.0 - 3.0, near.process.1 + 2.0), // nudge down-left
            status: (near.status.0 + 3.0, near.status.1 + 2.0), // nudge down-right
            anim_stats: (near.anim_stats.0, near.anim_stats.1 + 3.0), // nudge down
        }
    }

    /// Interpolate between two layouts
    fn lerp(from: &Self, to: &Self, t: f32) -> Self {
        let lerp_pos =
            |a: (f32, f32), b: (f32, f32)| (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t);
        Self {
            buffer: lerp_pos(from.buffer, to.buffer),
            layout_panel: lerp_pos(from.layout_panel, to.layout_panel),
            process: lerp_pos(from.process, to.process),
            status: lerp_pos(from.status, to.status),
            anim_stats: lerp_pos(from.anim_stats, to.anim_stats),
        }
    }
}

// Full animation cycle length
const CYCLE_LENGTH: f64 = 22.0;

#[derive(Clone)]
pub struct DashboardState {
    pub renderer: RendererStats,
    pub layout: LayoutStats,
    pub memory: MemoryStats,
    pub buffer: BufferStats,
    pub field: FieldStats,
    pub paused: bool,
    pub boot_start: Instant,
    pub field_time: f64,
    pub memory_update_counter: u32,
}

impl DashboardState {
    pub fn new() -> Self {
        Self {
            renderer: RendererStats::new(),
            layout: LayoutStats::new(),
            memory: MemoryStats::new(),
            buffer: BufferStats::new(),
            field: FieldStats::new(),
            paused: false,
            boot_start: Instant::now(),
            field_time: 0.0,
            memory_update_counter: 0,
        }
    }

    /// Get cycle time (loops every CYCLE_LENGTH seconds)
    pub fn cycle_time(&self) -> f64 {
        self.boot_start.elapsed().as_secs_f64() % CYCLE_LENGTH
    }

    pub fn restart_boot(&mut self) {
        self.boot_start = Instant::now();
        self.field_time = 0.0;
        self.renderer = RendererStats::new();
        self.layout = LayoutStats::new();
    }

    /// Animation timeline:
    /// 0-2s:   Logo alone
    /// 2-4s:   Panels fade in + fly in near logo
    /// 4-8s:   Boxes orbit around logo (one loop)
    /// 8-9s:   Logo fades out
    /// 9-10s:  Boxes move to left-aligned
    /// 10-16s: Git tree shown on right
    /// 16-18s: Everything fades out
    /// 18-20s: Logo fades back in
    /// 20-22s: Logo alone (seamless loop back to start)
    fn logo_opacity(&self) -> f64 {
        let t = self.cycle_time();

        // Logo is visible 0-8s, fades out 8-9s, hidden 9-18s, fades in 18-20s, visible 20-22s
        // This creates seamless loop (ends at 1.0, starts at 1.0)
        if t < 8.0 {
            1.0 // Full visibility at start (seamless from end of cycle)
        } else if t < 9.0 {
            1.0 - (t - 8.0) // Fade out
        } else if t < 18.0 {
            0.0 // Hidden during git tree
        } else if t < 20.0 {
            (t - 18.0) / 2.0 // Fade back in
        } else {
            1.0 // Full at end (seamless to start)
        }
    }

    fn panel_opacity(&self) -> f64 {
        let t = self.cycle_time();

        if t < 2.0 {
            0.0 // Logo alone
        } else if t < 4.0 {
            (t - 2.0) / 2.0 // Fade in
        } else if t < 16.0 {
            1.0 // Visible
        } else if t < 18.0 {
            1.0 - (t - 16.0) / 2.0 // Fade out
        } else {
            0.0 // Hidden during logo return
        }
    }

    fn git_tree_opacity(&self) -> f64 {
        let t = self.cycle_time();

        if t < 10.0 {
            0.0 // Not shown yet
        } else if t < 11.0 {
            t - 10.0 // Fade in
        } else if t < 15.0 {
            1.0 // Full visibility
        } else if t < 16.0 {
            1.0 - (t - 15.0) // Fade out
        } else {
            0.0 // Hidden
        }
    }

    fn current_positions(&self) -> LayoutPositions {
        let t = self.cycle_time();

        // Phase 1 (0-2s): Logo alone, panels offscreen
        if t < 2.0 {
            return LayoutPositions::offscreen();
        }

        // Phase 2 (2-4s): Panels fly in near logo
        if t < 4.0 {
            let progress = ease_snap(((t - 2.0) / 2.0) as f32);
            return LayoutPositions::lerp(
                &LayoutPositions::offscreen(),
                &LayoutPositions::near_logo(),
                progress,
            );
        }

        // Phase 3 (4-8s): Orbit around logo
        if t < 8.0 {
            let orbit_progress = ((t - 4.0) / 4.0) as f32; // 0 to 1 over 4 seconds
            return LayoutPositions::orbit(orbit_progress);
        }

        // Phase 4 (8-9s): Logo fading, boxes at orbit end position
        if t < 9.0 {
            return LayoutPositions::orbit(1.0);
        }

        // Phase 5 (9-10s): Boxes move to left-aligned
        if t < 10.0 {
            let progress = ease_snap(((t - 9.0) / 1.0) as f32);
            return LayoutPositions::lerp(
                &LayoutPositions::orbit(1.0),
                &LayoutPositions::left_aligned(),
                progress,
            );
        }

        // Phase 6 (10-18s): Stay left-aligned while git tree shows
        if t < 18.0 {
            return LayoutPositions::left_aligned();
        }

        // Phase 7 (18-22s): Everything fading, stay left-aligned
        LayoutPositions::left_aligned()
    }

    pub fn update(
        &mut self,
        dt: f64,
        render_time_ms: f32,
        field_energy: f64,
        node_count: u32,
        tree_depth: u8,
    ) {
        if !self.paused {
            self.field_time += dt * 0.15;
        }

        self.renderer.update(dt, render_time_ms);
        self.layout.update(node_count, tree_depth);
        self.buffer.update(node_count);
        self.field.update(field_energy);

        self.memory_update_counter += 1;
        if self.memory_update_counter >= 30 {
            self.memory.update();
            self.memory_update_counter = 0;
        }
    }
}

/// Smooth easing function (ease-in-out cubic)
fn ease_snap(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
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

pub fn panel_field_energy(panel_x: f64, panel_y: f64, time: f64, params: &FieldParams) -> f64 {
    let value = plasma_value(panel_x, panel_y, time, params);
    (value + 1.0) / 2.0 // Convert -1..1 to 0..1
}

fn value_to_char(v: f64) -> char {
    if v > 0.6 {
        SHADES[0]
    } else if v > 0.3 {
        SHADES[1]
    } else if v > 0.0 {
        SHADES[2]
    } else if v > -0.3 {
        SHADES[3]
    } else if v > -0.6 {
        SHADES[4]
    } else {
        SHADES[5]
    }
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

fn text_color_with_glow(field_energy: f64) -> Color {
    if field_energy > 0.6 {
        let warmth = ((field_energy - 0.6) * 100.0) as u8;
        Color::Rgb(200 + warmth / 2, 180, 160)
    } else {
        palette::TEXT
    }
}

/// Get the exact field color at a given position (for seamless blending)
fn field_bg_at(nx: f64, ny: f64, time: f64, params: &FieldParams, intensity: f64) -> Color {
    let v = plasma_value(nx, ny, time, params);
    field_color(v, time, intensity)
}

// ============================================================================
// Panel Rendering (borderless - text floats on field)
// ============================================================================

/// Brighter text color for contrast against field background
fn bright_text(field_energy: f64) -> Color {
    // Brighter white/cream that's readable on the field
    let boost = (field_energy * 30.0) as u8;
    Color::Rgb(220 + boost.min(35), 225 + boost.min(30), 235)
}

fn render_panel(
    title: &str,
    content: Element,
    field_energy: f64,
    opacity: f64,
    bg: Color,
    width: f32,
) -> Element {
    if opacity <= 0.0 {
        return Element::Empty;
    }

    let title_color = text_color_with_glow(field_energy);

    // Minimal panel box with explicit width
    element! {
        Box(
            flex_direction: FlexDirection::Column,
            background_color: bg,
            padding_left: 1.0,
            padding_right: 1.0,
            width: width,
        ) {
            Text(content: format!("{}", title), color: title_color, bold: true, bg_color: bg)
            #(content)
        }
    }
}

fn render_layout_panel(
    stats: &LayoutStats,
    field_energy: f64,
    opacity: f64,
    time: f64,
    pos: (f64, f64),
    params: &FieldParams,
) -> Element {
    let text_color = bright_text(field_energy);
    let bg = field_bg_at(pos.0, pos.1, time, params, 1.0);
    let content = element! {
        Box(flex_direction: FlexDirection::Column) {
            Text(content: format!("nodes  {:>6}", stats.nodes), color: text_color, bg_color: bg)
            Text(content: format!("depth  {:>6}", stats.depth), color: text_color, bg_color: bg)
            Text(content: format!("renders{:>6}", stats.renders), color: text_color, bg_color: bg)
        }
    };

    render_panel("LAYOUT", content, field_energy, opacity, bg, 17.0)
}

fn render_memory_panel(
    stats: &MemoryStats,
    field_energy: f64,
    opacity: f64,
    time: f64,
    pos: (f64, f64),
    params: &FieldParams,
) -> Element {
    let delta = stats.delta();
    let text_color = bright_text(field_energy);
    let bg = field_bg_at(pos.0, pos.1, time, params, 1.0);
    let delta_color = if delta > 1.0 {
        palette::WARNING
    } else {
        text_color
    };

    let content = element! {
        Box(flex_direction: FlexDirection::Column) {
            Text(content: format!("rss  {:>5.1} MB", stats.used_mb), color: text_color, bg_color: bg)
            Text(content: format!("peak {:>5.1} MB", stats.peak_mb), color: text_color, bg_color: bg)
            Text(content: format!("alloc{:>+4.1} MB", delta), color: delta_color, bg_color: bg)
        }
    };

    render_panel("PROCESS", content, field_energy, opacity, bg, 17.0)
}

// FIELD panel commented out for new animation sequence
// fn render_entropy_panel(...) { ... }

fn render_status_panel(
    field_energy: f64,
    opacity: f64,
    time: f64,
    pos: (f64, f64),
    params: &FieldParams,
) -> Element {
    // Dormant module occasionally flickers
    let panic_color = if ((time * 3.0).sin() > 0.95) && field_energy > 0.7 {
        palette::WARNING
    } else {
        Color::Rgb(100, 100, 120) // dimmed but visible on field
    };

    let bg = field_bg_at(pos.0, pos.1, time, params, 1.0);
    let label_color = Color::Rgb(160, 165, 180); // dimmed labels
    let content = element! {
        Box(flex_direction: FlexDirection::Column) {
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "[core]  ", color: label_color, bg_color: bg)
                Text(content: "ok", color: palette::OK, bg_color: bg)
            }
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "[render]", color: label_color, bg_color: bg)
                Text(content: "ok", color: palette::OK, bg_color: bg)
            }
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "[input] ", color: label_color, bg_color: bg)
                Text(content: "ok", color: palette::OK, bg_color: bg)
            }
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "[panic] ", color: label_color, bg_color: bg)
                Text(content: "dormant", color: panic_color, bg_color: bg)
            }
        }
    };

    render_panel("STATUS", content, field_energy, opacity, bg, 17.0)
}

fn render_timeline_panel(
    phase_name: &str,
    cycle_time: f64,
    field_energy: f64,
    opacity: f64,
    time: f64,
    pos: (f64, f64),
    params: &FieldParams,
) -> Element {
    if opacity <= 0.0 {
        return Element::Empty;
    }

    let text_color = bright_text(field_energy);
    let bg = field_bg_at(pos.0, pos.1, time, params, 1.0);

    // Progress bar for cycle
    let bar_width = 10;
    let cycle_progress = (cycle_time / CYCLE_LENGTH) as f32;
    let filled = (cycle_progress * bar_width as f32) as usize;
    let bar: String = "█".repeat(filled) + &"░".repeat(bar_width - filled);

    let content = element! {
        Box(flex_direction: FlexDirection::Column) {
            Text(content: format!("phase {:>7}", phase_name), color: text_color, bg_color: bg)
            Text(content: format!("time  {:>5.1}s", cycle_time), color: text_color, bg_color: bg)
            Text(content: format!("[{}]", bar), color: text_color, bg_color: bg)
        }
    };

    render_panel("TIMELINE", content, field_energy, opacity, bg, 17.0)
}

fn render_buffer_panel(
    stats: &BufferStats,
    field_energy: f64,
    opacity: f64,
    time: f64,
    pos: (f64, f64),
    params: &FieldParams,
) -> Element {
    if opacity <= 0.0 {
        return Element::Empty;
    }

    let text_color = bright_text(field_energy);
    let bg = field_bg_at(pos.0, pos.1, time, params, 1.0);

    let content = element! {
        Box(flex_direction: FlexDirection::Column) {
            Text(content: format!("cells {:>6}", stats.total_cells), color: text_color, bg_color: bg)
            Text(content: format!("fill  {:>5.1}%", stats.fill_pct), color: text_color, bg_color: bg)
            Text(content: format!("writes{:>6}", stats.writes_per_frame), color: text_color, bg_color: bg)
        }
    };

    render_panel("BUFFER", content, field_energy, opacity, bg, 17.0)
}

// ============================================================================
// Git Tree Example
// ============================================================================

fn render_git_tree(opacity: f64, _time: f64, _params: &FieldParams) -> Element {
    if opacity <= 0.0 {
        return Element::Empty;
    }

    // Git tree ASCII art
    let tree_lines = [
        "* 4a3b2c1  feat: add animation timeline",
        "|\\",
        "| * 3b2a1c0  fix: orbit calculations",
        "| * 2c1b0a9  refactor: simplify state",
        "|/",
        "* 1d0c9b8  docs: add RFC for timelines",
        "*   0e9d8c7  Merge branch 'plasma'",
        "|\\",
        "| * 9f8e7d6  feat: plasma field effect",
        "| * 8g7f6e5  feat: stacked logo effect",
        "|/",
        "* 7h6g5f4  chore: bump version to 0.2",
        "* 6i5h4g3  feat: dashboard example",
        "* 5j4i3h2  init: blaeck project",
    ];

    // Colors
    let hash_color = Color::Rgb(
        (180.0 * opacity) as u8,
        (140.0 * opacity) as u8,
        (255.0 * opacity) as u8,
    );
    let branch_color = Color::Rgb(
        (100.0 * opacity) as u8,
        (200.0 * opacity) as u8,
        (100.0 * opacity) as u8,
    );
    let msg_color = Color::Rgb(
        (180.0 * opacity) as u8,
        (185.0 * opacity) as u8,
        (190.0 * opacity) as u8,
    );

    let mut rows: Vec<Element> = Vec::new();

    // Title
    let title_color = Color::Rgb(
        (220.0 * opacity) as u8,
        (220.0 * opacity) as u8,
        (230.0 * opacity) as u8,
    );
    rows.push(element! {
        Text(content: "GIT LOG", color: title_color, bold: true)
    });
    rows.push(element! { Newline });

    for line in &tree_lines {
        let mut chars: Vec<Element> = Vec::new();

        let mut in_hash = false;
        let mut hash_count = 0;

        for ch in line.chars() {
            let color = if ch == '*' || ch == '|' || ch == '\\' || ch == '/' {
                branch_color
            } else if ch.is_ascii_hexdigit() && hash_count < 7 {
                hash_count += 1;
                in_hash = true;
                hash_color
            } else {
                if in_hash && ch == ' ' {
                    in_hash = false;
                }
                msg_color
            };

            chars.push(element! {
                Text(content: ch.to_string(), color: color)
            });
        }

        rows.push(Element::row(chars));
    }

    element! {
        Box(flex_direction: FlexDirection::Column) {
            #(Element::column(rows))
        }
    }
}

// ============================================================================
// Field Background Rendering
// ============================================================================

fn build_field_row(
    y: usize,
    width: usize,
    height: usize,
    time: f64,
    params: &FieldParams,
    intensity: f64,
) -> Element {
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

fn build_field_background(
    width: usize,
    height: usize,
    time: f64,
    params: &FieldParams,
    intensity: f64,
) -> Element {
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
            .collect(),
    )
}

// ============================================================================
// Main Dashboard Layout
// ============================================================================

pub fn build_dashboard(state: &DashboardState, params: &FieldParams) -> Element {
    let time = state.field_time;

    // Field is always at full intensity (continuous plasma)
    let field_intensity = 1.0;

    // Get opacities from state
    let panel_opacity = state.panel_opacity();
    let logo_opacity = state.logo_opacity();
    let git_tree_opacity = state.git_tree_opacity();

    // Build the field background (always present, continuous)
    let field_background = build_field_background(WIDTH, HEIGHT, time, params, field_intensity);

    // Get animated panel positions
    let positions = state.current_positions();

    // Panel positions for field energy (normalized 0-1)
    let buffer_pos = (
        positions.buffer.0 as f64 / WIDTH as f64,
        positions.buffer.1 as f64 / HEIGHT as f64,
    );
    let layout_pos = (
        positions.layout_panel.0 as f64 / WIDTH as f64,
        positions.layout_panel.1 as f64 / HEIGHT as f64,
    );
    let memory_pos = (
        positions.process.0 as f64 / WIDTH as f64,
        positions.process.1 as f64 / HEIGHT as f64,
    );
    let status_pos = (
        positions.status.0 as f64 / WIDTH as f64,
        positions.status.1 as f64 / HEIGHT as f64,
    );
    let anim_pos = (
        positions.anim_stats.0 as f64 / WIDTH as f64,
        positions.anim_stats.1 as f64 / HEIGHT as f64,
    );

    // Calculate field energy at panel positions
    let buffer_energy = panel_field_energy(buffer_pos.0, buffer_pos.1, time, params);
    let layout_energy = panel_field_energy(layout_pos.0, layout_pos.1, time, params);
    let memory_energy = panel_field_energy(memory_pos.0, memory_pos.1, time, params);
    let status_energy = panel_field_energy(status_pos.0, status_pos.1, time, params);
    let anim_energy = panel_field_energy(anim_pos.0, anim_pos.1, time, params);

    // Build panels with animated positions
    let buffer_panel = render_buffer_panel(
        &state.buffer,
        buffer_energy,
        panel_opacity,
        time,
        buffer_pos,
        params,
    );
    let layout_panel = render_layout_panel(
        &state.layout,
        layout_energy,
        panel_opacity,
        time,
        layout_pos,
        params,
    );
    let memory_panel = render_memory_panel(
        &state.memory,
        memory_energy,
        panel_opacity,
        time,
        memory_pos,
        params,
    );
    let status_panel = render_status_panel(status_energy, panel_opacity, time, status_pos, params);

    // Timeline panel (shows current phase)
    let cycle_time = state.cycle_time();
    let phase_name = if cycle_time < 2.0 {
        "logo"
    } else if cycle_time < 4.0 {
        "fly-in"
    } else if cycle_time < 8.0 {
        "orbit"
    } else if cycle_time < 10.0 {
        "settle"
    } else if cycle_time < 16.0 {
        "tree"
    } else {
        "outro"
    };
    let anim_panel = render_timeline_panel(
        phase_name,
        cycle_time,
        anim_energy,
        panel_opacity,
        time,
        anim_pos,
        params,
    );

    // Git tree (right side, during tree phase)
    let git_tree = render_git_tree(git_tree_opacity, time, params);

    // Big centered logo with stacked offset stroke effect
    let big_logo = if logo_opacity > 0.0 {
        render_logo_stacked(logo_opacity, time, params)
    } else {
        Element::Empty
    };

    // Small version label (bottom right, shows during panels phase)
    let version_label = if panel_opacity > 0.0 {
        let label_pos = (0.9, 0.9);
        let label_color = Color::Rgb(
            (140.0 * panel_opacity) as u8,
            (145.0 * panel_opacity) as u8,
            (160.0 * panel_opacity) as u8,
        );
        let v = plasma_value(label_pos.0, label_pos.1, time, params);
        let label_bg = field_color(v, time, field_intensity);
        element! { Text(content: "blaeck 0.2", color: label_color, bg_color: label_bg) }
    } else {
        Element::Empty
    };

    // Simple FPS counter (bottom left)
    let fps_label = {
        let label_pos = (0.1, 0.9);
        let label_color = Color::Rgb(100, 105, 120);
        let v = plasma_value(label_pos.0, label_pos.1, time, params);
        let label_bg = field_color(v, time, field_intensity);
        element! { Text(content: format!("{:.0} fps", state.renderer.fps), color: label_color, bg_color: label_bg) }
    };

    // Absolute positioned panels
    let panels_layout = element! {
        Box(
            position: Position::Relative,
            width: WIDTH as f32,
            height: HEIGHT as f32,
        ) {
            // BUFFER panel
            Box(position: Position::Absolute, inset_top: positions.buffer.1, inset_left: positions.buffer.0) {
                #(buffer_panel)
            }
            // LAYOUT panel
            Box(position: Position::Absolute, inset_top: positions.layout_panel.1, inset_left: positions.layout_panel.0) {
                #(layout_panel)
            }
            // PROCESS panel
            Box(position: Position::Absolute, inset_top: positions.process.1, inset_left: positions.process.0) {
                #(memory_panel)
            }
            // STATUS panel
            Box(position: Position::Absolute, inset_top: positions.status.1, inset_left: positions.status.0) {
                #(status_panel)
            }
            // TIMELINE panel
            Box(position: Position::Absolute, inset_top: positions.anim_stats.1, inset_left: positions.anim_stats.0) {
                #(anim_panel)
            }
            // Git tree (right side)
            Box(position: Position::Absolute, inset_top: 3.0, inset_left: 45.0) {
                #(git_tree)
            }
            // Version label (bottom right)
            Box(position: Position::Absolute, inset_bottom: 1.0, inset_right: 2.0) {
                #(version_label)
            }
            // FPS counter (bottom left)
            Box(position: Position::Absolute, inset_bottom: 1.0, inset_left: 2.0) {
                #(fps_label)
            }
        }
    };

    // Big centered logo (positioned in the middle)
    let logo_x = ((WIDTH - LOGO_WIDTH) / 2) as f32;
    let logo_y = ((HEIGHT - LOGO_HEIGHT) / 2) as f32;
    let logo_layer = element! {
        Box(position: Position::Absolute, inset_top: logo_y, inset_left: logo_x) {
            #(big_logo)
        }
    };

    // Compose with absolute positioning:
    // - Container (relative) contains:
    //   - Field background (absolute, fills container)
    //   - Logo (absolute, centered)
    //   - Panels grid (absolute, on top)
    element! {
        Box(
            position: Position::Relative,
            width: WIDTH as f32,
            height: HEIGHT as f32,
        ) {
            // Field background layer (rendered first = behind)
            Box(
                position: Position::Absolute,
                inset_top: 0.0,
                inset_left: 0.0,
                width: WIDTH as f32,
                height: HEIGHT as f32,
            ) {
                #(field_background)
            }
            // Big centered logo layer (rendered second)
            Box(
                position: Position::Absolute,
                inset_top: 0.0,
                inset_left: 0.0,
                width: WIDTH as f32,
                height: HEIGHT as f32,
            ) {
                #(logo_layer)
            }
            // Panels overlay layer (rendered third = on top)
            Box(
                position: Position::Absolute,
                inset_top: 0.0,
                inset_left: 0.0,
                width: WIDTH as f32,
                height: HEIGHT as f32,
            ) {
                #(panels_layout)
            }
        }
    }
}

/// Static preview with dashboard at a fixed time snapshot
pub fn build_ui() -> Element {
    let params = FieldParams::default();
    let state = DashboardState::new();
    build_dashboard(&state, &params)
}
