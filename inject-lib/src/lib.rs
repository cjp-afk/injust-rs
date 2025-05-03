#![allow(non_snake_case)]

use core::ffi::c_void;
use windows_sys::Win32::{
    Foundation::{BOOL, HINSTANCE},
    UI::WindowsAndMessaging::{MessageBoxW, MB_OK},
};

// Helper: convert a Rust string literal to a 0-terminated wide string at compile time.
macro_rules! wide {
    ($lit:literal) => {{
        const WIDE: &[u16] = {
            // Encode UTF-16, append \0
            const CHARS: &[u16] = &{
                const S: &str = $lit;
                let mut tmp = [0u16; $lit.len() + 1];
                let mut i = 0;
                while i < S.len() {
                    tmp[i] = S.as_bytes()[i] as u16;
                    i += 1;
                }
                tmp
            };
            CHARS
        };
        WIDE.as_ptr()
    }};
}

/// Minimal `DllMain`.
///
/// You *must* use the exact `extern "system"` ABI so Windows can call it.
#[no_mangle]
pub extern "system" fn DllMain(_hinstance: HINSTANCE, reason: u32, _reserved: *mut c_void) -> BOOL {
    // 1 == DLL_PROCESS_ATTACH
    if reason == 1 {
        unsafe {
            MessageBoxW(
                0 as *mut c_void,
                wide!("Hello from injected DLL!\0"),
                wide!("Injection success\0"),
                MB_OK,
            );
        }
    }
    1 // TRUE => load succeeds
}
