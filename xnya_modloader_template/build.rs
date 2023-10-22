use std::env;
use std::error::Error;

use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "cargo:rustc-cdylib-link-arg=/DEF:exports.def"
    );

    EmitBuilder::builder().git_sha(true).emit_and_set()?;

    tauri_winres::WindowsResource::new()
        .set("ProductVersion", &env::var("VERGEN_GIT_SHA")?)
        .compile()?;

    Ok(())
}