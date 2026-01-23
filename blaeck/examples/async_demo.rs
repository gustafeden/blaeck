//! Async Demo - Using blaeck with async/await
//!
//! This example demonstrates how to integrate async operations
//! (like D-Bus calls, HTTP requests, etc.) with blaeck's UI.
//!
//! Run with: cargo run --example async_demo --features async

use blaeck::prelude::*;
use blaeck::async_runtime::Sender;
use blaeck::Blaeck;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::event::{Event, EventStream};
use futures::StreamExt;
use std::io;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::interval;

/// Application state
struct AppState {
    counter: u32,
    messages: Vec<String>,
    loading: bool,
}

/// Messages from background tasks
#[derive(Clone, Debug)]
enum Msg {
    /// Data loaded from "async" operation
    DataLoaded(String),
}

fn render(state: &AppState) -> Element {
    let loading_text = if state.loading {
        "⏳ Loading..."
    } else {
        "✓ Ready"
    };
    let loading_color = if state.loading { Color::Yellow } else { Color::Green };

    let mut children: Vec<Element> = vec![
        element! { Text(content: "Async Blaeck Demo", bold: true, color: Color::Cyan) },
        Element::text(""),
        Element::node::<Box>(
            BoxProps { flex_direction: FlexDirection::Row, ..Default::default() },
            vec![
                element! { Text(content: "Status: ") },
                element! { Text(content: loading_text, color: loading_color, italic: state.loading) },
            ],
        ),
        Element::text(""),
        Element::node::<Box>(
            BoxProps { flex_direction: FlexDirection::Row, ..Default::default() },
            vec![
                element! { Text(content: "Counter: ") },
                element! { Text(content: format!("{}", state.counter), bold: true, color: Color::Magenta) },
            ],
        ),
        Element::text(""),
        element! { Text(content: "Messages from background tasks:", bold: true) },
    ];

    // Add messages
    if state.messages.is_empty() {
        children.push(element! { Text(content: "  (no messages yet)", dim: true, italic: true) });
    } else {
        for msg in &state.messages {
            children.push(element! { Text(content: format!("  • {}", msg), dim: true) });
        }
    }

    children.extend(vec![
        Element::text(""),
        element! { Text(content: "Controls:", bold: true) },
        element! { Text(content: "  [space] - Trigger async operation", dim: true) },
        element! { Text(content: "  [+]     - Increment counter", dim: true) },
        element! { Text(content: "  [q]     - Quit", dim: true) },
    ]);

    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            padding: 1.0,
            border_style: BorderStyle::Round,
            ..Default::default()
        },
        children,
    )
}

/// Simulates an async operation (like D-Bus call)
async fn simulate_async_work(tx: Sender<Msg>, id: u32) {
    // Simulate network/D-Bus latency
    tokio::time::sleep(Duration::from_millis(500 + (id * 100) as u64)).await;

    // Send result back to UI
    let message = format!("Async task #{} completed!", id);
    let _ = tx.send(Msg::DataLoaded(message)).await;
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut blaeck = Blaeck::new(io::stdout())?;
    let (tx, mut rx) = mpsc::channel::<Msg>(32);

    let mut state = AppState {
        counter: 0,
        messages: Vec::new(),
        loading: false,
    };

    let mut task_id = 0u32;

    enable_raw_mode()?;

    // Initial render
    blaeck.render(render(&state))?;

    // Event stream for keyboard input
    let mut event_stream = EventStream::new();
    let mut tick = interval(Duration::from_millis(100));

    loop {
        tokio::select! {
            // Keyboard events
            maybe_event = event_stream.next() => {
                match maybe_event {
                    Some(Ok(Event::Key(key_event))) => {
                        let key = blaeck::Key::from(key_event);

                        if key.is_ctrl_c() || key.is_char('q') {
                            break;
                        } else if key.is_char(' ') {
                            // Trigger async operation
                            state.loading = true;
                            task_id += 1;
                            let tx_clone = tx.clone();
                            let id = task_id;
                            tokio::spawn(async move {
                                simulate_async_work(tx_clone, id).await;
                            });
                        } else if key.is_char('+') || key.is_char('=') {
                            state.counter += 1;
                        }

                        blaeck.render(render(&state))?;
                    }
                    Some(Ok(_)) => {} // Ignore other events
                    Some(Err(_)) => {}
                    None => break,
                }
            }
            // Messages from background tasks
            Some(msg) = rx.recv() => {
                match msg {
                    Msg::DataLoaded(data) => {
                        state.messages.push(data);
                        state.loading = false;
                        // Keep only last 5 messages
                        if state.messages.len() > 5 {
                            state.messages.remove(0);
                        }
                    }
                }
                blaeck.render(render(&state))?;
            }
            // Periodic tick for animations (if needed)
            _ = tick.tick() => {
                // Could update animations here
            }
        }
    }

    disable_raw_mode()?;
    blaeck.unmount()?;

    println!("Goodbye!");
    Ok(())
}
