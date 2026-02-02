//! Plasma & Lava Lamp effects - flowing colors and floating blobs.
//!
//! Two modes:
//! - Plasma: Layered sine waves (classic demo effect)
//! - Lava Lamp: Metaball simulation with rising/falling blobs

#[path = "previews/mod.rs"]
mod previews;

use blaeck::prelude::*;
use crossterm::event::{poll, read, Event, KeyCode};
use std::time::{Duration, Instant};

use previews::plasma::{
    build_display, build_info, LavaLamp, Mode, Params, DEFAULT_HEIGHT, DEFAULT_WIDTH,
};

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

    // Get initial terminal size, capped at default max
    let (mut width, mut height) = crossterm::terminal::size()
        .map(|(w, h)| (w as usize, h as usize))
        .unwrap_or((DEFAULT_WIDTH + 4, DEFAULT_HEIGHT + 4));
    // Leave room for border (2) and help text (1), cap at defaults
    width = width.saturating_sub(4).clamp(40, DEFAULT_WIDTH);
    height = height.saturating_sub(4).clamp(10, DEFAULT_HEIGHT);

    crossterm::terminal::enable_raw_mode()?;

    loop {
        if poll(Duration::from_millis(30))? {
            match read()? {
                Event::Resize(w, h) => {
                    width = (w as usize).saturating_sub(4).clamp(40, DEFAULT_WIDTH);
                    height = (h as usize).saturating_sub(4).clamp(10, DEFAULT_HEIGHT);
                    let _ = blaeck.handle_resize(w, h);
                }
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('c')
                            if key
                                .modifiers
                                .contains(crossterm::event::KeyModifiers::CONTROL) =>
                        {
                            break
                        }

                        // Mode toggle
                        KeyCode::Char('m') | KeyCode::Tab => {
                            params.mode = match params.mode {
                                Mode::Plasma => {
                                    params.theme_idx = 7;
                                    Mode::LavaLamp
                                }
                                Mode::LavaLamp => {
                                    params.theme_idx = 0;
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
                        KeyCode::Char(c @ '1'..='8') => {
                            params.apply_preset((c as usize) - ('1' as usize))
                        }

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
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            params.speed = (params.speed + 0.1).min(5.0)
                        }
                        KeyCode::Char('-') | KeyCode::Char('_') => {
                            params.speed = (params.speed - 0.1).max(0.1)
                        }

                        // Zoom (for lava mode)
                        KeyCode::Char('z') | KeyCode::Char('[') => {
                            params.zoom = (params.zoom * 0.85).max(0.1)
                        }
                        KeyCode::Char('Z') | KeyCode::Char(']') => {
                            params.zoom = (params.zoom * 1.15).min(5.0)
                        }

                        _ => {}
                    }
                }
                _ => {}
            }
        }

        let now = start.elapsed().as_secs_f64();
        let raw_dt = now - last_time;
        last_time = now;

        // Update lava lamp physics
        if params.mode == Mode::LavaLamp {
            lava.update(raw_dt, params.speed);
        }

        let time = now * params.speed;
        let display = build_display(width, height, time, &params, &lava);

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
            Mode::Plasma => {
                "m:lava  t/T:theme  p/P:preset  1-8:presets  r:random  +/-:speed  i:info  q:quit"
            }
            Mode::LavaLamp => "m:plasma  t/T:theme  z/Z:zoom  +/-:speed  i:info  q:quit",
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
