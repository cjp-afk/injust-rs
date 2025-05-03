use std::ffi::c_void;
use std::io;
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
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

    pub fn close(&self) -> io::Result<()> {
        let handle = self.0;

        std::mem::forget(self);
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
            unsafe { CloseHandle(self.0) };
        }
    }
}

pub struct RemoteMemory<'a> {
    handle: &'a SafeHANDLE,
    addr: *mut c_void,
    size: usize,
}

impl<'a> RemoteMemory<'a> {
    pub fn new(
        handle: &'a SafeHANDLE,
        size: usize,
        alloc_type: u32,
        prot: u32,
    ) -> io::Result<Self> {
        let addr = unsafe { VirtualAllocEx(handle.as_raw(), null_mut(), size, alloc_type, prot) };

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
}

impl<'a> Drop for RemoteMemory<'a> {
    fn drop(&mut self) {
        unsafe { VirtualFreeEx(self.handle.as_raw(), self.addr, 0, MEM_RELEASE) };
    }
}
