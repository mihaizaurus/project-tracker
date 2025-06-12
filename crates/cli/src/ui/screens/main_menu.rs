use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame,
};

pub struct MainMenu {
    items: Vec<String>,
    state: ListState,
}

impl MainMenu {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        
        Self {
            items: vec![
                "Create New Project".to_string(),
                "Exit".to_string(),
            ],
            state,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn get_selected_action(&self) -> Option<MainMenuAction> {
        match self.state.selected() {
            Some(0) => Some(MainMenuAction::CreateProject),
            Some(1) => Some(MainMenuAction::Exit),
            _ => None,
        }
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7),  // Title
                Constraint::Min(8),     // Menu items
                Constraint::Length(3),  // Instructions
            ])
            .split(area);

        self.render_title(f, chunks[0]);
        self.render_menu_items(f, chunks[1]);
        self.render_instructions(f, chunks[2]);
    }

    fn render_title(&self, f: &mut Frame, area: Rect) {
        let title_lines = vec![
            Line::from(vec![
                Span::styled(
                    "╔═══════════════════════════════════════╗",
                    Style::default().fg(Color::Cyan),
                ),
            ]),
            Line::from(vec![
                Span::styled("║", Style::default().fg(Color::Cyan)),
                Span::styled(
                    "         PROJECT TRACKER CLI          ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("║", Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("║", Style::default().fg(Color::Cyan)),
                Span::styled(
                    "       Business Logic Test Tool       ",
                    Style::default().fg(Color::White),
                ),
                Span::styled("║", Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled(
                    "╚═══════════════════════════════════════╝",
                    Style::default().fg(Color::Cyan),
                ),
            ]),
        ];

        let title = Paragraph::new(title_lines)
            .alignment(Alignment::Center);

        f.render_widget(title, area);
    }

    fn render_menu_items(&mut self, f: &mut Frame, area: Rect) {
        let menu_items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if Some(i) == self.state.selected() {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if Some(i) == self.state.selected() {
                    "► "
                } else {
                    "  "
                };

                ListItem::new(format!("{}{}", prefix, item)).style(style)
            })
            .collect();

        let menu_list = List::new(menu_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Main Menu")
                    .border_style(Style::default().fg(Color::Blue)),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_stateful_widget(menu_list, area, &mut self.state);
    }

    fn render_instructions(&self, f: &mut Frame, area: Rect) {
        let instructions = Paragraph::new("↑/↓: Navigate • Enter: Select • q: Quit")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Controls"),
            )
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);

        f.render_widget(instructions, area);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MainMenuAction {
    CreateProject,
    Exit,
}

impl Default for MainMenu {
    fn default() -> Self {
        Self::new()
    }
}