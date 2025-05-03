use std::thread;
use std::time::Duration;

fn main() {
    let pid = std::process::id();
    println!("target_app running – PID = {pid}. Waiting for injection…");

    // Keep the process alive indefinitely so the injector has time.
    loop {
        thread::sleep(Duration::from_secs(120));
    }
}
