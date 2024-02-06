use std::sync::{Arc, Mutex};
use crate::response::ScraperState;

#[derive(Clone)]
pub struct StateKeeper<T: Clone + Send> {
    state: Arc<Mutex<T>>,
}

impl<T: Clone + Send> StateKeeper<T> {
    pub fn new(val: T) -> Self {
        StateKeeper { state: Arc::new(Mutex::new(val)) }
    }

    pub fn change_state(&self, new_state: T) {
        let mut state = self.state
            .lock()
            .expect("Thread panicked, Mutex is in unrecoverable state");
        *state = new_state;
    }

    pub fn get_state(&self) -> T {
        let state = self.state
            .lock()
            .expect("Thread panicked, Mutex is in unrecoverable state");
        state.clone()
    }
}

impl Default for StateKeeper<ScraperState> {
    fn default() -> Self {
        StateKeeper::new(ScraperState::Idle)
    }
}