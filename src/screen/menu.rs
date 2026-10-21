use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, buffer::Buffer, layout::Rect, style::Modifier, widgets::{List, ListState, StatefulWidget}};
use crate::{event::AppEvent, screen::{HandledEvent, Screen, counter::CounterScreen, job::JobScreen, settings::SettingsScreen}, state::AppState};


const MENU_ITEMS: [(&str, AppEvent); 4] = [
    ("Counter", AppEvent::PushScreen(|| Box::new(CounterScreen))), 
    ("Job", AppEvent::PushScreen(|| Box::new(JobScreen))),
    ("Settings", AppEvent::PushScreen(|| Box::new(SettingsScreen::default()))),
    ("Quit", AppEvent::Quit),
];

#[derive(Debug)]
pub struct MenuScreen {
    selected: usize,
}

impl Default for MenuScreen {
    fn default() -> Self {
        Self { selected: 0 }
    }
}

impl Screen for MenuScreen {

    fn render(&self, frame: &mut Frame, area: Rect, _state: &mut AppState) {
        let list = List::new(MENU_ITEMS.iter().map(|(name, _)| *name))
            .highlight_symbol("> ")
            .highlight_style(Modifier::REVERSED);

        let mut list_state = ListState::default().with_selected(Some(self.selected));

        frame.render_stateful_widget(list, area, &mut list_state);
    }

    fn handle_key(&mut self, key: KeyEvent, _state: &mut AppState) -> HandledEvent {
        match key.code {
            KeyCode::Up => {
                self.selected = if self.selected == 0 {
                    MENU_ITEMS.len() - 1
                } else {
                    self.selected - 1
                };
                HandledEvent::Consumed
            },
            KeyCode::Down => {
                self.selected = if self.selected >= MENU_ITEMS.len() - 1 {
                    0
                } else {
                    self.selected + 1
                };
                HandledEvent::Consumed
            },
            KeyCode::Enter => {
                return HandledEvent::Event(MENU_ITEMS[self.selected].1.clone());
            },
            _ => HandledEvent::Ignored,
        }
    }   
}