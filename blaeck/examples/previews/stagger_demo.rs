use blaeck::animation::Easing;
use blaeck::prelude::*;
use blaeck::reactive::*;

const PANEL_COUNT: usize = 7;

/// The reactive component — used by both the standalone example and the viewer.
pub fn component(cx: Scope) -> Element {
    let order_idx = use_state(cx.clone(), || 0usize);

    let order = match order_idx.get() {
        0 => StaggerOrder::Forward,
        1 => StaggerOrder::Reverse,
        2 => StaggerOrder::CenterOut,
        3 => StaggerOrder::EdgesIn,
        _ => StaggerOrder::Random,
    };

    let timeline_def = Timeline::new()
        .act(
            Act::new("fade_in")
                .duration(2.0)
                .stagger_config(
                    "opacity",
                    StaggerConfig::new(PANEL_COUNT, 0.0f64, 1.0)
                        .delay(0.12)
                        .order(order)
                        .easing(Easing::EaseOutCubic),
                )
                .stagger_config(
                    "scale",
                    StaggerConfig::new(PANEL_COUNT, 0.5f64, 1.0)
                        .delay(0.12)
                        .order(order)
                        .easing(Easing::EaseOutElastic),
                ),
        )
        .act(
            Act::new("hold")
                .duration(1.5)
                .stagger("opacity", PANEL_COUNT, 1.0f64, 1.0, Easing::Linear)
                .stagger("scale", PANEL_COUNT, 1.0f64, 1.0, Easing::Linear),
        )
        .act(
            Act::new("fade_out")
                .duration(1.5)
                .stagger_config(
                    "opacity",
                    StaggerConfig::new(PANEL_COUNT, 1.0f64, 0.0)
                        .delay(0.1)
                        .order(order)
                        .easing(Easing::EaseInCubic),
                )
                .stagger_config(
                    "scale",
                    StaggerConfig::new(PANEL_COUNT, 1.0f64, 0.3)
                        .delay(0.1)
                        .order(order)
                        .easing(Easing::EaseInCubic),
                ),
        )
        .loop_from("fade_in");

    let timeline = use_timeline(cx.clone(), timeline_def);

    let tl = timeline.clone();
    let order_handler = order_idx.clone();
    use_input(cx, move |key| {
        use crossterm::event::KeyCode;
        match key.code {
            KeyCode::Char(' ') => tl.toggle_pause(),
            KeyCode::Char('r') => tl.restart(),
            KeyCode::Char('1') => order_handler.set(0),
            KeyCode::Char('2') => order_handler.set(1),
            KeyCode::Char('3') => order_handler.set(2),
            KeyCode::Char('4') => order_handler.set(3),
            KeyCode::Char('5') => order_handler.set(4),
            _ => {}
        }
    });

    let opacities: Vec<f64> = timeline.get_stagger_all("opacity", 0.0);
    let scales: Vec<f64> = timeline.get_stagger_all("scale", 1.0);
    let act_name = timeline.current_act();
    let elapsed = timeline.elapsed();
    let paused = timeline.is_paused();

    let order_name = match order_idx.get() {
        0 => "Forward",
        1 => "Reverse",
        2 => "CenterOut",
        3 => "EdgesIn",
        _ => "Random",
    };

    render_stagger(&opacities, &scales, &act_name, elapsed, paused, order_name)
}

/// Render the stagger UI given animated values.
pub fn render_stagger(
    opacities: &[f64],
    scales: &[f64],
    act_name: &str,
    elapsed: f64,
    paused: bool,
    order_name: &str,
) -> Element {
    let status = if paused { "PAUSED" } else { "PLAYING" };
    let status_color = if paused { Color::Yellow } else { Color::Green };

    let panels: Vec<Element> = (0..PANEL_COUNT)
        .map(|i| {
            let opacity = opacities.get(i).copied().unwrap_or(0.0);
            let scale = scales.get(i).copied().unwrap_or(1.0);

            let brightness = (opacity * 255.0) as u8;
            let color = Color::Rgb(brightness, brightness, brightness);

            let bar_height = (scale * 5.0) as usize;
            let bar = (0..bar_height).map(|_| "█").collect::<String>();
            let padding = " ".repeat(5 - bar_height);

            element! {
                Box(flex_direction: FlexDirection::Column, align_items: AlignItems::Center, width: 8.0) {
                    Text(content: format!("{}", padding), color: color)
                    Text(content: format!("{}", bar), color: color)
                    Text(content: format!("P{}", i + 1), dim: opacity < 0.5)
                }
            }
        })
        .collect();

    element! {
        Box(flex_direction: FlexDirection::Column, padding: 1.0, border_style: BorderStyle::Round) {
            Text(content: "Stagger Animation Demo", bold: true, color: Color::Cyan)
            Newline

            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Act: ", dim: true)
                Text(content: act_name.to_string(), color: Color::Magenta, bold: true)
                Text(content: format!("  Time: {:.2}s", elapsed), dim: true)
            }
            Box(flex_direction: FlexDirection::Row) {
                Text(content: "Order: ", dim: true)
                Text(content: order_name.to_string(), color: Color::Blue, bold: true)
            }
            Newline

            Box(flex_direction: FlexDirection::Row, gap: 1.0) {
                #(Element::Fragment(panels))
            }
            Newline

            Text(content: status, color: status_color, bold: true)
            Newline

            Text(content: "1-5: Change order | Space: pause | r: restart | q: quit", dim: true)
        }
    }
}

/// Static snapshot for the example viewer preview panel.
pub fn build_ui() -> Element {
    // Show panels at varying opacities as if mid-stagger
    let opacities = vec![1.0, 0.9, 0.7, 0.5, 0.3, 0.1, 0.0];
    let scales = vec![1.0, 1.0, 0.9, 0.8, 0.6, 0.5, 0.5];
    render_stagger(&opacities, &scales, "fade_in", 1.2, false, "Forward")
}
