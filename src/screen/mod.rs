use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use crate::app::App;
use crate::event::AppEvent;
use crate::screen::job::JobScreen;
use crate::screen::{counter::CounterScreen, menu::MenuScreen};

pub mod menu;
pub mod counter;
pub mod job;

#[derive(Debug)]
pub enum Screen {
    Menu(MenuScreen),
    Counter(CounterScreen),
    Job(JobScreen),
}

impl Screen {
    pub fn handle_key(&mut self, key: KeyEvent) -> HandledEvent {
        match self {
            Screen::Menu(menu) => menu.handle_key(key),
            Screen::Counter(counter) => counter.handle_key(key),
            Screen::Job(job) => job.handle_key(key),
        }
    }

    
    pub fn render(&self, area: Rect, buf: &mut Buffer, app: &App) {
        match self {
            Screen::Menu(menu) => menu.render(area, buf, app),
            Screen::Counter(counter) => counter.render(area, buf, app),
            Screen::Job(job) => job.render(area, buf, app),
        }
    }
}

pub enum HandledEvent {
    Consumed,
    Ignored,
    Event(AppEvent),
}
    
