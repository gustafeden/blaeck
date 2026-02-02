//! Blaeck Showcase Demo
//!
//! The definitive demo for blaeck's capabilities.
//! 12-second cinematic loop showcasing:
//! - Plasma background with bloom effects
//! - Animated logo with elastic physics
//! - Live system metrics
//! - Component showcase (progress, sparkline, gradient)
//! - Theme switching
//!
//! Controls:
//!   q/Esc  - Quit
//!   Space  - Pause/resume
//!   t      - Cycle themes
//!   r      - Restart animation

#[path = "previews/mod.rs"]
mod previews;

use blaeck::prelude::*;
use crossterm::event::{poll, read, Event, KeyCode};
use std::time::{Duration, Instant};

use previews::showcase::{build_showcase, ShowcaseState};

fn main() -> std::io::Result<()> {
    // Optional: --duration <secs> for auto-quit (useful for scripted recordings)
    let duration_limit = {
        let args: Vec<String> = std::env::args().collect();
        args.iter()
            .position(|a| a == "--duration")
            .and_then(|i| args.get(i + 1))
            .and_then(|s| s.parse::<f64>().ok())
    };

    let mut blaeck = Blaeck::new(std::io::stdout())?;
    let mut state = ShowcaseState::new();
    let mut last_time = Instant::now();
    let start_time = Instant::now();

    crossterm::terminal::enable_raw_mode()?;
    blaeck.set_cursor_visible(false);

    loop {
        if let Some(limit) = duration_limit {
            if start_time.elapsed().as_secs_f64() >= limit {
                break;
            }
        }

        if poll(Duration::from_millis(16))? {
            match read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => break,
                    KeyCode::Char(' ') => state.toggle_pause(),
                    KeyCode::Char('t') => state.next_theme(),
                    KeyCode::Char('r') => state.restart(),
                    _ => {}
                },
                Event::Resize(w, h) => {
                    let _ = blaeck.handle_resize(w, h);
                }
                _ => {}
            }
        }

        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f64();
        last_time = now;

        let render_start = Instant::now();
        let ui = build_showcase(&state);
        blaeck.render(ui)?;
        let render_ms = render_start.elapsed().as_secs_f32() * 1000.0;

        state.update(dt, render_ms);
    }

    blaeck.set_cursor_visible(true);
    crossterm::terminal::disable_raw_mode()?;
    blaeck.unmount()?;

    Ok(())
}
