use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use project_tracker_core::models::project::Project;

use crate::ui::components::{FormButtons, Button, ButtonAction};

pub struct ProjectOutput {
    project: Project,
    buttons: FormButtons,
}

impl ProjectOutput {
    pub fn new(project: Project) -> Self {
        let buttons = FormButtons::new()
            .add_button(Button::new("Create Another", ButtonAction::Next))
            .add_button(Button::new("Main Menu", ButtonAction::Back))
            .add_button(Button::new("Exit", ButtonAction::Cancel));

        Self {
            project,
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
                Constraint::Min(8),     // Project output
                Constraint::Length(3),  // Buttons
            ])
            .split(area);

        self.render_title(f, chunks[0]);
        self.render_project_output(f, chunks[1]);
        self.render_buttons(f, chunks[2]);
    }

    fn render_title(&self, f: &mut Frame, area: Rect) {
        let title = Paragraph::new("Project Created Successfully!")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green)),
            )
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Center);

        f.render_widget(title, area);
    }

    fn render_project_output(&self, f: &mut Frame, area: Rect) {
        let output_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),     // Display implementation
                Constraint::Percentage(50),     // Debug implementation
            ])
            .split(area);

        self.render_display_output(f, output_chunks[0]);
        self.render_debug_output(f, output_chunks[1]);
    }

    fn render_display_output(&self, f: &mut Frame, area: Rect) {
        let display_text = format!("{}", self.project);

        let paragraph = Paragraph::new(display_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Project Display Output"),
            )
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn render_debug_output(&self, f: &mut Frame, area: Rect) {
        let debug_text = format!("{:#?}", self.project);

        let paragraph = Paragraph::new(debug_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Project Debug Output"),
            )
            .style(Style::default().fg(Color::Yellow))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn render_buttons(&mut self, f: &mut Frame, area: Rect) {
        self.buttons.render(f, area);
    }
}