use crossterm::event::{Event, KeyCode};
use ratatui::{Frame, layout::{Constraint, Layout, Rect}, style::{Color, Stylize}, widgets::{Block, Padding, Paragraph}};
use tui_input::backend::crossterm::EventHandler;

use crate::{component::{input::render_input, spinner}, event::AppEvent, screen::{HandledEvent, Screen}, state::{AppState, settings::SettingStatus}};

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

const FIELD_COUNT: usize = 4;

impl Screen for SettingsScreen {
    fn handle_key(&mut self, key: crossterm::event::KeyEvent, state: &mut AppState) -> super::HandledEvent {
        match key.code {
            KeyCode::Esc => {
                return HandledEvent::Event(AppEvent::PopScreen);
            },
            KeyCode::Down => {
                if !matches!(state.settings.status, SettingStatus::Checking(_, _)) {
                    self.focus = (self.focus + 1).min(FIELD_COUNT - 1);
                }
                return HandledEvent::Consumed;
            }
            KeyCode::Up => {
                if !matches!(state.settings.status, SettingStatus::Checking(_, _)) {
                    self.focus = self.focus.saturating_sub(1);
                }
                return HandledEvent::Consumed;
            }
            KeyCode::Enter => {
                if self.focus == 3 {
                    if !matches!(state.settings.status, SettingStatus::Checking(_, _)) {
                        state.settings.health_check();
                    }
                }
                return HandledEvent::Consumed;
            }
            _ => {
                let input = match self.focus {
                    0 => &mut state.settings.host,
                    1 => &mut state.settings.model_name,
                    2 => &mut state.settings.api_key,
                    _ => {return HandledEvent::Consumed}
                };
                if let Some(_) = input.handle_event(&Event::Key(key)) {
                    return HandledEvent::Consumed;
                }
                HandledEvent::Ignored
            }
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, state: &mut AppState) {

        let [host_area, model_name_area, api_key_area, save_area, _, _rest] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ]).areas(area);

        render_input(frame, host_area, &state.settings.host, "Host", "https://ai.satreci.com:8443", self.focus == 0);
        render_input(frame, model_name_area, &state.settings.model_name, "Model Name", "gpt", self.focus == 1);
        render_input(frame, api_key_area, &state.settings.api_key, "API Key", "sk-dwewqe#@3wdk", self.focus == 2);

        let mut save_button = Paragraph::new("Check & Save");
        if self.focus == 3 {
            save_button = save_button.bg(Color::White)
                .fg(Color::Black);
        }
        
        frame.render_widget(save_button, save_area);


        let [health_check_area, real_rest] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
        ]).areas(_rest);

        let check_text = match state.settings.status {
            SettingStatus::Checking(started, _) => {
                state.settings.tick();
                let spinner = spinner::spinner_text_from_start(started);
                format!("{} Health Checking...", spinner)
            }
            SettingStatus::Unhealthy(msg) => {
                format!("❌ Health Check Failed: {}", msg)
            },
            SettingStatus::Healthy => "✅ Health Check Passed And Saved".to_string(),
        };

        frame.render_widget(Paragraph::new(check_text), health_check_area);
        

        frame.render_widget(Paragraph::new("
Press `Esc` to return to the menu.
        "), real_rest);



    }
}

