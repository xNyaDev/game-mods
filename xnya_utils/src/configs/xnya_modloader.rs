use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub loading: Loading,
    pub logging: Logging,
}

#[derive(Serialize, Deserialize)]
pub struct Loading {
    pub load_paths: Vec<String>,
    pub change_workdir: bool,
    pub delayed_load: bool,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Logging {
    pub enable_logging: bool,
    pub alloc_console: bool,
}

impl Default for Loading {
    fn default() -> Self {
        Loading {
            load_paths: vec![
                "xnya_*.dll".to_string()
            ],
            change_workdir: false,
            delayed_load: false,
        }
    }
}
