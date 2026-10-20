use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, style::Modifier, widgets::{List, ListState, StatefulWidget}};
use crate::{app::App, event::AppEvent, screen::HandledEvent};


const MENU_ITEMS: [(&str, AppEvent); 3] = [
    ("Counter", AppEvent::OpenCounter), 
    ("Job", AppEvent::OpenJob),
    ("Quit", AppEvent::Quit),
];

#[derive(Debug)]
pub struct MenuScreen {
    selected: usize,
}

impl MenuScreen {
    pub fn new() -> Self {
        Self { selected: 0 }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer, _app: &App) {
        let list = List::new(MENU_ITEMS.iter().map(|(name, _)| *name))
            .highlight_symbol("> ")
            .highlight_style(Modifier::REVERSED);

        let mut list_state = ListState::default().with_selected(Some(self.selected));

        StatefulWidget::render(list, area, buf, &mut list_state);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> HandledEvent {
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
                HandledEvent::Event(MENU_ITEMS[self.selected].1.clone())
            },
            _ => HandledEvent::Ignored,
        }
    }   
}