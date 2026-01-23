//! 3D ASCII Cube - Rotating cube rendered in the terminal
//!
//! Demonstrates:
//! - 3D math (rotation matrices, perspective projection)
//! - ASCII line drawing (Bresenham's algorithm)
//! - Animation with Blaeck's async runtime

use blaeck::prelude::*;
use blaeck::{AppEvent, AsyncApp, AsyncAppConfig};
use crossterm::event::KeyCode;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::io;
use std::rc::Rc;
use std::time::Duration;

const WIDTH: usize = 60;
const HEIGHT: usize = 30;
const FOV: f32 = 60.0;
const DISTANCE: f32 = 4.0;

/// 3D point
#[derive(Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Rotate around X axis
    fn rotate_x(self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            x: self.x,
            y: self.y * cos - self.z * sin,
            z: self.y * sin + self.z * cos,
        }
    }

    /// Rotate around Y axis
    fn rotate_y(self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            x: self.x * cos + self.z * sin,
            y: self.y,
            z: -self.x * sin + self.z * cos,
        }
    }

    /// Rotate around Z axis
    fn rotate_z(self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
            z: self.z,
        }
    }

    /// Project 3D point to 2D screen coordinates
    fn project(self, width: usize, height: usize) -> Option<(i32, i32)> {
        let z = self.z + DISTANCE;
        if z <= 0.1 {
            return None; // Behind camera
        }

        let fov_rad = FOV * PI / 180.0;
        let scale = (fov_rad / 2.0).tan();

        // Perspective projection
        let x_proj = self.x / (z * scale);
        let y_proj = self.y / (z * scale);

        // Convert to screen coordinates (account for character aspect ratio ~2:1)
        let screen_x = ((x_proj + 1.0) * 0.5 * width as f32) as i32;
        let screen_y = ((1.0 - y_proj) * 0.5 * height as f32 / 2.0 + height as f32 / 4.0) as i32;

        Some((screen_x, screen_y))
    }
}

/// ASCII frame buffer
struct FrameBuffer {
    chars: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl FrameBuffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            chars: vec![vec![' '; width]; height],
            width,
            height,
        }
    }

    #[allow(dead_code)]
    fn clear(&mut self) {
        for row in &mut self.chars {
            for c in row {
                *c = ' ';
            }
        }
    }

    fn set(&mut self, x: i32, y: i32, c: char) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.chars[y as usize][x as usize] = c;
        }
    }

    /// Draw a line using Bresenham's algorithm
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, c: char) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;

        loop {
            self.set(x, y, c);
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn to_lines(&self) -> Vec<String> {
        self.chars.iter().map(|row| row.iter().collect()).collect()
    }
}

/// Cube state
struct CubeState {
    angle_x: f32,
    angle_y: f32,
    angle_z: f32,
    auto_rotate: bool,
}

impl CubeState {
    fn new() -> Self {
        Self {
            angle_x: 0.3,
            angle_y: 0.5,
            angle_z: 0.0,
            auto_rotate: true,
        }
    }
}

/// Define cube vertices (unit cube centered at origin)
fn cube_vertices() -> [Vec3; 8] {
    let s = 1.0; // half-size
    [
        Vec3::new(-s, -s, -s), // 0: back-bottom-left
        Vec3::new(s, -s, -s),  // 1: back-bottom-right
        Vec3::new(s, s, -s),   // 2: back-top-right
        Vec3::new(-s, s, -s),  // 3: back-top-left
        Vec3::new(-s, -s, s),  // 4: front-bottom-left
        Vec3::new(s, -s, s),   // 5: front-bottom-right
        Vec3::new(s, s, s),    // 6: front-top-right
        Vec3::new(-s, s, s),   // 7: front-top-left
    ]
}

/// Define cube edges as pairs of vertex indices
fn cube_edges() -> [(usize, usize); 12] {
    [
        // Back face
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        // Front face
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        // Connecting edges
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ]
}

/// Render the cube to a frame buffer
fn render_cube(state: &CubeState) -> Vec<String> {
    let mut fb = FrameBuffer::new(WIDTH, HEIGHT);
    let vertices = cube_vertices();
    let edges = cube_edges();

    // Transform and project vertices
    let projected: Vec<Option<(i32, i32)>> = vertices
        .iter()
        .map(|v| {
            v.rotate_x(state.angle_x)
                .rotate_y(state.angle_y)
                .rotate_z(state.angle_z)
                .project(WIDTH, HEIGHT)
        })
        .collect();

    // Character based on depth for pseudo-shading
    let edge_chars = ['·', '·', '─', '│', '╱', '╲', '█'];

    // Draw edges
    for (i, (a, b)) in edges.iter().enumerate() {
        if let (Some((x0, y0)), Some((x1, y1))) = (projected[*a], projected[*b]) {
            // Pick character based on edge index for variety
            let c = edge_chars[i % edge_chars.len()];
            fb.draw_line(x0, y0, x1, y1, c);
        }
    }

    // Draw vertices as dots
    for p in &projected {
        if let Some((x, y)) = p {
            fb.set(*x, *y, '●');
        }
    }

    fb.to_lines()
}

fn build_ui(state: &CubeState) -> Element {
    let lines = render_cube(state);

    let mut children: Vec<Element> = vec![
        // Title
        Element::node::<Text>(
            TextProps::new("  3D ASCII Cube").color(Color::Cyan).bold(),
            vec![],
        ),
        Element::node::<Spacer>(SpacerProps::lines(1), vec![]),
    ];

    // Render cube lines
    for line in lines {
        children.push(Element::node::<Text>(
            TextProps::new(line).color(Color::Green),
            vec![],
        ));
    }

    // Controls
    children.push(Element::node::<Spacer>(SpacerProps::lines(1), vec![]));
    children.push(Element::node::<Text>(
        TextProps::new(format!(
            "  Rotation: X={:.1}° Y={:.1}° Z={:.1}°  Auto: {}",
            state.angle_x.to_degrees(),
            state.angle_y.to_degrees(),
            state.angle_z.to_degrees(),
            if state.auto_rotate { "ON" } else { "OFF" }
        ))
        .color(Color::Gray),
        vec![],
    ));
    children.push(Element::node::<Text>(
        TextProps::new("  [Space] Toggle auto-rotate  [←→↑↓] Manual rotate  [R] Reset  [Q] Quit")
            .color(Color::Yellow),
        vec![],
    ));

    Element::node::<Box>(BoxProps::column(), children)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let config = AsyncAppConfig {
        tick_interval: Some(Duration::from_millis(50)), // 20 FPS
        exit_on_ctrl_c: true,
        ..Default::default()
    };

    let app: AsyncApp<io::Stdout, ()> = AsyncApp::with_config(config)?;
    let state = Rc::new(RefCell::new(CubeState::new()));
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
                } else if key.is_char(' ') {
                    let mut s = state_handle.borrow_mut();
                    s.auto_rotate = !s.auto_rotate;
                } else if key.is_char('r') || key.is_char('R') {
                    let mut s = state_handle.borrow_mut();
                    s.angle_x = 0.3;
                    s.angle_y = 0.5;
                    s.angle_z = 0.0;
                } else if key.code == KeyCode::Up {
                    state_handle.borrow_mut().angle_x -= 0.1;
                } else if key.code == KeyCode::Down {
                    state_handle.borrow_mut().angle_x += 0.1;
                } else if key.code == KeyCode::Left {
                    state_handle.borrow_mut().angle_y -= 0.1;
                } else if key.code == KeyCode::Right {
                    state_handle.borrow_mut().angle_y += 0.1;
                }
            }
            AppEvent::Tick => {
                let mut s = state_handle.borrow_mut();
                if s.auto_rotate {
                    s.angle_y += 0.03;
                    s.angle_x += 0.01;
                }
            }
            AppEvent::Exit => app.exit(),
            _ => {}
        },
    )
    .await?;

    println!("Thanks for watching the cube!");
    Ok(())
}
