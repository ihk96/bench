use ratatui::{Frame, crossterm::event::Event};

use crate::screen::{Screen, settings::SettingsScreen};

pub enum Action {
    None,
    Quit,
    SettingDone,
}

pub struct App {
    screen: Box<dyn Screen>,
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        let state = AppState::default();
        Self {
            screen: Box::new(SettingsScreen::new(&state)),
            state: state,
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        self.screen.render(frame, &self.state);
    }

    /// false를 반환하면 앱 종료
    pub fn handle_event(&mut self, event: &Event) -> bool {
        let action = self.screen.handle_event(event);

        match action {
            Action::None => {}
            Action::Quit => return false,
            Action::SettingDone => {
                // Handle setting done action here if needed
                println!("Settings have been updated.");
            }
        }
        true
    }
}



#[derive(Default)]
pub struct AppState {
    pub settings: Settings,
}

#[derive(Default)]
pub struct Settings {
    pub host: Option<String>,
    pub model_name: Option<String>,
    pub api_key: Option<String>,
}