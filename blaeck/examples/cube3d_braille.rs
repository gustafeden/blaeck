//! 3D Braille Renderer - High-resolution 3D graphics using braille characters
//!
//! Uses Unicode braille characters (⠀-⣿) as 2x4 pixel grids for 8x higher
//! resolution than standard ASCII. Combined with RGB colors for shading.
//!
//! Techniques:
//! - Braille subpixel rendering (2x4 dots per character)
//! - Flat shading with directional light
//! - Z-buffer for depth sorting
//! - Triangle rasterization

#[path = "previews/mod.rs"]
mod previews;

use blaeck::{AppEvent, AsyncApp, AsyncAppConfig};
use crossterm::event::KeyCode;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::time::Duration;

use previews::cube3d_braille::AppState;

#[tokio::main]
async fn main() -> io::Result<()> {
    let config = AsyncAppConfig {
        tick_interval: Some(Duration::from_millis(33)),
        exit_on_ctrl_c: true,
        ..Default::default()
    };

    let app: AsyncApp<io::Stdout, ()> = AsyncApp::with_config(config)?;
    let state = Rc::new(RefCell::new(AppState::new()));
    let state_render = Rc::clone(&state);
    let state_handle = Rc::clone(&state);

    app.run(
        move |_app| {
            let s = state_render.borrow();
            previews::cube3d_braille::build_ui_with_state(&s)
        },
        move |app, event| match event {
            AppEvent::Key(key) => {
                if key.is_char('q') || key.is_char('Q') {
                    app.exit();
                } else if key.is_char('s') || key.is_char('S') {
                    state_handle.borrow_mut().next_shape();
                } else if key.is_char('c') || key.is_char('C') {
                    state_handle.borrow_mut().next_color();
                } else if key.is_char(' ') {
                    state_handle.borrow_mut().auto_rotate ^= true;
                } else if key.is_char('r') || key.is_char('R') {
                    let mut s = state_handle.borrow_mut();
                    s.angle_x = 0.4;
                    s.angle_y = 0.6;
                    s.angle_z = 0.0;
                } else if key.code == KeyCode::Up {
                    state_handle.borrow_mut().angle_x -= 0.15;
                } else if key.code == KeyCode::Down {
                    state_handle.borrow_mut().angle_x += 0.15;
                } else if key.code == KeyCode::Left {
                    state_handle.borrow_mut().angle_y -= 0.15;
                } else if key.code == KeyCode::Right {
                    state_handle.borrow_mut().angle_y += 0.15;
                }
            }
            AppEvent::Tick => {
                let mut s = state_handle.borrow_mut();
                if s.auto_rotate {
                    s.angle_y += 0.04;
                    s.angle_x += 0.015;
                }
            }
            AppEvent::Exit => app.exit(),
            _ => {}
        },
    )
    .await?;

    println!("Thanks for watching!");
    Ok(())
}
