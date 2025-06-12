use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct TextInput {
    title: String,
    value: String,
    is_focused: bool,
    is_multiline: bool,
    max_length: usize,
    placeholder: String,
}

impl TextInput {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            value: String::new(),
            is_focused: false,
            is_multiline: false,
            max_length: 100,
            placeholder: String::new(),
        }
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = value;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.is_focused = focused;
        self
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.is_multiline = multiline;
        self
    }

    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = max_length;
        self
    }

    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let style = if self.is_focused {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let border_style = if self.is_focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Gray)
        };

        let display_value = if self.value.is_empty() && !self.placeholder.is_empty() {
            self.placeholder.clone()
        } else {
            self.value.clone()
        };

        let text_style = if self.value.is_empty() && !self.placeholder.is_empty() {
            Style::default().fg(Color::DarkGray)
        } else {
            style
        };

        let height = if self.is_multiline { 4 } else { 3 };
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(height),
            ])
            .split(area);

        let mut lines = vec![];
        if self.is_multiline {
            // Split the text into lines for multiline display
            let wrapped_lines: Vec<&str> = display_value.lines().collect();
            
            // Handle empty multiline input with cursor
            if wrapped_lines.is_empty() && self.is_focused {
                lines.push(Line::from(Span::styled("█", text_style)));
            } else {
                for (i, line) in wrapped_lines.iter().enumerate() {
                    let mut line_text = line.to_string();
                    // Add cursor to the last line if focused
                    if self.is_focused && i == wrapped_lines.len() - 1 {
                        line_text.push('█');
                    }
                    lines.push(Line::from(Span::styled(line_text, text_style)));
                }
            }
        } else {
            // Single line with cursor indicator
            let mut display_text = display_value;
            if self.is_focused && !self.is_multiline {
                // Use a more visible cursor character that works well across platforms
                display_text.push('█'); // Block cursor for better visibility
            }
            lines.push(Line::from(Span::styled(display_text, text_style)));
        }

        // Add character count for inputs with max length
        let char_count = format!(" ({}/{}) ", self.value.len(), self.max_length);
        let title_with_count = format!("{}{}", self.title, char_count);

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title_with_count)
                    .border_style(border_style),
            )
            .style(style);

        f.render_widget(paragraph, chunks[0]);
    }
}