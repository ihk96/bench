use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, style::{Color, Stylize}, text, widgets::{Paragraph, Widget}};

use crate::{component::spinner, event::AppEvent, screen::{HandledEvent, Screen}, state::{AppState, job::JobStatus}};

#[derive(Debug)]
pub struct JobScreen;

impl Screen for JobScreen {

    fn render(&self, frame: &mut Frame, area: Rect, state: &mut AppState) {
        let text = match state.job.job_status {
            JobStatus::Pending => String::from("Enter로 시작"),
            JobStatus::Running(started,_) => {
                state.job.tick();
                let spinner = spinner::spinner_text(started.elapsed().as_millis());
                format!("{} 실행 중... 경과 {:.1?}초", spinner, started.elapsed())
            },
            JobStatus::Completed(duration) => format!("완료: {:.1?}초, Enter로 다시 실행", duration),
        };

        let paragraph = Paragraph::new(text)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        frame.render_widget(paragraph, area);
    }

    fn handle_key(&mut self, key: KeyEvent, state: &mut AppState) -> HandledEvent {
        match key.code {
            KeyCode::Enter => {
                state.job.start();

                return HandledEvent::Consumed;
            },
            KeyCode::Esc => {
                return HandledEvent::Event(AppEvent::PopScreen);
            },
            _ => HandledEvent::Ignored
        }
    }
}
