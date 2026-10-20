use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, style::{Color, Stylize}, text, widgets::{Paragraph, Widget}};

use crate::{app::{App, JobStatus}, event::AppEvent, screen::HandledEvent};

const JOB_SPINNER: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

#[derive(Debug)]
pub struct JobScreen;

impl JobScreen {

    pub fn render(&self, area: Rect, buf: &mut Buffer, app: &App) {
        let text = match app.job_status {
            JobStatus::Pending => String::from("Enter로 시작"),
            JobStatus::Running(started) => {
                let spinner = JOB_SPINNER[(started.elapsed().as_millis() / 100) as usize % JOB_SPINNER.len()];
                format!("{} 실행 중... 경과 {:.1?}초", spinner, started.elapsed())
            },
            JobStatus::Completed(duration) => format!("완료: {:.1?}초, Enter로 다시 실행", duration),
        };

        let paragraph = Paragraph::new(text)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        paragraph.render(area, buf);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> HandledEvent {
        match key.code {
            KeyCode::Enter => {
                return HandledEvent::Event(AppEvent::StartJob);
            },
            KeyCode::Esc => {
                return HandledEvent::Event(AppEvent::OpenMenu);
            },
            _ => HandledEvent::Ignored
        }
    }
}
