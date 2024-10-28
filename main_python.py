import time


def multiply_values(values, factor):
    return [x * factor for x in values]


if __name__ == "__main__":
    input_values = [10.0] * 1_000_000

    start = time.time()
    result = multiply_values(input_values, 2.0)  # Multiply each element by 2.0
    end = time.time()

    print(result[:5])  # Display first few results for verification
    print("Execution time:", end - start)
