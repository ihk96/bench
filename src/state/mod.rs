use settings::Settings;

use crate::state::counter::CounterState;
use crate::state::job::{JobState};

pub mod settings;
pub mod counter;
pub mod job;

#[derive(Debug)]
pub struct AppState {
    pub settings: Settings,
    pub counter: CounterState,
    pub job: JobState
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            counter: CounterState::default(),
            job: JobState::default(),
        }
    }
}
