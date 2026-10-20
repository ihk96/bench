use crossterm::event::{Event, KeyCode};
use ratatui::{Frame, layout::Rect};
use tui_input::backend::crossterm::EventHandler;

use crate::{component::input::render_input, screen::{HandledEvent, Screen}, state::AppState};

#[derive(Debug)]
pub struct SettingsScreen {
    focus: usize,
}

impl Default for SettingsScreen {
    fn default() -> Self {
        Self {
            focus: 0,
        }
    }
}

const FIELD_COUNT: usize = 3;

impl Screen for SettingsScreen {
    fn handle_key(&mut self, key: crossterm::event::KeyEvent, state: &mut AppState) -> super::HandledEvent {
        match key.code {
            KeyCode::Esc => {
                return HandledEvent::Event(crate::event::AppEvent::Quit);
            },
            KeyCode::Down => {
                self.focus = (self.focus + 1).min(FIELD_COUNT - 1);
                return HandledEvent::Consumed;
            }
            KeyCode::Up => {
                self.focus = self.focus.saturating_sub(1);
                return HandledEvent::Consumed;
            }
            _ => {
                let input = match self.focus {
                    0 => &mut state.settings.host,
                    1 => &mut state.settings.model_name,
                    _ => &mut state.settings.api_key,
                };
                if let Some(_) = input.handle_event(&Event::Key(key)) {
                    return HandledEvent::Consumed;
                }
                HandledEvent::Ignored
            }
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, state: &AppState) {
        render_input(frame, area, &state.settings.host, "Host", "https://ai.satreci.com:8443", true);
        render_input(frame, area, &state.settings.model_name, "Model Name", "gpt", true);
        render_input(frame, area, &state.settings.api_key, "API Key", "sk-dwewqe#@3wdk", true);
    }
}

