//! Dashboard - Multi-panel status dashboard
//!
//! Run with: cargo run --example dashboard

use blaeck::prelude::*;
use blaeck::Blaeck;
use std::time::{Duration, Instant};
use std::{io, thread};

fn progress_bar(percent: u32, width: usize) -> String {
    let filled = (width * percent as usize) / 100;
    let empty = width - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

fn status_indicator(healthy: bool) -> (&'static str, Color) {
    if healthy {
        ("●", Color::Green)
    } else {
        ("●", Color::Red)
    }
}

fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;

    // Limit to 30 FPS - smooth animations without excessive CPU usage
    blaeck.set_max_fps(30);

    let start = Instant::now();

    for frame in 0..200 {
        let elapsed = start.elapsed().as_secs_f32();

        // Simulate varying metrics
        let cpu = (50.0 + 30.0 * (elapsed * 0.5).sin()) as u32;
        let memory = (65.0 + 15.0 * (elapsed * 0.3).cos()) as u32;
        let disk = 42;
        let network = (frame * 17) % 100;

        let api_healthy = frame % 50 != 0; // Flicker unhealthy occasionally
        let db_healthy = true;
        let cache_healthy = frame % 70 != 0;

        let requests = frame * 23;
        let errors = frame / 10;
        let latency = 45 + (elapsed * 10.0).sin() as i32;

        let (api_icon, api_color) = status_indicator(api_healthy);
        let (db_icon, db_color) = status_indicator(db_healthy);
        let (cache_icon, cache_color) = status_indicator(cache_healthy);

        let ui = element! {
            Box(flex_direction: FlexDirection::Column, width: 70.0) {
                // Header
                Box(flex_direction: FlexDirection::Row) {
                    Text(content: "System Dashboard", bold: true, color: Color::Cyan)
                    Spacer
                    Text(content: format!("Uptime: {:.0}s", elapsed), dim: true)
                }
                Text(content: "")

                // Top row: Resources + Services side by side
                Box(flex_direction: FlexDirection::Row, gap: 2.0) {
                    // Resources panel
                    Box(border_style: BorderStyle::Round, padding: 1.0, width: 34.0) {
                        Text(content: "Resources", bold: true)
                        Text(content: "")
                        Box(flex_direction: FlexDirection::Row) {
                            Text(content: "CPU    ", dim: true)
                            Text(content: progress_bar(cpu, 15))
                            Text(content: format!(" {:>3}%", cpu), color: if cpu > 80 { Color::Red } else { Color::Green })
                        }
                        Box(flex_direction: FlexDirection::Row) {
                            Text(content: "Memory ", dim: true)
                            Text(content: progress_bar(memory, 15))
                            Text(content: format!(" {:>3}%", memory), color: if memory > 80 { Color::Red } else { Color::Green })
                        }
                        Box(flex_direction: FlexDirection::Row) {
                            Text(content: "Disk   ", dim: true)
                            Text(content: progress_bar(disk, 15))
                            Text(content: format!(" {:>3}%", disk), color: Color::Green)
                        }
                        Box(flex_direction: FlexDirection::Row) {
                            Text(content: "Network", dim: true)
                            Text(content: progress_bar(network, 15))
                            Text(content: format!(" {:>3}%", network), color: if network > 80 { Color::Yellow } else { Color::Green })
                        }
                    }

                    // Services panel
                    Box(border_style: BorderStyle::Round, padding: 1.0, width: 30.0) {
                        Text(content: "Services", bold: true)
                        Text(content: "")
                        Box(flex_direction: FlexDirection::Row) {
                            Text(content: api_icon, color: api_color)
                            Text(content: " API Gateway")
                            Spacer
                            Text(content: if api_healthy { "healthy" } else { "down" },
                                color: api_color, dim: true)
                        }
                        Box(flex_direction: FlexDirection::Row) {
                            Text(content: db_icon, color: db_color)
                            Text(content: " Database")
                            Spacer
                            Text(content: "healthy", color: db_color, dim: true)
                        }
                        Box(flex_direction: FlexDirection::Row) {
                            Text(content: cache_icon, color: cache_color)
                            Text(content: " Cache")
                            Spacer
                            Text(content: if cache_healthy { "healthy" } else { "degraded" },
                                color: cache_color, dim: true)
                        }
                    }
                }

                Text(content: "")

                // Metrics panel
                Box(border_style: BorderStyle::Single, padding: 1.0) {
                    Text(content: "Metrics", bold: true)
                    Text(content: "")
                    Box(flex_direction: FlexDirection::Row) {
                        Box(flex_direction: FlexDirection::Column, width: 20.0) {
                            Text(content: "Requests", dim: true)
                            Text(content: format!("{}", requests), color: Color::Cyan, bold: true)
                        }
                        Box(flex_direction: FlexDirection::Column, width: 20.0) {
                            Text(content: "Errors", dim: true)
                            Text(content: format!("{}", errors), color: if errors > 10 { Color::Red } else { Color::Green }, bold: true)
                        }
                        Box(flex_direction: FlexDirection::Column, width: 20.0) {
                            Text(content: "Latency", dim: true)
                            Text(content: format!("{}ms", latency), color: Color::Yellow, bold: true)
                        }
                    }
                }

                Text(content: "")
                Text(content: "Press Ctrl+C to exit", dim: true, italic: true)
            }
        };

        blaeck.render(ui)?;
        thread::sleep(Duration::from_millis(100));
    }

    blaeck.unmount()?;
    Ok(())
}
