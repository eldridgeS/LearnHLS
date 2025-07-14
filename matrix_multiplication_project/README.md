# Vitis HLS Fixed-Point Matrix Multiplication

This repository contains a Vitis HLS project for fixed-point matrix multiplication. The goal is to demonstrate how High-Level Synthesis (HLS) can be used to accelerate C++ code into efficient hardware, specifically focusing on fixed-point arithmetic and the impact of various HLS optimization pragmas.

## 1. Project Overview
This project implements a fixed-point matrix multiplication for 4x4 matrices. Fixed-point arithmetic is chosen for its balance between precision and hardware resource efficiency, making it suitable for FPGA implementations. The design is intended to be synthesized into hardware using Xilinx Vitis HLS.
**Matrix Dimensions:**
- X = 4 (Rows of Matrix A, Rows of Result Matrix C)
- Y = 4 (Columns of Matrix A, Rows of Matrix B)
- Z = 4 (Columns of Matrix B, Columns of Result Matrix C)

**Fixed-Point Precision:**
typedef ap_fixed<16, 8> fixed_digit;
This defines a 16-bit fixed-point number with 8 integer bits and 8 fractional bits.

## 2. Key Concepts
- Fixed-Point Data Types (ap_fixed): Vitis HLS provides the ap_fixed data type for arbitrary precision fixed-point numbers. This allows designers to specify the total bit width and the number of integer bits, enabling fine-grained control over precision and dynamic range, which directly impacts hardware resource usage.
- HLS Directives (Pragmas): These are special commands (#pragma HLS ...) embedded in the C++ code that guide the Vitis HLS compiler in optimizing the design for hardware. Common directives include:
  - PIPELINE: Enables parallel execution of loop iterations.
  - UNROLL: Expands a loop into parallel hardware, eliminating loop control overhead.
  - ARRAY_PARTITION: Divides large arrays into smaller arrays or individual registers to enable parallel memory access.
  - DATAFLOW: Enables task-level pipelining, allowing independent functions or loop nests to run concurrently and stream data.
Test Bench: A C++ test bench is essential for verifying the functional correctness of the HLS design. It simulates inputs, calls the HLS function, and compares the hardware-simulated outputs against a known good software reference.

## 3. Code Structure
The project consists of three main C++ files:
a. Design (matrix_mult.cpp)
This file contains the core C++ function (matrix_mult) that performs the fixed-point matrix multiplication. It is the target for HLS synthesis.
b. Header (matrix_mult.h)
This header file defines the fixed-point type, matrix dimensions, and the function prototype for matrix_mult, ensuring consistency across the design and test bench files.
c. Test Bench (matrix_mult_tb.cpp)
The test bench initializes input matrices, calls the matrix_mult function, performs a software reference calculation, and then compares the hardware-simulated results with the software reference to verify correctness.

## 4. Performance Analysis
We compared three different configurations of the matrix_mult.cpp design, analyzing their synthesis reports (csynth.rpt) for performance (Latency, II) and resource utilization (DSPs, FFs, LUTs).
### Configuration 1: No Pragmas (Baseline)
Observations from Report:
Overall Latency: 39 cycles (312 ns)
row_loop_col_loop: Pipelined with II=2.
Resources: 4 DSPs, 137 FFs, 318 LUTs, 0 BRAMs.
Analysis: Vitis HLS intelligently applied default optimizations, including pipelining the outer loops and inferring parallelism (using 4 DSPs), resulting in a surprisingly good baseline performance. Arrays were mapped to AP_MEMORY (likely BRAM-like structures with limited ports).
### Configuration 2: Array Partition + Inner Loop Pipeline
![Array Partition](images/ap_uncommented.png) 
![Inner Loop Pipeline](images/pipeline_uncommented.png) 
Observations from Report:
Overall Latency: 177 cycles (1416 ns)
product_loop: Pipelined with II=1.
row_loop_col_loop: Not pipelined (sequential execution of outer loops).
Resources: 1 DSP, 182 FFs, 824 LUTs, 0 BRAMs.
Analysis: While ARRAY_PARTITION complete successfully mapped arrays to registers and product_loop achieved II=1, the overall latency significantly increased. This happened because explicitly pipelining only the innermost loop prevented Vitis HLS from applying its more effective default parallelism to the outer loops. The design became largely sequential at the outer loop level, losing the benefit of the 4 parallel DSPs seen in the baseline.
### Configuration 3: Array Partition + Dataflow + Outer Loop Pipeline
![Dataflow](images/dataflow.png) 
![Unroll](images/unroll.png) 
Observations from Report:
Overall Latency: - (Dataflow - streaming)
col_loop: Pipelined with II=1.
Resources: 13 DSPs, 4402 FFs, 3241 LUTs, 0 BRAMs.
Analysis: This configuration achieved the highest throughput. The DATAFLOW pragma enabled task-level pipelining, allowing different parts of the matrix multiplication to execute concurrently and stream data. The col_loop achieving an II=1 means a new output element calculation can start every clock cycle. The dramatic increase to 13 DSPs confirms that Vitis HLS instantiated significant parallel hardware, making this the fastest design for continuous data processing. This performance comes at a much higher resource cost.
Summary Comparison Table
![Comparison](images/comparison.png) 

## Conclusion on Performance
- The "AP + Dataflow + Outer Pipe" configuration (Configuration 3) is the fastest in terms of throughput for continuous matrix multiplication. While its single-run latency is not directly reported (due to the streaming nature of DATAFLOW), its ability to start a new column computation every clock cycle (II=1 for col_loop) and its high degree of parallelism (13 DSPs) means it can process a stream of matrices much faster than the other configurations.
![Configuration 3](images/ap_dataflow_outer_loop_pipeline.png) 
- The "No Pragmas" baseline (Configuration 1) performed surprisingly well due to Vitis HLS's intelligent default optimizations. 
![Configuration 1](images/no_pragma.png) 
- Configuration 2, by explicitly optimizing only the inner loop, inadvertently removed the higher-level parallelism that HLS was inferring by default, leading to worse overall latency.
![Configuration 2](images/ap_inner_loop_pipeline.png) 


## Bonus: Configuration 3 with ap_fixed<32, 16>
Out of curiosity, I increased the precision of the fixed point numbers from ap_fixed<16, 8> to ap_fixed<32, 16>
![Change ap_fixed](images/change_ap_fixed.png) 
The synthesis report showed a significant increase in resource utilization compared to the previous ap_fixed<16, 8> version, while maintaining the same high throughput. The number of DSPs used jumped from 13 to 39, and FFs increased from 4402 to 8369, with LUTs rising from 3241 to 4694. This is expected, as processing data with double the bit-width requires more complex and larger arithmetic units and wider registers. Crucially, the design successfully maintains its DATAFLOW characteristic (latency still -) and the col_loop still achieves an II=1, indicating that the increased precision did not compromise the high throughput and parallelism achieved by the DATAFLOW and PIPELINE pragmas. This demonstrates the direct relationship between data precision and hardware resource consumption in HLS, where higher precision translates to more demanding hardware.
![ap fixed 32 16](images/ap_fixed_32_16.png) 
