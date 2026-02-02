//! Sparkline example - Animated mini inline charts
//!
//! Run with: cargo run --example sparkline

#[path = "previews/mod.rs"]
mod previews;

use blaeck::input::poll_key;
use blaeck::Blaeck;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    let mut cpu_data: Vec<f64> = vec![
        30.0, 35.0, 40.0, 38.0, 42.0, 45.0, 50.0, 48.0, 52.0, 55.0, 53.0, 50.0,
    ];
    let mut mem_data: Vec<f64> = vec![
        45.0, 46.0, 47.0, 48.0, 49.0, 50.0, 51.0, 52.0, 53.0, 54.0, 55.0, 56.0,
    ];
    let mut net_data: Vec<f64> = vec![
        10.0, 15.0, 8.0, 20.0, 12.0, 25.0, 18.0, 30.0, 22.0, 35.0, 28.0, 40.0,
    ];
    let mut audio_data: Vec<f64> = vec![20.0; 24];

    let mut seed: u64 = 12345;
    let random = |seed: &mut u64| -> f64 {
        *seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        ((*seed >> 16) & 0x7FFF) as f64 / 32767.0
    };

    let max_points = 20;

    enable_raw_mode()?;

    loop {
        let cpu_last = *cpu_data.last().unwrap_or(&50.0);
        let cpu_new = (cpu_last + (random(&mut seed) - 0.5) * 20.0).clamp(10.0, 95.0);
        cpu_data.push(cpu_new);
        if cpu_data.len() > max_points {
            cpu_data.remove(0);
        }

        let mem_last = *mem_data.last().unwrap_or(&50.0);
        let mem_new = (mem_last + (random(&mut seed) - 0.4) * 5.0).clamp(30.0, 90.0);
        mem_data.push(mem_new);
        if mem_data.len() > max_points {
            mem_data.remove(0);
        }

        let net_last = *net_data.last().unwrap_or(&25.0);
        let net_new = (net_last + (random(&mut seed) - 0.5) * 30.0).clamp(0.0, 100.0);
        net_data.push(net_new);
        if net_data.len() > max_points {
            net_data.remove(0);
        }

        for value in &mut audio_data {
            let target = random(&mut seed) * 100.0;
            let current = *value;
            if target > current {
                *value = target;
            } else {
                *value = (current - 8.0).max(5.0);
            }
        }

        let ui = previews::sparkline::build_ui_with_data(
            &cpu_data, cpu_new, &mem_data, mem_new, &net_data, net_new, &audio_data,
        );
        blaeck.render(ui)?;

        if let Some(key) = poll_key(Duration::from_millis(100))? {
            if key.is_ctrl_c() {
                break;
            }
            match key.code {
                crossterm::event::KeyCode::Char('q') | crossterm::event::KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    blaeck.unmount()?;
    println!("Goodbye!");

    Ok(())
}
