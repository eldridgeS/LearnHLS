#include "sim_component.h"
#include <iostream> 
#include <string> // For string operations

// Simple BRAM calculation logic to mimic fifo::get_bram_count
//https://github.com/sharc-lab/LightningSim/blob/main/backend/lightningsim-core/fifo.rs
int calculate_bram_count(int width, int depth) {
    // Very simplified: assuming 1 BRAM per 1024 bits (128 bytes) of capacity
    // and a minimum of 1 BRAM if depth > 0
    if (depth == 0) return 0;
    int bits = width * depth;
    return (bits + 1023) / 1024; // 1025 bits = 2 BRAMs
}

// Simple calculation function (Test Case 1) to mimic bram usage and latency calculation
SimResult simulate_hls_module(FifoId fifo_id, int fifo_depth, AxiAddress axi_addr, ClockCycle axi_delay) {
    std::cout << "C++ Component: simulate_hls_module called with:" << std::endl;
    std::cout << "  FIFO ID: " << fifo_id << ", Depth: " << fifo_depth << std::endl;
    std::cout << "  AXI Address: " << std::hex << axi_addr << ", Delay: " << axi_delay << std::endl;

    SimResult result = {0, 0, 0}; // Initialize result struct

    // Simulate DeadlockDetected (Test Case 2): If FIFO depth is 0, consider it a deadlock
    if (fifo_depth <= 0) {
        std::cerr << "C++ Component: Error - DeadlockDetected due to zero or negative FIFO depth!" << std::endl;
        result.error_code = 1; // Arbitrary error code for deadlock
        result.latency = 0; // Indicate no valid latency
        result.bram_count = 0;
        return result;
    }

    // Simulate FifoDepthNotProvided (Test Case 3)
    if (fifo_depth == -1) { // Use -1 as a special "not provided" indicator
        std::cerr << "C++ Component: Error - FifoDepthNotProvided for FIFO ID " << fifo_id << "!" << std::endl;
        result.error_code = 2;
        result.latency = 0;
        result.bram_count = 0;
        return result;
    }
    
    //Simulate AxiDelayNotProvided (Test Case 4)
    if (axi_delay == 0) { // Use 0 as a special "not provided" indicator for delay
        std::cerr << "C++ Component: Error - AxiDelayNotProvided for AXI Address " << std::hex << axi_addr << "!" << std::endl;
        result.error_code = 3;
        result.latency = 0;
        result.bram_count = 0;
        return result;
    }

    // Basic latency calculation which increases with FIFO depth and AXI delay
    result.latency = (ClockCycle)fifo_depth * 10 + axi_delay * 5;

    // Assume a fixed FIFO width for this component, e.g., 32 bits
    result.bram_count = calculate_bram_count(32, fifo_depth);

    std::cout << "C++ Component: Simulation successful. Latency: " << result.latency
              << ", BRAMs: " << result.bram_count << std::endl;
    return result;
}

#ifdef __cplusplus //only compiled if C++ compiler

int cpp_mangled_function(int x, float y) {
// This function will be name-mangled
    std::cout << "C++ Component: cpp_mangled_function(" << x << ", " << y << ") called." << std::endl;
    return x + static_cast<int>(y);
}
#endif
