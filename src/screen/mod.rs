use ratatui::{Frame, crossterm::event::Event};

use crate::app::AppState;

pub mod settings;


pub trait Screen {
    fn render(&self, frame: &mut Frame, state: &AppState);
    fn handle_event(&mut self, event: &Event) -> crate::app::Action;
}