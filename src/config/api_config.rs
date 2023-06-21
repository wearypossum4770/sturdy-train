use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AppState {
  pub app_name: String,
}

impl AppState {
  pub fn new() -> AppState {
    AppState::default()
  }
}