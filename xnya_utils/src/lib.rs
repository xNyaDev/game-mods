use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub use toml_comments::DocumentedStruct;
pub use toml_comments::serialize_with_comments;

pub mod configs;
mod toml_comments;

/// Read a toml document from a file
pub fn read_toml<T: serde::de::DeserializeOwned>(name: &str) -> Result<Option<T>, Box<dyn Error>> {
    if Path::new(name).exists() {
        let mut file = File::open(name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(toml::from_str(&contents)?)
    } else {
        Ok(None)
    }
}

/// Write a toml document into a file
pub fn write_toml<T: serde::ser::Serialize>(name: &str, data: &T) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(name)?;
    file.write_all(toml::to_string_pretty(data)?.as_bytes())?;
    Ok(())
}

/// Write a toml document into a file, with comments
pub fn write_toml_comments<T: serde::ser::Serialize + DocumentedStruct>(name: &str, data: &T) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(name)?;
    file.write_all(serialize_with_comments(data)?.trim().as_bytes())?;
    Ok(())
}

pub fn enable_logging(disable_colors: bool) {
    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or("info")
            .default_write_style_or(if disable_colors { "never" } else { "auto" }),
    )
        .init();
}
