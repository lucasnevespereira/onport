use std::process::Command;

const COL_ETIME: usize = 0;
const COL_CPU: usize = 1;
const COL_RSS: usize = 2;
const MIN_PS_COLUMNS: usize = COL_RSS + 1;
const KB_PER_MB: u64 = 1024;

pub struct ProcessInfo {
    pub uptime: String,
    pub cpu: String,
    pub mem: String,
}

pub fn get_process_info(pid: &str) -> Option<ProcessInfo> {
    let output = Command::new("ps")
        .args(["-p", pid, "-o", "etime=,pcpu=,rss="])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let cols: Vec<&str> = stdout.trim().split_whitespace().collect();
    if cols.len() < MIN_PS_COLUMNS {
        return None;
    }

    Some(ProcessInfo {
        uptime: cols[COL_ETIME].to_string(),
        cpu: format!("{}%", cols[COL_CPU]),
        mem: format!(
            "{}MB",
            cols[COL_RSS].parse::<u64>().unwrap_or(0) / KB_PER_MB
        ),
    })
}
