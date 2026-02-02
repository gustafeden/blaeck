//! Timeline Animation Demo
//!
//! Demonstrates the declarative timeline animation system.
//!
//! Controls:
//! - Space: Pause/resume
//! - r: Restart
//! - q/Esc: Quit

#[path = "previews/mod.rs"]
mod previews;

use blaeck::prelude::*;
use blaeck::animation::Easing;
use crossterm::event::{poll, read, Event, KeyCode};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let timeline = Timeline::new()
        .act(Act::new("fade_in")
            .duration(1.5)
            .animate("opacity", 0.0f64, 1.0, Easing::EaseOutCubic)
            .animate("scale", 0.5f64, 1.0, Easing::EaseOutElastic))
        .act(Act::new("hold")
            .duration(2.0)
            .animate("opacity", 1.0f64, 1.0, Easing::Linear)
            .animate("scale", 1.0f64, 1.0, Easing::Linear))
        .act(Act::new("pulse")
            .duration(1.0)
            .track("opacity", Track::new()
                .keyframe(0.0, 1.0f64, Easing::Linear)
                .keyframe(0.5, 0.6, Easing::EaseInOutCubic)
                .keyframe(1.0, 1.0, Easing::EaseInOutCubic))
            .animate("scale", 1.0f64, 1.1, Easing::EaseInOutCubic))
        .act(Act::new("fade_out")
            .duration(1.5)
            .animate("opacity", 1.0f64, 0.0, Easing::EaseInCubic)
            .animate("scale", 1.0f64, 0.8, Easing::EaseInCubic))
        .loop_from("fade_in");

    let mut playing = timeline.start();
    let mut blaeck = Blaeck::new(std::io::stdout())?;

    crossterm::terminal::enable_raw_mode()?;

    loop {
        if poll(Duration::from_millis(16))? {
            match read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => break,
                    KeyCode::Char(' ') => playing.toggle_pause(),
                    KeyCode::Char('r') => playing.restart(),
                    _ => {}
                },
                _ => {}
            }
        }

        let opacity = playing.get_or("opacity", 1.0f64);
        let scale = playing.get_or("scale", 1.0f64);
        let act_name = playing.current_act();
        let elapsed = playing.elapsed();

        let ui = previews::timeline_demo::render(opacity, scale, &act_name, elapsed, playing.is_paused());
        blaeck.render(ui)?;
    }

    crossterm::terminal::disable_raw_mode()?;
    blaeck.unmount()?;

    Ok(())
}
