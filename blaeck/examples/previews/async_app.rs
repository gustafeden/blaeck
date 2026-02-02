use blaeck::prelude::*;
use std::time::Instant;

/// Events from the background "worker" task
#[derive(Debug)]
pub enum BackgroundEvent {
    /// New data chunk received
    DataChunk(String),
    /// Progress update
    Progress(u32),
    /// Work complete
    Done,
}

/// Application state
pub struct AppState {
    /// Collected text from background task
    pub text: String,
    /// Current progress (0-100)
    pub progress: u32,
    /// Whether background work is running
    pub is_loading: bool,
    /// User input buffer
    pub input: String,
    /// Status message
    pub status: String,
    /// Spinner start time (for animation)
    pub spinner_start: Instant,
    /// Should exit
    pub should_exit: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            progress: 0,
            is_loading: false,
            input: String::new(),
            status: "Type something and press Enter".to_string(),
            spinner_start: Instant::now(),
            should_exit: false,
        }
    }
}

/// Spinner frames for animation
pub const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub fn get_spinner_frame(start: Instant) -> &'static str {
    let elapsed_ms = start.elapsed().as_millis() as usize;
    let frame_idx = (elapsed_ms / 80) % SPINNER_FRAMES.len();
    SPINNER_FRAMES[frame_idx]
}

/// Render the UI based on current state
pub fn render(state: &AppState) -> Element {
    let mut children: Vec<Element> = vec![
        element! {
            Text(content: "Async App Demo", bold: true, color: Color::Cyan)
        },
        element! {
            Text(content: state.status.clone(), dim: true)
        },
        Element::text(""),
    ];

    // Show collected text if any
    if !state.text.is_empty() {
        children.push(element! {
            Text(content: state.text.clone(), color: Color::Green)
        });
        children.push(Element::text(""));
    }

    // Show loading state with spinner
    if state.is_loading {
        let spinner = get_spinner_frame(state.spinner_start);
        let progress_bar = "█".repeat((state.progress / 5) as usize);
        let empty_bar = "░".repeat((20 - state.progress / 5) as usize);

        children.push(element! {
            Box(flex_direction: FlexDirection::Row) {
                Text(content: format!("{} ", spinner), color: Color::Yellow)
                Text(content: format!("Working... {}% ", state.progress))
                Text(content: format!("[{}{}]", progress_bar, empty_bar), dim: true)
            }
        });
    } else {
        // Show input prompt
        children.push(element! {
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "> ", color: Color::Blue)
                Text(content: state.input.clone())
                Text(content: "_", dim: true)
            }
        });
    }

    children.push(Element::text(""));
    children.push(element! {
        Text(content: "Press Ctrl+C to exit", dim: true)
    });

    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        children,
    )
}

/// Handle events from background task
pub fn handle_background_event(state: &mut AppState, event: BackgroundEvent) {
    match event {
        BackgroundEvent::DataChunk(chunk) => {
            state.text.push_str(&chunk);
        }
        BackgroundEvent::Progress(p) => {
            state.progress = p;
        }
        BackgroundEvent::Done => {
            state.is_loading = false;
            state.status = "Done! Type something else".to_string();
        }
    }
}

/// Static preview with default state
pub fn build_ui() -> Element {
    render(&AppState::new())
}
