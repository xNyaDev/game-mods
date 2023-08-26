#![allow(clippy::missing_safety_doc)]

use std::error::Error;
use std::{ptr, slice};

use windows::core::{s, PCSTR};
use windows::Win32::Foundation::HWND;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_ICONERROR};

use xnya_utils::configs::xnya_rallytrophy_cryptutil::Config;

unsafe fn main() -> Result<(), Box<dyn Error>> {
    let mut config: Config =
        xnya_utils::read_toml("xnya_rallytrophy_cryptutil.toml")?.unwrap_or_default();
    if config.dump_key {
        let mut keys =
            xnya_utils::read_toml("Keys.toml")?.unwrap_or(bfstool::keys::Keys { bzf2001: None });

        let data = slice::from_raw_parts(0x501D10 as *const u8, 1024);
        let mut key = [0; 256];

        data.iter()
            .step_by(4)
            .enumerate()
            .for_each(|(index, value)| {
                key[index] = *value;
            });

        keys.bzf2001 = Some(bfstool::keys::Bzf2001Keys { key });

        xnya_utils::write_toml("Keys.toml", &keys)?;
        config.dump_key = false;
    }
    if config.disable_encryption {
        ptr::write_bytes(0x501D10 as *mut u8, 0, 1024);
    }
    xnya_utils::write_toml("xnya_rallytrophy_cryptutil.toml", &config)?;
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
