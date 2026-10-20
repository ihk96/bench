use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame, layout::{Alignment, Rect}, style::{Color,Stylize}, widgets::{Block, BorderType, Paragraph, Widget}};
use crate::{event::AppEvent, screen::{HandledEvent, Screen}, state::AppState};

#[derive(Debug)]
pub struct CounterScreen;

impl Screen for CounterScreen {
    fn render(&self, frame: &mut Frame, area: Rect, state: &AppState) {
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
            state.counter.counter
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        frame.render_widget(paragraph, area);
    }
    fn handle_key(&mut self, key: KeyEvent, state: &mut AppState) -> HandledEvent {
        match key.code {
            KeyCode::Right => {
                state.counter.increment();
                HandledEvent::Consumed
            },
            KeyCode::Left => {
                state.counter.decrement();
                HandledEvent::Consumed
            },
            KeyCode::Esc => HandledEvent::Event(AppEvent::PopScreen),
            _ => HandledEvent::Ignored,
        }
    }
}