use std::process::Command;

pub fn inspect_all() -> Result<(), String> {
    let output = Command::new("lsof")
        .args(["-i", "-P", "-n"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("inspect_all: {}", error_msg.trim()));
    }

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

pub fn inspect(port: u16) -> Result<(), String> {
    let formatted_port = format!(":{}", port);
    let output = Command::new("lsof")
        .args(["-i", &formatted_port, "-P", "-n"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("inspect: {}", error_msg.trim()));
    }

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
