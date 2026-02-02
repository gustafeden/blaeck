use blaeck::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Field {
    Username,
    Password,
    RememberMe,
    Newsletter,
    Submit,
}

impl Field {
    pub fn next(&self) -> Field {
        match self {
            Field::Username => Field::Password,
            Field::Password => Field::RememberMe,
            Field::RememberMe => Field::Newsletter,
            Field::Newsletter => Field::Submit,
            Field::Submit => Field::Username,
        }
    }

    pub fn prev(&self) -> Field {
        match self {
            Field::Username => Field::Submit,
            Field::Password => Field::Username,
            Field::RememberMe => Field::Password,
            Field::Newsletter => Field::RememberMe,
            Field::Submit => Field::Newsletter,
        }
    }
}

pub struct FormState {
    pub focused: Field,
    pub username: TextInputState,
    pub password: TextInputState,
    pub remember_me: bool,
    pub newsletter: bool,
    pub submitted: bool,
}

impl FormState {
    pub fn new() -> Self {
        Self {
            focused: Field::Username,
            username: TextInputState::new(),
            password: TextInputState::new(),
            remember_me: false,
            newsletter: true,
            submitted: false,
        }
    }

    pub fn handle_key(&mut self, key: &blaeck::input::Key) -> bool {
        // Global keys
        match key.code {
            crossterm::event::KeyCode::Esc => return true,
            crossterm::event::KeyCode::Tab => {
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::SHIFT)
                {
                    self.focused = self.focused.prev();
                } else {
                    self.focused = self.focused.next();
                }
                return false;
            }
            crossterm::event::KeyCode::Up => {
                self.focused = self.focused.prev();
                return false;
            }
            crossterm::event::KeyCode::Down => {
                self.focused = self.focused.next();
                return false;
            }
            _ => {}
        }

        // Field-specific handling
        match self.focused {
            Field::Username => self.handle_text_input(&mut self.username.clone(), key),
            Field::Password => self.handle_text_input(&mut self.password.clone(), key),
            Field::RememberMe => {
                if matches!(
                    key.code,
                    crossterm::event::KeyCode::Char(' ') | crossterm::event::KeyCode::Enter
                ) {
                    self.remember_me = !self.remember_me;
                }
            }
            Field::Newsletter => {
                if matches!(
                    key.code,
                    crossterm::event::KeyCode::Char(' ') | crossterm::event::KeyCode::Enter
                ) {
                    self.newsletter = !self.newsletter;
                }
            }
            Field::Submit => {
                if key.code == crossterm::event::KeyCode::Enter {
                    self.submitted = true;
                }
            }
        }

        // Update text input states (workaround for borrow checker)
        match self.focused {
            Field::Username => {
                let mut state = self.username.clone();
                self.handle_text_input(&mut state, key);
                self.username = state;
            }
            Field::Password => {
                let mut state = self.password.clone();
                self.handle_text_input(&mut state, key);
                self.password = state;
            }
            _ => {}
        }

        false
    }

    fn handle_text_input(&self, state: &mut TextInputState, key: &blaeck::input::Key) {
        let shift = key
            .modifiers
            .contains(crossterm::event::KeyModifiers::SHIFT);
        let ctrl = key
            .modifiers
            .contains(crossterm::event::KeyModifiers::CONTROL);

        match key.code {
            crossterm::event::KeyCode::Char('a') if ctrl => state.select_all(),
            crossterm::event::KeyCode::Char(c) => state.insert(c),
            crossterm::event::KeyCode::Backspace => {
                state.backspace();
            }
            crossterm::event::KeyCode::Delete => {
                state.delete();
            }
            crossterm::event::KeyCode::Left if shift => {
                state.select_left();
            }
            crossterm::event::KeyCode::Left => {
                state.move_left();
            }
            crossterm::event::KeyCode::Right if shift => {
                state.select_right();
            }
            crossterm::event::KeyCode::Right => {
                state.move_right();
            }
            crossterm::event::KeyCode::Home if shift => state.select_to_home(),
            crossterm::event::KeyCode::Home => state.move_home(),
            crossterm::event::KeyCode::End if shift => state.select_to_end(),
            crossterm::event::KeyCode::End => state.move_end(),
            _ => {}
        }
    }
}

pub fn render(state: &FormState) -> Element {
    if state.submitted {
        return Element::node::<Box>(
            BoxProps {
                flex_direction: FlexDirection::Column,
                border_style: BorderStyle::Round,
                padding: 2.0,
                border_color: Some(Color::Green),
                ..Default::default()
            },
            vec![
                Element::node::<Text>(
                    TextProps {
                        content: "Form Submitted!".into(),
                        bold: true,
                        color: Some(Color::Green),
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::text(""),
                Element::node::<Text>(
                    TextProps {
                        content: format!("Username: {}", state.username.value()),
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::node::<Text>(
                    TextProps {
                        content: format!("Password: {}", "*".repeat(state.password.value().len())),
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::node::<Text>(
                    TextProps {
                        content: format!(
                            "Remember me: {}",
                            if state.remember_me { "Yes" } else { "No" }
                        ),
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::node::<Text>(
                    TextProps {
                        content: format!(
                            "Newsletter: {}",
                            if state.newsletter { "Yes" } else { "No" }
                        ),
                        ..Default::default()
                    },
                    vec![],
                ),
                Element::text(""),
                Element::node::<Text>(
                    TextProps {
                        content: "Press Esc to exit".into(),
                        dim: true,
                        ..Default::default()
                    },
                    vec![],
                ),
            ],
        );
    }

    Element::node::<Box>(
        BoxProps {
            flex_direction: FlexDirection::Column,
            border_style: BorderStyle::Round,
            padding: 1.0,
            ..Default::default()
        },
        vec![
            // Title
            Element::node::<Text>(
                TextProps {
                    content: "Login Form".into(),
                    bold: true,
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                vec![],
            ),
            Element::node::<Text>(
                TextProps {
                    content: "Tab/Arrows to navigate, Space/Enter to toggle".into(),
                    dim: true,
                    ..Default::default()
                },
                vec![],
            ),
            Element::text(""),
            // Username field
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: if state.focused == Field::Username {
                                "> "
                            } else {
                                "  "
                            }
                            .into(),
                            color: Some(Color::Cyan),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: "Username: ".into(),
                            bold: state.focused == Field::Username,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<TextInput>(
                        TextInputProps::new(state.username.value())
                            .cursor(state.username.cursor)
                            .selection(state.username.selection_anchor)
                            .placeholder("Enter username")
                            .placeholder_color(Color::DarkGray)
                            .focused(state.focused == Field::Username)
                            .color(Color::White),
                        vec![],
                    ),
                ],
            ),
            // Password field
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: if state.focused == Field::Password {
                                "> "
                            } else {
                                "  "
                            }
                            .into(),
                            color: Some(Color::Cyan),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Text>(
                        TextProps {
                            content: "Password: ".into(),
                            bold: state.focused == Field::Password,
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<TextInput>(
                        TextInputProps::new(state.password.value())
                            .cursor(state.password.cursor)
                            .selection(state.password.selection_anchor)
                            .placeholder("Enter password")
                            .placeholder_color(Color::DarkGray)
                            .focused(state.focused == Field::Password)
                            .mask()
                            .color(Color::White),
                        vec![],
                    ),
                ],
            ),
            Element::text(""),
            // Checkboxes
            Element::node::<Checkbox>(
                CheckboxProps::with_label("Remember me")
                    .checked(state.remember_me)
                    .focused(state.focused == Field::RememberMe)
                    .checked_color(Color::Green),
                vec![],
            ),
            Element::node::<Checkbox>(
                CheckboxProps::with_label("Subscribe to newsletter")
                    .checked(state.newsletter)
                    .focused(state.focused == Field::Newsletter)
                    .checked_color(Color::Green),
                vec![],
            ),
            Element::text(""),
            // Submit button
            Element::node::<Box>(
                BoxProps {
                    flex_direction: FlexDirection::Row,
                    align_items: Some(AlignItems::Center),
                    ..Default::default()
                },
                vec![
                    Element::node::<Text>(
                        TextProps {
                            content: if state.focused == Field::Submit {
                                "> "
                            } else {
                                "  "
                            }
                            .into(),
                            color: Some(Color::Cyan),
                            ..Default::default()
                        },
                        vec![],
                    ),
                    Element::node::<Box>(
                        BoxProps {
                            border_style: if state.focused == Field::Submit {
                                BorderStyle::Double
                            } else {
                                BorderStyle::Single
                            },
                            padding_left: Some(2.0),
                            padding_right: Some(2.0),
                            border_color: if state.focused == Field::Submit {
                                Some(Color::Green)
                            } else {
                                None
                            },
                            ..Default::default()
                        },
                        vec![Element::node::<Text>(
                            TextProps {
                                content: "Submit".into(),
                                bold: state.focused == Field::Submit,
                                color: if state.focused == Field::Submit {
                                    Some(Color::Green)
                                } else {
                                    None
                                },
                                ..Default::default()
                            },
                            vec![],
                        )],
                    ),
                ],
            ),
            Element::text(""),
            Element::node::<Text>(
                TextProps {
                    content: "Press Esc to cancel".into(),
                    dim: true,
                    italic: true,
                    ..Default::default()
                },
                vec![],
            ),
        ],
    )
}

pub fn build_ui() -> Element {
    let state = FormState::new();
    render(&state)
}
