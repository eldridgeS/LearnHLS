# **My Hardware Acceleration & Compiler Tools Projects**

This repository showcases a collection of projects exploring various aspects of hardware acceleration using Xilinx Vitis HLS and compiler toolchain development with Rust. It includes demonstrations of High-Level Synthesis (HLS) for image processing and matrix multiplication, alongside a custom tool for analyzing LLVM Intermediate Representation (IR).

## **1\. Vitis HLS Fixed-Point Matrix Multiplication**

**Description:** This project demonstrates fixed-point matrix multiplication accelerated using Vitis HLS, focusing on how HLS directives (pragmas) optimize C++ code for efficient hardware synthesis on FPGAs.  
**Key Technologies/Concepts:** Vitis HLS, Fixed-Point Arithmetic (ap\_fixed), HLS Pragmas (PIPELINE, UNROLL, ARRAY\_PARTITION, DATAFLOW), DSP utilization, Latency, Initiation Interval (II).  
**Location:** matrix\_multiplication\_project/ 

## **2\. Box Blur Filter HLS Optimization Project**

**Description:** This project implements and optimizes a Box Blur image filter using Vitis HLS, showcasing iterative hardware optimization techniques and demonstrating integration with a Rust host application via Foreign Function Interface (FFI).  
**Key Technologies/Concepts:** Vitis HLS, Image Processing, On-Chip Memory (Line Buffer, ARRAY\_PARTITION), AXI4-Lite Interface, Rust FFI, C/C++ Shared Libraries.  
**Location:** box\_blur\_app/ 

## **3\. LLVM IR Analysis Tool**

**Description:** This Rust-based command-line tool helps users understand LLVM Intermediate Representation (IR) by counting instruction opcodes, displaying function source, and providing line-by-line explanations of common LLVM IR instructions.  
**Key Technologies/Concepts:** Rust, LLVM IR, regex crate, llvm-ir crate, Compiler Toolchain, Debug Information (\!dbg).  
**Location:** llvm\_ir\_tool/ 

## **4\. HLS Component Simulation with Rust & C++ FFI**

**Description:** This project demonstrates dynamic loading and Foreign Function Interface (FFI) interactions between Rust and C++, simulating a simplified High-Level Synthesis (HLS) component. It illustrates key concepts of dynamic linking, symbol resolution, and C++ name mangling, while mimicking HLS simulation logic for latency and BRAM estimation.
**Key Technologies/Concepts:** Rust, C++, FFI (libloading), Dynamic Linking, Symbol Resolution, Name Mangling (extern "C"), GNU Binutils (nm), Simulated HLS Concepts (Latency, BRAM, Deadlocks), Error Propagation.
**Location:** hls\_sim\_loader/ 

Each project resides in its own subdirectory and contains a more detailed README.md with specific instructions for setup, compilation, and usage. Please refer to those individual READMEs for in-depth information.
