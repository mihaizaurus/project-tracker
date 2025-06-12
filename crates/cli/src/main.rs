mod app;
mod ui;
mod models;
mod handlers;

use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use anyhow::Result;

use app::{App, AppResult};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup panic hook to ensure terminal is properly restored
    setup_panic_hook();
    
    // Setup terminal with proper error handling
    let mut terminal = setup_terminal()?;

    // Create app and run it
    let app = App::new().await?;
    let res = run_app(&mut terminal, app).await;

    // Restore terminal
    restore_terminal(&mut terminal)?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn setup_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Attempt to restore terminal on panic
        let _ = disable_raw_mode();
        let _ = execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        original_hook(panic_info);
    }));
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    // Check if we're in a TTY before attempting to set raw mode
    use std::io::IsTerminal;
    if !io::stdout().is_terminal() {
        return Err(anyhow::anyhow!("Not running in a terminal"));
    }
    
    // Enable raw mode first
    enable_raw_mode()?;
    
    let mut stdout = io::stdout();
    
    // Configure terminal for better cross-platform compatibility
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture
    )?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Hide cursor initially
    terminal.hide_cursor()?;
    
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    // Show cursor before restoring
    terminal.show_cursor()?;
    
    // Disable raw mode
    disable_raw_mode()?;
    
    // Restore terminal state
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    
    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> AppResult<()> {
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            // Filter key events to avoid double key presses on Windows
            // Only process KeyEventKind::Press to avoid duplicate events
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc if app.should_quit() => {
                        return Ok(());
                    }
                    _ => {
                        app.handle_event(key).await?;
                    }
                }
            }
        }

        if app.should_quit() {
            return Ok(());
        }
    }
}