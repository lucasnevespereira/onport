use crate::port::PortInfo;
use colored::Colorize;

pub fn print_table(entries: &[PortInfo]) {
    println!(
        "{:<8} {:<8} {:<12} {:<10} {:<10} {:<8} {}",
        "PORT".bold(),
        "PID".bold(),
        "PROCESS".bold(),
        "USER".bold(),
        "UPTIME".bold(),
        "CPU".bold(),
        "MEM".bold()
    );
    for e in entries {
        let cpu = colorize_cpu(&e.cpu, 8);
        let mem = colorize_mem(&e.mem);
        println!(
            "{:<8} {:<8} {:<12} {:<10} {:<10} {:<8} {}",
            e.port, e.pid, e.process, e.user, e.uptime, cpu, mem
        );
    }
}

fn colorize_cpu(cpu: &str, width: usize) -> String {
    let padded = format!("{:<width$}", cpu);
    let val: f64 = cpu.trim_end_matches('%').parse().unwrap_or(0.0);
    if val > 50.0 {
        padded.red().to_string()
    } else if val > 20.0 {
        padded.yellow().to_string()
    } else {
        padded.green().to_string()
    }
}

fn colorize_mem(mem: &str) -> String {
    let val: u64 = mem.trim_end_matches("MB").parse().unwrap_or(0);
    if val > 512 {
        mem.red().to_string()
    } else if val > 256 {
        mem.yellow().to_string()
    } else {
        mem.green().to_string()
    }
}
