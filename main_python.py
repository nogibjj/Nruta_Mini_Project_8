import time


def multiply_values(values, factor):
    return [x * factor for x in values]


def save_to_md(result, exec_time):
    markdown_content = "# Performance Report\n\n"
    markdown_content += "## Output\n\n"
    markdown_content += "| Value |\n"
    markdown_content += "|-------|\n"

    for value in result[:5]:
        markdown_content += f"| {value} |\n"

    markdown_content += "\n## Execution Time\n"
    markdown_content += f"Execution time: **{exec_time:.6f} seconds**\n"

    # Write to a markdown file
    with open("performance_report.md", "w") as file:
        file.write(markdown_content)


if __name__ == "__main__":
    input_values = [10.0] * 1_000_000

    start = time.time()
    result = multiply_values(input_values, 2.0)  # Multiply each element by 2.0
    end = time.time()

    print(result[:5])
    print("Execution time:", end - start)
