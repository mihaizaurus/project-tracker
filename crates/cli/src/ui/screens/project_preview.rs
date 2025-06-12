use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{
    models::ProjectFormState,
    ui::components::{FormButtons, Button, ButtonAction},
};

pub struct ProjectPreview {
    form_state: ProjectFormState,
    buttons: FormButtons,
}

impl ProjectPreview {
    pub fn new(form_state: ProjectFormState) -> Self {
        let buttons = FormButtons::new()
            .add_button(Button::new("Confirm", ButtonAction::Submit))
            .add_button(Button::new("Back to Edit", ButtonAction::Back))
            .add_button(Button::new("Cancel", ButtonAction::Cancel));

        Self {
            form_state,
            buttons,
        }
    }

    pub fn next_button(&mut self) {
        self.buttons.next_button();
    }

    pub fn previous_button(&mut self) {
        self.buttons.previous_button();
    }

    pub fn get_selected_action(&self) -> Option<ButtonAction> {
        self.buttons.get_focused_action()
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Min(10),    // Preview content
                Constraint::Length(3),  // Buttons
            ])
            .split(area);

        self.render_title(f, chunks[0]);
        self.render_preview_content(f, chunks[1]);
        self.render_buttons(f, chunks[2]);
    }

    fn render_title(&self, f: &mut Frame, area: Rect) {
        let title = Paragraph::new("Project Preview")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);

        f.render_widget(title, area);
    }

    fn render_preview_content(&self, f: &mut Frame, area: Rect) {
        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4),  // Name
                Constraint::Min(6),     // Description
                Constraint::Length(4),  // Tags
                Constraint::Length(3),  // User assignment
            ])
            .split(area);

        self.render_name_preview(f, content_chunks[0]);
        self.render_description_preview(f, content_chunks[1]);
        self.render_tags_preview(f, content_chunks[2]);
        self.render_user_assignment(f, content_chunks[3]);
    }

    fn render_name_preview(&self, f: &mut Frame, area: Rect) {
        let name_text = if self.form_state.name.is_empty() {
            "[No name provided]".to_string()
        } else {
            self.form_state.name.clone()
        };

        let paragraph = Paragraph::new(name_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Project Name"),
            )
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn render_description_preview(&self, f: &mut Frame, area: Rect) {
        let description_text = if self.form_state.description.trim().is_empty() {
            "[No description provided]".to_string()
        } else {
            self.form_state.description.clone()
        };

        let paragraph = Paragraph::new(description_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Description"),
            )
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn render_tags_preview(&self, f: &mut Frame, area: Rect) {
        let tags_content = if self.form_state.tags.is_empty() {
            Line::from(Span::styled(
                "[No tags added]",
                Style::default().fg(Color::DarkGray),
            ))
        } else {
            let mut spans = vec![];
            for (i, tag) in self.form_state.tags.iter().enumerate() {
                if i > 0 {
                    spans.push(Span::raw(" "));
                }
                spans.push(Span::styled(
                    format!("[{}]", tag),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ));
            }
            Line::from(spans)
        };

        let paragraph = Paragraph::new(vec![tags_content])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Tags ({})", self.form_state.tags.len())),
            )
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn render_user_assignment(&self, f: &mut Frame, area: Rect) {
        let assignment_text = "Will be assigned to: Default Test User";

        let paragraph = Paragraph::new(assignment_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Assignment"),
            )
            .style(Style::default().fg(Color::Green));

        f.render_widget(paragraph, area);
    }

    fn render_buttons(&mut self, f: &mut Frame, area: Rect) {
        self.buttons.render(f, area);
    }
}