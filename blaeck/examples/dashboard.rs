//! Scalar Field Dashboard - A living system monitor where UI emerges from chaos.
//!
//! The plasma field isn't decoration—it's the system's energy substrate.
//! UI panels react to the underlying field, creating a terminal that feels sentient.
//!
//! Controls:
//! - q/Esc: Quit
//! - r: Restart boot sequence
//! - Space: Pause/resume field animation

#[path = "previews/mod.rs"]
mod previews;

use blaeck::prelude::*;
use crossterm::event::{poll, read, Event, KeyCode};
use std::time::{Duration, Instant};

use previews::dashboard::{
    build_dashboard, count_element_tree, panel_field_energy, DashboardState, FieldParams,
};

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
