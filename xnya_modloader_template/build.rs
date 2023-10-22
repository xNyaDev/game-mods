use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "cargo:rustc-cdylib-link-arg=/DEF:exports.def"
    );

    tauri_winres::WindowsResource::new().compile()?;

    Ok(())
}