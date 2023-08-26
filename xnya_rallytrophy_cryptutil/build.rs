use std::env;
use std::error::Error;

use xnya_utils::configs::xnya_rallytrophy_cryptutil::Config;

fn main() -> Result<(), Box<dyn Error>> {
    xnya_utils::write_toml(
        &format!(
            "{}/../target/xnya_rallytrophy_cryptutil.toml",
            env::var("CARGO_MANIFEST_DIR").unwrap()
        ),
        &Config::default(),
    )?;
    Ok(())
}
