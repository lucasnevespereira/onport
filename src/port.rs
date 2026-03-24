use std::process::Command;

pub fn list_all() {
    let output = Command::new("lsof")
        .args(["-i", "-P", "-n"])
        .output()
        .expect("failed to execute lsof");

    match output.status.success() {
        true => {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
        false => {
            eprintln!(
                "failed to list ports: {}",
                String::from_utf8_lossy(&output.stderr)
            )
        }
    }
}
