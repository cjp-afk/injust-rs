# Injust-rs

## A simple DLL injector built in rust
Currently this is a simple proof-of-concept injector built in rust for target windows applications.
The injector follows a classic:


-- **OpenProcess** -> *Handle* -> **VirtualAlloEx** -> **WriteProcessMemory** -> **CreateRemoteThread**


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
✔️✖️
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

To start, we use the [windows-sys](https://crates.io/crates/windows-sys) crate -- not to confuse with the [windows](https://crates.io/crates/windows) crate. 
**windows**
>>Raw bindings for C-style Windows APIs.



```rust
// 
```















