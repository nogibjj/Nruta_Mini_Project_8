# IDS 706 Week 8 Mini Project - Rewrite a Python Script in Rust

## 🏗️ Requirements
- Take an existing Python script for data processing
- Rewrite it in Rust
- Highlight improvements in speed and resource usage

## 📂 Project Structure
```
.
├── Cargo.lock
├── Cargo.toml
├── Makefile
├── README.md
├── benches
│   └── benchmark.rs
├── main_python.py
├── python_performance_report.md
├── requirements.txt
├── rust_performance_report.md
├── src
│   └── main.rs
└── test_main.py
```

## 🛠️ Setup Instructions
1. Clone the repository:
```
git clone https://github.com/nogibjj/Nruta_Mini_Project_8
```

2. Navigate to the project directory:
```
cd Nruta_Mini_Project_8
```

3. Set up Python dependencies:
- Ensure you have Python installed. You can download it from python.org.
- Install the required Python dependencies:
```
pip install -r requirements.txt
```

4. Set up Rust dependencies:
- Ensure you have Rust installed. You can download it from rust-lang.org.
- Build the Rust project and install dependencies:
```
cargo build --release
```

5. Run the Project
- For Python, use the following command:
```
python main_python.py
```

- For Rust, use the following command:
```
cargo run
```

## 📦 Comparing Python and Rust based on speed and resource usage
To compare the speed and resource usage between Python and Rust, a simple multiplication function has been set up. The input values are 10, 20, 30, 40 and 50 and these numbers are being multiplied by 2.

### General Results
These are the output values when the input values are multiplied by **2**:

| Input Value | Output Value |
|-------------|--------------|
| 10.00 | 20.00 |
| 20.00 | 40.00 |
| 30.00 | 60.00 |
| 40.00 | 80.00 |
| 50.00 | 100.00 |

### Python performance
#### Execution Time
Execution time: **0.000155 seconds**

#### Resource Usage
Memory usage: **0.02 MB**
CPU usage: **1.90%**

### Rust performance
#### Execution Time
Execution time: **5.220958ms**
#### Resource Usage
Memory Usage: **0 bytes**
Initial CPU Usage: **9.959038%**
Final CPU Usage: **11.5671%**

From the performance analysis of both Python and Rust implementations, we observe the following key points:

1.	Execution Time:
The execution time for the Rust implementation is significantly lower than that of the Python implementation. Rust completed the task in 5.22 milliseconds, while Python took 0.000155 seconds (which converts to 0.155 milliseconds). This indicates that Rust offers superior performance in terms of speed for the given task.

2.	Resource Usage:
- Memory Usage: The memory usage for the Python implementation is measured at 0.02 MB, whereas the Rust implementation reports 0 bytes. This discrepancy suggests that Rust may handle memory more efficiently for this particular task, potentially due to its lack of garbage collection and more direct memory management.
- CPU Usage: The initial CPU usage for Rust was 9.96%, which increased to 11.57% by the end of execution. In comparison, Python maintained a CPU usage of 1.90% throughout its execution. This suggests that while Rust may utilize more CPU resources, it does so efficiently, leading to faster execution times.


These results have been rendered in [Rust Performance](rust_performance_report.md) and [Python Performance](python_performance_report.md)