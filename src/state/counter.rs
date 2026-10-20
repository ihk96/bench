#[derive(Debug, Clone)]
pub struct CounterState {
    pub counter: u32,
}

impl Default for CounterState {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

impl CounterState {
    pub fn increment(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    pub fn decrement(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }
}