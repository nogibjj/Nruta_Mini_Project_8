use std::fs::File;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use sysinfo::{CpuExt, System, SystemExt};

fn save_to_md(
    input: &[f64],
    output: &[f64],
    exec_time: &std::time::Duration,
    mem_usage: u64,
    cpu_usage: f32,
    initial_cpu_usage: f32,
) -> io::Result<()> {
    let mut file = File::create("rust_performance_report.md")?;

    // Prepare the markdown content
    writeln!(file, "# Performance Report")?;
    writeln!(file, "## Output Values")?;
    writeln!(
        file,
        "These are the output values when the input values are multiplied by **2**:\n"
    )?;
    writeln!(
        file,
        "| Input Value | Output Value |\n|-------------|--------------|"
    )?;

    for (&input_val, &output_val) in input.iter().zip(output.iter()) {
        writeln!(file, "| {:.2} | {:.2} |", input_val, output_val)?;
    }

    writeln!(file, "## Execution Time")?;
    writeln!(file, "Execution time: **{:?}**", exec_time)?;
    writeln!(file, "## Resource Usage")?;
    writeln!(file, "Memory Usage: **{} bytes**", mem_usage)?;
    writeln!(file, "Initial CPU Usage: **{}%**", initial_cpu_usage)?; // Add initial CPU usage
    writeln!(file, "Final CPU Usage: **{}%**", cpu_usage)?; // Rename to Final CPU Usage

    Ok(())
}

fn multiply_values(input: &[f64], multiplier: f64) -> Vec<f64> {
    input.iter().map(|&x| x * multiplier).collect()
}

fn main() {
    let input_values = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    let memory_before = get_memory_usage(); // Replace with your actual function if different
    let start = Instant::now();

    let mut system = System::new(); // Use new() instead of new_all()
    system.refresh_all(); // Get initial system info

    // Get initial CPU usage
    let initial_cpu_usage = system.global_cpu_info().cpu_usage(); // Directly access cpu_usage

    let result = multiply_values(&input_values, 2.0);

    let duration = start.elapsed();
    let memory_after = get_memory_usage(); // Replace with your actual function if different
    let memory_usage = memory_after - memory_before;

    // Pause for a short duration to allow for a proper measurement
    std::thread::sleep(Duration::from_secs(1));

    // Refresh system info again to get final CPU usage
    system.refresh_all();
    let final_cpu_usage = system.global_cpu_info().cpu_usage(); // Directly access cpu_usage

    if let Err(e) = save_to_md(
        &input_values,
        &result,
        &duration,
        memory_usage,
        final_cpu_usage,
        initial_cpu_usage,
    ) {
        eprintln!("Error saving to markdown: {}", e);
    }
}

// Implement the function to get memory usage if needed
fn get_memory_usage() -> u64 {
    // Implement your logic to get memory usage here, e.g., using sysinfo or other methods.
    0 // Placeholder, replace with actual logic
}
