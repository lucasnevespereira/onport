use std::io::{self, Write};
use std::process::Command;

use crate::port::find_by_port;

pub fn kill(port: u16) -> Result<(), String> {
    let entry = find_by_port(port)?;

    print!(
        "Kill {} (PID {}) on port {}? [y/N] ",
        entry.process, entry.pid, port
    );
    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    if input.trim().to_lowercase() != "y" {
        println!("aborted");
        return Ok(());
    }

    Command::new("kill")
        .arg(&entry.pid)
        .output()
        .map_err(|e| format!("kill: {}", e))?;

    println!("killed {} (PID {})", entry.process, entry.pid);
    Ok(())
}
