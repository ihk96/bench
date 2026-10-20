use std::time::Duration;

use crate::{event::{AppEvent, Event, EventHandler}, screen::{HandledEvent, Screen, counter::CounterScreen, job::JobScreen, menu::MenuScreen}};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;

#[derive(Debug)]
pub enum JobStatus {
    Pending,
    Running(std::time::Instant),
    Completed(std::time::Duration),
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Counter.
    pub counter: u8,
    /// Event handler.
    pub events: EventHandler,
    pub screen: Screen,
    pub job_status: JobStatus,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            events: EventHandler::new(),
            screen: Screen::Menu(MenuScreen::new()),
            job_status: JobStatus::Pending,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event)
                        if key_event.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        self.handle_key_events(key_event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Increment => self.increment_counter(),
                    AppEvent::Decrement => self.decrement_counter(),
                    AppEvent::Quit => self.quit(),
                    AppEvent::OpenMenu => {self.screen = Screen::Menu(MenuScreen::new());},
                    AppEvent::OpenCounter => {self.screen = Screen::Counter(CounterScreen);},
                    AppEvent::JobDone(elapsed) => {
                        self.job_status = JobStatus::Completed(elapsed);
                    },
                    AppEvent::StartJob => {
                        if !matches!(self.job_status, JobStatus::Running(_)) {
                            let start_time = std::time::Instant::now();
                            self.job_status = JobStatus::Running(start_time);
                            let sender = self.events.clone_sender();
                            tokio::spawn(async move {
                                tokio::time::sleep(Duration::from_secs(3)).await;
                                let duration = start_time.elapsed();
                                let _ = sender.send(Event::App(AppEvent::JobDone(duration)));
                            });
                        }
                    },
                    AppEvent::OpenJob => {self.screen = Screen::Job(JobScreen);},
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        if matches!(key_event.code, KeyCode::Char('c' | 'C')) {
            if key_event.modifiers == KeyModifiers::CONTROL {
                self.events.send(AppEvent::Quit);
                return Ok(())
            }
        }
        match self.screen.handle_key(key_event) {
            HandledEvent::Event(app_event) => self.events.send(app_event),
            HandledEvent::Ignored => {
                if matches!(key_event.code, KeyCode::Esc | KeyCode::Char('q')) {
                    self.events.send(AppEvent::Quit)
                }
            },
            HandledEvent::Consumed => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }
}
