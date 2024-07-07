use std::env;
use std::error::Error;

use xnya_utils::configs::xnya_modloader::Config;

fn main() -> Result<(), Box<dyn Error>> {
    if env::var("TARGET").unwrap().contains("msvc") {
        println!("cargo:rustc-cdylib-link-arg=/DEF:exports.def");
    }
    else {
        println!("cargo:rustc-cdylib-link-arg=exports.def");
    }

    tauri_winres::WindowsResource::new()
        .compile()?;

    xnya_utils::write_toml_comments(
        &format!(
            "{}/target/xnya_modloader.toml",
            env::var("CARGO_MANIFEST_DIR").unwrap()
        ),
        &Config::default(),
    )?;

    Ok(())
}
