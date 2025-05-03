use crate::winapi::types::{Process, SafeHANDLE};

use std::io;

use windows_sys::Win32::Foundation::{CloseHandle, BOOL, HANDLE, HWND, LPARAM, RECT};
use windows_sys::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_CLOAKED};
use windows_sys::Win32::System::Threading::{
    GetCurrentProcessId, OpenProcess, PROCESS_ACCESS_RIGHTS,
};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::IsWindowEnabled;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindow, GetWindowLongPtrW, GetWindowRect, GetWindowTextLengthW, GetWindowTextW,
    GetWindowThreadProcessId, IsWindowVisible, GWL_EXSTYLE, GW_OWNER, WS_EX_TOOLWINDOW,
};

pub fn safe_enum_windows() -> io::Result<Vec<Process>> {
    let mut wins: Vec<Process> = Vec::new();

    unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        if IsWindowVisible(hwnd) == 0 as BOOL                    // must be visible
            || IsWindowEnabled(hwnd) == 0 as BOOL            // must be enabled
            || !GetWindow(hwnd, GW_OWNER)                       // owner window test
            .is_null()
            || (GetWindowLongPtrW(hwnd, GWL_EXSTYLE) & WS_EX_TOOLWINDOW as isize) != 0isize
            || GetWindowTextLengthW(hwnd) == 0 as BOOL
        {
            return 1;
        }

        // DWM cloak status
        let mut cloaked: u32 = 0;
        DwmGetWindowAttribute(
            hwnd,
            DWMWA_CLOAKED as u32,
            &mut cloaked as *mut _ as *mut _,
            size_of::<u32>() as u32,
        );
        if cloaked != 0 {
            return 1;
        }

        // Geometry check
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        if GetWindowRect(hwnd, &mut rect) == 0
            || rect.right - rect.left == 0
            || rect.bottom - rect.top == 0
        {
            return 1;
        }

        // Title acquisition
        let len = GetWindowTextLengthW(hwnd);
        let mut buf = vec![0u16; len as usize + 1];
        GetWindowTextW(hwnd, buf.as_mut_ptr(), buf.len() as i32);
        let title = String::from_utf16_lossy(&buf[..len as usize]);

        // PID filter
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, &mut pid);
        if pid == GetCurrentProcessId() {
            return 1;
        }

        let windows = &mut *(lparam as *mut Vec<Process>);

        windows.push(Process { pid, title });

        1
    }

    let payload = &mut wins as *mut _ as LPARAM;

    let ok = unsafe { EnumWindows(Some(enum_callback), payload) };

    if ok == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(wins)
    }
}

pub fn safe_open_process(
    dwdaccess: PROCESS_ACCESS_RIGHTS,
    inherithnd: BOOL,
    pid: u32,
) -> io::Result<SafeHANDLE> {
    let raw: HANDLE = unsafe { OpenProcess(dwdaccess, inherithnd, pid) };

    SafeHANDLE::new(raw)
}
