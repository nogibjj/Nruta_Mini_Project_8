import time
import psutil


def multiply_values(values, factor):
    return [x * factor for x in values]


def save_to_md(inputs, outputs, execution_time, mem_use, cpu_use):
    # Write the markdown table to a file
    with open("python_performance_report.md", "w", encoding="utf-8") as file:
        file.write("# Performance Report\n")
        file.write("## Output Values\n")
        file.write(
            "These are the output values when the input values are multiplied by **2**:\n\n"
        )
        file.write("| Input Value | Output Value |\n")
        file.write("|-------------|--------------|\n")

        # Write input and output values to the markdown table
        for input_val, output_val in zip(inputs, outputs):
            file.write(f"| {input_val:.2f} | {output_val:.2f} |\n")

        file.write("## Execution Time\n")
        file.write(f"Execution time: **{execution_time:.6f} seconds**\n")
        file.write("## Resource Usage\n")
        file.write(f"Memory usage: **{mem_use:.2f} MB**\n")
        file.write(f"CPU usage: **{cpu_use:.2f}%**\n")


if __name__ == "__main__":
    input_values = [10.0, 20.0, 30.0, 40.0, 50.0]
    start = time.time()

    # Get initial CPU and memory usage
    process = psutil.Process()
    initial_memory = process.memory_info().rss / (1024**2)  # Convert to MB
    initial_cpu = process.cpu_percent(interval=None)  # Get CPU percent

    result = multiply_values(input_values, 2.0)

    end = time.time()

    # Get final CPU and memory usage
    final_memory = process.memory_info().rss / (1024**2)
    final_cpu = process.cpu_percent(interval=None)

    # Calculate total execution time and resource usage
    exec_time = end - start
    mem_usage = final_memory - initial_memory
    cpu_usage = final_cpu - initial_cpu

    print(result[:5])
    print("Execution time:", exec_time)
    print("Memory usage:", mem_usage)
    print("CPU usage:", cpu_usage)

    # Save performance report to markdown
    save_to_md(input_values, result, exec_time, mem_usage, cpu_usage)
