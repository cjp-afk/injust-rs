# injust‑rs

> **Does the world need another DLL injector?**  
> Probably not—but writing one is **an unbeatable way to interrogate how Windows really works**.  
> `injust‑rs` is my tiny, opinionated, proof‑of‑concept DLL injector written in **Rust** with a sleek TUI powered by [ratatui].

---

## ✨&nbsp;Motivation

* **Learn Rust**: borrow‑checker or bust.  
* **Demystify the Windows API**: instead of cargo‑culting snippets, I wanted to walk every call on the wire.  
* **Embrace safety _without_ ceremony**: keep `unsafe` narrowly scoped behind ergonomic wrappers.  
* …and, honestly, **have fun hacking on low‑level stuff**.

---

## 🔨&nbsp;How it works — the classic chain

| Step | API call | Purpose |
|------|----------|---------|
| 1 | `OpenProcess` | Gain a handle (`PROCESS_ALL_ACCESS`) to the target PID. |
| 2 | `VirtualAllocEx` | Reserve **and** commit RW memory in the remote process for the DLL path. |
| 3 | `WriteProcessMemory` | Copy our UTF‑16 encoded DLL path into that region. |
| 4 | `CreateRemoteThread` | Start a thread at `LoadLibraryW`, passing the address of the path—Windows obligingly loads the DLL. |
| 5 | `WaitForSingleObject` → `GetExitCodeThread` | (Optional) Observe completion & exit status. |
| 6 | `VirtualFreeEx`, `CloseHandle` | Clean up remote memory and local handles. |

> Skeptical thought‑starter: **couldn’t anti‑cheat block every one of these calls?**  
> They often try—experiment and find out.

---

## 📦&nbsp;Project structure

```
injust-rs/
├─ Cargo.toml
├─ inject-lib/        # Example DLL (pops a MessageBox on DllMain)
├─ injust/            # CLI injector with ratatui UI
└─ example_target/    # Tiny Win32 app to receive the injection
```

---

## 🚀&nbsp;Building & running

```console
# prerequisite: stable Rust toolchain on Windows 10/11 (x86_64)
git clone https://github.com/cjp-afk/injust-rs.git
cd injust-rs

# build everything in release mode
cargo build --release

# run the example target in one terminal
target\release\example_target.exe

# inject it from another
target\release\injust.exe
```

If you prefer names over PIDs, there’s a helper flag:

```console
injust.exe --process-name example_target.exe --dll ...
```

---

## 🛡️&nbsp;Safety abstractions

The Windows API is *decidedly* not memory‑safe.  
`injust‑rs` squirrels every raw handle away inside a thin wrapper:

```rust
pub struct SafeHandle(HANDLE);

impl Drop for SafeHandle {
    fn drop(&mut self) { unsafe { CloseHandle(self.0); } }
}
```

You still see `unsafe { … }`—but only at the wrapper edge, **never in the application logic**.  
_Is that enough?_ Jury’s out, but it already caught double‑frees during my tests.

---

## 🛣&nbsp;Roadmap

- [ ] Remove the _hard‑coded_ DLL path (CLI arg done, config TOML next)
- [ ] Architecture guard (x86 ↔ x64 mismatch detection)
- [ ] Publish a `libinjust` crate so other projects can call `inject(pid, path)`
- [ ] Implement alternate techniques (APC queue, `SetWindowsHookEx`, reflective loading)
- [ ] Add CI on Windows runners

---

## 🤔&nbsp;FAQ

* **Q: Will this bypass antivirus?**  
  A: **Unlikely.** Static DLL injection is a solved problem for defenders. Expect detection.

* **Q: Can I use this against machines I don’t own?**  
  A: _Absolutely not._ See the disclaimer below.

---

## 📜&nbsp;Disclaimer

This repository is provided **solely for educational and research purposes**.  
Running `injust‑rs` on computers you do not own _or_ do not have explicit permission to test **may violate local laws and the Computer Misuse Act** (or your jurisdiction’s equivalent).  
You are responsible for obeying the law.

---

## 🪪&nbsp;License

`injust‑rs` is licensed under the MIT License—see [`LICENSE`](LICENSE) for details.

---

*Happy hacking—question everything and verify twice.*  
— **cjp**
