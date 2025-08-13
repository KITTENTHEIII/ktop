use sysinfo::System;
use crossterm::{
    ExecutableCommand,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};
use std::io::{stdout, Write};

fn main() {
    // Create a System object to access system and process info
    let mut sys = System::new_all();

    // Refresh system information (CPU, memory, processes)
    sys.refresh_all();

    // Get all processes
    let processes = sys.processes();

    // Clear terminal and move cursor to top-left
    stdout()
        .execute(Clear(ClearType::All))
        .unwrap()
        .execute(MoveTo(0, 0))
        .unwrap();

    // Print table header
    println!("PID\tCPU%\tMemory(MB)\tName");

    // Collect processes into a vector and sort by CPU usage (descending)
    let mut processes_sorted: Vec<_> = processes.values().collect();
    processes_sorted.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());

    // Display top 20 processes
    for process in processes_sorted.iter().take(20) {
        let pid = process.pid();
        let cpu = process.cpu_usage();
        let mem = process.memory() / 1024; // Convert KB to MB

        // Convert OsStr to String safely for display
        let name = process.name().to_string_lossy();

        println!("{:<8}{:<8.2}{:<12}{}", pid, cpu, mem, name);
    }

    // Flush stdout to make sure all output is printed
    stdout().flush().unwrap();
}
