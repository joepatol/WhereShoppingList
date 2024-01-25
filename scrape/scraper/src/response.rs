use serde::Serialize;

#[derive(Serialize, Clone, PartialEq, Eq)]
pub enum ScraperState {
    Started,
    Running,
    Idle,
    Success,
    Failed,
}

#[derive(Serialize)]
pub struct ScraperStateResponse {
    status: ScraperState
}

impl ScraperStateResponse {
    pub fn new(status: ScraperState) -> Self {
        ScraperStateResponse { status }
    }
}