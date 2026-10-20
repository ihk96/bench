use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer, layout::{Alignment, Rect}, style::{Color,Stylize}, widgets::{Block, BorderType, Paragraph, Widget}};
use crate::{app::App, event::AppEvent, screen::HandledEvent};

#[derive(Debug)]
pub struct CounterScreen;

impl CounterScreen {
    pub fn render(&self, area: Rect, buf: &mut Buffer, app: &App) {
        let block = Block::bordered()
            .title("llm-instance-bench")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = format!(
            "This is a tui template.\n\
                Press `Esc` to return to the menu.\n\
                Press `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.counter
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        paragraph.render(area, buf);
    }
    pub fn handle_key(&mut self, key: KeyEvent) -> HandledEvent {
        match key.code {
            KeyCode::Right => HandledEvent::Event(AppEvent::Increment),
            KeyCode::Left => HandledEvent::Event(AppEvent::Decrement),
            KeyCode::Esc => HandledEvent::Event(AppEvent::OpenMenu),
            _ => HandledEvent::Ignored,
        }
    }
}