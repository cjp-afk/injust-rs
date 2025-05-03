#![allow(dead_code)]

use crate::winapi::types::{Process, SafeHANDLE};
use std::ffi::c_void;

use std::io;
use std::mem::transmute;
use std::ptr::null_mut;

use windows_sys::core::PCSTR;
use windows_sys::Win32::Foundation::{BOOL, HANDLE, HWND, LPARAM, RECT};
use windows_sys::Win32::Foundation::{FARPROC, HINSTANCE};
use windows_sys::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_CLOAKED};
use windows_sys::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows_sys::Win32::System::Threading::{
    CreateRemoteThread, GetCurrentProcessId, OpenProcess, LPTHREAD_START_ROUTINE,
    PROCESS_ACCESS_RIGHTS,
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

pub fn safe_write_process_memory(
    phandle: HANDLE,
    m_base_address: *mut c_void,
    path_w: &[u16],
    byte_count: usize,
    overwritten: *mut usize,
) -> io::Result<()> {
    let ok = unsafe {
        WriteProcessMemory(
            phandle,
            m_base_address,
            path_w.as_ptr() as *const _,
            byte_count,
            overwritten,
        )
    };
    if ok == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn safe_create_remote_thread(
    process: HANDLE,
    start: FARPROC,
    param: *mut c_void,
    flags: u32,
) -> io::Result<SafeHANDLE> {
    let start_routine: LPTHREAD_START_ROUTINE = unsafe { transmute(start) };

    let mut tid = 0u32;
    let h_thread = unsafe {
        CreateRemoteThread(
            process,
            null_mut(),
            0,
            start_routine,
            param,
            flags,
            &mut tid,
        )
    };
    if h_thread.is_null() {
        Err(io::Error::last_os_error())
    } else {
        Ok(SafeHANDLE::new(h_thread)?)
    }
}

pub fn safe_wait_for_single_object(h: HANDLE, ms: u32) -> io::Result<()> {
    use windows_sys::Win32::System::Threading::WaitForSingleObject;
    let res = unsafe { WaitForSingleObject(h, ms) };
    if res == 0xFFFFFFFF {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn get_loadlibraryw_addr() -> FARPROC {
    unsafe {
        // Kernel32 is guaranteed to be loaded already.
        let k32: HINSTANCE = GetModuleHandleA(c"kernel32.dll".as_ptr() as PCSTR);
        GetProcAddress(k32, c"LoadLibraryW".as_ptr() as PCSTR)
    }
}
