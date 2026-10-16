use ratatui::{Frame, crossterm::event::{Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout}, widgets::Paragraph};
use tui_input::{Input, backend::crossterm::EventHandler};

use crate::{app::{Action, AppState}, component::input::render_input, screen::Screen};

pub struct SettingsScreen {
    host: Input,
    model_name: Input,
    focus: usize,
}

impl SettingsScreen {
    const FIELD_COUNT: usize = 2;
    
    pub fn new(state: &AppState) -> Self {
        Self {
            host: Input::new(state.settings.host.clone().or(Some(String::new())).unwrap()),
            model_name: Input::new(state.settings.model_name.clone().or(Some(String::new())).unwrap()),
            focus: 0,
        }
    }
}

impl Screen for SettingsScreen {
    
    fn render(&self, frame: &mut Frame, state: &AppState) {
        let [host_area, model_name_area, _rest] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ]).areas(frame.area());

        render_input(frame, host_area, &self.host, "Host", "ex) http://localhost:8000", self.focus == 0);
        render_input(frame, model_name_area, &self.model_name, "Model Name", "ex) gpt-4", self.focus == 1);

        let paragraph = Paragraph::new("Press Esc to quit, Enter to move to the next field. Up/Down to navigate.");
        frame.render_widget(paragraph, _rest);
    }

    fn handle_event(&mut self, event: &Event) -> Action {
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return Action::None;
            }
            match key.code {
                KeyCode::Esc => return Action::Quit,
                KeyCode::Down | KeyCode::Enter => {
                    self.focus = (self.focus + 1).min(Self::FIELD_COUNT - 1);
                    if key.code == KeyCode::Enter {
                        return Action::SettingDone;
                    }
                    return Action::None;
                }
                KeyCode::Up => {
                    self.focus = self.focus.saturating_sub(1);
                    return Action::None;
                }
                _ => {}
            }
        }

        // 이동 키가 아니면 포커스된 입력칸에 넘김 (문자, Backspace, 좌우 화살표, Home/End...)
        let input = match self.focus {
            0 => &mut self.host,
            _ => &mut self.model_name,
        };
        input.handle_event(event);
        Action::None
    }
}