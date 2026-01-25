//! Plasma & Lava Lamp effects - flowing colors and floating blobs.
//!
//! Two modes:
//! - Plasma: Layered sine waves (classic demo effect)
//! - Lava Lamp: Metaball simulation with rising/falling blobs

use blaeck::prelude::*;
use crossterm::event::{poll, read, Event, KeyCode};
use std::time::{Duration, Instant};

const WIDTH: usize = 120;
const HEIGHT: usize = 30;

const LOGO: &[&str] = &[
    "██████╗ ██╗      █████╗ ███████╗ ██████╗██╗  ██╗",
    "██╔══██╗██║     ██╔══██╗██╔════╝██╔════╝██║ ██╔╝",
    "██████╔╝██║     ███████║█████╗  ██║     █████╔╝ ",
    "██╔══██╗██║     ██╔══██║██╔══╝  ██║     ██╔═██╗ ",
    "██████╔╝███████╗██║  ██║███████╗╚██████╗██║  ██╗",
    "╚═════╝ ╚══════╝╚═╝  ╚═╝╚══════╝ ╚═════╝╚═╝  ╚═╝",
    "",
    "       Terminal UI · Flexbox · Rust       ",
];
const LOGO_WIDTH: usize = 48;
const LOGO_HEIGHT: usize = 8;

const SHADES: [char; 6] = ['█', '▓', '▒', '░', '·', ' '];

// ============================================================================
// Color Themes
// ============================================================================

#[derive(Clone, Copy)]
struct ColorTheme {
    name: &'static str,
    r_mult: f64, g_mult: f64, b_mult: f64,
    r_base: f64, g_base: f64, b_base: f64,
    r_phase: f64, g_phase: f64, b_phase: f64,
}

const THEMES: &[ColorTheme] = &[
    ColorTheme { name: "Nocturne", r_mult: 60.0, g_mult: 40.0, b_mult: 80.0, r_base: 80.0, g_base: 30.0, b_base: 140.0, r_phase: 0.0, g_phase: 2.0, b_phase: 4.0 },
    ColorTheme { name: "Ocean", r_mult: 20.0, g_mult: 60.0, b_mult: 80.0, r_base: 10.0, g_base: 80.0, b_base: 160.0, r_phase: 0.0, g_phase: 1.0, b_phase: 2.0 },
    ColorTheme { name: "Inferno", r_mult: 100.0, g_mult: 60.0, b_mult: 20.0, r_base: 150.0, g_base: 50.0, b_base: 10.0, r_phase: 0.0, g_phase: 0.5, b_phase: 1.0 },
    ColorTheme { name: "Matrix", r_mult: 15.0, g_mult: 80.0, b_mult: 25.0, r_base: 0.0, g_base: 120.0, b_base: 20.0, r_phase: 0.0, g_phase: 0.0, b_phase: 3.0 },
    ColorTheme { name: "Sunset", r_mult: 80.0, g_mult: 50.0, b_mult: 60.0, r_base: 180.0, g_base: 80.0, b_base: 100.0, r_phase: 0.0, g_phase: 1.5, b_phase: 3.0 },
    ColorTheme { name: "Aurora", r_mult: 50.0, g_mult: 70.0, b_mult: 90.0, r_base: 60.0, g_base: 140.0, b_base: 120.0, r_phase: 2.0, g_phase: 0.0, b_phase: 4.0 },
    ColorTheme { name: "Synthwave", r_mult: 90.0, g_mult: 30.0, b_mult: 90.0, r_base: 150.0, g_base: 20.0, b_base: 180.0, r_phase: 0.0, g_phase: 3.0, b_phase: 1.0 },
    ColorTheme { name: "Lava", r_mult: 80.0, g_mult: 40.0, b_mult: 10.0, r_base: 180.0, g_base: 60.0, b_base: 0.0, r_phase: 0.0, g_phase: 0.3, b_phase: 0.6 },
];

// ============================================================================
// Wave Presets (for Plasma mode)
// ============================================================================

#[derive(Clone, Copy)]
struct WavePreset {
    name: &'static str,
    freq1: f64, freq2: f64, freq3: f64, freq4: f64,
    speed: f64,
}

const PRESETS: &[WavePreset] = &[
    WavePreset { name: "Classic", freq1: 12.0, freq2: 10.0, freq3: 8.0, freq4: 15.0, speed: 1.0 },
    WavePreset { name: "Slow Flow", freq1: 6.0, freq2: 5.0, freq3: 4.0, freq4: 8.0, speed: 0.5 },
    WavePreset { name: "Ripples", freq1: 3.0, freq2: 3.0, freq3: 2.0, freq4: 25.0, speed: 1.2 },
    WavePreset { name: "Turbulent", freq1: 20.0, freq2: 18.0, freq3: 15.0, freq4: 22.0, speed: 1.5 },
    WavePreset { name: "Waves", freq1: 15.0, freq2: 4.0, freq3: 10.0, freq4: 5.0, speed: 0.8 },
    WavePreset { name: "Vortex", freq1: 5.0, freq2: 5.0, freq3: 3.0, freq4: 35.0, speed: 2.0 },
    WavePreset { name: "Gentle", freq1: 4.0, freq2: 3.0, freq3: 2.0, freq4: 6.0, speed: 0.3 },
    WavePreset { name: "Chaotic", freq1: 17.0, freq2: 13.0, freq3: 19.0, freq4: 11.0, speed: 1.8 },
];

// ============================================================================
// Lava Lamp Simulation
// ============================================================================

#[derive(Clone)]
struct Blob {
    x: f64,      // 0.0 to 1.0
    y: f64,      // 0.0 to 1.0
    vx: f64,
    vy: f64,
    radius: f64, // blob size
}

#[derive(Clone)]
struct LavaLamp {
    blobs: Vec<Blob>,
    time: f64,
}

impl LavaLamp {
    fn new(num_blobs: usize, seed: u64) -> Self {
        let mut s = seed;
        let mut rng = || -> f64 {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
            ((s >> 33) as f64) / (u32::MAX as f64)
        };

        // Distribute blobs evenly across the space
        let blobs = (0..num_blobs)
            .map(|i| {
                let row = i / 3;
                let col = i % 3;
                Blob {
                    x: 0.25 + (col as f64) * 0.25 + (rng() - 0.5) * 0.1,
                    y: (row as f64) / (num_blobs as f64 / 3.0) + rng() * 0.1,
                    vx: 0.0,
                    vy: 0.0,
                    radius: 0.06 + rng() * 0.06,  // Smaller blobs
                }
            })
            .collect();

        Self { blobs, time: 0.0 }
    }

    fn update(&mut self, dt: f64, speed: f64) {
        // Lava lamp runs at 0.1x speed by default
        let lava_speed = speed * 0.1;
        self.time += dt * lava_speed;
        let dt = dt.min(0.05) * lava_speed;
        let n = self.blobs.len();

        // First pass: calculate blob-to-blob interactions
        let mut forces: Vec<(f64, f64)> = vec![(0.0, 0.0); n];

        for i in 0..n {
            for j in (i + 1)..n {
                let dx = self.blobs[j].x - self.blobs[i].x;
                let dy = self.blobs[j].y - self.blobs[i].y;
                let dist = (dx * dx + dy * dy).sqrt().max(0.01);
                let combined_radius = self.blobs[i].radius + self.blobs[j].radius;

                if dist < combined_radius * 2.0 {
                    // Blobs are close - calculate interaction
                    let overlap = combined_radius * 1.5 - dist;

                    if overlap > 0.0 {
                        // Soft repulsion when overlapping (prevent complete merge)
                        let repel = overlap * 0.02;
                        let fx = (dx / dist) * repel;
                        let fy = (dy / dist) * repel;
                        forces[i].0 -= fx;
                        forces[i].1 -= fy;
                        forces[j].0 += fx;
                        forces[j].1 += fy;
                    }

                    // Velocity coupling - blobs that are close move together (viscous)
                    let coupling = 0.1 * (1.0 - dist / (combined_radius * 2.0));
                    let dvx = self.blobs[j].vx - self.blobs[i].vx;
                    let dvy = self.blobs[j].vy - self.blobs[i].vy;
                    forces[i].0 += dvx * coupling;
                    forces[i].1 += dvy * coupling;
                    forces[j].0 -= dvx * coupling;
                    forces[j].1 -= dvy * coupling;
                }
            }
        }

        // Second pass: apply convection and interactions
        for (i, blob) in self.blobs.iter_mut().enumerate() {
            // Heat source at bottom - blobs near bottom get kicked up
            let heat = if blob.y > 0.85 {
                // Strong upward push at very bottom
                -0.003 * (1.0 + (self.time * 2.0 + i as f64).sin() * 0.5)
            } else if blob.y > 0.7 {
                // Moderate heat
                -0.001
            } else {
                0.0
            };
            blob.vy += heat;

            // Cooling at top - blobs slow down and sink
            if blob.y < 0.15 {
                blob.vy += 0.002;  // Push down
            }

            // Gentle convection: warm center rises, cool edges sink
            let center_dist = (blob.x - 0.5).abs();

            // Target vertical velocity based on horizontal position
            let target_vy = if center_dist < 0.15 {
                -0.015  // Rise in center (slower)
            } else if center_dist > 0.35 {
                0.012  // Sink at edges
            } else {
                let t = (center_dist - 0.15) / 0.2;
                -0.015 + t * 0.027
            };

            // Target horizontal velocity: drift toward center at bottom, away at top
            let target_vx = if blob.y > 0.7 {
                (0.5 - blob.x) * 0.03
            } else if blob.y < 0.3 {
                (blob.x - 0.5) * 0.02
            } else {
                0.0
            };

            // Apply convection (smoothly approach target)
            blob.vx += (target_vx - blob.vx) * 0.05 * dt * 60.0;
            blob.vy += (target_vy - blob.vy) * 0.05 * dt * 60.0;

            // Apply blob-to-blob forces
            blob.vx += forces[i].0 * dt * 60.0;
            blob.vy += forces[i].1 * dt * 60.0;

            // Add continuous wobble/chaos to prevent equilibrium
            let wobble_phase = self.time * 0.3 + i as f64 * 1.7;
            blob.vx += wobble_phase.sin() * 0.0008;
            blob.vy += (wobble_phase * 0.7).cos() * 0.0005;

            // Random thermal kicks (simulates heat convection turbulence)
            let thermal = ((self.time * 5.0 + blob.x * 13.0 + blob.y * 17.0).sin() * 12345.6789).fract();
            if thermal > 0.98 {
                blob.vy -= 0.003;  // Occasional upward kick
            }

            // Damping
            blob.vx *= 0.992;
            blob.vy *= 0.992;

            // Clamp velocities (slower max speed)
            blob.vx = blob.vx.clamp(-0.025, 0.025);
            blob.vy = blob.vy.clamp(-0.02, 0.015);

            // Update position
            blob.x += blob.vx * dt * 60.0;
            blob.y += blob.vy * dt * 60.0;

            // Soft bounce off walls
            if blob.x < 0.12 {
                blob.x = 0.12;
                blob.vx = blob.vx.abs() * 0.3;
            }
            if blob.x > 0.88 {
                blob.x = 0.88;
                blob.vx = -blob.vx.abs() * 0.3;
            }

            // Wrap vertically with repositioning
            if blob.y < -0.05 {
                blob.y = 1.05;
                blob.x = 0.4 + (blob.x - 0.5).clamp(-0.15, 0.15) + 0.1;
                blob.vy = 0.002;
            }
            if blob.y > 1.05 {
                blob.y = -0.05;
                blob.x = 0.5 + (blob.x - 0.5).clamp(-0.2, 0.2);
                blob.vy = -0.002;
            }
        }
    }

    #[allow(dead_code)]
    fn field_at(&self, x: f64, y: f64) -> f64 {
        // Metaball field: sum of 1/distance² for each blob
        let mut field = 0.0;
        for blob in &self.blobs {
            let dx = x - blob.x;
            let dy = (y - blob.y) * 2.0;  // Stretch vertically (terminal chars are taller)
            let dist_sq = dx * dx + dy * dy + 0.001;
            field += (blob.radius * blob.radius) / dist_sq;
        }
        field
    }
}

// ============================================================================
// Combined Parameters
// ============================================================================

#[derive(Clone, Copy, PartialEq, Debug)]
enum Mode {
    Plasma,
    LavaLamp,
}

#[derive(Clone)]
struct Params {
    mode: Mode,
    // Plasma params
    freq1: f64, freq2: f64, freq3: f64, freq4: f64,
    speed: f64,
    theme_idx: usize,
    preset_idx: usize,
    seed: u64,
    // Lava lamp params
    num_blobs: usize,
}

impl Params {
    fn new(seed: u64) -> Self {
        let p = &PRESETS[0];
        Self {
            mode: Mode::Plasma,
            freq1: p.freq1, freq2: p.freq2, freq3: p.freq3, freq4: p.freq4,
            speed: p.speed,
            theme_idx: 0,
            preset_idx: 0,
            seed,
            num_blobs: 8,
        }
    }

    fn theme(&self) -> &'static ColorTheme { &THEMES[self.theme_idx] }
    fn preset(&self) -> &'static WavePreset { &PRESETS[self.preset_idx] }

    fn apply_preset(&mut self, idx: usize) {
        self.preset_idx = idx % PRESETS.len();
        let p = &PRESETS[self.preset_idx];
        self.freq1 = p.freq1; self.freq2 = p.freq2; self.freq3 = p.freq3; self.freq4 = p.freq4;
        self.speed = p.speed;
    }

    fn next_theme(&mut self) { self.theme_idx = (self.theme_idx + 1) % THEMES.len(); }
    fn prev_theme(&mut self) { self.theme_idx = if self.theme_idx == 0 { THEMES.len() - 1 } else { self.theme_idx - 1 }; }
    fn next_preset(&mut self) { self.apply_preset(self.preset_idx + 1); }
    fn prev_preset(&mut self) { self.apply_preset(if self.preset_idx == 0 { PRESETS.len() - 1 } else { self.preset_idx - 1 }); }

    fn randomize_plasma(&mut self) {
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let mut s = self.seed;
        let r = |seed: &mut u64, min: f64, max: f64| -> f64 {
            *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            min + ((*seed >> 33) as f64) / (u32::MAX as f64) * (max - min)
        };
        self.freq1 = r(&mut s, 3.0, 25.0);
        self.freq2 = r(&mut s, 3.0, 25.0);
        self.freq3 = r(&mut s, 2.0, 20.0);
        self.freq4 = r(&mut s, 5.0, 40.0);
        self.speed = r(&mut s, 0.3, 2.0);
    }
}

// ============================================================================
// Rendering
// ============================================================================

fn plasma_value(nx: f64, ny: f64, time: f64, p: &Params) -> f64 {
    let v1 = (nx * p.freq1 + time).sin();
    let v2 = (ny * p.freq2 + time).cos();
    let v3 = ((nx + ny) * p.freq3 + time).sin();
    let v4 = ((nx * nx + ny * ny).sqrt() * p.freq4 - time).cos();
    (v1 + v2 + v3 + v4) / 4.0
}

/// Lava lamp mode - slow, blobby plasma with gentle movement
fn lava_plasma_value(nx: f64, ny: f64, time: f64, _p: &Params) -> f64 {
    // Use fixed low frequencies for big blob-like shapes
    // Ignore preset frequencies - lava lamp has its own look
    let slow_time = time * 0.08;  // Very slow animation

    // Big, slow-moving blobs
    let v1 = (nx * 4.0 + slow_time).sin();
    let v2 = (ny * 3.0 - slow_time * 0.7).cos();
    let v3 = ((nx + ny) * 2.5 + slow_time * 0.5).sin();
    let v4 = ((nx * 0.5).sin() * 3.0 + (ny * 0.5).cos() * 3.0 + slow_time * 0.3).cos();

    // Radial blob component - creates round shapes
    let cx = nx - 0.5;
    let cy = ny - 0.5;
    let r = (cx * cx + cy * cy).sqrt();
    let radial = (r * 6.0 - slow_time * 0.4).sin();

    (v1 + v2 + v3 + v4 * 0.5 + radial * 0.8) / 4.3
}

fn value_to_char(v: f64) -> char {
    if v > 0.6 { SHADES[0] }
    else if v > 0.3 { SHADES[1] }
    else if v > 0.0 { SHADES[2] }
    else if v > -0.3 { SHADES[3] }
    else if v > -0.6 { SHADES[4] }
    else { SHADES[5] }
}

#[allow(dead_code)]
fn lava_to_char(field: f64) -> char {
    if field > 1.5 { SHADES[0] }
    else if field > 1.0 { SHADES[1] }
    else if field > 0.6 { SHADES[2] }
    else if field > 0.3 { SHADES[3] }
    else if field > 0.15 { SHADES[4] }
    else { SHADES[5] }
}

fn plasma_color(v: f64, time: f64, theme: &ColorTheme) -> Color {
    let r = ((v * 3.0 + time + theme.r_phase).sin() * theme.r_mult + theme.r_base).clamp(0.0, 255.0) as u8;
    let g = ((v * 3.0 + time + theme.g_phase).sin() * theme.g_mult + theme.g_base).clamp(0.0, 255.0) as u8;
    let b = ((v * 3.0 + time + theme.b_phase).sin() * theme.b_mult + theme.b_base).clamp(0.0, 255.0) as u8;
    Color::Rgb(r, g, b)
}

#[allow(dead_code)]
fn lava_color(field: f64, time: f64, theme: &ColorTheme) -> Color {
    // Brighter in blob centers
    let intensity = (field.min(2.0) / 2.0).sqrt();
    let pulse = (time * 0.5).sin() * 0.1 + 0.9;
    let r = (theme.r_base * intensity * pulse + theme.r_mult * (1.0 - intensity) * 0.3).clamp(0.0, 255.0) as u8;
    let g = (theme.g_base * intensity * pulse + theme.g_mult * (1.0 - intensity) * 0.2).clamp(0.0, 255.0) as u8;
    let b = (theme.b_base * intensity * pulse * 0.5).clamp(0.0, 255.0) as u8;
    Color::Rgb(r, g, b)
}

fn logo_color(time: f64, theme: &ColorTheme) -> Color {
    let pulse = (time * 2.0).sin() * 0.5 + 0.5;
    let r = (theme.r_base + pulse * 60.0).clamp(100.0, 255.0) as u8;
    let g = (theme.g_base + pulse * 40.0).clamp(80.0, 255.0) as u8;
    let b = (theme.b_base + pulse * 50.0).clamp(120.0, 255.0) as u8;
    Color::Rgb(r, g, b)
}

fn get_logo_char(x: usize, y: usize, width: usize, height: usize) -> Option<(char, bool)> {
    let logo_x = (width - LOGO_WIDTH) / 2;
    let logo_y = (height - LOGO_HEIGHT) / 2;
    if x >= logo_x && x < logo_x + LOGO_WIDTH && y >= logo_y && y < logo_y + LOGO_HEIGHT {
        let lx = x - logo_x;
        let ly = y - logo_y;
        LOGO[ly].chars().nth(lx).map(|c| (c, ly >= 7))
    } else {
        None
    }
}

fn build_row(y: usize, width: usize, height: usize, time: f64, p: &Params, _lava: &LavaLamp) -> Element {
    let ny = y as f64 / height as f64;
    let theme = p.theme();

    let cells: Vec<Element> = (0..width)
        .map(|x| {
            // Logo overlay
            if let Some((ch, is_sub)) = get_logo_char(x, y, width, height) {
                if ch != ' ' {
                    let color = if is_sub { Color::Rgb(140, 140, 160) } else { logo_color(time, theme) };
                    return element! { Text(content: ch.to_string(), color: color, bold: !is_sub) };
                }
            }

            let nx = x as f64 / width as f64;

            match p.mode {
                Mode::Plasma => {
                    let v = plasma_value(nx, ny, time, p);
                    element! { Text(content: value_to_char(v).to_string(), color: plasma_color(v, time, theme)) }
                }
                Mode::LavaLamp => {
                    // Use plasma sine waves but with lava lamp convection flow
                    let v = lava_plasma_value(nx, ny, time, p);
                    element! { Text(content: value_to_char(v).to_string(), color: plasma_color(v, time, theme)) }
                }
            }
        })
        .collect();

    Element::row(cells)
}

fn build_display(width: usize, height: usize, time: f64, p: &Params, lava: &LavaLamp) -> Element {
    Element::column((0..height).map(|y| build_row(y, width, height, time, p, lava)).collect())
}

fn build_info(p: &Params) -> Element {
    let theme = p.theme();
    let mode_str = match p.mode {
        Mode::Plasma => format!("Plasma ({})", p.preset().name),
        Mode::LavaLamp => format!("Lava Lamp ({} blobs)", p.num_blobs),
    };

    element! {
        Box(flex_direction: FlexDirection::Column, padding: 1.0) {
            Text(content: format!("Mode: {}", mode_str), color: Color::Cyan, bold: true)
            Text(content: format!("Theme: {}", theme.name), color: Color::Yellow)
            Newline
            Text(content: format!("speed: {:.1}", p.speed), dim: true)
        }
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() -> std::io::Result<()> {
    let mut blaeck = Blaeck::new(std::io::stdout())?;
    let start = Instant::now();

    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(12345);

    let mut params = Params::new(seed);
    let mut lava = LavaLamp::new(params.num_blobs, seed);
    let mut show_info = true;
    let mut last_time = start.elapsed().as_secs_f64();

    crossterm::terminal::enable_raw_mode()?;

    loop {
        if poll(Duration::from_millis(30))? {
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => break,

                    // Mode toggle
                    KeyCode::Char('m') | KeyCode::Tab => {
                        params.mode = match params.mode {
                            Mode::Plasma => {
                                params.theme_idx = 7;  // Switch to Lava theme
                                Mode::LavaLamp
                            }
                            Mode::LavaLamp => {
                                params.theme_idx = 0;  // Switch to Nocturne
                                Mode::Plasma
                            }
                        };
                    }

                    // Theme controls
                    KeyCode::Char('t') | KeyCode::Right => params.next_theme(),
                    KeyCode::Char('T') | KeyCode::Left => params.prev_theme(),

                    // Preset controls (plasma only)
                    KeyCode::Char('p') | KeyCode::Down => params.next_preset(),
                    KeyCode::Char('P') | KeyCode::Up => params.prev_preset(),
                    KeyCode::Char(c @ '1'..='8') => params.apply_preset((c as usize) - ('1' as usize)),

                    // Randomize
                    KeyCode::Char('r') => {
                        params.seed = params.seed.wrapping_add(1);
                        if params.mode == Mode::Plasma {
                            params.randomize_plasma();
                        } else {
                            lava = LavaLamp::new(params.num_blobs, params.seed);
                        }
                    }

                    // Blob count (lava lamp)
                    KeyCode::Char('b') => {
                        params.num_blobs = (params.num_blobs + 2).min(20);
                        lava = LavaLamp::new(params.num_blobs, params.seed);
                    }
                    KeyCode::Char('B') => {
                        params.num_blobs = (params.num_blobs.saturating_sub(2)).max(2);
                        lava = LavaLamp::new(params.num_blobs, params.seed);
                    }

                    // Toggle info
                    KeyCode::Char('i') => show_info = !show_info,

                    // Speed
                    KeyCode::Char('+') | KeyCode::Char('=') => params.speed = (params.speed + 0.1).min(5.0),
                    KeyCode::Char('-') | KeyCode::Char('_') => params.speed = (params.speed - 0.1).max(0.1),

                    _ => {}
                }
            }
        }

        let now = start.elapsed().as_secs_f64();
        let raw_dt = now - last_time;
        last_time = now;

        // Update lava lamp physics (handles its own speed scaling)
        if params.mode == Mode::LavaLamp {
            lava.update(raw_dt, params.speed);
        }

        let _dt = raw_dt * params.speed;

        let time = now * params.speed;
        let display = build_display(WIDTH, HEIGHT, time, &params, &lava);

        let content = if show_info {
            let info = build_info(&params);
            element! {
                Box(flex_direction: FlexDirection::Row) {
                    Box(border_style: BorderStyle::Round, border_color: Color::Rgb(100, 60, 160)) {
                        #(display)
                    }
                    #(info)
                }
            }
        } else {
            element! {
                Box(border_style: BorderStyle::Round, border_color: Color::Rgb(100, 60, 160)) {
                    #(display)
                }
            }
        };

        let help = match params.mode {
            Mode::Plasma => "m:lava  t/T:theme  p/P:preset  1-8:presets  r:random  +/-:speed  i:info  q:quit",
            Mode::LavaLamp => "m:plasma  t/T:theme  b/B:blobs  r:reset  +/-:speed  i:info  q:quit",
        };

        blaeck.render(element! {
            Box(flex_direction: FlexDirection::Column) {
                #(content)
                Text(content: help, dim: true)
            }
        })?;
    }

    crossterm::terminal::disable_raw_mode()?;
    blaeck.unmount()?;
    println!("\nMode: {:?} | Theme: {}", params.mode, params.theme().name);

    Ok(())
}
