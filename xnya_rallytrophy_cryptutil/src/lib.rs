#![allow(clippy::missing_safety_doc)]

use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::{ptr, slice};

use serde::{Deserialize, Serialize};
use windows::core::PCSTR;
use windows::s;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_ICONERROR};

#[derive(Serialize, Deserialize)]
struct Config {
    pub dump_key: bool,
    pub disable_encryption: bool,
}

unsafe fn main() -> Result<(), Box<dyn Error>> {
    let mut config = if Path::new("xnya_rallytrophy_cryptutil.toml").exists() {
        let mut file = File::open("xnya_rallytrophy_cryptutil.toml")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        toml::from_str(&contents)?
    } else {
        Config {
            dump_key: true,
            disable_encryption: false,
        }
    };
    if config.dump_key {
        let mut keys = if Path::new("Keys.toml").exists() {
            let mut file = File::open("Keys.toml")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            toml::from_str(&contents)?
        } else {
            bfstool::keys::Keys { bzf2001: None }
        };

        let data = slice::from_raw_parts(0x501D10 as *const u8, 1024);
        let mut key = [0; 256];

        data.iter()
            .step_by(4)
            .enumerate()
            .for_each(|(index, value)| {
                key[index] = *value;
            });

        keys.bzf2001 = Some(bfstool::keys::Bzf2001Keys { key });

        let mut file = File::create("Keys.toml")?;
        file.write_all(toml::to_string_pretty(&keys)?.as_bytes())?;
        config.dump_key = false;
    }
    if config.disable_encryption {
        ptr::write_bytes(0x501D10 as *mut u8, 0, 1024);
    }
    let mut file = File::create("xnya_rallytrophy_cryptutil.toml")?;
    file.write_all(toml::to_string_pretty(&config)?.as_bytes())?;
    Ok(())
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(_: usize, call_reason: u32, _: usize) -> i32 {
    if call_reason == DLL_PROCESS_ATTACH {
        std::thread::spawn(|| match main() {
            Ok(_) => {}
            Err(e) => {
                MessageBoxA(
                    HWND::default(),
                    PCSTR::from_raw(e.to_string().as_ptr()),
                    s!("xnya_rallytrophy_cryptutil failure"),
                    MB_ICONERROR,
                );
            }
        });
    }
    1
}
