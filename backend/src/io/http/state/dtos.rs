use serde::Serialize;

#[derive(Serialize)]
pub struct StateRefresh {
    pub result: String,
}

pub fn state_refreshed() -> StateRefresh {
    StateRefresh {
        result: "state refreshed".to_string(),
    }
}
