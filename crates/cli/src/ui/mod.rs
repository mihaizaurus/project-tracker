pub mod components;
pub mod screens;

pub use components::*;
pub use screens::*;

use ratatui::Frame;
use crate::app::{App, Screen};

pub fn draw(f: &mut Frame, app: &mut App) {
    match app.current_screen() {
        Screen::MainMenu => {
            app.main_menu().render(f, f.area());
        }
        Screen::ProjectForm => {
            app.project_form().render(f, f.area());
        }
        Screen::ProjectPreview => {
            if let Some(preview) = app.project_preview() {
                preview.render(f, f.area());
            }
        }
        Screen::ProjectOutput(_) => {
            if let Some(output) = app.project_output() {
                output.render(f, f.area());
            }
        }
    }
}