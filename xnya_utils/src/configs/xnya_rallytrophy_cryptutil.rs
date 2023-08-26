use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub dump_key: bool,
    pub disable_encryption: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            dump_key: true,
            disable_encryption: false,
        }
    }
}
