use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{mem, slice};

use clap::Parser;
use memchr::memmem;
use pelite::image::{
    IMAGE_OPTIONAL_HEADER32, IMAGE_OPTIONAL_HEADER64, IMAGE_SUBSYSTEM_WINDOWS_CUI,
};
use pelite::Wrap;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input exe file to process
    input: PathBuf,
    /// Output exe file to write to. If not provided, input will have .console.exe added at the end.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let output = args
        .output
        .unwrap_or(format!("{}.console.exe", args.input.to_string_lossy()).into());

    let file_map = pelite::FileMap::open(&args.input)?;
    let image = pelite::PeFile::from_bytes(file_map.as_ref())?;

    let (data, new_data): (Vec<u8>, Vec<u8>) = match image.optional_header() {
        Wrap::T32(optional_header) => {
            let data = *optional_header;
            let new_data = IMAGE_OPTIONAL_HEADER32 {
                Subsystem: IMAGE_SUBSYSTEM_WINDOWS_CUI,
                ..data
            };

            let data_size = mem::size_of::<IMAGE_OPTIONAL_HEADER32>();

            let data = unsafe {
                slice::from_raw_parts(
                    &data as *const IMAGE_OPTIONAL_HEADER32 as *const u8,
                    data_size,
                )
            };
            let new_data = unsafe {
                slice::from_raw_parts(
                    &new_data as *const IMAGE_OPTIONAL_HEADER32 as *const u8,
                    data_size,
                )
            };

            (data.to_vec(), new_data.to_vec())
        }
        Wrap::T64(optional_header) => {
            let data = *optional_header;
            let new_data = IMAGE_OPTIONAL_HEADER64 {
                Subsystem: IMAGE_SUBSYSTEM_WINDOWS_CUI,
                ..data
            };

            let data_size = mem::size_of::<IMAGE_OPTIONAL_HEADER64>();

            let data = unsafe {
                slice::from_raw_parts(
                    &data as *const IMAGE_OPTIONAL_HEADER64 as *const u8,
                    data_size,
                )
            };
            let new_data = unsafe {
                slice::from_raw_parts(
                    &new_data as *const IMAGE_OPTIONAL_HEADER64 as *const u8,
                    data_size,
                )
            };

            (data.to_vec(), new_data.to_vec())
        }
    };

    let mut file_data = Vec::new();
    let mut file = File::open(&args.input)?;
    file.read_to_end(&mut file_data)?;

    let position = memmem::find(file_data.as_slice(), data.as_slice()).unwrap();

    file_data[position..(new_data.len() + position)].copy_from_slice(new_data.as_slice());

    let mut new_file = File::create(output)?;
    new_file.write_all(file_data.as_slice())?;

    Ok(())
}
