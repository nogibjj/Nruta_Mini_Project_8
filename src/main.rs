use std::fs::File;
use std::io::{self, Write};
use std::time::Instant; // Importing the time module

fn multiply_values(values: &[f64], factor: f64) -> Vec<f64> {
    values.iter().map(|&x| x * factor).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_values() {
        let values = vec![10.0, 20.0, 30.0];
        let factor = 2.0;
        let expected = vec![20.0, 40.0, 60.0];
        assert_eq!(multiply_values(&values, factor), expected);
    }

    #[test]
    fn test_multiply_by_zero() {
        let values = vec![10.0, 20.0, 30.0];
        let factor = 0.0;
        let expected = vec![0.0, 0.0, 0.0];
        assert_eq!(multiply_values(&values, factor), expected);
    }

    #[test]
    fn test_empty_list() {
        let values: Vec<f64> = vec![];
        let factor = 2.0;
        let expected: Vec<f64> = vec![];
        assert_eq!(multiply_values(&values, factor), expected);
    }
}

fn save_to_md(result: &[f64], exec_time: &std::time::Duration) -> io::Result<()> {
    let mut file = File::create("rust_performance_report.md")?;
    
    // Prepare the markdown content
    writeln!(file, "# Performance Report")?;
    writeln!(file, "## Output")?;
    writeln!(file, "| Value |")?;
    writeln!(file, "|-------|")?;
    
    // Write the first few results to the markdown table
    for &value in &result[0..result.len().min(5)] { // Change this slice if you want more or fewer results
        writeln!(file, "| {:.2} |", value)?;
    }
    
    writeln!(file, "## Execution Time")?;
    writeln!(file, "Execution time: **{:?}**", exec_time)?;
    
    Ok(())
}

fn main() {
    let values = vec![10.0, 20.0, 30.0, 40.0, 50.0]; // Replace with a large array for testing

    // Start the timer
    let start = Instant::now();

    let result = multiply_values(&values, 2.0); // Multiply each by 2.0

    // Stop the timer
    let duration = start.elapsed(); // Calculate elapsed time

    // Print the results
    println!("Result: {:?}", result); // Verify result
    println!("Execution time: {:?}", duration); // Print execution time

    // Save the results to a markdown file
    if let Err(e) = save_to_md(&result, &duration) {
        eprintln!("Error writing to markdown file: {}", e);
    }
}
