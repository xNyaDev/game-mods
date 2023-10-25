use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub load_paths: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            load_paths: vec![
                "xnya_*.dll".to_string()
            ]
        }
    }
}
