use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub struct TagSelector {
    tags: Vec<String>,
    input_value: String,
    is_focused: bool,
}

impl TagSelector {
    pub fn new() -> Self {
        Self {
            tags: Vec::new(),
            input_value: String::new(),
            is_focused: false,
        }
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn input_value(mut self, value: String) -> Self {
        self.input_value = value;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.is_focused = focused;
        self
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // Input area
                Constraint::Min(3),    // Tags display area
            ])
            .split(area);

        self.render_input(f, chunks[0]);
        self.render_tags(f, chunks[1]);
    }

    fn render_input(&self, f: &mut Frame, area: Rect) {
        let border_style = if self.is_focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Gray)
        };

        let text_style = if self.is_focused {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let mut display_text = if self.input_value.is_empty() {
            "Type tag name and press Enter...".to_string()
        } else {
            self.input_value.clone()
        };

        let content_style = if self.input_value.is_empty() {
            Style::default().fg(Color::DarkGray)
        } else {
            text_style
        };

        if self.is_focused && !self.input_value.is_empty() {
            display_text.push('|'); // Cursor
        }

        let paragraph = Paragraph::new(display_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Add Tag")
                    .border_style(border_style),
            )
            .style(content_style);

        f.render_widget(paragraph, area);
    }

    fn render_tags(&self, f: &mut Frame, area: Rect) {
        if self.tags.is_empty() {
            let paragraph = Paragraph::new("No tags added yet")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Current Tags"),
                )
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);
            f.render_widget(paragraph, area);
            return;
        }

        // Create a line with all tags
        let mut spans = vec![];
        for (i, tag) in self.tags.iter().enumerate() {
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

        let line = Line::from(spans);

        let paragraph = Paragraph::new(vec![line])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Current Tags ({})", self.tags.len())),
            )
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }
}