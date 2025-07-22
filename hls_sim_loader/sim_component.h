#ifndef SIM_COMPONENT_H
#define SIM_COMPONENT_H

#ifdef __cplusplus //only compile if C++
extern "C" { // Don't mangle
#endif

// Define simple types to mimic HLS concepts
typedef unsigned long long ClockCycle; // Corresponds to Rust's u64
typedef unsigned int FifoId;
typedef unsigned int AxiAddress;

// Struct to hold simulation results like Dsepoint
//https://github.com/sharc-lab/LightningSim/blob/main/backend/lightningsim-core/lib.rs#L106
typedef struct {
    ClockCycle latency; // 0 for no result/deadlock
    int bram_count;
    int error_code; // 0 for success
} SimResult;

// Function mimicking CompiledSimulation::resolve or DSE logic
// Inputs:
//   fifo_depth: A single FIFO depth for simplicity
//   axi_delay: A single AXI delay for simplicity
// Output: SimResult struct with calculated latency, BRAMs, and potential error.
SimResult simulate_hls_module(FifoId fifo_id, int fifo_depth, AxiAddress axi_addr, ClockCycle axi_delay);

#ifdef __cplusplus
}
#endif

// A C++-only function (without extern "C") to demonstrate name mangling
#ifdef __cplusplus
int cpp_mangled_function(int x, float y);
#endif


#endif // SIM_COMPONENT_H
