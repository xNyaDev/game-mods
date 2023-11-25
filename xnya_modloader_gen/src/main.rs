use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use clap::Parser;
use pelite::FileMap;
use pelite::pe::{Pe, PeFile};
use toml_edit::{Document, value};
use windows::Win32::Foundation::MAX_PATH;
use windows::Win32::System::SystemInformation::GetSystemDirectoryA;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input DLL file to process. Has to be available in the system directory
    dll_file: String,
    /// Output directory in which to place the resulting project
    #[arg(short, long)]
    out: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut buffer = [0u8; MAX_PATH as usize];
    unsafe {
        GetSystemDirectoryA(Some(&mut buffer));
    }
    let original_library = format!(
        "{}\\{}",
        String::from_utf8_lossy(&buffer).trim_matches(char::from(0)),
        args.dll_file
    );

    let file_map = FileMap::open(&original_library).unwrap();
    let pe_file = PeFile::from_bytes(file_map.as_ref()).unwrap();

    let exports = pe_file.exports().unwrap().by().unwrap().iter_names().filter_map(|(name, _)| {
        match name {
            Ok(name) => Some(name.to_string()),
            Err(_) => None
        }
    }).collect::<Vec<String>>();

    match args.out.parent() {
        None => {}
        Some(parent) => {
            fs::create_dir_all(parent).unwrap();
        }
    }

    copy_dir::copy_dir("xnya_modloader_template", &args.out).unwrap();

    let mut original_library_name = File::create(args.out.join("src/original_library_name.txt")).unwrap();
    original_library_name.write_all(args.dll_file.as_bytes()).unwrap();

    let mut function_count = File::create(args.out.join("src/function_count.txt")).unwrap();
    function_count.write_all(exports.len().to_string().as_bytes()).unwrap();

    let mut jumps = File::create(args.out.join("src/jumps.S")).unwrap();
    let mut load_original_functions = File::create(args.out.join("src/load_original_functions.txt")).unwrap();
    let mut exports_def = File::create(args.out.join("exports.def")).unwrap();

    load_original_functions.write_all(b"{\n").unwrap();
    exports_def.write_all(b"EXPORTS\n").unwrap();

    for (index, export) in exports.into_iter().enumerate() {
        let label = format!(".globl {export}\n{export}:\n    ");
        #[cfg(target_pointer_width = "32")]
            let jump = format!("jmp ds:[_ORIGINAL_FUNCTIONS + {index} * 4]");
        #[cfg(target_pointer_width = "64")]
            let jump = format!("jmp qword ptr [rip + ORIGINAL_FUNCTIONS + {index} * 8]");

        jumps.write_all(format!("{}{}\n", label, jump).as_bytes()).unwrap();

        let load_original = format!("ORIGINAL_FUNCTIONS[{index}] = GetProcAddress(original_library.0, s!(\"{export}\").0).unwrap() as usize;\n");

        load_original_functions.write_all(load_original.as_bytes()).unwrap();

        exports_def.write_all(format!("{export}\n").as_bytes()).unwrap();
    }

    load_original_functions.write_all(b"}\n").unwrap();

    let mut cargo_toml = File::open(args.out.join("Cargo.toml")).unwrap();
    let mut cargo_toml_contents = String::new();
    cargo_toml.read_to_string(&mut cargo_toml_contents).unwrap();

    let mut cargo_toml = cargo_toml_contents.parse::<Document>().unwrap();
    let xnya_utils_path = fs::canonicalize("./xnya_utils").unwrap().to_string_lossy().to_string();
    cargo_toml["dependencies"]["xnya_utils"]["path"] = value(&xnya_utils_path);
    cargo_toml["build-dependencies"]["xnya_utils"]["path"] = value(&xnya_utils_path);
    File::create(
        args.out.join("Cargo.toml")
    ).unwrap().write_all(
        cargo_toml.to_string().as_bytes()
    ).unwrap();
}
