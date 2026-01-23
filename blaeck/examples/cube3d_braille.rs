//! 3D Braille Renderer - High-resolution 3D graphics using braille characters
//!
//! Uses Unicode braille characters (⠀-⣿) as 2x4 pixel grids for 8x higher
//! resolution than standard ASCII. Combined with RGB colors for shading.
//!
//! Techniques:
//! - Braille subpixel rendering (2x4 dots per character)
//! - Flat shading with directional light
//! - Z-buffer for depth sorting
//! - Triangle rasterization

use crossterm::event::KeyCode;
use blaeck::prelude::*;
use blaeck::{AppEvent, AsyncApp, AsyncAppConfig};
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::time::Duration;

// Display dimensions in characters (compact)
const CHAR_WIDTH: usize = 40;
const CHAR_HEIGHT: usize = 20;

// Pixel dimensions (2x width, 4x height due to braille grid)
const PIXEL_WIDTH: usize = CHAR_WIDTH * 2;
const PIXEL_HEIGHT: usize = CHAR_HEIGHT * 4;

const DISTANCE: f32 = 3.5;

/// RGB color
#[derive(Clone, Copy, Default)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// 3D vector
#[derive(Clone, Copy, Default)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn rotate_x(self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x: self.x,
            y: self.y * cos - self.z * sin,
            z: self.y * sin + self.z * cos,
        }
    }

    fn rotate_y(self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x: self.x * cos + self.z * sin,
            y: self.y,
            z: -self.x * sin + self.z * cos,
        }
    }

    fn rotate_z(self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
            z: self.z,
        }
    }

    /// Project to 2D screen coordinates
    fn project(self) -> Option<(f32, f32, f32)> {
        let z = self.z + DISTANCE;
        if z <= 0.1 {
            return None;
        }

        let scale = 1.0 / z;
        // Larger projection scale for bigger cube
        let proj_scale = 50.0;
        let x_proj = self.x * scale * proj_scale + (PIXEL_WIDTH as f32 / 2.0);
        let y_proj = -self.y * scale * proj_scale + (PIXEL_HEIGHT as f32 / 2.0);

        Some((x_proj, y_proj, z))
    }
}

/// Triangle with vertices and color
struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    color: Rgb,
}

/// Braille pixel buffer with Z-buffer
struct BrailleBuffer {
    pixels: Vec<bool>,
    colors: Vec<Rgb>,
    depth: Vec<f32>,
    width: usize,
    height: usize,
}

impl BrailleBuffer {
    fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            pixels: vec![false; size],
            colors: vec![Rgb::default(); size],
            depth: vec![f32::INFINITY; size],
            width,
            height,
        }
    }

    fn clear(&mut self) {
        self.pixels.fill(false);
        self.colors.iter_mut().for_each(|c| *c = Rgb::default());
        self.depth.fill(f32::INFINITY);
    }

    fn set_pixel(&mut self, x: i32, y: i32, z: f32, color: Rgb) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let idx = y as usize * self.width + x as usize;
            if z < self.depth[idx] {
                self.depth[idx] = z;
                self.pixels[idx] = true;
                self.colors[idx] = color;
            }
        }
    }

    /// Draw a thick line for edges (2px wide, drawn on top)
    fn draw_edge(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgb) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let steps = (dx.max(dy) * 2.0) as i32; // More steps for smoother line

        if steps == 0 {
            return;
        }

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let cx = x0 + t * (x1 - x0);
            let cy = y0 + t * (y1 - y0);

            // Draw 2x2 pixel block for thickness
            for ox in 0..2 {
                for oy in 0..2 {
                    let x = cx as i32 + ox;
                    let y = cy as i32 + oy;

                    if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                        let idx = y as usize * self.width + x as usize;
                        self.pixels[idx] = true;
                        self.colors[idx] = color;
                    }
                }
            }
        }
    }

    /// Fill a triangle using scanline rasterization
    fn fill_triangle(&mut self, tri: &Triangle) {
        // Project vertices
        let p0 = match tri.v0.project() {
            Some(p) => p,
            None => return,
        };
        let p1 = match tri.v1.project() {
            Some(p) => p,
            None => return,
        };
        let p2 = match tri.v2.project() {
            Some(p) => p,
            None => return,
        };

        // Color is pre-computed per face, just use it directly
        let shaded_color = tri.color;

        // Bounding box
        let min_x = p0.0.min(p1.0).min(p2.0).max(0.0) as i32;
        let max_x = p0.0.max(p1.0).max(p2.0).min(self.width as f32 - 1.0) as i32;
        let min_y = p0.1.min(p1.1).min(p2.1).max(0.0) as i32;
        let max_y = p0.1.max(p1.1).max(p2.1).min(self.height as f32 - 1.0) as i32;

        // Rasterize using barycentric coordinates
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let px = x as f32 + 0.5;
                let py = y as f32 + 0.5;

                // Barycentric coordinates
                let area = edge_function(p0.0, p0.1, p1.0, p1.1, p2.0, p2.1);
                if area.abs() < 0.001 {
                    continue;
                }

                let w0 = edge_function(p1.0, p1.1, p2.0, p2.1, px, py) / area;
                let w1 = edge_function(p2.0, p2.1, p0.0, p0.1, px, py) / area;
                let w2 = edge_function(p0.0, p0.1, p1.0, p1.1, px, py) / area;

                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    // Interpolate depth
                    let z = w0 * p0.2 + w1 * p1.2 + w2 * p2.2;
                    self.set_pixel(x, y, z, shaded_color);
                }
            }
        }
    }

        /// Convert pixel buffer to braille characters with colors
    fn to_lines(&self) -> Vec<(String, Vec<Rgb>)> {
        let char_width = self.width / 2;
        let char_height = self.height / 4;
        let mut lines = Vec::with_capacity(char_height);

        for cy in 0..char_height {
            let mut line = String::with_capacity(char_width);
            let mut line_colors = Vec::with_capacity(char_width);

            for cx in 0..char_width {
                let px = cx * 2;
                let py = cy * 4;

                let dot_positions = [
                    (0, 0, 0), (0, 1, 1), (0, 2, 2), (1, 0, 3),
                    (1, 1, 4), (1, 2, 5), (0, 3, 6), (1, 3, 7),
                ];

                let mut dots = 0u8;
                let mut brightest_color = Rgb::default();
                let mut brightest_value = 0u16;
                let mut has_pixels = false;

                for (dx, dy, bit) in dot_positions {
                    let x = px + dx;
                    let y = py + dy;
                    if x < self.width && y < self.height {
                        let idx = y * self.width + x;
                        if self.pixels[idx] {
                            dots |= 1 << bit;
                            has_pixels = true;
                            let c = self.colors[idx];
                            let brightness = c.r as u16 + c.g as u16 + c.b as u16;
                            if brightness > brightest_value {
                                brightest_value = brightness;
                                brightest_color = c;
                            }
                        }
                    }
                }

                let braille_char = char::from_u32(0x2800 + dots as u32).unwrap_or(' ');
                line.push(braille_char);

                if has_pixels {
                    line_colors.push(brightest_color);
                } else {
                    line_colors.push(Rgb::new(30, 30, 30));
                }
            }

            lines.push((line, line_colors));
        }

        lines
    }
}

/// Edge function for triangle rasterization
fn edge_function(x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (x2 - x0) * (y1 - y0) - (y2 - y0) * (x1 - x0)
}

/// Create cube triangles with proper face shading
fn cube_triangles(size: f32, angle_x: f32, angle_y: f32, angle_z: f32, base_color: Rgb) -> Vec<Triangle> {
    let s = size;

    // Cube vertices
    let vertices = [
        Vec3::new(-s, -s, -s), // 0
        Vec3::new(s, -s, -s),  // 1
        Vec3::new(s, s, -s),   // 2
        Vec3::new(-s, s, -s),  // 3
        Vec3::new(-s, -s, s),  // 4
        Vec3::new(s, -s, s),   // 5
        Vec3::new(s, s, s),    // 6
        Vec3::new(-s, s, s),   // 7
    ];

    // Transform vertices
    let transformed: Vec<Vec3> = vertices
        .iter()
        .map(|v| v.rotate_x(angle_x).rotate_y(angle_y).rotate_z(angle_z))
        .collect();

    // Face definitions with outward normals
    let faces: [([usize; 3], [usize; 3], Vec3); 6] = [
        ([4, 5, 6], [4, 6, 7], Vec3::new(0.0, 0.0, 1.0)),   // Front +Z
        ([0, 3, 2], [0, 2, 1], Vec3::new(0.0, 0.0, -1.0)),  // Back -Z
        ([3, 7, 6], [3, 6, 2], Vec3::new(0.0, 1.0, 0.0)),   // Top +Y
        ([0, 1, 5], [0, 5, 4], Vec3::new(0.0, -1.0, 0.0)),  // Bottom -Y
        ([1, 2, 6], [1, 6, 5], Vec3::new(1.0, 0.0, 0.0)),   // Right +X
        ([0, 4, 7], [0, 7, 3], Vec3::new(-1.0, 0.0, 0.0)),  // Left -X
    ];

    // Light direction (from upper-right-front)
    let light_dir = Vec3::new(0.5, 0.7, -0.5);
    let light_len = (light_dir.x * light_dir.x + light_dir.y * light_dir.y + light_dir.z * light_dir.z).sqrt();
    let light_dir = Vec3::new(light_dir.x / light_len, light_dir.y / light_len, light_dir.z / light_len);

    let mut triangles = Vec::new();

    for (tri1, tri2, normal) in faces {
        // Rotate normal
        let rotated_normal = normal.rotate_x(angle_x).rotate_y(angle_y).rotate_z(angle_z);

        // Calculate lighting (dot product with light direction)
        let dot = rotated_normal.x * light_dir.x + rotated_normal.y * light_dir.y + rotated_normal.z * light_dir.z;

        // Map to brightness: ambient (0.3) + diffuse (0.7 * max(0, dot))
        let brightness = 0.3 + 0.7 * dot.max(0.0);

        let shaded_color = Rgb::new(
            (base_color.r as f32 * brightness).min(255.0) as u8,
            (base_color.g as f32 * brightness).min(255.0) as u8,
            (base_color.b as f32 * brightness).min(255.0) as u8,
        );

        triangles.push(Triangle {
            v0: transformed[tri1[0]],
            v1: transformed[tri1[1]],
            v2: transformed[tri1[2]],
            color: shaded_color,
        });

        triangles.push(Triangle {
            v0: transformed[tri2[0]],
            v1: transformed[tri2[1]],
            v2: transformed[tri2[2]],
            color: shaded_color,
        });
    }

    triangles
}

/// Shape types
#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Cube,
    Pyramid,
    Octahedron,
}

const SHAPES: [(Shape, &str); 3] = [
    (Shape::Cube, "Cube"),
    (Shape::Pyramid, "Pyramid"),
    (Shape::Octahedron, "Octahedron"),
];

/// Color presets
const COLOR_PRESETS: [(Rgb, &str); 6] = [
    (Rgb { r: 0, g: 200, b: 255 }, "Cyan"),
    (Rgb { r: 255, g: 100, b: 150 }, "Pink"),
    (Rgb { r: 150, g: 255, b: 100 }, "Lime"),
    (Rgb { r: 255, g: 180, b: 50 }, "Orange"),
    (Rgb { r: 180, g: 100, b: 255 }, "Purple"),
    (Rgb { r: 255, g: 255, b: 255 }, "White"),
];

/// Application state
struct AppState {
    angle_x: f32,
    angle_y: f32,
    angle_z: f32,
    auto_rotate: bool,
    color_index: usize,
    shape_index: usize,
}

impl AppState {
    fn new() -> Self {
        Self {
            angle_x: 0.4,
            angle_y: 0.6,
            angle_z: 0.0,
            auto_rotate: true,
            color_index: 0,
            shape_index: 0,
        }
    }

    fn current_color(&self) -> Rgb {
        COLOR_PRESETS[self.color_index].0
    }

    fn current_color_name(&self) -> &'static str {
        COLOR_PRESETS[self.color_index].1
    }

    fn current_shape(&self) -> Shape {
        SHAPES[self.shape_index].0
    }

    fn current_shape_name(&self) -> &'static str {
        SHAPES[self.shape_index].1
    }

    fn next_shape(&mut self) {
        self.shape_index = (self.shape_index + 1) % SHAPES.len();
    }

    fn next_color(&mut self) {
        self.color_index = (self.color_index + 1) % COLOR_PRESETS.len();
    }
}

/// Get projected cube edges for drawing - silhouette and visible ridges only
fn cube_edges(size: f32, angle_x: f32, angle_y: f32, angle_z: f32) -> Vec<((f32, f32), (f32, f32))> {
    let s = size;

    let vertices = [
        Vec3::new(-s, -s, -s), // 0
        Vec3::new(s, -s, -s),  // 1
        Vec3::new(s, s, -s),   // 2
        Vec3::new(-s, s, -s),  // 3
        Vec3::new(-s, -s, s),  // 4
        Vec3::new(s, -s, s),   // 5
        Vec3::new(s, s, s),    // 6
        Vec3::new(-s, s, s),   // 7
    ];

    // Transform vertices
    let transformed: Vec<Vec3> = vertices
        .iter()
        .map(|v| v.rotate_x(angle_x).rotate_y(angle_y).rotate_z(angle_z))
        .collect();

    // Project vertices
    let projected: Vec<Option<(f32, f32)>> = transformed
        .iter()
        .map(|v| v.project().map(|(x, y, _)| (x, y)))
        .collect();

    let edge_defs: [(usize, usize); 12] = [
        (0, 1), (1, 2), (2, 3), (3, 0), // Back face
        (4, 5), (5, 6), (6, 7), (7, 4), // Front face
        (0, 4), (1, 5), (2, 6), (3, 7), // Connecting edges
    ];

    let mut edges = Vec::new();
    for (a, b) in edge_defs {
        if let (Some(p0), Some(p1)) = (projected[a], projected[b]) {
            edges.push((p0, p1));
        }
    }
    edges
}

/// Create pyramid (tetrahedron) triangles with shading
fn pyramid_triangles(size: f32, angle_x: f32, angle_y: f32, angle_z: f32, base_color: Rgb) -> Vec<Triangle> {
    let s = size;
    let h = s * 1.5; // Height

    // Tetrahedron vertices (base triangle + apex)
    let vertices = [
        Vec3::new(0.0, h * 0.5, 0.0),           // 0: Apex (top)
        Vec3::new(-s, -h * 0.5, s * 0.577),     // 1: Base front-left
        Vec3::new(s, -h * 0.5, s * 0.577),      // 2: Base front-right
        Vec3::new(0.0, -h * 0.5, -s * 1.155),   // 3: Base back
    ];

    let transformed: Vec<Vec3> = vertices
        .iter()
        .map(|v| v.rotate_x(angle_x).rotate_y(angle_y).rotate_z(angle_z))
        .collect();

    // Face definitions: (vertices, normal approximation)
    let faces: [([usize; 3], Vec3); 4] = [
        ([0, 1, 2], Vec3::new(0.0, 0.5, 0.866)),   // Front face
        ([0, 2, 3], Vec3::new(0.866, 0.5, -0.5)),  // Right face
        ([0, 3, 1], Vec3::new(-0.866, 0.5, -0.5)), // Left face
        ([1, 3, 2], Vec3::new(0.0, -1.0, 0.0)),    // Base
    ];

    let light_dir = Vec3::new(0.5, 0.7, -0.5);
    let light_len = (light_dir.x * light_dir.x + light_dir.y * light_dir.y + light_dir.z * light_dir.z).sqrt();
    let light_dir = Vec3::new(light_dir.x / light_len, light_dir.y / light_len, light_dir.z / light_len);

    let mut triangles = Vec::new();
    for (tri, normal) in faces {
        let rotated_normal = normal.rotate_x(angle_x).rotate_y(angle_y).rotate_z(angle_z);
        let dot = rotated_normal.x * light_dir.x + rotated_normal.y * light_dir.y + rotated_normal.z * light_dir.z;
        let brightness = 0.3 + 0.7 * dot.max(0.0);

        let shaded_color = Rgb::new(
            (base_color.r as f32 * brightness).min(255.0) as u8,
            (base_color.g as f32 * brightness).min(255.0) as u8,
            (base_color.b as f32 * brightness).min(255.0) as u8,
        );

        triangles.push(Triangle {
            v0: transformed[tri[0]],
            v1: transformed[tri[1]],
            v2: transformed[tri[2]],
            color: shaded_color,
        });
    }
    triangles
}

/// Get pyramid edges
fn pyramid_edges(size: f32, angle_x: f32, angle_y: f32, angle_z: f32) -> Vec<((f32, f32), (f32, f32))> {
    let s = size;
    let h = s * 1.5;

    let vertices = [
        Vec3::new(0.0, h * 0.5, 0.0),
        Vec3::new(-s, -h * 0.5, s * 0.577),
        Vec3::new(s, -h * 0.5, s * 0.577),
        Vec3::new(0.0, -h * 0.5, -s * 1.155),
    ];

    let transformed: Vec<Vec3> = vertices
        .iter()
        .map(|v| v.rotate_x(angle_x).rotate_y(angle_y).rotate_z(angle_z))
        .collect();

    let projected: Vec<Option<(f32, f32)>> = transformed
        .iter()
        .map(|v| v.project().map(|(x, y, _)| (x, y)))
        .collect();

    let edge_indices = [
        (0, 1), (0, 2), (0, 3), // Apex to base
        (1, 2), (2, 3), (3, 1), // Base triangle
    ];

    let mut edges = Vec::new();
    for (a, b) in edge_indices {
        if let (Some(p0), Some(p1)) = (projected[a], projected[b]) {
            edges.push((p0, p1));
        }
    }
    edges
}

/// Create octahedron triangles with shading
fn octahedron_triangles(size: f32, angle_x: f32, angle_y: f32, angle_z: f32, base_color: Rgb) -> Vec<Triangle> {
    let s = size;

    // Octahedron vertices (6 points: +/- on each axis)
    let vertices = [
        Vec3::new(0.0, s, 0.0),  // 0: Top
        Vec3::new(0.0, -s, 0.0), // 1: Bottom
        Vec3::new(s, 0.0, 0.0),  // 2: Right
        Vec3::new(-s, 0.0, 0.0), // 3: Left
        Vec3::new(0.0, 0.0, s),  // 4: Front
        Vec3::new(0.0, 0.0, -s), // 5: Back
    ];

    let transformed: Vec<Vec3> = vertices
        .iter()
        .map(|v| v.rotate_x(angle_x).rotate_y(angle_y).rotate_z(angle_z))
        .collect();

    // 8 triangular faces
    let faces: [([usize; 3], Vec3); 8] = [
        ([0, 4, 2], Vec3::new(1.0, 1.0, 1.0)),   // Top-front-right
        ([0, 2, 5], Vec3::new(1.0, 1.0, -1.0)),  // Top-back-right
        ([0, 5, 3], Vec3::new(-1.0, 1.0, -1.0)), // Top-back-left
        ([0, 3, 4], Vec3::new(-1.0, 1.0, 1.0)),  // Top-front-left
        ([1, 2, 4], Vec3::new(1.0, -1.0, 1.0)),  // Bottom-front-right
        ([1, 5, 2], Vec3::new(1.0, -1.0, -1.0)), // Bottom-back-right
        ([1, 3, 5], Vec3::new(-1.0, -1.0, -1.0)),// Bottom-back-left
        ([1, 4, 3], Vec3::new(-1.0, -1.0, 1.0)), // Bottom-front-left
    ];

    let light_dir = Vec3::new(0.5, 0.7, -0.5);
    let light_len = (light_dir.x * light_dir.x + light_dir.y * light_dir.y + light_dir.z * light_dir.z).sqrt();
    let light_dir = Vec3::new(light_dir.x / light_len, light_dir.y / light_len, light_dir.z / light_len);

    let mut triangles = Vec::new();
    for (tri, normal) in faces {
        let n_len = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
        let normal = Vec3::new(normal.x / n_len, normal.y / n_len, normal.z / n_len);
        let rotated_normal = normal.rotate_x(angle_x).rotate_y(angle_y).rotate_z(angle_z);
        let dot = rotated_normal.x * light_dir.x + rotated_normal.y * light_dir.y + rotated_normal.z * light_dir.z;
        let brightness = 0.3 + 0.7 * dot.max(0.0);

        let shaded_color = Rgb::new(
            (base_color.r as f32 * brightness).min(255.0) as u8,
            (base_color.g as f32 * brightness).min(255.0) as u8,
            (base_color.b as f32 * brightness).min(255.0) as u8,
        );

        triangles.push(Triangle {
            v0: transformed[tri[0]],
            v1: transformed[tri[1]],
            v2: transformed[tri[2]],
            color: shaded_color,
        });
    }
    triangles
}

/// Get octahedron edges
fn octahedron_edges(size: f32, angle_x: f32, angle_y: f32, angle_z: f32) -> Vec<((f32, f32), (f32, f32))> {
    let s = size;

    let vertices = [
        Vec3::new(0.0, s, 0.0),
        Vec3::new(0.0, -s, 0.0),
        Vec3::new(s, 0.0, 0.0),
        Vec3::new(-s, 0.0, 0.0),
        Vec3::new(0.0, 0.0, s),
        Vec3::new(0.0, 0.0, -s),
    ];

    let transformed: Vec<Vec3> = vertices
        .iter()
        .map(|v| v.rotate_x(angle_x).rotate_y(angle_y).rotate_z(angle_z))
        .collect();

    let projected: Vec<Option<(f32, f32)>> = transformed
        .iter()
        .map(|v| v.project().map(|(x, y, _)| (x, y)))
        .collect();

    // 12 edges
    let edge_indices = [
        (0, 2), (0, 3), (0, 4), (0, 5), // Top to middle
        (1, 2), (1, 3), (1, 4), (1, 5), // Bottom to middle
        (2, 4), (4, 3), (3, 5), (5, 2), // Middle ring
    ];

    let mut edges = Vec::new();
    for (a, b) in edge_indices {
        if let (Some(p0), Some(p1)) = (projected[a], projected[b]) {
            edges.push((p0, p1));
        }
    }
    edges
}

fn render_frame(state: &AppState) -> Vec<(String, Vec<Rgb>)> {
    let mut buffer = BrailleBuffer::new(PIXEL_WIDTH, PIXEL_HEIGHT);
    buffer.clear();

    let base_color = state.current_color();
    let (ax, ay, az) = (state.angle_x, state.angle_y, state.angle_z);

    // Get triangles and edges based on current shape (sizes adjusted for visual equality)
    let (triangles, edges) = match state.current_shape() {
        Shape::Cube => {
            let size = 1.2;
            (
                cube_triangles(size, ax, ay, az, base_color),
                cube_edges(size, ax, ay, az),
            )
        }
        Shape::Pyramid => {
            let size = 1.6; // Scaled up - tetrahedron is naturally smaller
            (
                pyramid_triangles(size, ax, ay, az, base_color),
                pyramid_edges(size, ax, ay, az),
            )
        }
        Shape::Octahedron => {
            let size = 2.0; // Scaled up - octahedron vertices don't reach cube corners
            (
                octahedron_triangles(size, ax, ay, az, base_color),
                octahedron_edges(size, ax, ay, az),
            )
        }
    };

    // Draw filled faces
    for tri in &triangles {
        buffer.fill_triangle(tri);
    }

    // Draw white edges on top
    let edge_color = Rgb::new(255, 255, 255);
    for ((x0, y0), (x1, y1)) in &edges {
        buffer.draw_edge(*x0, *y0, *x1, *y1, edge_color);
    }

    buffer.to_lines()
}

fn build_ui(state: &AppState) -> Element {
    let frame = render_frame(state);

    let mut children: Vec<Element> = vec![
        Element::node::<Text>(
            TextProps::new("  3D Braille Renderer")
                .color(Color::Cyan)
                .bold(),
            vec![],
        ),
        Element::node::<Text>(
            TextProps::new("  Using Unicode braille for 8x subpixel resolution")
                .color(Color::Gray),
            vec![],
        ),
        Element::node::<Spacer>(SpacerProps::lines(1), vec![]),
    ];

    // Render each line with per-character colors
    for (line, colors) in frame {
        let mut segments: Vec<Element> = vec![
            Element::node::<Text>(TextProps::new("  "), vec![]), // indent
        ];

        let chars: Vec<char> = line.chars().collect();
        for (i, ch) in chars.iter().enumerate() {
            let c = colors.get(i).copied().unwrap_or(Rgb::new(100, 100, 100));
            segments.push(Element::node::<Text>(
                TextProps::new(ch.to_string()).color(Color::Rgb(c.r, c.g, c.b)),
                vec![],
            ));
        }

        children.push(Element::node::<Box>(BoxProps::row(), segments));
    }

    // Controls
    children.push(Element::node::<Spacer>(SpacerProps::lines(1), vec![]));

    let color = state.current_color();
    children.push(Element::node::<Text>(
        TextProps::new(format!(
            "  Shape: {}  Color: {}  Auto: {}",
            state.current_shape_name(),
            state.current_color_name(),
            if state.auto_rotate { "ON" } else { "OFF" }
        ))
        .color(Color::Rgb(color.r, color.g, color.b)),
        vec![],
    ));
    children.push(Element::node::<Text>(
        TextProps::new("  [S] Shape  [C] Color  [Space] Auto  [Arrows] Rotate  [Q] Quit")
            .color(Color::Gray),
        vec![],
    ));

    Element::node::<Box>(BoxProps::column(), children)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let config = AsyncAppConfig {
        tick_interval: Some(Duration::from_millis(33)), // ~30 FPS
        exit_on_ctrl_c: true,
        ..Default::default()
    };

    let app: AsyncApp<io::Stdout, ()> = AsyncApp::with_config(config)?;
    let state = Rc::new(RefCell::new(AppState::new()));
    let state_render = Rc::clone(&state);
    let state_handle = Rc::clone(&state);

    app.run(
        move |_app| {
            let s = state_render.borrow();
            build_ui(&s)
        },
        move |app, event| match event {
            AppEvent::Key(key) => {
                if key.is_char('q') || key.is_char('Q') {
                    app.exit();
                } else if key.is_char('s') || key.is_char('S') {
                    state_handle.borrow_mut().next_shape();
                } else if key.is_char('c') || key.is_char('C') {
                    state_handle.borrow_mut().next_color();
                } else if key.is_char(' ') {
                    state_handle.borrow_mut().auto_rotate ^= true;
                } else if key.is_char('r') || key.is_char('R') {
                    let mut s = state_handle.borrow_mut();
                    s.angle_x = 0.4;
                    s.angle_y = 0.6;
                    s.angle_z = 0.0;
                } else if key.code == KeyCode::Up {
                    state_handle.borrow_mut().angle_x -= 0.15;
                } else if key.code == KeyCode::Down {
                    state_handle.borrow_mut().angle_x += 0.15;
                } else if key.code == KeyCode::Left {
                    state_handle.borrow_mut().angle_y -= 0.15;
                } else if key.code == KeyCode::Right {
                    state_handle.borrow_mut().angle_y += 0.15;
                }
            }
            AppEvent::Tick => {
                let mut s = state_handle.borrow_mut();
                if s.auto_rotate {
                    s.angle_y += 0.04;
                    s.angle_x += 0.015;
                }
            }
            AppEvent::Exit => app.exit(),
            _ => {}
        },
    )
    .await?;

    println!("Thanks for watching!");
    Ok(())
}
