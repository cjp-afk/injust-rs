use crate::winapi::types::{RemoteMemory, SafeHANDLE};
use crate::winapi::winsafe::{
    get_loadlibraryw_addr, safe_create_remote_thread, safe_open_process,
    safe_wait_for_single_object, safe_write_process_memory,
};

use std::ffi::OsStr;
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

use windows_sys::Win32::System::Memory::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};
use windows_sys::Win32::System::Threading::{
    INFINITE, PROCESS_CREATE_THREAD, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION,
    PROCESS_VM_READ, PROCESS_VM_WRITE,
};

pub struct OxidisingAgent {
    path_w: Vec<u16>,
    phandle: SafeHANDLE,
    rmemory: RemoteMemory,
}

impl OxidisingAgent {
    pub fn new(pid: u32, dll_path: String) -> io::Result<Self> {
        let access_rights = PROCESS_CREATE_THREAD
            | PROCESS_QUERY_INFORMATION
            | PROCESS_VM_OPERATION
            | PROCESS_VM_WRITE
            | PROCESS_VM_READ;

        let phandle: SafeHANDLE = safe_open_process(access_rights, 0, pid)?;

        let path_w: Vec<u16> = OsStr::new(&dll_path)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let rmemory: RemoteMemory = RemoteMemory::new(
            phandle.as_raw(),
            path_w.len() * 2,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        )?;

        Ok(Self {
            path_w,
            phandle,
            rmemory,
        })
    }

    pub fn oxidise(&mut self) -> io::Result<()> {
        safe_write_process_memory(
            self.phandle.as_raw(),
            self.rmemory.address(),
            &self.path_w,
            self.path_w.len() * 2,
            null_mut(),
        )?;

        let ll_addr = get_loadlibraryw_addr();

        let thread =
            safe_create_remote_thread(self.phandle.as_raw(), ll_addr, self.rmemory.address(), 0)?;

        safe_wait_for_single_object(thread.as_raw(), INFINITE)?;

        Ok(())
    }
}
