use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub mod configs;

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
