use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct StateKeeper<T: Clone + Send> {
    state: Arc<Mutex<T>>,
}

impl<T: Clone + Send> StateKeeper<T> {
    pub fn new(val: T) -> Self {
        StateKeeper { state: Arc::new(Mutex::new(val)) }
    }

    pub async fn change_state(&self, new_state: T) {
        let mut state = self.state.lock().await;
        *state = new_state;
    }

    pub async fn get_state(&self) -> T {
        let state = self.state.lock().await;
        state.clone()
    }
}