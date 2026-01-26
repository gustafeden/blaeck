//! Timeline Debug Visualization Demo
//!
//! Demonstrates the TimelineDebugInfo feature for visualizing
//! timeline state during development.
//!
//! Controls:
//! - Space: Pause/resume
//! - r: Restart
//! - Left/Right: Seek backward/forward
//! - q/Esc: Quit

use blaeck::animation::Easing;
use blaeck::prelude::*;
use blaeck::reactive::*;

fn debug_dashboard(cx: Scope) -> Element {
    // Create a timeline with multiple acts
    let timeline_def = Timeline::new()
        .act(
            Act::new("intro")
                .duration(2.0)
                .animate("progress", 0.0f64, 1.0, Easing::EaseOutCubic),
        )
        .act(
            Act::new("main_sequence")
                .duration(4.0)
                .animate("progress", 0.0f64, 1.0, Easing::Linear),
        )
        .act(
            Act::new("transition")
                .duration(1.5)
                .animate("progress", 0.0f64, 1.0, Easing::EaseInOutCubic),
        )
        .act(
            Act::new("finale")
                .duration(2.5)
                .animate("progress", 0.0f64, 1.0, Easing::EaseOutElastic),
        )
        .loop_from("main_sequence");

    let timeline = use_timeline(cx.clone(), timeline_def);

    // Input handling
    let tl = timeline.clone();
    use_input(cx, move |key| {
        use crossterm::event::KeyCode;
        match key.code {
            KeyCode::Char(' ') => tl.toggle_pause(),
            KeyCode::Char('r') => tl.restart(),
            KeyCode::Left => tl.seek((tl.elapsed() - 0.5).max(0.0)),
            KeyCode::Right => tl.seek(tl.elapsed() + 0.5),
            _ => {}
        }
    });

    // Get debug info
    let debug = timeline.debug_info().unwrap_or_else(|| {
        // Fallback for empty timeline
        TimelineDebugInfo {
            duration: 0.0,
            elapsed: 0.0,
            progress: 0.0,
            current_act: String::new(),
            act_index: 0,
            act_count: 0,
            act_progress: 0.0,
            act_duration: 0.0,
            is_paused: false,
            speed: 1.0,
            loop_count: 0,
            loop_behavior: String::new(),
            acts: vec![],
        }
    });

    // Status colors
    let status_text = if debug.is_paused { "PAUSED" } else { "PLAYING" };
    let status_color = if debug.is_paused {
        Color::Yellow
    } else {
        Color::Green
    };

    // Build act list
    let act_elements: Vec<Element> = debug
        .acts
        .iter()
        .enumerate()
        .map(|(i, (name, duration))| {
            let is_current = i == debug.act_index;
            let marker = if is_current { ">>" } else { "  " };
            let color = if is_current {
                Color::Cyan
            } else {
                Color::White
            };

            element! {
                Box(flex_direction: FlexDirection::Row) {
                    Text(content: marker, color: Color::Yellow, bold: is_current)
                    Text(content: format!(" {:2}. ", i + 1), dim: !is_current)
                    Text(content: format!("{:20}", name), color: color, bold: is_current)
                    Text(content: format!(" ({:.1}s)", duration), dim: true)
                }
            }
        })
        .collect();

    // Progress bar width
    let bar_width = 50;

    element! {
        Box(flex_direction: FlexDirection::Column, padding: 1.0, border_style: BorderStyle::Round) {
            // Title
            Text(content: "Timeline Debug Visualization", bold: true, color: Color::Cyan)
            Newline

            // Compact status line
            Box(flex_direction: FlexDirection::Row) {
                Text(content: debug.to_compact_string(), color: Color::White)
            }
            Newline

            // Overall progress bar
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Overall:  ", dim: true)
                Text(content: debug.progress_bar(bar_width), color: Color::Blue)
                Text(content: format!(" {:.1}%", debug.progress * 100.0), dim: true)
            }

            // Act visualization
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Acts:     ", dim: true)
                Text(content: debug.act_visualization(bar_width), color: Color::Magenta)
            }
            Newline

            // Detailed info
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Time: ", dim: true)
                Text(content: format!("{:.2}s / {:.2}s", debug.elapsed, debug.duration), color: Color::White)
            }
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Act: ", dim: true)
                Text(content: format!("{} ", debug.current_act), color: Color::Cyan, bold: true)
                Text(content: format!("({:.1}% of {:.1}s)", debug.act_progress * 100.0, debug.act_duration), dim: true)
            }
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Loop: ", dim: true)
                Text(content: debug.loop_behavior.clone(), color: Color::White)
                Text(content: format!(" (count: {})", debug.loop_count), dim: true)
            }
            Newline

            // Act list
            Text(content: "Acts:", bold: true)
            Box(flex_direction: FlexDirection::Column) {
                #(Element::Fragment(act_elements))
            }
            Newline

            // Status and controls
            Box(flex_direction: FlexDirection::Row) {
                Text(content: status_text, color: status_color, bold: true)
            }
            Text(content: "Space: pause | r: restart | Left/Right: seek | q: quit", dim: true)
        }
    }
}

fn main() -> std::io::Result<()> {
    ReactiveApp::run(debug_dashboard)?;
    Ok(())
}
