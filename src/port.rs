use crate::output::print_table;
use crate::process::get_process_info;
use std::process::Command;

pub struct PortInfo {
    pub port: String,
    pub pid: String,
    pub process: String,
    pub user: String,
    pub uptime: String,
    pub cpu: String,
    pub mem: String,
}

const COL_COMMAND: usize = 0;
const COL_PID: usize = 1;
const COL_USER: usize = 2;
const COL_NAME: usize = 8; // NAME is the last column in lsof output
const MIN_COLUMNS: usize = COL_NAME + 1;

pub fn inspect_all() -> Result<(), String> {
    let output = Command::new("lsof")
        .args(["-i", "-P", "-n"])
        .output()
        .map_err(|e| format!("inspect_all: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut entries = parse_lsof(&stdout);
    for entry in &mut entries {
        if let Some(info) = get_process_info(&entry.pid) {
            entry.uptime = info.uptime;
            entry.cpu = info.cpu;
            entry.mem = info.mem;
        }
    }

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
    let mut entries = parse_lsof(&stdout);
    for entry in &mut entries {
        if let Some(info) = get_process_info(&entry.pid) {
            entry.uptime = info.uptime;
            entry.cpu = info.cpu;
            entry.mem = info.mem;
        }
    }

    if entries.is_empty() {
        println!("nothing running on port {}", port);
    } else {
        print_table(&entries);
    }
    Ok(())
}

pub fn find_by_port(port: u16) -> Result<PortInfo, String> {
    let formatted_port = format!(":{}", port);
    let output = Command::new("lsof")
        .args(["-i", &formatted_port, "-P", "-n"])
        .output()
        .map_err(|e| format!("find_by_port: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let entries = parse_lsof(&stdout);

    entries
        .into_iter()
        .next()
        .ok_or_else(|| format!("nothing running on port {}", port))
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
                process: cols[COL_COMMAND].to_string(),
                pid: cols[COL_PID].to_string(),
                user: cols[COL_USER].to_string(),
                port: port.to_string(),
                uptime: String::new(),
                cpu: String::new(),
                mem: String::new(),
            })
        })
        .collect()
}
