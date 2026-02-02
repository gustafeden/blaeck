# Animation Timeline System for Blaeck

## Status: Implemented

This document describes the declarative animation timeline system in blaeck.

## Overview

The timeline system replaces manual state-based animation code with a declarative API inspired by video editing software, CSS animations, and game engines.

**Before (manual timing):**
```rust
fn logo_opacity(&self) -> f64 {
    let t = self.boot_time();
    if t < 0.5 { t * 2.0 }
    else if t < 4.0 { 1.0 }
    else if t < 5.0 { 1.0 - (t - 4.0) }
    else { 0.0 }
}
```

**After (declarative timeline):**
```rust
let timeline = Timeline::new()
    .act(Act::new("fade_in").duration(0.5)
        .animate("opacity", 0.0, 1.0, Easing::EaseOutCubic))
    .act(Act::hold("visible", 3.5))
    .act(Act::new("fade_out").duration(1.0)
        .animate("opacity", 1.0, 0.0, Easing::EaseInCubic));

// Query at any time
let opacity: f64 = timeline.at(2.0).get("opacity").unwrap_or(1.0);
```

## Core Types

### Timeline

A sequence of **Acts** that play in order, with optional looping.

```rust
use blaeck::timeline::*;
use blaeck::animation::Easing;

let sequence = Timeline::new()
    .act(Act::new("intro").duration(2.0)
        .animate("opacity", 0.0, 1.0, Easing::EaseOutCubic))
    .act(Act::new("main").duration(5.0))
    .act(Act::new("outro").duration(2.0)
        .animate("opacity", 1.0, 0.0, Easing::EaseInCubic))
    .loop_forever();  // or .loop_from("main")
```

**Loop behaviors:**
- `loop_forever()` - Loop from the beginning
- `loop_from("act_name")` - Play intro once, then loop from specified act

### Act

A named time segment containing animated properties.

```rust
Act::new("panels_enter")
    .duration(2.0)
    // Simple from→to animation
    .animate("opacity", 0.0, 1.0, Easing::EaseOutCubic)
    // Spring physics
    .spring("position", 0.0, 100.0, Spring::preset_bouncy())
    // Staggered animation for multiple items
    .stagger("panel_alpha", 5, 0.0, 1.0, Easing::EaseOutCubic)
    // Callbacks
    .on_enter(|| println!("Act started"))
    .on_exit(|| println!("Act ended"))
```

**Convenience constructors:**
- `Act::new(name)` - Standard act
- `Act::hold(name, duration)` - No animations, just wait
- `Act::transition(name, duration)` - Semantic alias for transitions

### Track

Keyframe-based animation for a single property. Created automatically via `.animate()` or manually for complex sequences:

```rust
let track = Track::new()
    .keyframe(0.0, 0.0f64, Easing::Linear)    // Start at 0
    .keyframe(0.3, 1.0, Easing::EaseOutCubic)  // Quick rise
    .keyframe(0.7, 0.8, Easing::Linear)        // Slight dip
    .keyframe(1.0, 1.0, Easing::EaseInCubic);  // Return to full

Act::new("complex")
    .duration(2.0)
    .track("custom_prop", track)
```

### Animatable Trait

Values that can be interpolated. Built-in implementations:

```rust
pub trait Animatable: Clone + Send + Sync + 'static {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self;
}

// Implemented for:
// - f32, f64, i32, u8
// - (f32, f32), (f64, f64) - positions
// - (u8, u8, u8) - RGB
// - (u8, u8, u8, u8) - RGBA
```

## Advanced Features

### Spring Physics

Natural, physics-based motion:

```rust
Act::new("bounce")
    .duration(1.5)
    .spring("y_position", 0.0, 100.0, Spring::preset_bouncy())
```

**Spring presets:**
- `Spring::preset_gentle()` - Smooth, minimal overshoot
- `Spring::preset_bouncy()` - Playful with visible overshoot
- `Spring::preset_stiff()` - Quick, snappy motion
- `Spring::preset_slow()` - Heavy, deliberate movement

**Custom springs:**
```rust
Spring::new(stiffness, damping)  // mass defaults to 1.0
Spring::with_mass(stiffness, damping, mass)
```

### Stagger Animations

Animate multiple items with cascading delays:

```rust
Act::new("list_enter")
    .duration(1.0)
    .stagger("item_opacity", 5, 0.0, 1.0, Easing::EaseOutCubic)
    .stagger_config("item_opacity", |cfg| cfg
        .delay(0.1)  // 10% of duration between items
        .order(StaggerOrder::Forward))
```

**Stagger orders:**
- `Forward` - First to last (0, 1, 2, ...)
- `Reverse` - Last to first
- `CenterOut` - Center items first, edges last
- `EdgesIn` - Edge items first, center last
- `Random` - Deterministic pseudo-random

**Retrieving stagger values:**
```rust
let item_3_opacity: f64 = state.get_stagger("item_opacity", 3).unwrap_or(0.0);
let all_opacities: Vec<f64> = state.get_stagger_all("item_opacity", 5);
```

### Timeline Callbacks

```rust
Timeline::new()
    .act(Act::new("intro")
        .on_enter(|| log("intro started"))
        .on_exit(|| log("intro ended")))
    .on_loop(|count| log(&format!("Loop #{}", count)))
    .on_act_enter(|name| log(&format!("Entered: {}", name)))
    .on_act_exit(|name| log(&format!("Exited: {}", name)))
```

## Usage

### Static Queries

Query timeline state at any point:

```rust
let timeline = Timeline::new()
    .act(Act::new("fade").duration(1.0)
        .animate("opacity", 0.0, 1.0, Easing::Linear));

let state = timeline.at(0.5);  // Halfway through
let opacity: f64 = state.get("opacity").unwrap_or(0.0);
assert_eq!(opacity, 0.5);
```

### Playing Timeline

For real-time animation:

```rust
let mut playing = timeline.start();

// In render loop:
let state = playing.state();
let opacity: f64 = state.get("opacity").unwrap_or(0.0);

// Controls
playing.pause();
playing.play();
playing.toggle_pause();
playing.seek(1.5);       // Jump to 1.5 seconds
playing.restart();
playing.set_speed(2.0);  // 2x playback

// Queries
playing.elapsed();       // Current time in seconds
playing.progress();      // 0.0-1.0 for non-looping
playing.current_act();   // "fade"
playing.act_progress();  // Progress within current act
playing.is_paused();
```

### Reactive Hook

For use with blaeck's reactive system:

```rust
use blaeck::reactive::*;

fn animated_component(cx: Scope) -> Element {
    let timeline = use_timeline(cx, Timeline::new()
        .act(Act::new("pulse").duration(2.0)
            .animate("scale", 1.0, 1.2, Easing::EaseInOutCubic))
        .loop_forever());

    let scale: f64 = timeline.get_or("scale", 1.0);

    element! {
        Box(width: 10.0 * scale as f32) {
            Text(content: "Pulsing!")
        }
    }
}
```

**TimelineHandle methods:**
```rust
// Values
timeline.get::<T>(property)
timeline.get_or::<T>(property, default)
timeline.get_stagger::<T>(property, index)
timeline.get_stagger_all::<T>(property, count)

// State
timeline.elapsed()
timeline.current_act()
timeline.act_progress()
timeline.progress()
timeline.loop_count()

// Controls
timeline.pause()
timeline.play()
timeline.toggle_pause()
timeline.seek(time)
timeline.restart()
timeline.set_speed(multiplier)

// Debug
timeline.debug_info()  // For visualization
timeline.update()      // Fire pending callbacks
```

### Chaining Timelines

```rust
let intro = Timeline::new()
    .act(Act::new("logo").duration(2.0));

let main = Timeline::new()
    .act(Act::new("content").duration(5.0))
    .loop_forever();

let full = intro.then(main);  // Plays intro, then loops main
```

## Debug Visualization

The `TimelineDebugInfo` struct provides data for building debug UIs:

```rust
let debug = timeline.debug_info();

debug.elapsed           // Current time
debug.total_duration    // Full timeline length
debug.current_act_name  // Active act
debug.current_act_index // Active act index
debug.act_progress      // Progress in current act
debug.loop_count        // Times looped
debug.is_paused         // Playback state
debug.speed             // Playback speed
debug.acts              // Vec of (name, start_time, duration)
```

## Examples

**Basic fade sequence:**
```rust
Timeline::new()
    .act(Act::new("in").duration(0.5)
        .animate("opacity", 0.0, 1.0, Easing::EaseOutCubic))
    .act(Act::hold("visible", 3.0))
    .act(Act::new("out").duration(0.5)
        .animate("opacity", 1.0, 0.0, Easing::EaseInCubic))
```

**Staggered list with spring:**
```rust
Timeline::new()
    .act(Act::new("enter").duration(1.5)
        .stagger("y", 10, -50.0, 0.0, Easing::EaseOutCubic)
        .stagger_config("y", |c| c.delay(0.08).order(StaggerOrder::Forward))
        .stagger("opacity", 10, 0.0, 1.0, Easing::EaseOutCubic))
```

**Looping ambient animation:**
```rust
Timeline::new()
    .act(Act::new("breathe_in").duration(2.0)
        .animate("scale", 1.0, 1.05, Easing::EaseInOutCubic))
    .act(Act::new("breathe_out").duration(2.0)
        .animate("scale", 1.05, 1.0, Easing::EaseInOutCubic))
    .loop_forever()
```

## Files

- `blaeck/src/timeline.rs` - Core module
- `blaeck/src/reactive/hooks.rs` - `use_timeline` hook
- `blaeck/examples/timeline_demo.rs` - Basic example
- `blaeck/examples/reactive_timeline.rs` - Reactive example
- `blaeck/examples/stagger_demo.rs` - Stagger example
- `blaeck/examples/timeline_debug.rs` - Debug visualization

## Future Work

- [ ] Timeline serialization (YAML/JSON)
- [ ] Timeline interruption/blending
- [ ] Parallel tracks (multiple properties animating independently)
- [ ] Derive macro for custom Animatable types
- [ ] Timeline editor/visualizer tool
