#![allow(clippy::missing_safety_doc)]

use windows::core::PCSTR;
use windows::imp::LoadLibraryA;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(_: usize, call_reason: u32, _: usize) -> i32 {
    if call_reason == DLL_PROCESS_ATTACH {
        glob::glob("xnya_*.dll").unwrap().for_each(|result| {
            let name = result.unwrap().to_string_lossy().to_string();
            LoadLibraryA(PCSTR::from_raw(name.as_ptr()));
        })
    }
    1
}
