import time


def multiply_values(values, factor):
    return [x * factor for x in values]


def save_to_md(inputs, outputs, exec_time):
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
        file.write(f"Execution time: **{exec_time:.6f} seconds**\n")


if __name__ == "__main__":
    input_values = [10.0, 20.0, 30.0, 40.0, 50.0]

    start = time.time()
    output_values = multiply_values(input_values, 2.0)  # Multiply each element by 2.0
    end = time.time()

    save_to_md(input_values, output_values, end - start)
    print("Execution time:", end - start)
