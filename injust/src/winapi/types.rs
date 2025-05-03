#![allow(dead_code)]

use std::ffi::c_void;
use std::io;
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::{CloseHandle, BOOL, HANDLE};
use windows_sys::Win32::System::Memory::{VirtualAllocEx, VirtualFreeEx, MEM_RELEASE};

#[derive(Debug)]
pub(crate) struct Process {
    pub(crate) pid: u32,
    pub(crate) title: String,
}

pub struct SafeHANDLE(HANDLE);
impl SafeHANDLE {
    pub fn new(hnd: HANDLE) -> io::Result<Self> {
        if hnd.is_null() {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "received null Windows handle",
            ))
        } else {
            Ok(Self(hnd))
        }
    }

    pub fn as_raw(&self) -> HANDLE {
        self.0
    }

    pub fn close(self) -> io::Result<()> {
        let handle = self.0;

        let _ = self;
        let ok = unsafe { CloseHandle(handle) };
        if ok == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

impl Drop for SafeHANDLE {
    fn drop(&mut self) {
        if !self.0.is_null() {
            let _ = unsafe { CloseHandle(self.0) };
        }
    }
}

pub struct RemoteMemory {
    handle: HANDLE,
    addr: *mut c_void,
    size: usize,
}

impl RemoteMemory {
    pub fn new(handle: HANDLE, size: usize, alloc_type: u32, prot: u32) -> io::Result<Self> {
        let addr = unsafe { VirtualAllocEx(handle, null_mut(), size, alloc_type, prot) };

        if addr.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self { handle, addr, size })
        }
    }

    pub fn address(&self) -> *mut c_void {
        self.addr
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn de_alloc(&mut self) -> io::Result<()> {
        let ok: BOOL = if !self.handle.is_null() {
            unsafe { VirtualFreeEx(self.handle, self.addr, 0, MEM_RELEASE) }
        } else {
            1
        };
        if ok == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

impl Drop for RemoteMemory {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { VirtualFreeEx(self.handle, self.addr, 0, MEM_RELEASE) };
        }
    }
}
