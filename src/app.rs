
use crate::{event::{AppEvent, Event, EventHandler}, screen::{HandledEvent, Screen, menu::MenuScreen}, state::AppState};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};


/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Counter.
    pub events: EventHandler,
    pub screen_stack: Vec<Box<dyn Screen>>,
    pub state: AppState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            screen_stack: vec![Box::new(MenuScreen::default())],
            state: AppState::default(),
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
            terminal.draw(|frame| self.render(frame))?;
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
                    AppEvent::Quit => self.quit(),
                    AppEvent::PushScreen(open_screen) => {self.screen_stack.push(open_screen());},
                    AppEvent::PopScreen => { self.screen_stack.pop(); },
                },
            }
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        if let Some(screen) = self.screen_stack.last() {
            let area = frame.area();
            screen.render(frame, area, &self.state);
        }
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        if matches!(key_event.code, KeyCode::Char('c' | 'C')) {
            if key_event.modifiers == KeyModifiers::CONTROL {
                self.events.send(AppEvent::Quit);
                return Ok(())
            }
        }
        if let Some(screen) = self.screen_stack.last_mut() {
            match screen.handle_key(key_event, &mut self.state) {
                HandledEvent::Event(app_event) => self.events.send(app_event),
                HandledEvent::Ignored => {
                    if matches!(key_event.code, KeyCode::Esc | KeyCode::Char('q')) {
                        self.events.send(AppEvent::Quit)
                    }
                },
                HandledEvent::Consumed => {}
            }
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&mut self) {
        self.state.job.tick();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

}
