//! Sparkline example - Animated mini inline charts
//!
//! Run with: cargo run --example sparkline

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use blaeck::input::poll_key;
use blaeck::prelude::*;
use blaeck::Blaeck;
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    // Animated data - starts with some values
    let mut cpu_data: Vec<f64> = vec![30.0, 35.0, 40.0, 38.0, 42.0, 45.0, 50.0, 48.0, 52.0, 55.0, 53.0, 50.0];
    let mut mem_data: Vec<f64> = vec![45.0, 46.0, 47.0, 48.0, 49.0, 50.0, 51.0, 52.0, 53.0, 54.0, 55.0, 56.0];
    let mut net_data: Vec<f64> = vec![10.0, 15.0, 8.0, 20.0, 12.0, 25.0, 18.0, 30.0, 22.0, 35.0, 28.0, 40.0];
    let mut audio_data: Vec<f64> = vec![20.0; 24]; // Start with baseline

    // Simple pseudo-random using time
    let mut seed: u64 = 12345;
    let random = |seed: &mut u64| -> f64 {
        *seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        ((*seed >> 16) & 0x7FFF) as f64 / 32767.0
    };

    let max_points = 20;

    enable_raw_mode()?;

    loop {
        // Animate: add new random-ish data points
        let cpu_last = *cpu_data.last().unwrap_or(&50.0);
        let cpu_new = (cpu_last + (random(&mut seed) - 0.5) * 20.0).clamp(10.0, 95.0);
        cpu_data.push(cpu_new);
        if cpu_data.len() > max_points { cpu_data.remove(0); }

        let mem_last = *mem_data.last().unwrap_or(&50.0);
        let mem_new = (mem_last + (random(&mut seed) - 0.4) * 5.0).clamp(30.0, 90.0);
        mem_data.push(mem_new);
        if mem_data.len() > max_points { mem_data.remove(0); }

        let net_last = *net_data.last().unwrap_or(&25.0);
        let net_new = (net_last + (random(&mut seed) - 0.5) * 30.0).clamp(0.0, 100.0);
        net_data.push(net_new);
        if net_data.len() > max_points { net_data.remove(0); }

        // Audio viz - each bar jumps independently for that equalizer look
        for i in 0..audio_data.len() {
            let target = random(&mut seed) * 100.0;
            let current = audio_data[i];
            // Quick jump up, slower fall down (like real audio)
            if target > current {
                audio_data[i] = target;
            } else {
                audio_data[i] = (current - 8.0).max(5.0); // Decay
            }
        }

        let ui = Element::node::<Box>(
            BoxProps {
                flex_direction: FlexDirection::Column,
                padding: 1.0,
                border_style: BorderStyle::Round,
                ..Default::default()
            },
            vec![
                // Title
                Element::node::<Text>(
                    TextProps {
                        content: "Live Sparklines".into(),
                        bold: true,
                        color: Some(Color::Cyan),
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::text(""),
                // CPU sparkline
                Element::node::<Box>(
                    BoxProps {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    vec![
                        Element::node::<Text>(
                            TextProps {
                                content: "CPU:  ".into(),
                                color: Some(Color::DarkGray),
                                ..Default::default()
                            },
                            vec![],
                        ),
                        Element::node::<Sparkline>(
                            SparklineProps::new(cpu_data.clone())
                                .range(0.0, 100.0)
                                .color(Color::Green),
                            vec![],
                        ),
                        Element::node::<Text>(
                            TextProps {
                                content: format!(" {:>3.0}%", cpu_new),
                                color: Some(Color::Green),
                                ..Default::default()
                            },
                            vec![],
                        ),
                    ],
                ),
                Element::text(""),
                // Memory sparkline
                Element::node::<Box>(
                    BoxProps {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    vec![
                        Element::node::<Text>(
                            TextProps {
                                content: "MEM:  ".into(),
                                color: Some(Color::DarkGray),
                                ..Default::default()
                            },
                            vec![],
                        ),
                        Element::node::<Sparkline>(
                            SparklineProps::new(mem_data.clone())
                                .range(0.0, 100.0)
                                .color(Color::Yellow),
                            vec![],
                        ),
                        Element::node::<Text>(
                            TextProps {
                                content: format!(" {:>3.0}%", mem_new),
                                color: Some(Color::Yellow),
                                ..Default::default()
                            },
                            vec![],
                        ),
                    ],
                ),
                Element::text(""),
                // Network sparkline
                Element::node::<Box>(
                    BoxProps {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    vec![
                        Element::node::<Text>(
                            TextProps {
                                content: "NET:  ".into(),
                                color: Some(Color::DarkGray),
                                ..Default::default()
                            },
                            vec![],
                        ),
                        Element::node::<Sparkline>(
                            SparklineProps::new(net_data.clone())
                                .range(0.0, 100.0)
                                .color(Color::Cyan),
                            vec![],
                        ),
                        Element::node::<Text>(
                            TextProps {
                                content: format!(" {:>3.0} MB/s", net_new),
                                color: Some(Color::Cyan),
                                ..Default::default()
                            },
                            vec![],
                        ),
                    ],
                ),
                Element::text(""),
                // Audio visualizer
                Element::node::<Box>(
                    BoxProps {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    vec![
                        Element::node::<Text>(
                            TextProps {
                                content: "AUDIO:".into(),
                                color: Some(Color::DarkGray),
                                ..Default::default()
                            },
                            vec![],
                        ),
                        Element::node::<Sparkline>(
                            SparklineProps::new(audio_data.clone())
                                .range(0.0, 100.0)
                                .color(Color::Magenta),
                            vec![],
                        ),
                        Element::node::<Text>(
                            TextProps {
                                content: " ♪♫".into(),
                                color: Some(Color::Magenta),
                                ..Default::default()
                            },
                            vec![],
                        ),
                    ],
                ),
                Element::text(""),
                // Styles demo
                Element::node::<Divider>(
                    DividerProps::new()
                        .width(35)
                        .line_style(DividerStyle::Dashed)
                        .color(Color::DarkGray),
                    vec![],
                ),
                Element::node::<Text>(
                    TextProps {
                        content: "Styles:".into(),
                        dim: true,
                        ..Default::default()
                    },
                    vec![],
                ),
                // Block style
                Element::node::<Box>(
                    BoxProps {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    vec![
                        Element::node::<Text>(
                            TextProps {
                                content: "Block: ".into(),
                                color: Some(Color::DarkGray),
                                ..Default::default()
                            },
                            vec![],
                        ),
                        Element::node::<Sparkline>(
                            SparklineProps::new(vec![1, 2, 3, 5, 8, 5, 3, 2, 1, 2, 3, 5])
                                .style(SparklineStyle::Block)
                                .color(Color::Magenta),
                            vec![],
                        ),
                    ],
                ),
                // Dot style
                Element::node::<Box>(
                    BoxProps {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    vec![
                        Element::node::<Text>(
                            TextProps {
                                content: "Dot:   ".into(),
                                color: Some(Color::DarkGray),
                                ..Default::default()
                            },
                            vec![],
                        ),
                        Element::node::<Sparkline>(
                            SparklineProps::new(vec![1, 2, 3, 5, 8, 5, 3, 2, 1, 2, 3, 5])
                                .style(SparklineStyle::Dot)
                                .color(Color::Blue),
                            vec![],
                        ),
                    ],
                ),
                Element::text(""),
                Element::node::<Text>(
                    TextProps {
                        content: "Press q to quit".into(),
                        dim: true,
                        ..Default::default()
                    },
                    vec![],
                ),
            ],
        );

        blaeck.render(ui)?;

        // Poll with 100ms delay for animation speed
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
