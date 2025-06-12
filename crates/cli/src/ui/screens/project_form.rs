use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    models::{FormField, ProjectFormState},
    ui::components::{FormButtons, Button, ButtonAction, TextInput, TagSelector},
};

pub struct ProjectForm {
    form_state: ProjectFormState,
    buttons: FormButtons,
    current_tag_input: String,
}

impl ProjectForm {
    pub fn new() -> Self {
        let buttons = FormButtons::new()
            .add_button(Button::new("Create Project", ButtonAction::Submit))
            .add_button(Button::new("Cancel", ButtonAction::Cancel));

        Self {
            form_state: ProjectFormState::new(),
            buttons,
            current_tag_input: String::new(),
        }
    }

    pub fn with_form_state(mut self, form_state: ProjectFormState) -> Self {
        self.form_state = form_state;
        self
    }

    pub fn get_form_state(&self) -> &ProjectFormState {
        &self.form_state
    }

    pub fn get_form_state_mut(&mut self) -> &mut ProjectFormState {
        &mut self.form_state
    }

    pub fn handle_char_input(&mut self, c: char) {
        match self.form_state.current_field {
            FormField::Name => {
                // Name field: only allow printable characters and spaces, no newlines
                if (!c.is_control() && c.is_ascii_graphic()) || c == ' ' {
                    if self.form_state.name.len() < 100 {
                        self.form_state.name.push(c);
                    }
                }
            }
            FormField::Description => {
                // Description field: allow printable characters, spaces, and newlines
                if (!c.is_control() && c.is_ascii_graphic()) || c == ' ' || c == '\n' {
                    if self.form_state.description.len() < 500 {
                        self.form_state.description.push(c);
                    }
                }
            }
            FormField::Tags => {
                // Tags field: only allow printable characters and spaces, no newlines
                if (!c.is_control() && c.is_ascii_graphic()) || c == ' ' {
                    if self.current_tag_input.len() < 50 {
                        self.current_tag_input.push(c);
                    }
                }
            }
            FormField::Submit => {
                // No character input in submit mode
            }
        }
    }

    pub fn handle_backspace(&mut self) {
        match self.form_state.current_field {
            FormField::Name => {
                self.form_state.name.pop();
            }
            FormField::Description => {
                self.form_state.description.pop();
            }
            FormField::Tags => {
                self.current_tag_input.pop();
            }
            FormField::Submit => {
                // No backspace in submit mode
            }
        }
    }

    pub fn handle_enter(&mut self) {
        match self.form_state.current_field {
            FormField::Description => {
                // In description field, Enter adds a newline
                if self.form_state.description.len() < 500 {
                    self.form_state.description.push('\n');
                }
            }
            FormField::Tags => {
                if !self.current_tag_input.trim().is_empty() {
                    let tag = self.current_tag_input.trim().to_string();
                    if !self.form_state.tags.contains(&tag) {
                        self.form_state.tags.push(tag);
                    }
                    self.current_tag_input.clear();
                }
            }
            _ => {
                // Enter moves to next field for other fields
                self.next_field();
            }
        }
    }

    pub fn next_field(&mut self) {
        self.form_state.current_field = match self.form_state.current_field {
            FormField::Name => FormField::Description,
            FormField::Description => FormField::Tags,
            FormField::Tags => FormField::Submit,
            FormField::Submit => FormField::Name,
        };
    }

    pub fn previous_field(&mut self) {
        self.form_state.current_field = match self.form_state.current_field {
            FormField::Name => FormField::Submit,
            FormField::Description => FormField::Name,
            FormField::Tags => FormField::Description,
            FormField::Submit => FormField::Tags,
        };
    }

    pub fn next_button(&mut self) {
        if self.form_state.current_field == FormField::Submit {
            self.buttons.next_button();
        }
    }

    pub fn previous_button(&mut self) {
        if self.form_state.current_field == FormField::Submit {
            self.buttons.previous_button();
        }
    }

    pub fn get_selected_action(&self) -> Option<ButtonAction> {
        if self.form_state.current_field == FormField::Submit {
            self.buttons.get_focused_action()
        } else {
            None
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.form_state.name.trim().is_empty() {
            return Err("Project name is required".to_string());
        }
        if self.form_state.name.len() > 100 {
            return Err("Project name must be 100 characters or less".to_string());
        }
        if self.form_state.description.len() > 500 {
            return Err("Description must be 500 characters or less".to_string());
        }
        Ok(())
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Length(4),  // Name field
                Constraint::Length(6),  // Description field  
                Constraint::Length(7),  // Tags field
                Constraint::Length(3),  // Buttons
                Constraint::Min(2),     // Validation messages
            ])
            .split(area);

        self.render_title(f, chunks[0]);
        self.render_name_field(f, chunks[1]);
        self.render_description_field(f, chunks[2]);
        self.render_tags_field(f, chunks[3]);
        self.render_buttons(f, chunks[4]);
        self.render_validation(f, chunks[5]);
    }

    fn render_title(&self, f: &mut Frame, area: Rect) {
        let title = Paragraph::new("Create New Project")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::Yellow));

        f.render_widget(title, area);
    }

    fn render_name_field(&self, f: &mut Frame, area: Rect) {
        let text_input = TextInput::new("Project Name")
            .value(self.form_state.name.clone())
            .focused(self.form_state.current_field == FormField::Name)
            .max_length(100)
            .placeholder("Enter project name (required)");

        text_input.render(f, area);
    }

    fn render_description_field(&self, f: &mut Frame, area: Rect) {
        let text_input = TextInput::new("Description")
            .value(self.form_state.description.clone())
            .focused(self.form_state.current_field == FormField::Description)
            .multiline(true)
            .max_length(500)
            .placeholder("Enter project description (optional)");

        text_input.render(f, area);
    }

    fn render_tags_field(&self, f: &mut Frame, area: Rect) {
        let tag_selector = TagSelector::new()
            .tags(self.form_state.tags.clone())
            .input_value(self.current_tag_input.clone())
            .focused(self.form_state.current_field == FormField::Tags);

        tag_selector.render(f, area);
    }

    fn render_buttons(&mut self, f: &mut Frame, area: Rect) {
        if self.form_state.current_field == FormField::Submit {
            self.buttons.render(f, area);
        } else {
            let placeholder = Paragraph::new("Tab/Shift+Tab: Navigate fields • Enter: Next field")
                .block(Block::default().borders(Borders::ALL).title("Navigation"))
                .style(Style::default().fg(Color::Gray));
            f.render_widget(placeholder, area);
        }
    }

    fn render_validation(&self, f: &mut Frame, area: Rect) {
        let message = match self.validate() {
            Ok(_) => "Form is valid. Press Tab to access Submit button.".to_string(),
            Err(err) => format!("Validation Error: {}", err),
        };

        let style = match self.validate() {
            Ok(_) => Style::default().fg(Color::Green),
            Err(_) => Style::default().fg(Color::Red),
        };

        let validation = Paragraph::new(message)
            .block(Block::default().borders(Borders::ALL).title("Status"))
            .style(style);

        f.render_widget(validation, area);
    }
}

impl Default for ProjectForm {
    fn default() -> Self {
        Self::new()
    }
}