use crate::port::PortInfo;

pub fn print_table(entries: &[PortInfo]) {
    println!(
        "{:<8} {:<8} {:<12} {:<10} {:<10} {:<8} {}",
        "PORT", "PID", "PROCESS", "USER", "UPTIME", "CPU", "MEM"
    );
    for e in entries {
        println!(
            "{:<8} {:<8} {:<12} {:<10} {:<10} {:<8} {}",
            e.port, e.pid, e.process, e.user, e.uptime, e.cpu, e.mem
        );
    }
}
