#![allow(clippy::missing_safety_doc)]

use std::error::Error;
use std::{mem, slice};

use windows::core::{s, PCSTR};
use windows::Win32::Foundation::HWND;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_ICONERROR};

use xnya_utils::configs::xnya_retrodemo_cryptutil::Config;

static mut XXTEA_ORIG: usize = 0;

unsafe extern "cdecl" fn xxtea_hook(data: usize, key: usize, length: u32) {
    if key != 0x52FCA4 {
        let orig: fn(usize, usize, u32) = mem::transmute(XXTEA_ORIG);
        orig(data, key, length);
    }
}

unsafe fn main() -> Result<(), Box<dyn Error>> {
    let mut config: Config =
        xnya_utils::read_toml("xnya_retrodemo_cryptutil.toml")?.unwrap_or_default();
    if config.dump_key {
        let mut keys =
            xnya_utils::read_toml("Keys.toml")?.unwrap_or(bfstool::keys::Keys { bzf2001: None, bzf2002: None });

        let key = slice::from_raw_parts(0x52FCA4 as *const u8, 16);

        keys.bzf2002 = Some(bfstool::keys::Bzf2002Keys { key: key.try_into().unwrap() });

        xnya_utils::write_toml("Keys.toml", &keys)?;
        config.dump_key = false;
    }
    if config.disable_archive_encryption {
        XXTEA_ORIG = minhook::MinHook::create_hook(0x4A2B80 as _, xxtea_hook as _).unwrap() as _;
        minhook::MinHook::enable_hook(0x4A2B80 as _).unwrap();
    }
    xnya_utils::write_toml("xnya_retrodemo_cryptutil.toml", &config)?;
    Ok(())
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(_: usize, call_reason: u32, _: usize) -> i32 {
    if call_reason == DLL_PROCESS_ATTACH {
        match main() {
            Ok(_) => {}
            Err(e) => {
                MessageBoxA(
                    HWND::default(),
                    PCSTR::from_raw(e.to_string().as_ptr()),
                    s!("xnya_retrodemo_cryptutil failure"),
                    MB_ICONERROR,
                );
            }
        }
    }
    1
}
