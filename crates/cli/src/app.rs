use crossterm::event::KeyEvent;
use project_tracker_core::{
    models::{person::Person, project::Project},
    factories::person_factory::basic_person,
};
use anyhow::Result;

use crate::models::ProjectFormState;
use crate::handlers::project::ProjectHandler;
use crate::ui::{MainMenu, ProjectForm, ProjectPreview, ProjectOutput, MainMenuAction, ButtonAction};

pub type AppResult<T> = Result<T>;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    MainMenu,
    ProjectForm,
    ProjectPreview,
    ProjectOutput(Project),
}

pub struct App {
    current_screen: Screen,
    should_quit: bool,
    default_user: Person,
    project_handler: ProjectHandler,
    created_project: Option<Project>,
    // UI components
    main_menu: MainMenu,
    project_form: ProjectForm,
    project_preview: Option<ProjectPreview>,
    project_output: Option<ProjectOutput>,
}

impl App {
    pub async fn new() -> AppResult<Self> {
        let default_user = basic_person();
        let project_handler = ProjectHandler::new();

        Ok(Self {
            current_screen: Screen::MainMenu,
            should_quit: false,
            default_user,
            project_handler,
            created_project: None,
            main_menu: MainMenu::new(),
            project_form: ProjectForm::new(),
            project_preview: None,
            project_output: None,
        })
    }

    pub fn main_menu(&mut self) -> &mut MainMenu {
        &mut self.main_menu
    }

    pub fn project_form(&mut self) -> &mut ProjectForm {
        &mut self.project_form
    }

    pub fn project_preview(&mut self) -> &mut Option<ProjectPreview> {
        &mut self.project_preview
    }

    pub fn project_output(&mut self) -> &mut Option<ProjectOutput> {
        &mut self.project_output
    }

    pub fn current_screen(&self) -> &Screen {
        &self.current_screen
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub async fn handle_event(&mut self, key: KeyEvent) -> AppResult<()> {
        match &self.current_screen {
            Screen::MainMenu => self.handle_main_menu_event(key).await?,
            Screen::ProjectForm => self.handle_project_form_event(key).await?,
            Screen::ProjectPreview => self.handle_project_preview_event(key).await?,
            Screen::ProjectOutput(_) => self.handle_project_output_event(key).await?,
        }
        Ok(())
    }

    async fn handle_main_menu_event(&mut self, key: KeyEvent) -> AppResult<()> {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Up => {
                self.main_menu.previous();
            }
            KeyCode::Down => {
                self.main_menu.next();
            }
            KeyCode::Enter => {
                if let Some(action) = self.main_menu.get_selected_action() {
                    match action {
                        MainMenuAction::CreateProject => {
                            self.current_screen = Screen::ProjectForm;
                            self.project_form = ProjectForm::new();
                        }
                        MainMenuAction::Exit => {
                            self.should_quit = true;
                        }
                    }
                }
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_project_form_event(&mut self, key: KeyEvent) -> AppResult<()> {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Tab => {
                self.project_form.next_field();
            }
            KeyCode::BackTab => {
                self.project_form.previous_field();
            }
            KeyCode::Enter => {
                // Handle Enter differently based on current field
                if let Some(action) = self.project_form.get_selected_action() {
                    match action {
                        ButtonAction::Submit => {
                            if let Ok(()) = self.project_form.validate() {
                                let form_state = self.project_form.get_form_state();
                                self.project_preview = Some(ProjectPreview::new(form_state.clone()));
                                self.current_screen = Screen::ProjectPreview;
                            }
                        }
                        ButtonAction::Cancel => {
                            self.current_screen = Screen::MainMenu;
                        }
                        _ => {}
                    }
                } else {
                    self.project_form.handle_enter();
                }
            }
            KeyCode::Left => {
                self.project_form.previous_button();
            }
            KeyCode::Right => {
                self.project_form.next_button();
            }
            KeyCode::Esc => {
                self.current_screen = Screen::MainMenu;
            }
            KeyCode::Char(c) => {
                self.project_form.handle_char_input(c);
            }
            KeyCode::Backspace => {
                self.project_form.handle_backspace();
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_project_preview_event(&mut self, key: KeyEvent) -> AppResult<()> {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Left => {
                if let Some(ref mut preview) = self.project_preview {
                    preview.previous_button();
                }
            }
            KeyCode::Right => {
                if let Some(ref mut preview) = self.project_preview {
                    preview.next_button();
                }
            }
            KeyCode::Enter => {
                if let Some(ref preview) = self.project_preview {
                    if let Some(action) = preview.get_selected_action() {
                        match action {
                            ButtonAction::Submit => {
                                let form_state = self.project_form.get_form_state();
                                let project = self.project_handler.create_project(form_state, &self.default_user).await?;
                                self.created_project = Some(project.clone());
                                self.project_output = Some(ProjectOutput::new(project.clone()));
                                self.current_screen = Screen::ProjectOutput(project);
                            }
                            ButtonAction::Back => {
                                self.current_screen = Screen::ProjectForm;
                            }
                            ButtonAction::Cancel => {
                                self.current_screen = Screen::MainMenu;
                            }
                            _ => {}
                        }
                    }
                }
            }
            KeyCode::Esc => {
                self.current_screen = Screen::ProjectForm;
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_project_output_event(&mut self, key: KeyEvent) -> AppResult<()> {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Left => {
                if let Some(ref mut output) = self.project_output {
                    output.previous_button();
                }
            }
            KeyCode::Right => {
                if let Some(ref mut output) = self.project_output {
                    output.next_button();
                }
            }
            KeyCode::Enter => {
                if let Some(ref output) = self.project_output {
                    if let Some(action) = output.get_selected_action() {
                        match action {
                            ButtonAction::Next => {
                                // Create another project
                                self.current_screen = Screen::ProjectForm;
                                self.project_form = ProjectForm::new();
                                self.project_preview = None;
                                self.project_output = None;
                                self.created_project = None;
                            }
                            ButtonAction::Back => {
                                // Back to main menu
                                self.current_screen = Screen::MainMenu;
                                self.project_form = ProjectForm::new();
                                self.project_preview = None;
                                self.project_output = None;
                                self.created_project = None;
                            }
                            ButtonAction::Cancel => {
                                // Exit
                                self.should_quit = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
            KeyCode::Esc => {
                self.current_screen = Screen::MainMenu;
                self.project_form = ProjectForm::new();
                self.project_preview = None;
                self.project_output = None;
                self.created_project = None;
            }
            _ => {}
        }
        Ok(())
    }
}