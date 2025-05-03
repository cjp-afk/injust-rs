# Injust-rs

## A simple DLL injector built in rust

Currently this is a simple proof-of-concept injector built in rust for target windows applications. Build upon the [windows-rs](https://github.com/microsoft/windows-rs) bindings and [ratatui](https://github.com/ratatui/ratatui) UI.

The injector follows a classic:


-- **OpenProcess** -> ```Handle``` -> **VirtualAlloEx** -> **WriteProcessMemory** -> **CreateRemoteThread**


template for injection and execution.

## Overview
Currently this is not meant for use, nor is near production. I will continue to improve upon its functionality, potentially adding various injection methods and techniques.

The reason I started this project was for the following concepts:

> 1. To learn Rust :crab:
> 2. To further my understanding of the windows api and its interactions within the OS
> 3. To enforce stricter safety methods within my coding style from Rust's strict compile-time checker
> 4. To have fun
---
What does my project currently aim to implement?

> ✔️ Safe wrappers abstracting ontop of the windows-rs crate
>
> ✔️ Allows for arbritrary Dll injection into host processes using a terminal UI powered by ratatui
> 
> ✔️ An example DLL and Binary to test functionality
> 
> ✖️ Currently relies on a hardcoded Dll path
> 
> ✖️ No API or library for external use outside of this crate
>
> ✖️ Lacks functionality outside of the essentials
--- 
## The Code

As previously afformentioned, this crate used a generic toolchain from the windows c abstrations to hook into a process, allocate and write to memory and fire a remote thread for execution

To start, we use the [windows-sys](https://crates.io/crates/windows-sys) crate -- not to confuse with the [windows](https://crates.io/crates/windows) crate. What's the difference?

**windows-sys** 
> Raw bindings for C-style Windows APIs.

**windows**
> Safer bindings including C-style APIs as well as COM and WinRT APIs.

Essentially, windows-sys, allows for lower overhead and memory space at the cost of type safety and error handling. However; this is something we can, and did implement within our code as needed. By doing this we reduce redundancy, and if in a real developemnt environment, create a more efficient injector.

So we've spoke about unsafety and Rust's iron fist rule about it. What does it look like?

Our raw binding looks like this:

```rust
use windows_sys::Win32::System::Threading::OpenProcess;
use windows_sys::Win32::Foundation::HANDLE;

let raw: HANDLE = unsafe { OpenProcess(dwdaccess, inherithnd, pid) }
```

The unsafe block indicates a section of code that the compiler will not enforce its strict ruling on, this should only be because the compiler *can't* check this code. In our case it's because we are calling to an external C binding, the compiler cannot physically verify code that does not live within Rust's scope. Additionally, the return type is a ```HANDLE``` -- This is an ALIAS in windows for a 
```rust 
*mut c_void
``` 
which is a raw pointer to mutable data, of an unknown layout.

So how do we fix this? Through creating safe wrappers and abstractions ontop of unsafe code. For this code snippet, there is both a functional wrapper and a type wrapper we can create. Lets walk through making these.

Here we wrap the ```HANDLE``` type alias in a struct
```rust
use windows_sys::Win32::Foundation::{CloseHandle, BOOL, HANDLE};

pub struct SafeHANDLE(HANDLE);
```

Then we can implement our constructor which takes in a raw instance of a windows ```HANDLE```. We pipe our raw ```HANDLE``` type to our constructor. If ```OpenProcess() -> HANDLE``` failed, it would return nothing. As such, we check if ```hnd``` is null, and call to the last os error. We return a ```Result<(HANDLE)>``` containing either the Error or the ```Ok(hnd)``` value.

We also add two methods. 
```rust
pub fn as_raw(&self) {...}
//and
pub fn close(self) {...}
```
These methods are responsible for aiding the interactions within out code. Because the windows bindings expect a ```HANDLE``` not our ```SafeHANDLE```, we can use ```SafeHANDLE.as_raw() -> HANDLE``` which returns our raw HANDLE for use.

```SafeHANDLE.close()``` clears and closes the handle, cleaning up. 


```rust
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
```


```rust
impl Drop for SafeHANDLE {
    fn drop(&mut self) {
        if !self.0.is_null() {
            let _ = unsafe { CloseHandle(self.0) };
        }
    }
}
```















