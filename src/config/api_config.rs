use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub enum Environment {
    #[default]
    Development,
    Testing,
    Staging,
    Production,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct AppState {
    pub app_name: String,
    pub enable_log: bool,
    pub mode: Environment,
    pub enable_compression: bool,
    pub hosts: Vec<String>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState::default()
    }
    pub fn noop(&mut self) -> &mut Self {
        self
    }
    // pub fn parse_env_from_toml() {}
    // pub fn apply_settings(){}
    pub fn clear_app_name(&mut self) -> &mut Self {
        self.app_name.clear();
        self.noop()
    }
    pub fn set_app_name(&mut self, app_name: &str) -> &mut Self {
        self.clear_app_name();
        self.app_name.push_str(app_name);
        self.noop()
    }
}
