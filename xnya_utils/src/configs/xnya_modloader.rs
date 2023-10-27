use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub load_paths: Vec<String>,
    pub logging: Logging,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Logging {
    pub enable_logging: bool,
    pub alloc_console: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            load_paths: vec![
                "xnya_*.dll".to_string()
            ],
            logging: Default::default(),
        }
    }
}
