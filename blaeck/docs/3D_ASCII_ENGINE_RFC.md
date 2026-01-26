# RFC: 3D ASCII Rendering Engine for Blaeck

## Executive Summary

This RFC proposes adding a 3D rendering engine to blaeck that outputs to ASCII characters in the terminal. The goal is to enable mind-bending visual effects like ray-traced scenes, camera orbits, reflections, shadows, and progressive rendering - all rendered as ASCII art.

**Target experiences:**
- Watch a 3D scene "render" from noise to clarity
- Photorealistic-feeling scenes with proper lighting and shadows
- Camera movements with correct perspective shifts
- Reflective surfaces showing environment
- Infinite zoom / fractal effects

---

## Table of Contents

1. [Motivation](#motivation)
2. [Prior Art & Research](#prior-art--research)
3. [Core Mathematical Foundations](#core-mathematical-foundations)
4. [Architecture Overview](#architecture-overview)
5. [Implementation Phases](#implementation-phases)
6. [Technical Specifications](#technical-specifications)
7. [API Design](#api-design)
8. [Performance Considerations](#performance-considerations)
9. [Open Questions](#open-questions)
10. [References](#references)

---

## Motivation

### The Vision

Terminal UIs are typically flat, 2D affairs. But ASCII art has untapped potential for 3D visualization. Projects like `donut.c` proved that complex 3D rendering is possible in a terminal. We want to take this further - enabling blaeck users to create stunning 3D scenes that make viewers question whether they're looking at a terminal or a video game.

### Use Cases

1. **Boot sequences / splash screens** - Dramatic 3D logo reveals
2. **Data visualization** - 3D charts, graphs, terrain maps
3. **Interactive demos** - Rotating objects, explorable scenes
4. **Art installations** - Generative 3D ASCII art
5. **Games** - First-person or third-person 3D environments
6. **Loading animations** - Progressive "rendering" effects

### Why ASCII?

ASCII art provides surprising visual richness:
- **~95 printable characters** with varying visual densities
- **8-16 ANSI colors** (or 256 / true color in modern terminals)
- **Braille characters** (⠀⠁⠂...⣿) provide 2x4 "pixel" grids per cell
- Combined: pseudo-pixels with ~10+ brightness levels and full color

---

## Prior Art & Research

### Key Projects Analyzed

| Project | Language | Technique | Key Innovation |
|---------|----------|-----------|----------------|
| [donut.c](https://www.a1k0n.net/2011/07/20/donut-math.html) | C | Surface sampling | Pioneered ASCII 3D with minimal code |
| [asciimare](https://github.com/LingDong-/asciimare) | Python | Voxel raycasting | Font-analyzed brightness gradients |
| [ascii_raytracer](https://github.com/JayWalker512/ascii_raytracer) | C | Ray tracing | Terminal ray tracer with shadows |
| [AsciiEngine](https://github.com/sephirot47/AsciiEngine) | C++ | Rasterization | Full game engine (AsciiGL) |
| [Axis](https://github.com/DavidMANZI-093/Axis) | TypeScript | Projection | Clean vector math implementation |
| [euc](https://github.com/zesterer/euc) | Rust | Software raster | **Can render to char buffer** |
| [mtrebi/Raytracer](https://github.com/mtrebi/Raytracer) | C++ | Ray tracing | Shadows, reflections, refraction |

### Key Insight: `euc` Crate

The Rust `euc` crate is a software renderer that:
- Requires no GPU
- Supports custom pixel types (including `char`!)
- Implements vertex/fragment/blend shaders in pure Rust
- Supports multi-threaded rendering

From the euc documentation:
> "euc uses Rust's type system to allow rendering to unconventional framebuffer formats. Now you can render to a char buffer!"

This is our foundation for rasterization. For ray tracing, we'll implement from scratch using established algorithms.

### Rendering Approaches

**1. Rasterization (fast, good for real-time)**
- Project 3D triangles to 2D
- Fill triangles with interpolated values
- Use z-buffer for depth
- Best for: interactive scenes, games, real-time animation

**2. Ray Tracing (accurate, good for quality)**
- Cast rays from camera through each "pixel"
- Find intersections with scene geometry
- Calculate lighting, shadows, reflections
- Best for: photorealistic rendering, progressive reveals

**3. Hybrid (our approach)**
- Use rasterization for base geometry
- Add ray-traced effects (shadows, reflections) selectively
- Progressive refinement for "render reveal" effects

---

## Core Mathematical Foundations

### 3.1 Vector Mathematics

**Source:** Standard linear algebra, [Scratchapixel](https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/geometry/points-vectors-and-normals.html)

```rust
/// 3D Vector
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn dot(a: Vec3, b: Vec3) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    fn normalize(self) -> Vec3 {
        let len = (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
        Vec3 { x: self.x/len, y: self.y/len, z: self.z/len }
    }

    fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
        // I - 2(I·N)N
        let d = 2.0 * Vec3::dot(incident, normal);
        incident - normal * d
    }
}
```

### 3.2 Transformation Matrices

**Source:** [Scratchapixel - Geometry](https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/geometry/matrices.html)

```rust
/// 4x4 Transformation Matrix (row-major)
struct Mat4 {
    m: [[f64; 4]; 4],
}

impl Mat4 {
    /// Identity matrix
    fn identity() -> Self { ... }

    /// Translation matrix
    fn translate(x: f64, y: f64, z: f64) -> Self {
        // [1 0 0 x]
        // [0 1 0 y]
        // [0 0 1 z]
        // [0 0 0 1]
    }

    /// Rotation around Y axis (for camera orbit)
    fn rotate_y(angle: f64) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        // [c  0  s  0]
        // [0  1  0  0]
        // [-s 0  c  0]
        // [0  0  0  1]
    }

    /// Perspective projection matrix
    fn perspective(fov: f64, aspect: f64, near: f64, far: f64) -> Self {
        // Standard perspective projection
    }

    /// Look-at matrix (camera orientation)
    fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let forward = (target - eye).normalize();
        let right = Vec3::cross(forward, up).normalize();
        let up = Vec3::cross(right, forward);
        // Build view matrix from basis vectors
    }
}
```

### 3.3 Perspective Projection

**Source:** [donut.c math](https://www.a1k0n.net/2011/07/20/donut-math.html)

The fundamental projection from 3D to 2D screen coordinates:

```
screen_x = K1 * x / (K2 + z)
screen_y = K1 * y / (K2 + z)
```

Where:
- `K1` = scale factor (controls field of view)
- `K2` = distance from viewer to projection plane
- `(x, y, z)` = 3D point in camera space
- `(screen_x, screen_y)` = 2D screen coordinates

**For ASCII rendering:**
```rust
fn project_to_screen(point: Vec3, screen_width: usize, screen_height: usize) -> (usize, usize) {
    let k1 = screen_height as f64 * 0.5;  // Scale to screen
    let k2 = 5.0;  // Viewer distance

    let x = k1 * point.x / (k2 + point.z);
    let y = k1 * point.y / (k2 + point.z);

    // Map to screen coordinates (center origin)
    let screen_x = (screen_width as f64 / 2.0 + x) as usize;
    let screen_y = (screen_height as f64 / 2.0 - y) as usize;  // Y inverted

    (screen_x, screen_y)
}
```

### 3.4 Ray-Sphere Intersection

**Source:** [Kyle Halladay's Tutorial](https://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html), [Scratchapixel](https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection.html)

A ray is defined as: `P(t) = Origin + t * Direction`

For a sphere centered at `C` with radius `r`:
```
|P - C|² = r²
```

Substituting the ray equation and solving for `t`:
```
at² + bt + c = 0

where:
  a = D · D           (direction dot direction)
  b = 2 * D · (O - C) (direction dot origin-to-center)
  c = |O - C|² - r²   (distance squared minus radius squared)
```

```rust
struct Ray {
    origin: Vec3,
    direction: Vec3,
}

struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    /// Returns distance to intersection, or None if no hit
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin - self.center;

        let a = Vec3::dot(ray.direction, ray.direction);
        let b = 2.0 * Vec3::dot(ray.direction, oc);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;  // No intersection
        }

        let sqrt_d = discriminant.sqrt();
        let t1 = (-b - sqrt_d) / (2.0 * a);
        let t2 = (-b + sqrt_d) / (2.0 * a);

        // Return closest positive intersection
        if t1 > 0.0 { Some(t1) }
        else if t2 > 0.0 { Some(t2) }
        else { None }
    }

    /// Surface normal at a point on the sphere
    fn normal_at(&self, point: Vec3) -> Vec3 {
        (point - self.center).normalize()
    }
}
```

### 3.5 Ray-Plane Intersection

**Source:** [Scratchapixel](https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection.html)

A plane is defined by a point `P0` and normal `N`:
```
(P - P0) · N = 0
```

```rust
struct Plane {
    point: Vec3,   // Any point on the plane
    normal: Vec3,  // Surface normal
}

impl Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let denom = Vec3::dot(self.normal, ray.direction);

        // Check if ray is parallel to plane
        if denom.abs() < 1e-6 {
            return None;
        }

        let t = Vec3::dot(self.point - ray.origin, self.normal) / denom;

        if t > 0.0 { Some(t) } else { None }
    }
}
```

### 3.6 Lighting Model (Phong)

**Source:** [Scratchapixel - Introduction to Shading](https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/shading-normals.html)

The Phong illumination model combines three components:

```
Color = Ambient + Diffuse + Specular

Ambient  = Ka * ambient_color
Diffuse  = Kd * light_color * max(0, N · L)
Specular = Ks * light_color * max(0, R · V)^shininess
```

Where:
- `N` = surface normal
- `L` = direction to light (normalized)
- `V` = direction to viewer (normalized)
- `R` = reflection of light direction around normal
- `Ka, Kd, Ks` = material coefficients (ambient, diffuse, specular)

```rust
struct Material {
    ambient: f64,      // Ka: 0.0-1.0
    diffuse: f64,      // Kd: 0.0-1.0
    specular: f64,     // Ks: 0.0-1.0
    shininess: f64,    // Phong exponent (higher = tighter highlight)
    color: (u8, u8, u8),
    reflectivity: f64, // 0.0 = matte, 1.0 = mirror
}

fn shade_point(
    point: Vec3,
    normal: Vec3,
    view_dir: Vec3,
    light_dir: Vec3,
    material: &Material,
    in_shadow: bool,
) -> f64 {
    // Ambient (always present)
    let ambient = material.ambient;

    if in_shadow {
        return ambient;
    }

    // Diffuse (Lambert)
    let n_dot_l = Vec3::dot(normal, light_dir).max(0.0);
    let diffuse = material.diffuse * n_dot_l;

    // Specular (Phong)
    let reflect_dir = Vec3::reflect(-light_dir, normal);
    let r_dot_v = Vec3::dot(reflect_dir, view_dir).max(0.0);
    let specular = material.specular * r_dot_v.powf(material.shininess);

    (ambient + diffuse + specular).min(1.0)
}
```

### 3.7 Shadow Rays

**Source:** [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

To determine if a point is in shadow:

```rust
fn is_in_shadow(point: Vec3, light_pos: Vec3, scene: &Scene) -> bool {
    let to_light = light_pos - point;
    let distance_to_light = to_light.length();
    let shadow_ray = Ray {
        origin: point + normal * 0.001,  // Offset to avoid self-intersection
        direction: to_light.normalize(),
    };

    // Check if any object blocks the path to the light
    for object in &scene.objects {
        if let Some(t) = object.intersect(&shadow_ray) {
            if t < distance_to_light {
                return true;  // Blocked!
            }
        }
    }
    false
}
```

### 3.8 Reflections (Recursive Ray Tracing)

**Source:** [Scratchapixel - Whitted Ray Tracing](https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-overview/light-transport-ray-tracing-whitted.html)

```rust
fn trace_ray(ray: &Ray, scene: &Scene, depth: u32) -> f64 {
    if depth == 0 {
        return 0.0;  // Max recursion reached
    }

    // Find closest intersection
    let hit = scene.intersect(ray);
    if hit.is_none() {
        return 0.0;  // Background
    }

    let (point, normal, material) = hit.unwrap();

    // Direct lighting
    let mut color = shade_point(point, normal, -ray.direction, ...);

    // Reflection
    if material.reflectivity > 0.0 {
        let reflect_dir = Vec3::reflect(ray.direction, normal);
        let reflect_ray = Ray {
            origin: point + normal * 0.001,
            direction: reflect_dir,
        };
        let reflected_color = trace_ray(&reflect_ray, scene, depth - 1);
        color = color * (1.0 - material.reflectivity)
              + reflected_color * material.reflectivity;
    }

    color
}
```

### 3.9 ASCII Brightness Mapping

**Source:** [donut.c](https://www.a1k0n.net/2011/07/20/donut-math.html), [asciimare](https://github.com/LingDong-/asciimare)

Map luminance (0.0 - 1.0) to ASCII characters:

```rust
/// Standard luminance gradient (12 levels)
const BRIGHTNESS_CHARS: &str = " .:-=+*#%@█";

/// Extended gradient with more levels
const EXTENDED_CHARS: &str = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@█";

/// Braille-based (highest resolution, 2x4 dots per cell)
/// Requires combining multiple brightness values into one cell
const BRAILLE_BASE: char = '⠀';  // Empty braille

fn luminance_to_char(luminance: f64) -> char {
    let chars: Vec<char> = BRIGHTNESS_CHARS.chars().collect();
    let index = ((luminance * (chars.len() - 1) as f64) as usize)
        .min(chars.len() - 1);
    chars[index]
}

/// For "noisy" progressive rendering effect
fn luminance_to_noisy_char(luminance: f64, noise: f64) -> char {
    // Add noise that decreases as samples increase
    let noisy_lum = (luminance + noise * (rand::random::<f64>() - 0.5))
        .clamp(0.0, 1.0);
    luminance_to_char(noisy_lum)
}
```

### 3.10 Z-Buffer (Depth Buffer)

**Source:** [donut.c](https://www.a1k0n.net/2011/07/20/donut-math.html)

Store inverse depth (`1/z`) for efficient comparison:

```rust
struct FrameBuffer {
    width: usize,
    height: usize,
    chars: Vec<char>,
    z_buffer: Vec<f64>,  // Store 1/z (0 = infinitely far)
    colors: Vec<Color>,
}

impl FrameBuffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            chars: vec![' '; width * height],
            z_buffer: vec![0.0; width * height],  // 0 = infinite depth
            colors: vec![Color::White; width * height],
        }
    }

    fn clear(&mut self) {
        self.chars.fill(' ');
        self.z_buffer.fill(0.0);
    }

    fn set_pixel(&mut self, x: usize, y: usize, z: f64, ch: char, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        let idx = y * self.width + x;
        let inv_z = 1.0 / z;

        // Only draw if closer than existing pixel
        if inv_z > self.z_buffer[idx] {
            self.z_buffer[idx] = inv_z;
            self.chars[idx] = ch;
            self.colors[idx] = color;
        }
    }
}
```

---

## Architecture Overview

```
blaeck/src/scene3d/
├── mod.rs              // Public API exports
├── math/
│   ├── mod.rs
│   ├── vec3.rs         // Vec3, Vec4
│   ├── mat4.rs         // 4x4 matrices, transforms
│   ├── ray.rs          // Ray struct and utilities
│   └── bounds.rs       // AABB for acceleration (future)
├── geometry/
│   ├── mod.rs
│   ├── sphere.rs       // Sphere primitive
│   ├── plane.rs        // Infinite plane
│   ├── triangle.rs     // Triangle (for meshes)
│   ├── mesh.rs         // Triangle mesh + OBJ loader
│   └── traits.rs       // Intersectable trait
├── camera/
│   ├── mod.rs
│   ├── camera.rs       // Camera position, orientation
│   ├── orbit.rs        // Orbital camera controller
│   └── projection.rs   // Perspective/orthographic
├── lighting/
│   ├── mod.rs
│   ├── light.rs        // Light types (point, directional, area)
│   ├── material.rs     // Surface materials
│   └── shading.rs      // Phong/Lambert shading
├── render/
│   ├── mod.rs
│   ├── framebuffer.rs  // ASCII framebuffer + z-buffer
│   ├── rasterizer.rs   // Triangle rasterization
│   ├── raytracer.rs    // Ray tracing renderer
│   ├── progressive.rs  // Progressive refinement
│   └── ascii_map.rs    // Brightness to char mapping
├── scene/
│   ├── mod.rs
│   ├── scene.rs        // Scene graph, object collection
│   ├── object.rs       // SceneObject wrapper
│   └── builder.rs      // Scene builder DSL
└── effects/
    ├── mod.rs
    ├── noise.rs        // Noise generation for "render" effect
    ├── bloom.rs        // ASCII bloom/glow
    └── fog.rs          // Distance fog
```

### Data Flow

```
Scene Definition
       │
       ▼
┌─────────────────┐
│  Scene Graph    │  Objects, lights, camera
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Renderer       │  Rasterizer or Ray Tracer
│  (per frame)    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  FrameBuffer    │  Luminance values + depth
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  ASCII Mapper   │  Convert to characters
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Blaeck Output  │  Render to terminal
└─────────────────┘
```

---

## Implementation Phases

### Phase 1: Core Math & Primitives
**Goal:** Foundation for all 3D operations

**Deliverables:**
- [ ] `Vec3` with all standard operations (add, sub, mul, div, dot, cross, normalize, length)
- [ ] `Mat4` with identity, translation, rotation (X, Y, Z), scale, multiply
- [ ] `Ray` struct with origin and direction
- [ ] `Sphere` with ray intersection
- [ ] `Plane` with ray intersection
- [ ] Basic unit tests for all math

**Code Example:**
```rust
use blaeck::scene3d::math::*;

let v = Vec3::new(1.0, 2.0, 3.0);
let n = v.normalize();
assert!((n.length() - 1.0).abs() < 0.0001);

let ray = Ray::new(Vec3::ZERO, Vec3::Z);
let sphere = Sphere::new(Vec3::new(0.0, 0.0, 5.0), 1.0);
let hit = sphere.intersect(&ray);
assert!(hit.is_some());
```

**Estimated effort:** Core foundation, must be solid

---

### Phase 2: Camera & Projection
**Goal:** View the 3D world from any angle

**Deliverables:**
- [ ] `Camera` struct with position, target, up vector
- [ ] `look_at()` view matrix generation
- [ ] Perspective projection matrix
- [ ] Screen-space coordinate mapping
- [ ] `OrbitCamera` for easy rotation around a point

**Code Example:**
```rust
let camera = Camera::new()
    .position(Vec3::new(0.0, 2.0, -5.0))
    .look_at(Vec3::ZERO)
    .fov(60.0);

let orbit = OrbitCamera::new(Vec3::ZERO, 5.0)
    .angle(elapsed_time * 0.5);  // Rotate over time
```

**Key math:** View matrix = inverse of camera transform

---

### Phase 3: Basic Rasterization
**Goal:** Render simple shapes to ASCII

**Deliverables:**
- [ ] `FrameBuffer` with char array and z-buffer
- [ ] `AsciiMapper` with configurable character sets
- [ ] Point rendering with depth testing
- [ ] Line rendering (Bresenham's algorithm)
- [ ] Basic sphere rendering via point sampling (donut.c approach)

**Code Example:**
```rust
let mut fb = FrameBuffer::new(80, 24);
let mapper = AsciiMapper::standard();

// Sample points on sphere surface
for theta in (0..360).map(|i| i as f64 * PI / 180.0) {
    for phi in (0..180).map(|i| i as f64 * PI / 180.0) {
        let point = sphere.point_at(theta, phi);
        let (sx, sy) = camera.project(point, fb.width, fb.height);
        let brightness = compute_lighting(point, normal, light);
        fb.set_pixel(sx, sy, point.z, mapper.map(brightness));
    }
}
```

---

### Phase 4: Lighting & Materials
**Goal:** Objects look 3D with proper shading

**Deliverables:**
- [ ] `Material` struct (ambient, diffuse, specular, shininess, color)
- [ ] `PointLight` and `DirectionalLight`
- [ ] Phong shading model implementation
- [ ] Multiple light support
- [ ] Material presets (matte, glossy, metal, etc.)

**Code Example:**
```rust
let material = Material::glossy()
    .color(Color::Cyan)
    .shininess(32.0);

let light = PointLight::new(Vec3::new(5.0, 5.0, -5.0))
    .intensity(1.0)
    .color(Color::White);

let brightness = shade_phong(hit_point, normal, view_dir, &[light], &material);
```

---

### Phase 5: Ray Tracer Foundation
**Goal:** Accurate rendering with ray tracing

**Deliverables:**
- [ ] `RayTracer` renderer struct
- [ ] Primary ray generation from camera
- [ ] Scene intersection (find closest hit)
- [ ] Basic ray-traced sphere rendering
- [ ] Integration with FrameBuffer

**Code Example:**
```rust
let tracer = RayTracer::new(&scene, &camera);
let fb = tracer.render(80, 24);
```

**Algorithm:**
```
for each pixel (x, y):
    ray = camera.ray_through_pixel(x, y)
    color = trace_ray(ray, scene, max_depth=1)
    framebuffer[x, y] = color
```

---

### Phase 6: Shadows
**Goal:** Objects cast shadows for depth perception

**Deliverables:**
- [ ] Shadow ray casting
- [ ] Hard shadows (point lights)
- [ ] Soft shadows via multiple samples (area lights)
- [ ] Shadow bias to prevent acne

**Code Example:**
```rust
let in_shadow = scene.is_shadowed(hit_point, light.position);
let brightness = if in_shadow {
    material.ambient  // Only ambient in shadow
} else {
    shade_phong(...)
};
```

---

### Phase 7: Reflections
**Goal:** Mirror-like surfaces that reflect the environment

**Deliverables:**
- [ ] Reflection ray calculation
- [ ] Recursive ray tracing (configurable depth)
- [ ] Reflectivity material property
- [ ] Fresnel effect (optional, for realism)

**Code Example:**
```rust
let material = Material::chrome()
    .reflectivity(0.8);

// In trace_ray:
if material.reflectivity > 0.0 && depth > 0 {
    let reflect_ray = Ray::new(hit_point, reflect(ray.dir, normal));
    let reflected = trace_ray(reflect_ray, scene, depth - 1);
    color = lerp(color, reflected, material.reflectivity);
}
```

---

### Phase 8: Progressive Rendering ("The Render" Effect)
**Goal:** Scene emerges from noise to clarity

**Deliverables:**
- [ ] `ProgressiveRenderer` that accumulates samples
- [ ] Per-pixel sample count tracking
- [ ] Noise injection based on sample count
- [ ] Convergence visualization (noise → clean)
- [ ] Integration with blaeck's Timeline for choreography

**Code Example:**
```rust
let mut progressive = ProgressiveRenderer::new(&scene, &camera, 80, 24);

// Each frame adds more samples
loop {
    progressive.add_samples(10);  // 10 more samples per pixel
    let fb = progressive.to_framebuffer();

    // Noise decreases as samples increase
    // Frame 1: ░▒▓█▓▒░ (noisy)
    // Frame N: Clean sphere with shadows

    blaeck.render(fb.to_element())?;
}
```

**Noise formula:**
```rust
let noise_factor = 1.0 / (sample_count as f64).sqrt();
let noisy_value = value + noise_factor * random(-0.5, 0.5);
```

---

### Phase 9: Scene Builder & DSL
**Goal:** Easy scene construction

**Deliverables:**
- [ ] `SceneBuilder` with fluent API
- [ ] Predefined primitives (sphere, cube, plane, cylinder)
- [ ] Object transforms (position, rotation, scale)
- [ ] Scene presets (Cornell box, checkered floor, etc.)

**Code Example:**
```rust
let scene = Scene::builder()
    .camera(|c| c
        .position(0.0, 2.0, -5.0)
        .look_at(Vec3::ZERO)
        .fov(60.0))
    .light(|l| l
        .point(5.0, 5.0, -5.0)
        .color(Color::White))
    .object(Sphere::new(Vec3::ZERO, 1.0)
        .material(Material::chrome()))
    .object(Plane::xz()
        .material(Material::checkerboard()))
    .build();
```

---

### Phase 10: Reactive Integration
**Goal:** Seamless use with blaeck's reactive system

**Deliverables:**
- [ ] `use_scene` hook for reactive rendering
- [ ] `use_camera` hook for camera controls
- [ ] Scene state management
- [ ] Frame-rate independent animation
- [ ] Input handling for camera movement

**Code Example:**
```rust
fn render_demo(cx: Scope) -> Element {
    let scene = use_scene(cx, || Scene::builder()
        .sphere(Vec3::ZERO, 1.0, Material::chrome())
        .plane_xz(Material::checkerboard())
        .light_point(5.0, 5.0, -5.0)
        .build());

    let camera = use_orbit_camera(cx, Vec3::ZERO, 5.0);

    use_input(cx, move |key| {
        match key.code {
            KeyCode::Left => camera.rotate(-0.1),
            KeyCode::Right => camera.rotate(0.1),
            _ => {}
        }
    });

    // Renders at 60fps, camera orbits
    scene.to_element(80, 24, &camera)
}
```

---

### Phase 11: Advanced Effects (Future)
**Goal:** Mind-bending visuals

**Potential features:**
- [ ] Refraction (transparent materials)
- [ ] Depth of field (focal blur)
- [ ] Motion blur
- [ ] Ambient occlusion
- [ ] Global illumination (path tracing)
- [ ] Volumetric fog
- [ ] Particle systems
- [ ] Mesh loading (OBJ format)
- [ ] Procedural textures
- [ ] Infinite zoom / fractal rendering

---

## Technical Specifications

### Character Sets for ASCII Mapping

**Standard (10 levels):**
```
" .:-=+*#%@"
```

**Extended (70 levels):**
```
" `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@"
```

**Block characters (seamless gradients):**
```
" ░▒▓█"
```

**Braille (highest resolution):**
- 256 braille characters (⠀ to ⣿)
- Each cell represents 2x4 dot grid
- Combine with color for sub-character precision

### Performance Targets

| Renderer | Target FPS | Resolution |
|----------|------------|------------|
| Rasterizer | 60+ | 120x40 |
| Ray tracer (simple) | 30+ | 80x24 |
| Ray tracer (shadows) | 15+ | 80x24 |
| Ray tracer (reflections) | 10+ | 80x24 |
| Progressive | N/A | Any (converges over time) |

### Memory Layout

```rust
// Efficient framebuffer layout
struct FrameBuffer {
    width: u32,
    height: u32,
    // Interleaved for cache efficiency
    pixels: Vec<Pixel>,
}

struct Pixel {
    char: char,      // ASCII character
    fg: Color,       // Foreground color
    bg: Color,       // Background color (optional)
    depth: f32,      // Z-buffer value (1/z)
}
```

---

## API Design

### High-Level API (Recommended)

```rust
use blaeck::prelude::*;
use blaeck::scene3d::prelude::*;

fn main() -> std::io::Result<()> {
    ReactiveApp::run(|cx| {
        let elapsed = use_animation_frame(cx);

        let scene = Scene::cornell_box();
        let camera = Camera::orbit(Vec3::ZERO, 5.0, elapsed * 0.5);

        let frame = scene.render_ascii(80, 24, &camera, RenderMode::RayTraced);

        element! {
            Box(border_style: BorderStyle::Round) {
                AsciiCanvas(frame: frame)
            }
        }
    })
}
```

### Low-Level API (Full Control)

```rust
use blaeck::scene3d::*;

// Build scene manually
let mut scene = Scene::new();
scene.add(Sphere::new(Vec3::ZERO, 1.0).material(Material::matte(Color::Red)));
scene.add(Plane::xz(-1.0).material(Material::checkerboard()));
scene.add_light(PointLight::new(Vec3::new(5.0, 5.0, -5.0)));

// Configure camera
let camera = Camera::new()
    .position(Vec3::new(0.0, 2.0, -5.0))
    .look_at(Vec3::ZERO)
    .fov(60.0);

// Render with ray tracer
let mut tracer = RayTracer::new()
    .shadows(true)
    .reflections(true)
    .max_depth(3);

let mut fb = FrameBuffer::new(80, 24);
tracer.render(&scene, &camera, &mut fb);

// Convert to blaeck element
let element = fb.to_element();
```

### Progressive Rendering API

```rust
let mut renderer = ProgressiveRenderer::new(&scene, &camera, 80, 24);

// For "The Render" effect
let timeline = Timeline::new()
    .act(Act::new("noise")
        .duration(5.0)
        .animate("samples_per_frame", 1.0, 100.0, Easing::EaseInCubic));

loop {
    let samples = timeline.get_or("samples_per_frame", 1.0) as u32;
    renderer.add_samples(samples);

    let fb = renderer.to_framebuffer();
    blaeck.render(fb.to_element())?;

    if renderer.converged() {
        break;
    }
}
```

---

## Performance Considerations

### Optimization Strategies

1. **Spatial acceleration (Phase 11+)**
   - Bounding Volume Hierarchy (BVH) for complex scenes
   - Not needed for simple demos (< 10 objects)

2. **Multi-threading**
   - Render tiles in parallel (embarrassingly parallel)
   - Use `rayon` for easy parallelism

3. **Incremental rendering**
   - Only re-render changed regions
   - Cache static geometry

4. **Level of Detail**
   - Simpler shading for distant objects
   - Skip shadows beyond certain distance

5. **Fixed-point math (optional)**
   - For embedded/constrained environments
   - Trade precision for speed

### Benchmarking

```rust
// Built-in benchmarking
let stats = renderer.stats();
println!("Rays cast: {}", stats.rays_cast);
println!("Intersections: {}", stats.intersections_tested);
println!("Render time: {:?}", stats.render_time);
println!("Pixels/sec: {}", stats.pixels_per_second);
```

---

## Open Questions

1. **Should we use `euc` crate or implement from scratch?**
   - Pro: euc is battle-tested, supports char buffers
   - Con: Additional dependency, may not fit our exact needs
   - Recommendation: Start from scratch for ray tracing, evaluate euc for rasterization

2. **Color support scope?**
   - Basic: 8 ANSI colors
   - Extended: 256 colors
   - Full: 24-bit true color
   - Recommendation: Support all, default to true color with fallback

3. **Mesh loading formats?**
   - OBJ is simple and widely supported
   - Could add glTF for more complex scenes
   - Recommendation: Phase 11+, start with primitives only

4. **Integration with existing Timeline system?**
   - Camera animation via timeline acts
   - Object animation via animated properties
   - Recommendation: Yes, use existing Animatable trait

5. **WebAssembly support?**
   - Would enable web-based demos
   - Requires careful dependency management
   - Recommendation: Keep it possible, don't prioritize

---

## References

### Mathematical Foundations
- [Scratchapixel](https://www.scratchapixel.com/) - Comprehensive CG tutorials
- [Ray Tracing in One Weekend](https://raytracing.github.io/) - Excellent practical guide
- [donut.c explained](https://www.a1k0n.net/2011/07/20/donut-math.html) - ASCII 3D fundamentals

### Open Source Implementations
- [euc](https://github.com/zesterer/euc) - Rust software renderer
- [asciimare](https://github.com/LingDong-/asciimare) - Python ASCII 3D engine
- [AsciiEngine](https://github.com/sephirot47/AsciiEngine) - C++ ASCII game engine
- [mtrebi/Raytracer](https://github.com/mtrebi/Raytracer) - C++ ray tracer with reflections

### Academic Resources
- [Kyle Halladay - Ray-Sphere Intersection](https://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html)
- [Gabriel Gambetta - Computer Graphics from Scratch](https://gabrielgambetta.com/computer-graphics-from-scratch/)
- [Lighthouse3d](https://www.lighthouse3d.com/tutorials/) - Graphics tutorials

---

## Conclusion

This RFC outlines a comprehensive plan to add 3D ASCII rendering capabilities to blaeck. The phased approach allows for incremental development, with each phase delivering usable functionality.

The end goal is to enable blaeck users to create stunning visual experiences that push the boundaries of what's possible in a terminal - from simple rotating shapes to ray-traced scenes with shadows and reflections that emerge from noise like a photograph developing.

**Recommended starting point:** Phase 1 (Core Math) → Phase 2 (Camera) → Phase 3 (Basic Rendering) → Phase 4 (Lighting). This gives a solid foundation that can render a lit sphere in ~4 phases.

---

*This RFC is a living document. Update as implementation progresses.*
