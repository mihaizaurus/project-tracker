use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonAction {
    Submit,
    Cancel,
    Back,
    Next,
}

pub struct Button {
    pub label: String,
    pub action: ButtonAction,
    pub is_focused: bool,
}

impl Button {
    pub fn new(label: &str, action: ButtonAction) -> Self {
        Self {
            label: label.to_string(),
            action,
            is_focused: false,
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.is_focused = focused;
        self
    }
}

pub struct FormButtons {
    buttons: Vec<Button>,
    focused_index: usize,
}

impl FormButtons {
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
            focused_index: 0,
        }
    }

    pub fn with_buttons(buttons: Vec<Button>) -> Self {
        Self {
            buttons,
            focused_index: 0,
        }
    }

    pub fn add_button(mut self, button: Button) -> Self {
        self.buttons.push(button);
        self
    }

    pub fn focused_index(mut self, index: usize) -> Self {
        if index < self.buttons.len() {
            self.focused_index = index;
        }
        self
    }

    pub fn get_focused_action(&self) -> Option<ButtonAction> {
        self.buttons.get(self.focused_index).map(|b| b.action.clone())
    }

    pub fn next_button(&mut self) {
        if !self.buttons.is_empty() {
            self.focused_index = (self.focused_index + 1) % self.buttons.len();
        }
    }

    pub fn previous_button(&mut self) {
        if !self.buttons.is_empty() {
            self.focused_index = if self.focused_index == 0 {
                self.buttons.len() - 1
            } else {
                self.focused_index - 1
            };
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        if self.buttons.is_empty() {
            return;
        }

        let button_count = self.buttons.len();
        let constraints: Vec<Constraint> = (0..button_count)
            .map(|_| Constraint::Percentage(100 / button_count as u16))
            .collect();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(area);

        for (i, button) in self.buttons.iter().enumerate() {
            let is_focused = i == self.focused_index;
            self.render_button(f, chunks[i], button, is_focused);
        }
    }

    fn render_button(&self, f: &mut Frame, area: Rect, button: &Button, is_focused: bool) {
        let style = if is_focused {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray)
        };

        let border_style = if is_focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Gray)
        };

        let label_with_brackets = format!("[ {} ]", button.label);
        
        let paragraph = Paragraph::new(label_with_brackets)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style),
            )
            .style(style)
            .alignment(Alignment::Center);

        f.render_widget(paragraph, area);
    }
}

impl Default for FormButtons {
    fn default() -> Self {
        Self::new()
    }
}