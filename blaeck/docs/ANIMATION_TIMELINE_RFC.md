# RFC: Animation Timeline System for Blaeck

## Problem

Building complex animated sequences in blaeck currently requires manual state management:

```rust
// Current approach - manual timing hell
fn logo_opacity(&self) -> f64 {
    let boot = self.boot_time();
    if boot < 0.5 { boot * 2.0 }
    else if boot < 4.0 { 1.0 }
    else if boot < 5.0 { 1.0 - (boot - 4.0) }
    else { 0.0 }
}

fn current_positions(&self) -> LayoutPositions {
    let boot = self.boot_time();
    if boot < 2.0 { return LayoutPositions::offscreen(); }
    if boot < 4.0 { /* lerp logic */ }
    if boot < 5.0 { return LayoutPositions::around_logo(); }
    if boot < 6.0 { /* more lerp logic */ }
    // ... cycling logic
}
```

This is:
- **Error-prone**: Easy to get timing overlaps wrong
- **Hard to visualize**: Can't see the sequence at a glance
- **Not reusable**: Every animation needs custom state
- **Rigid**: Changing timing requires editing multiple places

## Proposal: Declarative Animation Timeline

A timeline-based animation system inspired by video editing NLEs, CSS animations, and game engines.

### Core Concepts

#### 1. Timeline

A sequence of **Acts** that play in order. Can loop.

```rust
let intro = Timeline::new()
    .act("logo_alone", 2.0)      // 2 seconds
    .act("panels_fly_in", 2.0)   // 2 seconds
    .act("logo_fade_out", 1.0)   // 1 second
    .act("panels_move", 1.0)     // 1 second
    .then_loop("cycling");       // Continue to cycling timeline
```

#### 2. Acts

Named time segments. Multiple properties can animate within an act.

```rust
Act::new("logo_alone")
    .duration(2.0)
    .on_enter(|cx| cx.set("logo_opacity", 0.0))
    .animate("logo_opacity", 0.0, 1.0, Easing::OutCubic)
    .hold("logo_opacity", 1.0)  // Stay at 1.0 for remainder
```

#### 3. Tracks

Parallel animation channels for different properties.

```rust
Timeline::new()
    .track("logo", logo_track)
    .track("panels", panels_track)
    .track("positions", position_track)
```

#### 4. Keyframes

Define values at specific times within an act.

```rust
Act::new("panels_fly_in")
    .keyframe(0.0, "panel_opacity", 0.0)
    .keyframe(0.3, "panel_opacity", 1.0)  // Fade in first 30%
    .keyframe(0.0, "panel_pos", Position::offscreen())
    .keyframe(1.0, "panel_pos", Position::around_logo())
    .easing(Easing::OutBack)  // Overshoot on position
```

#### 5. Easing Library

Built-in easing functions:

```rust
enum Easing {
    Linear,
    InQuad, OutQuad, InOutQuad,
    InCubic, OutCubic, InOutCubic,
    InBack, OutBack, InOutBack,  // Overshoot
    InElastic, OutElastic,        // Bounce
    InBounce, OutBounce,
    Custom(fn(f32) -> f32),
}
```

### API Design

#### Declarative Timeline Builder

```rust
use blaeck::animation::*;

let intro_sequence = Timeline::new()
    // Act 1: Logo fades in alone
    .act(Act::new("logo_intro")
        .duration(2.0)
        .animate("logo_opacity", 0.0 => 1.0)
        .easing(Easing::OutCubic)
        .hold_at_end())

    // Act 2: Panels fly in (logo stays visible)
    .act(Act::new("panels_enter")
        .duration(2.0)
        .animate("panel_opacity", 0.0 => 1.0)
        .animate("panel_positions", Positions::OFFSCREEN => Positions::AROUND_LOGO)
        .easing(Easing::OutBack))

    // Act 3: Logo fades out (panels stay)
    .act(Act::new("logo_exit")
        .duration(1.0)
        .animate("logo_opacity", 1.0 => 0.0)
        .easing(Easing::InCubic))

    // Act 4: Panels move to first layout
    .act(Act::new("panels_arrange")
        .duration(1.0)
        .animate("panel_positions", Positions::AROUND_LOGO => Positions::LEFT)
        .easing(Easing::InOutCubic));

let cycling = Timeline::new()
    .act(Act::hold("left", 5.0))
    .act(Act::transition("left_to_center", 1.4)
        .animate("panel_positions", Positions::LEFT => Positions::CENTER))
    .act(Act::hold("center", 5.0))
    .act(Act::transition("center_to_spread", 1.4)
        .animate("panel_positions", Positions::CENTER => Positions::SPREAD))
    .act(Act::hold("spread", 5.0))
    .act(Act::transition("spread_to_left", 1.4)
        .animate("panel_positions", Positions::SPREAD => Positions::LEFT))
    .loop_to("left");

let full_sequence = intro_sequence.then(cycling);
```

#### Using in Components

```rust
fn my_app(cx: Scope) -> Element {
    let timeline = use_timeline(cx, full_sequence);

    // Get current animated values
    let logo_opacity = timeline.get::<f64>("logo_opacity");
    let panel_positions = timeline.get::<Positions>("panel_positions");
    let current_act = timeline.current_act();

    element! {
        Box {
            // Logo with animated opacity
            Box(opacity: logo_opacity) {
                #(render_logo())
            }
            // Panels with animated positions
            #(render_panels(panel_positions))
        }
    }
}
```

#### Timeline Events

```rust
let timeline = Timeline::new()
    .act(Act::new("intro")
        .on_start(|| println!("Intro started"))
        .on_complete(|| println!("Intro done")))
    .on_loop(|count| println!("Loop iteration: {}", count));
```

### Advanced Features

#### 1. Stagger Animations

Animate multiple items with delay between each:

```rust
Act::new("panels_enter")
    .stagger("panel_opacity", 0.0 => 1.0)
    .stagger_delay(0.15)  // 150ms between each panel
    .stagger_order(StaggerOrder::LeftToRight)
```

#### 2. Spring Physics

For more natural motion:

```rust
Act::new("bounce_in")
    .spring("position", target, Spring {
        stiffness: 100.0,
        damping: 10.0,
        mass: 1.0,
    })
```

#### 3. Relative Timing

Acts relative to others:

```rust
Timeline::new()
    .act(Act::new("a").duration(2.0))
    .act(Act::new("b").starts_at("a", 0.5))  // Start 0.5s into "a"
    .act(Act::new("c").starts_after("a", 0.2))  // Start 0.2s after "a" ends
```

#### 4. Conditional Branching

```rust
Timeline::new()
    .act(intro)
    .branch(|state| {
        if state.user_skipped {
            Act::instant("skip_to_main")
        } else {
            Act::new("full_intro").duration(3.0)
        }
    })
```

#### 5. Timeline Controls

```rust
let timeline = use_timeline(cx, sequence);

// Playback control
timeline.play();
timeline.pause();
timeline.seek(3.5);  // Jump to 3.5 seconds
timeline.set_speed(2.0);  // 2x speed
timeline.reverse();

// State queries
timeline.elapsed();      // Current time
timeline.progress();     // 0.0 - 1.0
timeline.current_act();  // "panels_enter"
timeline.is_playing();
```

### Data Types

```rust
/// Animated value that can be interpolated
pub trait Animatable: Clone {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self;
}

// Built-in implementations
impl Animatable for f32 { ... }
impl Animatable for f64 { ... }
impl Animatable for Color { ... }
impl Animatable for (f32, f32) { ... }  // Positions
impl Animatable for LayoutPositions { ... }

/// Custom positions struct
#[derive(Animatable)]
struct PanelPositions {
    buffer: (f32, f32),
    layout: (f32, f32),
    process: (f32, f32),
    // ...
}
```

### Integration with Reactive System

```rust
fn dashboard(cx: Scope) -> Element {
    // Timeline as a signal
    let timeline = use_timeline(cx, dashboard_sequence);

    // Derived values automatically update
    let logo_opacity = timeline.get("logo_opacity");
    let positions = timeline.get("positions");

    // Timeline state in UI
    let act_name = timeline.current_act();
    let progress = timeline.progress();

    // Manual control
    use_input(cx, move |key| {
        if key.is_char('r') {
            timeline.restart();
        }
        if key.is_char(' ') {
            timeline.toggle_pause();
        }
    });

    element! { ... }
}
```

### File Format (Optional)

For complex sequences, support loading from a file:

```yaml
# animation.timeline.yaml
timeline:
  - act: logo_intro
    duration: 2s
    tracks:
      logo_opacity:
        from: 0
        to: 1
        easing: out-cubic

  - act: panels_fly_in
    duration: 2s
    tracks:
      panel_opacity:
        from: 0
        to: 1
      panel_positions:
        from: offscreen
        to: around_logo
        easing: out-back

  - act: logo_exit
    duration: 1s
    tracks:
      logo_opacity:
        from: 1
        to: 0

cycling:
  loop: true
  acts:
    - hold: left
      duration: 5s
    - transition: left_to_center
      duration: 1.4s
    # ...
```

## Implementation Plan

### Phase 1: Core Timeline Engine
- [ ] `Timeline` struct with acts
- [ ] `Act` struct with duration and keyframes
- [ ] Basic easing functions
- [ ] `Animatable` trait for interpolation

### Phase 2: Reactive Integration
- [ ] `use_timeline` hook
- [ ] Automatic re-render on value changes
- [ ] Timeline controls (play/pause/seek)

### Phase 3: Advanced Features
- [ ] Stagger animations
- [ ] Parallel tracks
- [ ] Timeline events/callbacks
- [ ] Spring physics

### Phase 4: Developer Experience
- [ ] Timeline visualization/debugger
- [ ] Hot-reload timeline changes
- [ ] YAML/JSON file format

## Prior Art

- **CSS Animations**: Keyframes, easing, fill-mode
- **Framer Motion**: Declarative React animations
- **GSAP**: JavaScript animation timeline
- **After Effects**: Composition-based timeline
- **Unity Animator**: State machine + timeline hybrid

## Open Questions

1. Should timelines be serializable for persistence?
2. How to handle timeline interruption (user input during animation)?
3. Should we support procedural/physics-based animation alongside keyframes?
4. Integration with existing `AnimationTimer` in blaeck?

---

*This RFC proposes making animation a first-class citizen in blaeck, reducing boilerplate and enabling complex choreographed sequences with a declarative API.*
