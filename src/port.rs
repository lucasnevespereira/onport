use std::process::Command;

struct PortInfo {
    port: String,
    pid: String,
    process: String,
    user: String,
}

// NAME is the last column in lsof output
const COL_NAME: usize = 8;
const MIN_COLUMNS: usize = COL_NAME + 1;

pub fn inspect_all() -> Result<(), String> {
    let output = Command::new("lsof")
        .args(["-i", "-P", "-n"])
        .output()
        .map_err(|e| format!("inspect_all: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let entries = parse_lsof(&stdout);
    if entries.is_empty() {
        println!("no open ports found");
    } else {
        print_table(&entries);
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
    let entries = parse_lsof(&stdout);
    if entries.is_empty() {
        println!("nothing running on port {}", port);
    } else {
        print_table(&entries);
    }
    Ok(())
}

fn parse_lsof(output: &str) -> Vec<PortInfo> {
    output
        .lines()
        .skip(1) // skip header
        .filter_map(|line| {
            let cols: Vec<&str> = line.split_whitespace().collect();
            if cols.len() < MIN_COLUMNS {
                return None;
            }
            let name = cols[COL_NAME];
            let port = name.rsplit(':').next()?;
            Some(PortInfo {
                process: cols[0].to_string(),
                pid: cols[1].to_string(),
                user: cols[2].to_string(),
                port: port.to_string(),
            })
        })
        .collect()
}

fn print_table(entries: &[PortInfo]) {
    println!("{:<8} {:<8} {:<12} {}", "PORT", "PID", "PROCESS", "USER");
    for e in entries {
        println!("{:<8} {:<8} {:<12} {}", e.port, e.pid, e.process, e.user);
    }
}
