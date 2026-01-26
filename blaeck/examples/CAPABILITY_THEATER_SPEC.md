# Capability Theater - Implementation Specification

## Vision

This is NOT a dashboard. This is a **theatrical montage of capabilities**, choreographed like a movie trailer.

**Goal:** Make competent developers feel slightly stupid for not already using this.

**One-sentence design principle:**

> "If I removed all text from the preview, would a Rust dev still understand what Blaeck is good at?"

The animation must be **honest**—it demonstrates real capabilities through motion, not decoration.

---

## Core Philosophy

### What We're NOT Doing
- Pretty colors for their own sake
- Static "poster" layout
- Feature cards with emojis
- Giant centered logo
- Anything that begs for attention

### What We ARE Doing
- Demonstrating capabilities through motion
- Showing inevitability ("of course it can do that")
- Calm confidence
- Making people quietly angry they didn't think of this first

---

## File Structure

Create: `blaeck/examples/theater.rs`

---

## The 3-Act Structure (15-20 second loop)

### ACT I — The Field (0-3s)
**Purpose:** Atmosphere & confidence. "We can do complex animation without trying."

### ACT II — Capabilities Emerge (3-12s)
**Purpose:** The money shot. Features demonstrated one by one through motion.

### ACT III — The Quiet Flex (12-15s)
**Purpose:** Everything settles. Small branding. "We're done proving ourselves."

Then loop back to Act I seamlessly.

---

## ACT I: The Field (0-3 seconds)

### Visual State
- Screen is the muted plasma field ONLY
- No UI elements yet
- Field is:
  - Low contrast (desaturated 40%)
  - Slow movement (speed 0.1)
  - Dark color palette
  - Reads as **substrate**, not spectacle

### Animation
```
T+0.0s    Field visible, very slow, dark
T+1.0s    Field continues, establishing calm
T+2.5s    Slight brightening (anticipation)
T+3.0s    First panel begins to fade in
```

### Color Palette for Act I
- Field: Deep blue-black (#0a0a12 to #1a1a2e)
- Blobs: Very muted purple/blue (#2a2a4a)
- Almost monochromatic

### Code Hint
```rust
// Muted field rendering
fn act1_field_color(value: f64, time: f64) -> Color {
    let base = 20 + (value.abs() * 30.0) as u8;
    Color::Rgb(base, base, base + 10) // Slightly cool
}
```

---

## ACT II: Capabilities Emerge (3-12 seconds)

This is the **money shot**. Each capability gets ~2 seconds to shine.

### Sequence Timeline

```
T+3.0s  CAPABILITY 1: Flexbox Layout
        - Single panel fades in, left side
        - Clean, minimal content

T+5.0s  CAPABILITY 2: Dynamic Reflow
        - Panel smoothly SPLITS into two
        - Content redistributes with easing
        - Field turbulence increases briefly during reflow

T+7.0s  CAPABILITY 3: Stateful Updates
        - Numbers in panels begin ticking
        - Progress bars ease (not jump)
        - Shows reactivity

T+9.0s  CAPABILITY 4: Theme Transition
        - Colors diffuse through the field
        - Old palette dissolves, new condenses
        - NOT instant swap—physical event

T+11.0s CAPABILITY 5: Animation Coupling
        - Field visibly responds to UI state
        - When panels are active → field more turbulent
        - Demonstrates integration
```

### Capability 1: Flexbox Layout (T+3-5s)

**What appears:**
```
┌─────────────────────────┐
│  RENDER PIPELINE        │
│                         │
│  nodes     47           │
│  layers     3           │
│  reflows    0           │
│                         │
└─────────────────────────┘
```

**Animation:**
- Fades in from 0% to 100% opacity over 500ms
- Slight scale animation (98% → 100%)
- Border draws itself (like a typewriter effect for the box)

**Field reaction:**
- Slight ripple emanates from panel position
- Field brightens 10% in panel area

---

### Capability 2: Dynamic Reflow (T+5-7s)

**What happens:**
The single panel smoothly **splits** into two side-by-side panels.

**Before:**
```
┌─────────────────────────┐
│  RENDER PIPELINE        │
│  nodes: 47  layers: 3   │
└─────────────────────────┘
```

**After:**
```
┌────────────┐ ┌────────────┐
│  RENDER    │ │  LAYOUT    │
│  nodes: 47 │ │  layers: 3 │
└────────────┘ └────────────┘
```

**Animation:**
- Panel width shrinks smoothly (easeOutCubic, 400ms)
- Second panel fades in as space opens
- Content reflows inside panels
- Gap between panels opens naturally

**Field reaction:**
- Turbulence INCREASES during reflow
- Blobs move faster, more chaotic
- Once settled → field calms back down

**Code hint:**
```rust
fn layout_stress_multiplier(is_reflowing: bool, time_since_reflow: f64) -> f64 {
    if is_reflowing {
        2.0  // Double turbulence during reflow
    } else {
        // Decay back to normal over 500ms
        1.0 + (1.0 - (time_since_reflow * 2.0).min(1.0))
    }
}
```

---

### Capability 3: Stateful Updates (T+7-9s)

**What happens:**
Numbers come alive. Progress bars move.

**Visual:**
```
┌────────────┐ ┌────────────┐
│  RENDER    │ │  LAYOUT    │
│  nodes: 52 │ │  layers: 3 │  ← nodes ticking up
│  ▓▓▓▓▓░░░░ │ │  fps: 60   │  ← bar animating
└────────────┘ └────────────┘
```

**Animation details:**
- Numbers tick with realistic cadence (not every frame)
- Progress bars use **easing**, not linear jumps
- Occasionally a number "hesitates" (human touch)
- FPS counter hovers around 58-62 (believable, not perfect 60)

**The key insight:**
Numbers that tick at inconsistent intervals feel more real than metronome precision.

```rust
fn should_tick_number(time: f64, base_rate: f64, jitter: f64) -> bool {
    let interval = base_rate + (time * 7.3).sin() * jitter;
    // Tick roughly every `interval` seconds with jitter
    (time % interval) < 0.016
}
```

---

### Capability 4: Theme Transition (T+9-11s)

**What happens:**
Colors change—but as a **physical event**, not instant swap.

**The effect:**
- New colors appear at edges of field
- They **diffuse** inward like ink in water
- Old colors dissolve as new ones take over
- Takes ~1.5 seconds to complete

**NOT this:**
```
Frame 1: Blue theme
Frame 2: Orange theme  // Instant, jarring
```

**YES this:**
```
Frame 1-30: Blue theme
Frame 31-60: Orange bleeding in from edges
Frame 61-90: Colors mixing, transitioning
Frame 91-120: Orange theme dominant
```

**Implementation approach:**
```rust
fn transitioning_color(
    old_theme: &ColorTheme,
    new_theme: &ColorTheme,
    progress: f64,  // 0.0 to 1.0
    x: f64, y: f64,  // position for spatial diffusion
) -> Color {
    // Progress is faster at edges, slower at center
    let edge_dist = ((x - 0.5).abs() + (y - 0.5).abs()) / 1.0;
    let local_progress = (progress * 1.5 - (1.0 - edge_dist) * 0.5).clamp(0.0, 1.0);

    // Lerp between old and new colors
    lerp_color(old_theme.base, new_theme.base, local_progress)
}
```

**Field reaction:**
- During transition, field becomes more active
- Blobs seem to "carry" the new color

---

### Capability 5: Animation Coupling (T+11-12s)

**What happens:**
Demonstrate that field and UI are **physically connected**.

**Visual:**
- A third panel appears (slides in from right)
- As it appears, the field underneath it brightens
- Panel content shows fake "activity"
- Field turbulence increases in that region

**The message:**
"Animation isn't decoration—it's integrated into the system."

```
┌────────────┐ ┌────────────┐ ┌────────────┐
│  RENDER    │ │  LAYOUT    │ │  ACTIVITY  │ ← new panel
│  nodes: 58 │ │  layers: 3 │ │  ▓▓▓▓▓▓▓░░ │
│  ▓▓▓▓▓▓░░░ │ │  fps: 61   │ │  events: 7 │
└────────────┘ └────────────┘ └────────────┘
                                    ↑
                              Field is brighter/
                              more active here
```

---

## ACT III: The Quiet Flex (12-15 seconds)

### What Happens
- Animation slows down
- Field becomes calmer, darker
- Panels remain but dim slightly
- Everything feels **stable**

### The Branding Moment (T+13s)
Small text fades in, bottom-right corner:

```
                                        blaeck 0.2
                                        Terminal UI · Rust
```

- Font: dim, not bold
- Size: small
- Position: bottom-right, tucked away
- Fade-in: slow, 800ms

**This says:** "We're done proving ourselves."

### Transition Back to Act I (T+14.5-15s)
- Panels fade out gracefully (400ms)
- Field continues
- Seamless loop back to Act I

---

## The Frame Time Indicator (Optional but Powerful)

Tiny corner element that Rust devs will notice:

```
┌─────────────────────┐
│ FRAME ▓▓░░░  8ms   │
└─────────────────────┘
```

**Behavior:**
- During calm moments: 6-10ms (green-ish)
- During reflow/transitions: 12-16ms (yellow-ish)
- Never red (we're not struggling)

**Position:** Top-right corner, very small, very dim.

**Why this matters:**
Rust developers care about performance. This whispers: "We think about frame budgets."

```rust
fn frame_time_display(last_frame_ms: f64, field_activity: f64) -> (String, Color) {
    // Fake it—actual frame time + perceived "cost" of animation
    let display_ms = last_frame_ms + field_activity * 4.0;
    let bar_fill = (display_ms / 20.0 * 5.0) as usize;
    let bar = format!("{}{}",
        "▓".repeat(bar_fill.min(5)),
        "░".repeat(5 - bar_fill.min(5))
    );
    let color = if display_ms < 10.0 {
        Color::Rgb(100, 160, 100)  // Calm green
    } else {
        Color::Rgb(180, 160, 100)  // Warm yellow
    };
    (format!("FRAME {} {:>4.1}ms", bar, display_ms), color)
}
```

---

## Layout Stress → Field Distortion

**Core rule:** When UI changes, field reacts.

| UI Event | Field Reaction |
|----------|----------------|
| Panel appears | Ripple emanates from panel center |
| Panel reflows | Turbulence increases for 500ms |
| Theme changes | Colors diffuse spatially |
| Numbers tick | Micro-pulse at number position |
| Panel disappears | Field "absorbs" the space |

**Implementation:**
Track "stress events" and apply them to field rendering:

```rust
struct StressEvent {
    position: (f64, f64),
    intensity: f64,
    start_time: f64,
    duration: f64,
}

fn apply_stress_to_field(
    base_value: f64,
    x: f64, y: f64,
    events: &[StressEvent],
    current_time: f64,
) -> f64 {
    let mut stress = 0.0;
    for event in events {
        let age = current_time - event.start_time;
        if age < event.duration {
            let dist = ((x - event.position.0).powi(2) +
                       (y - event.position.1).powi(2)).sqrt();
            let falloff = 1.0 / (1.0 + dist * 5.0);
            let decay = 1.0 - (age / event.duration);
            stress += event.intensity * falloff * decay;
        }
    }
    base_value + stress * 0.3  // Stress adds to field activity
}
```

---

## Color Palette

### Act I (Muted, Atmospheric)
- Field base: #0a0a12
- Field blobs: #1a1a2e
- Almost monochromatic blue-black

### Act II (Capabilities Shown)
- Panel borders: #3a506b (muted cyan)
- Panel background: #0f0f1a (near black)
- Text: #c9d1d9 (soft white)
- Accent: #4a9079 (muted teal)
- Active elements: #5eaaa8 (brighter teal)

### Theme Transition Target
- Warm palette: #e07a5f, #d4a574 (terracotta/amber)
- Demonstrates theme flexibility

### Act III (Settled)
- Everything dims 20%
- Branding text: #6a6a7a (dim gray)

---

## Keyboard Controls

Minimal:
- `q` / `Esc`: Quit
- `Space`: Pause/resume the sequence
- `r`: Restart from Act I
- `1-3`: Jump to specific act (for testing)

---

## Sound Design (Just Kidding... Unless?)

No actual sound, but the **visual rhythm** should feel like it has a soundtrack:
- Act I: Ambient drone (slow, minimal change)
- Act II: Building, purposeful (each capability is a "beat")
- Act III: Resolution, fade out

Design the timing as if you're scoring it.

---

## Implementation Checklist

1. [ ] Create `theater.rs` example file
2. [ ] Implement act state machine (Act1 → Act2 → Act3 → loop)
3. [ ] Implement muted field renderer for Act I
4. [ ] Panel fade-in with border animation
5. [ ] Panel split/reflow animation with easing
6. [ ] Number ticking with jitter
7. [ ] Progress bar easing
8. [ ] Theme transition with spatial diffusion
9. [ ] Layout stress → field distortion system
10. [ ] Frame time indicator
11. [ ] Small branding fade-in for Act III
12. [ ] Seamless loop back to Act I
13. [ ] Test timing feels "right" (this is subjective but critical)

---

## Success Criteria

After watching the demo, a Rust developer should think:

> "This looks like a system I could build serious tools with."

They should **not** think:
- "Cool animation"
- "Nice colors"
- "Interesting experiment"

They **should** think:
- "This handles layout properly"
- "This feels production-ready"
- "I could ship with this"

---

## The Unspoken Questions Your Demo Answers

| Question in Dev's Mind | How Demo Answers It |
|------------------------|---------------------|
| "Can it animate?" | Field is always moving, smoothly |
| "Does layout work?" | Panels split and reflow naturally |
| "Is it reactive?" | Numbers tick, bars ease |
| "Will it fight me?" | Everything feels controlled, calm |
| "Is it performant?" | Frame time indicator shows low cost |
| "Is it mature?" | Restrained design, no gimmicks |

---

## Final Note: Restraint is the Flex

The hardest thing to do is **not** show everything.

- Don't animate for the sake of animating
- Don't use bright colors because you can
- Don't make the logo big because you're proud

Make every motion **earn its pixels**.

The goal isn't to impress.
The goal is to make adoption feel **inevitable**.

---

## Reference Files

Build on:
- `plasma.rs` - has field rendering, `plasma_value()` function
- `SCALAR_FIELD_DASHBOARD_SPEC.md` - has field-UI coupling concepts

This demo is more choreographed/sequential than the dashboard. It's a **timeline**, not a **state**.
