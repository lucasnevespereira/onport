use crate::port::PortInfo;
use colored::Colorize;

// CPU usage thresholds (percentage)
const CPU_HIGH: f64 = 50.0;
const CPU_WARN: f64 = 20.0;

// Memory usage thresholds (MB)
const MEM_HIGH: u64 = 512;
const MEM_WARN: u64 = 256;

pub fn print_table(entries: &[PortInfo]) {
    if entries.is_empty() {
        return;
    }

    let w_port = entries
        .iter()
        .map(|e| e.port.len())
        .max()
        .unwrap_or(0)
        .max("PORT".len());
    let w_pid = entries
        .iter()
        .map(|e| e.pid.len())
        .max()
        .unwrap_or(0)
        .max("PID".len());
    let w_proc = entries
        .iter()
        .map(|e| e.process.len())
        .max()
        .unwrap_or(0)
        .max("PROCESS".len());
    let w_user = entries
        .iter()
        .map(|e| e.user.len())
        .max()
        .unwrap_or(0)
        .max("USER".len());
    let w_up = entries
        .iter()
        .map(|e| e.uptime.len())
        .max()
        .unwrap_or(0)
        .max("UPTIME".len());
    let w_cpu = entries
        .iter()
        .map(|e| e.cpu.len())
        .max()
        .unwrap_or(0)
        .max("CPU".len());

    println!(
        "{:<w_port$}  {:<w_pid$}  {:<w_proc$}  {:<w_user$}  {:<w_up$}  {:<w_cpu$}  {}",
        "PORT".bold(),
        "PID".bold(),
        "PROCESS".bold(),
        "USER".bold(),
        "UPTIME".bold(),
        "CPU".bold(),
        "MEM".bold()
    );
    for e in entries {
        let cpu = colorize_cpu(&e.cpu, w_cpu);
        let mem = colorize_mem(&e.mem);
        println!(
            "{:<w_port$}  {:<w_pid$}  {:<w_proc$}  {:<w_user$}  {:<w_up$}  {}  {}",
            e.port, e.pid, e.process, e.user, e.uptime, cpu, mem
        );
    }
}

fn colorize_cpu(cpu: &str, width: usize) -> String {
    let padded = format!("{:<width$}", cpu);
    let val: f64 = cpu.trim_end_matches('%').parse().unwrap_or(0.0);
    if val > CPU_HIGH {
        padded.red().to_string()
    } else if val > CPU_WARN {
        padded.yellow().to_string()
    } else {
        padded.green().to_string()
    }
}

fn colorize_mem(mem: &str) -> String {
    let val: u64 = mem.trim_end_matches("MB").parse().unwrap_or(0);
    if val > MEM_HIGH {
        mem.red().to_string()
    } else if val > MEM_WARN {
        mem.yellow().to_string()
    } else {
        mem.green().to_string()
    }
}
