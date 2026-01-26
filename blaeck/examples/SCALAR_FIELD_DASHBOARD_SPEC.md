# Scalar Field Dashboard - Implementation Specification

## Vision

Create a demo that makes the terminal feel **sentient**. The plasma animation isn't decoration—it's the system's energy substrate. UI panels emerge from and react to this living field.

**One-sentence pitch:** "The field is system entropy—UI emerges from chaos and draws energy from it."

This is NOT another logo-with-pretty-background. This is a **living system monitor** where animation and UI are physically coupled.

---

## Core Concept: Field-Coupled UI

The plasma/sine wave field we already have becomes a **scalar field F(x, y, t)** that UI elements sample and react to.

### Field Properties at Each Point
- **Value**: -1.0 to 1.0 (from plasma_value function)
- **Intensity**: absolute value, 0.0 to 1.0
- **Temperature**: mapped to color warmth

### How UI Reacts to Field

| UI Element | Field Reaction |
|------------|----------------|
| Panel border | Thickness/brightness increases when hot blob passes underneath |
| Panel background | Color temperature shifts with local field value |
| Text color | Subtle glow when field intensity is high |
| Numbers | Tick faster when field is active nearby |
| Progress bars | Color sampled from field at bar position |

---

## File Structure

Create: `blaeck/examples/dashboard.rs`

---

## Phase 1: Boot Sequence (First 3-4 seconds)

The demo does NOT start with full UI. It **boots up**.

### Timeline

```
T+0.0s    Black screen
T+0.3s    Field begins to ignite (dim, low saturation plasma appears)
T+0.8s    Field reaches full intensity
T+1.2s    First text appears: "initializing..." (dim, bottom left)
T+1.5s    System checks begin printing:
            "renderer ........ OK"
            "scheduler ....... OK"
            "entropy pool .... OK"
            "field stable .... OK"
T+2.5s    Checks complete, text fades
T+3.0s    UI panels begin fading in (one by one, 200ms apart)
T+3.8s    All panels visible
T+4.5s    Small "BLAECK" text appears bottom-right (subtle, earned)
```

### Boot Text Style
- Monospace, dim gray
- Each line appears with slight delay
- "OK" appears green after dots animate
- Use actual timing, not instant

---

## Phase 2: Steady State Layout

```
┌────────────────────────────────────────────────────────────────────────────────┐
│                                                                                │
│   ┌─ RENDERER ─────────┐    ┌─ LAYOUT ──────────┐    ┌─ MEMORY ─────────┐     │
│   │ frames   1,847     │    │ nodes      42     │    │ used    12.4 MB  │     │
│   │ fps         60     │    │ depth       7     │    │ peak    18.2 MB  │     │
│   │ latency   2.1ms    │    │ reflows     3     │    │ gc         0     │     │
│   └────────────────────┘    └───────────────────┘    └──────────────────┘     │
│                                                                                │
│   ┌─ ENTROPY ──────────────────────────────────────────────────────────────┐  │
│   │ ░▒▓█▓▒░░▒▓██▓▒░░▒▓█▓▒░░▒▓██▓▒░░▒▓█▓▒░░▒▓██▓▒░░▒▓█▓▒░░▒▓██▓▒░░▒▓█▓▒░  │  │
│   └────────────────────────────────────────────────────────────────────────┘  │
│                                                                                │
│   ┌─ SIGNAL ───────────┐    ┌─ STATUS ──────────────────────────────────┐     │
│   │ ▕▕▕▕▕▕▕▕▕▕░░░░░░░░ │    │ [module::core]      operational          │     │
│   │ strength    67%    │    │ [module::render]    operational          │     │
│   │ drift    +0.003    │    │ [module::input]     operational          │     │
│   └────────────────────┘    │ [module::panic]     dormant              │     │
│                             └───────────────────────────────────────────┘     │
│                                                                                │
│                                                                    blaeck 0.2 │
└────────────────────────────────────────────────────────────────────────────────┘
```

### Panel Placement Rules
- NOT centered
- Asymmetric, functional layout
- Panels cluster toward top-left (like a real control panel)
- Logo is SMALL, bottom-right, appears LAST

---

## Phase 3: Field-UI Coupling Implementation

### Sampling the Field

For each panel, calculate its "field energy" by sampling the plasma field at the panel's center:

```rust
fn panel_field_energy(panel_x: f64, panel_y: f64, time: f64, params: &Params) -> f64 {
    // Sample field at panel center (normalized 0-1 coordinates)
    let value = plasma_value(panel_x, panel_y, time, params);
    // Return intensity (0.0 to 1.0)
    (value + 1.0) / 2.0
}
```

### Border Brightness

Panel borders should pulse when a hot blob passes underneath:

```rust
fn border_color(base_color: Color, field_energy: f64) -> Color {
    // base_color is the dim default
    // When field_energy is high, brighten the border
    let boost = (field_energy * 80.0) as u8;
    match base_color {
        Color::Rgb(r, g, b) => Color::Rgb(
            r.saturating_add(boost),
            g.saturating_add(boost / 2),
            b.saturating_add(boost / 3),
        ),
        _ => base_color,
    }
}
```

### Number Animation

Numbers should "tick" faster when field is active:

```rust
fn animated_number(base_value: u64, time: f64, field_energy: f64) -> u64 {
    // Add subtle jitter proportional to field energy
    let jitter = ((time * 10.0).sin() * field_energy * 5.0) as i64;
    (base_value as i64 + jitter).max(0) as u64
}
```

### Text Glow

When field intensity is high under text, add subtle color shift:

```rust
fn text_color_with_glow(base: Color, field_energy: f64) -> Color {
    // Shift toward warmer color when field is hot
    if field_energy > 0.6 {
        let warmth = ((field_energy - 0.6) * 100.0) as u8;
        Color::Rgb(200 + warmth/2, 180, 160)
    } else {
        base
    }
}
```

---

## Phase 4: The Entropy Bar

A special element that directly visualizes field activity:

```
┌─ ENTROPY ──────────────────────────────────────────────────────────────┐
│ ░▒▓█▓▒░░▒▓██▓▒░░▒▓█▓▒░░▒▓██▓▒░░▒▓█▓▒░░▒▓██▓▒░░▒▓█▓▒░░▒▓██▓▒░░▒▓█▓▒░  │
└────────────────────────────────────────────────────────────────────────┘
```

### Implementation

Sample field at multiple x positions along the bar:

```rust
fn entropy_bar(width: usize, y_pos: f64, time: f64, params: &Params) -> String {
    let mut bar = String::new();
    for x in 0..width {
        let nx = x as f64 / width as f64;
        let value = plasma_value(nx, y_pos, time, params);
        let char = value_to_shade(value); // Use existing shade mapping
        bar.push(char);
    }
    bar
}
```

This bar becomes a **live cross-section** of the field.

---

## Color Palette

Use a restrained, serious palette. NOT neon cyberpunk.

### Base Colors (when field is calm)
- Background field: Deep blue-gray (#1a1a2e to #16213e)
- Panel borders: Dim cyan (#3a506b)
- Panel background: Near-black with slight transparency feel (#0f0f1a)
- Text: Soft white (#c9d1d9)
- Accent: Muted teal (#4a9079)

### Hot Colors (when field is active)
- Field blobs: Warm amber/orange (#d4a574 to #e07a5f)
- Boosted borders: Brighter cyan (#5eaaa8)
- Active text: Warm white (#f0e6d3)

### Status Colors
- OK/operational: Muted green (#6bcb77)
- Dormant: Dim gray (#4a4a5a)
- Warning (unused but define): Amber (#ffc93c)
- Error (unused but define): Soft red (#e07a5f)

---

## Animation Timing

### Field Animation
- Base speed: 0.15 (very slow, meditative)
- Use "Slow Flow" preset frequencies or similar
- Field should feel like breathing, not churning

### Panel Reactions
- Border brightness: Smooth lerp, ~200ms response time
- Number jitter: Immediate but subtle
- Don't make it twitchy—make it organic

### Idle Behavior
- After 10+ seconds of no input, field becomes even slower
- Panels dim slightly
- System feels "at rest"

---

## Fake Data (Important!)

All data is fake but **believable**:

### RENDERER Panel
```rust
struct RendererStats {
    frames: u64,      // Increments ~60/sec
    fps: u8,          // Hovers around 58-62
    latency_ms: f32,  // Fluctuates 1.8-2.4
}
```

### LAYOUT Panel
```rust
struct LayoutStats {
    nodes: u16,       // Stable at 42 (or similar)
    depth: u8,        // Stable at 7
    reflows: u16,     // Occasionally increments
}
```

### MEMORY Panel
```rust
struct MemoryStats {
    used_mb: f32,     // Slowly fluctuates 12-14
    peak_mb: f32,     // Stable or slowly grows
    gc_count: u16,    // Usually 0, occasionally 1
}
```

### SIGNAL Panel
```rust
struct SignalStats {
    strength_pct: u8, // Fluctuates 55-75
    drift: f32,       // Tiny value like +0.003, changes slowly
}
```

### STATUS Panel
- Static text, but "dormant" module occasionally flickers

---

## Keyboard Controls

Minimal:
- `q` or `Esc`: Quit
- `r`: Restart boot sequence
- `Space`: Pause/resume field animation

---

## Implementation Checklist

1. [ ] Create `dashboard.rs` example file
2. [ ] Implement boot sequence with timing
3. [ ] Create panel structs with position/size
4. [ ] Implement field sampling at panel positions
5. [ ] Implement border brightness coupling
6. [ ] Implement entropy bar
7. [ ] Add fake data generators
8. [ ] Add number jitter based on field
9. [ ] Final layout polish
10. [ ] Test at different terminal sizes

---

## What This Is NOT

- NOT a fullscreen logo reveal
- NOT neon/cyberpunk aesthetic
- NOT symmetrical "poster" layout
- NOT feature tiles with emojis
- NOT trying to impress with color

## What This IS

- A living system that breathes
- UI that emerges from chaos
- Restrained, serious, confident
- Makes people ask "how did they do that"
- Terminal feeling sentient

---

## Reference

The existing `plasma.rs` example has:
- `plasma_value(nx, ny, time, params)` - use this
- Shade characters: `█ ▓ ▒ ░ · `
- Color themes - use "Nocturne" as base, desaturate further

Build on that foundation. The field rendering already works—now make UI react to it.

---

## Success Criteria

When someone sees this demo, they should think:

> "This isn't a theme. This is an operating environment."

The animation isn't showing off. It's **doing something**.

That's the difference.
