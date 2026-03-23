use std::process::Command;

fn main() {
    let list_ports = Command::new("lsof")
        .args(["-i", "-P", "-n"])
        .output()
        .expect("failed to get ports");

    match list_ports.status.success() {
        true => {
            print!("{}", String::from_utf8_lossy(&list_ports.stdout));
        }
        false => {
            eprintln!(
                "failed to list ports: {}",
                String::from_utf8_lossy(&list_ports.stderr)
            )
        }
    }
}
