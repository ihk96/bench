use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::event::AppEvent;
use crate::state::{AppState};

pub mod menu;
pub mod counter;
pub mod job;
pub mod settings;

pub trait Screen: std::fmt::Debug {
    fn handle_key(&mut self, key: KeyEvent, state: &mut AppState) -> HandledEvent;
    fn render(&self, frame: &mut Frame, area: Rect, state: &AppState);
}

pub enum HandledEvent {
    Consumed,
    Ignored,
    Event(AppEvent),
}
    
