use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::port::{find_by_port, is_port_listening};

pub fn kill(port: u16, force: bool) -> Result<(), String> {
    let entry = find_by_port(port)?;
    let signal = if force { "-KILL" } else { "-TERM" };

    let output = Command::new("kill")
        .args([signal, &entry.pid])
        .output()
        .map_err(|e| format!("could not run kill: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "could not stop {} (PID {}): {}",
            entry.process,
            entry.pid,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    for _ in 0..20 {
        if !is_port_listening(port)? {
            println!("freed port {port} ({} · PID {})", entry.process, entry.pid);
            return Ok(());
        }
        thread::sleep(Duration::from_millis(100));
    }

    Err(format!(
        "sent {} to {} (PID {}), but port {} is still listening",
        if force { "SIGKILL" } else { "SIGTERM" },
        entry.process,
        entry.pid,
        port
    ))
}
