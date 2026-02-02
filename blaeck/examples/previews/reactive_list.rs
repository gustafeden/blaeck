use blaeck::element;
use blaeck::prelude::*;
use blaeck::reactive::*;
use crossterm::event::KeyCode;

/// The reactive component — used by both the standalone example and the viewer.
pub fn list_app(cx: Scope) -> Element {
    let items = use_state(cx.clone(), || {
        vec![
            "First item".to_string(),
            "Second item".to_string(),
            "Third item".to_string(),
        ]
    });
    let selected = use_state(cx.clone(), || 0usize);
    let item_counter = use_state(cx.clone(), || 4usize);

    let items_handler = items.clone();
    let selected_handler = selected.clone();
    let item_counter_handler = item_counter.clone();

    use_input(cx, move |key| {
        let current_items = items_handler.get();
        let current_selected = selected_handler.get();
        let len = current_items.len();

        match key.code {
            KeyCode::Up => {
                if current_selected > 0 {
                    selected_handler.set(current_selected - 1);
                }
            }
            KeyCode::Down => {
                if len > 0 && current_selected < len - 1 {
                    selected_handler.set(current_selected + 1);
                }
            }
            KeyCode::Char('a') => {
                let mut new_items = current_items;
                let counter = item_counter_handler.get();
                new_items.push(format!("Item #{}", counter));
                items_handler.set(new_items);
                item_counter_handler.set(counter + 1);
            }
            KeyCode::Char('d') => {
                if !current_items.is_empty() {
                    let mut new_items = current_items;
                    new_items.remove(current_selected);
                    items_handler.set(new_items.clone());
                    if current_selected >= new_items.len() && !new_items.is_empty() {
                        selected_handler.set(new_items.len() - 1);
                    }
                }
            }
            _ => {}
        }
    });

    let current_items = items.get();
    let current_selected = selected.get();

    render_list(&current_items, current_selected)
}

/// Render the list UI given items and selection index.
pub fn render_list(items: &[String], selected: usize) -> Element {
    element! {
        Box(flex_direction: FlexDirection::Column, padding: 1.0, border_style: BorderStyle::Round) {
            Text(content: "Reactive List", bold: true, color: Color::Cyan)
            Newline
            #(if items.is_empty() {
                element! {
                    Text(content: "No items. Press 'a' to add one.", dim: true)
                }
            } else {
                Element::column(
                    items.iter().enumerate().map(|(i, item)| {
                        let is_selected = i == selected;
                        let prefix = if is_selected { "> " } else { "  " };
                        let color = if is_selected { Color::Green } else { Color::White };
                        element! {
                            Text(
                                content: format!("{}{}", prefix, item),
                                color: color,
                                bold: is_selected
                            )
                        }
                    }).collect()
                )
            })
            Newline
            Text(content: format!("Items: {} | Selected: {}", items.len(), selected + 1), dim: true)
            Newline
            Text(content: "Up/Down: Navigate | a: Add | d: Delete | q: Quit", dim: true)
        }
    }
}

/// Static snapshot for the example viewer preview panel.
pub fn build_ui() -> Element {
    let items = vec![
        "First item".to_string(),
        "Second item".to_string(),
        "Third item".to_string(),
    ];
    render_list(&items, 0)
}
