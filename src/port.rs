use crate::output::print_table;
use crate::process::get_process_info;
use std::collections::HashMap;
use std::process::Command;

pub struct PortInfo {
    pub port: String,
    pub address: String,
    pub pid: String,
    pub process: String,
    pub user: String,
    pub uptime: String,
    pub cpu: String,
    pub mem: String,
}

pub fn inspect_all() -> Result<(), String> {
    let mut entries = listeners(None)?;
    enrich_process_info(&mut entries);
    if entries.is_empty() {
        println!("no listening ports found");
    } else {
        print_table(&entries);
    }
    Ok(())
}

pub fn inspect(port: u16) -> Result<(), String> {
    let mut entries = listeners(Some(port))?;
    enrich_process_info(&mut entries);
    if entries.is_empty() {
        println!("nothing listening on port {port}");
    } else {
        print_table(&entries);
    }
    Ok(())
}

pub fn find_by_port(port: u16) -> Result<PortInfo, String> {
    let mut entries = listeners(Some(port))?;
    match entries.len() {
        0 => Err(format!("nothing listening on port {port}")),
        1 => Ok(entries.remove(0)),
        _ => Err(format!(
            "multiple processes listen on port {port}: {}. Stop them by PID before retrying",
            entries
                .iter()
                .map(|entry| entry.pid.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

pub fn is_port_listening(port: u16) -> Result<bool, String> {
    Ok(!listeners(Some(port))?.is_empty())
}

fn listeners(port: Option<u16>) -> Result<Vec<PortInfo>, String> {
    let mut command = Command::new("lsof");
    command.args(["-nP", "-sTCP:LISTEN", "-FpcLfn"]);
    if let Some(port) = port {
        command.arg(format!("-iTCP:{port}"));
    } else {
        command.arg("-iTCP");
    }
    let output = command
        .output()
        .map_err(|e| format!("could not run lsof: {e}"))?;

    // lsof exits with 1 when it finds no matching files.
    let no_matches =
        output.status.code() == Some(1) && output.stdout.is_empty() && output.stderr.is_empty();
    if !(output.status.success() || no_matches) {
        return Err(format!(
            "lsof failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    let mut entries = parse_lsof(&String::from_utf8_lossy(&output.stdout));
    if let Some(port) = port {
        entries.retain(|entry| entry.port == port.to_string());
    }
    Ok(entries)
}

fn enrich_process_info(entries: &mut [PortInfo]) {
    let mut process_info = HashMap::new();
    for entry in entries {
        let info = process_info
            .entry(entry.pid.clone())
            .or_insert_with(|| get_process_info(&entry.pid));
        if let Some(info) = info {
            entry.uptime = info.uptime.clone();
            entry.cpu = info.cpu.clone();
            entry.mem = info.mem.clone();
        }
    }
}

fn parse_lsof(output: &str) -> Vec<PortInfo> {
    let mut entries: Vec<PortInfo> = Vec::new();
    let mut by_process_port = HashMap::new();
    let (mut pid, mut process, mut user) = (String::new(), String::new(), String::new());

    for line in output.lines() {
        let Some((field, value)) = line.split_at_checked(1) else {
            continue;
        };
        match field {
            "p" => {
                pid = value.to_string();
                process.clear();
                user.clear();
            }
            "c" => process = value.to_string(),
            "L" => user = value.to_string(),
            "n" if !pid.is_empty() => {
                let Some((address, port)) = value.rsplit_once(':') else {
                    continue;
                };
                let Ok(port) = port.parse::<u16>() else {
                    continue;
                };
                let key = (pid.clone(), port);
                if let Some(index) = by_process_port.get(&key).copied() {
                    let entry: &mut PortInfo = &mut entries[index];
                    if !entry.address.split(',').any(|part| part == address) {
                        entry.address.push(',');
                        entry.address.push_str(address);
                    }
                } else {
                    by_process_port.insert(key, entries.len());
                    entries.push(PortInfo {
                        port: port.to_string(),
                        address: address.to_string(),
                        pid: pid.clone(),
                        process: process.clone(),
                        user: if user.is_empty() {
                            "-".into()
                        } else {
                            user.clone()
                        },
                        uptime: "-".into(),
                        cpu: "-".into(),
                        mem: "-".into(),
                    });
                }
            }
            _ => {}
        }
    }

    entries.sort_by_key(|entry| (entry.port.parse::<u16>().unwrap_or(0), entry.pid.clone()));
    entries
}

#[cfg(test)]
mod tests {
    use super::parse_lsof;

    #[test]
    fn groups_duplicate_listeners_and_preserves_addresses() {
        let output = "p42\ncNode Server\nLlucas\nf1\nn*:3000\nf2\nn*:3000\nf3\nn127.0.0.1:3000\np7\ncother\nLroot\nf4\nn[::1]:8080\n";
        let entries = parse_lsof(output);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].pid, "42");
        assert_eq!(entries[0].process, "Node Server");
        assert_eq!(entries[0].address, "*,127.0.0.1");
        assert_eq!(entries[0].port, "3000");
        assert_eq!(entries[1].address, "[::1]");
    }

    #[test]
    fn skips_names_without_numeric_ports() {
        let entries = parse_lsof("p42\ncservice\nn*:*\nnnot-a-socket\n");
        assert!(entries.is_empty());
    }
}
