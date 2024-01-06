#![feature(naked_functions)]
#![allow(clippy::missing_safety_doc)]

use std::{env, mem};
use std::arch::global_asm;
use std::error::Error;
use std::path::Path;

use log::{error, info};
use windows::core::{PCSTR, s};
#[allow(unused_imports)]
use windows::core::imp::GetProcAddress;
use windows::Win32::Foundation::{HWND, MAX_PATH};
use windows::Win32::System::Console::AllocConsole;
use windows::Win32::System::SystemInformation::GetSystemDirectoryA;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MessageBoxA};

use xnya_utils::configs::xnya_modloader::Config;

unsafe fn main() -> Result<(), Box<dyn Error>> {
    let config: Config =
        xnya_utils::read_toml("xnya_modloader.toml")?.unwrap_or_default();

    if config.logging.alloc_console {
        match AllocConsole() {
            Ok(_) => {
                if config.logging.enable_logging {
                    xnya_utils::enable_logging();
                }
                info!("Allocated a new console")
            }
            Err(_) => {
                if config.logging.enable_logging {
                    xnya_utils::enable_logging();
                }
                error!("Failed allocating a new console");
            }
        }
    } else if config.logging.enable_logging {
        xnya_utils::enable_logging();
    }

    let starting_workdir = env::current_dir()?;

    for load_path in config.loading.load_paths {
        glob::glob(&load_path).unwrap().filter_map(Result::ok).try_for_each(|path| {
            if config.loading.change_workdir {
                let absolute_path = path.canonicalize()?;
                let new_workdir = absolute_path.parent();
                if let Some(workdir) = new_workdir {
                    env::set_current_dir(workdir)?;
                }
            }
            let name = path.to_string_lossy().to_string();
            info!("Loading {name}");
            match libloading::Library::new(&name) {
                Ok(library) => {
                    info!("Loaded {name}");
                    mem::forget(library);
                }
                Err(e) => {
                    match e {
                        libloading::Error::LoadLibraryExW { source } => {
                            error!("Failed while loading {name}: LoadLibraryExW failed with error: {source:?}");
                        }
                        _ => {
                            error!("Failed while loading {name}: {e}");
                            Err(e)?
                        }
                    }
                }
            }
            Ok::<(), Box<dyn Error>>(())
        })?;
    }

    if config.loading.change_workdir {
        env::set_current_dir(starting_workdir)?;
    }

    Ok(())
}

// The file function_count is generated by xnya_modloader_gen and has the number of functions in
// the original DLL
#[no_mangle]
pub static mut ORIGINAL_FUNCTIONS: [usize; include!("function_count.txt")] = [0; include!("function_count.txt")];

// The file jumps is generated by xnya_modloader_gen and looks like this:
// .globl GetFileVersionInfoA
// GetFileVersionInfoA:
//     jmp ds:[_ORIGINAL_FUNCTIONS + 0 * 4] - For i686
//     jmp qword ptr [rip + ORIGINAL_FUNCTIONS + 0 * 8] - For x86_64
global_asm!(include_str!("jumps.S"));

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(_: usize, call_reason: u32, _: usize) -> i32 {
    if call_reason == DLL_PROCESS_ATTACH {
        let mut buffer = [0u8; MAX_PATH as usize];
        GetSystemDirectoryA(Some(&mut buffer));
        let original_library = Path::new(
            String::from_utf8_lossy(&buffer).trim_matches(char::from(0))
        ).join(include_str!("original_library_name.txt"));
        match libloading::os::windows::Library::new(original_library) {
            #[allow(unused_variables)] Ok(original_library) => {
                // The file load_original_functions is generated by xnya_modloader_gen and looks like this:
                // ORIGINAL_FUNCTIONS[0] = original_library.get::<unsafe extern fn()>(b"GetFileVersionInfoA\0").unwrap().into_raw().unwrap() as usize;
                include!("load_original_functions.txt");

                mem::forget(original_library);

                // Post-proxy
                std::thread::spawn(|| match main() {
                    Ok(_) => {}
                    Err(e) => {
                        MessageBoxA(
                            HWND::default(),
                            PCSTR::from_raw(e.to_string().as_ptr()),
                            s!("xnya_modloader failure"),
                            MB_ICONERROR,
                        );
                    }
                });
            }
            Err(e) => {
                std::thread::spawn(move || {
                    MessageBoxA(
                        HWND::default(),
                        PCSTR::from_raw(e.to_string().as_ptr()),
                        s!("xnya_modloader failure"),
                        MB_ICONERROR,
                    );
                });
            }
        }
    }
    1
}
