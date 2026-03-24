use std::process::Command;

pub fn inspect_all() -> Result<(), String> {
    let output = Command::new("lsof")
        .args(["-i", "-P", "-n"])
        .output()
        .map_err(|e| format!("inspect_all: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        println!("no open ports found");
    } else {
        println!("{}", stdout);
    }
    Ok(())
}

pub fn inspect(port: u16) -> Result<(), String> {
    let formatted_port = format!(":{}", port);
    let output = Command::new("lsof")
        .args(["-i", &formatted_port, "-P", "-n"])
        .output()
        .map_err(|e| format!("inspect: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        println!("nothing running on port {}", port);
    } else {
        println!("{}", stdout);
    }
    Ok(())
}
