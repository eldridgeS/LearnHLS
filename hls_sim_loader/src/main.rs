use libloading::{Library, Symbol};
use std::error::Error;
use std::fmt;
use std::ffi::{c_int, c_uint}; // For C types: int, unsigned int

// Rust representation of SimResult, match C implementation
#[repr(C)]
#[derive(Debug)]
struct SimResultC {
    latency: u64, // ClockCycle in C
    bram_count: c_int,
    error_code: c_int,
}

// Rust enum to map C error codes to meaningful errors, similar to SimulationError
#[derive(Debug)]
enum SimulationErrorRust {
    DeadlockDetected,
    FifoDepthNotProvided(u32),
    AxiDelayNotProvided(u32),
    OtherCError(c_int), // For unhandled C error codes
    LibraryLoadError(String),
    SymbolNotFound(String),
    // Add more as needed
}
impl fmt::Display for SimulationErrorRust {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimulationErrorRust::DeadlockDetected => write!(f, "Deadlock detected (simulated by C component)"),
            SimulationErrorRust::FifoDepthNotProvided(id) => write!(f, "FIFO depth not provided for ID {} (simulated by C component)", id),
            SimulationErrorRust::AxiDelayNotProvided(addr) => write!(f, "AXI delay not provided for address {:#010x} (simulated by C component)", addr),
            SimulationErrorRust::OtherCError(code) => write!(f, "Unhandled C component error code: {}", code),
            SimulationErrorRust::LibraryLoadError(msg) => write!(f, "Failed to load library: {}", msg),
            SimulationErrorRust::SymbolNotFound(name) => write!(f, "Symbol '{}' not found in library", name),
        }
    }
}
impl Error for SimulationErrorRust {} // Make it a proper error type

// Define the type of the C function for Rust's FFI
type SimulateHlsModuleFn = unsafe extern "C" fn(c_uint, c_int, c_uint, u64) -> SimResultC;
type CppMangledFunctionFn = unsafe extern "C" fn(c_int, f32) -> c_int; // For the mangled function

fn main() -> Result<(), Box<dyn Error>> {
    let lib_path = "./libsim_component.so";

    // Load the shared library
    println!("Rust: Attempting to load library from: {}", lib_path);
    let lib = unsafe {
        Library::new(lib_path)
            .map_err(|e| SimulationErrorRust::LibraryLoadError(e.to_string()))?
    };
    println!("Rust: Library loaded successfully.\n");

    // Resolve the 'simulate_hls_module' symbol
    println!("Rust: Resolving symbol 'simulate_hls_module'...");
    let simulate_hls_module: Symbol<SimulateHlsModuleFn> = unsafe {
        lib.get(b"simulate_hls_module\0")
            .map_err(|e| SimulationErrorRust::SymbolNotFound(format!("simulate_hls_module: {}", e)))?
    };
    println!("Rust: Symbol 'simulate_hls_module' resolved.\n");

    // --- Test Cases ---

    // Test Case 1: Successful simulation
    println!("--- Test Case 1: Successful Simulation ---");
    let result1 = unsafe { simulate_hls_module(101, 64, 0x1000, 50) };
    match result1.error_code {
        0 => println!("Rust: Simulation successful. Latency: {}, BRAMs: {}\n", result1.latency, result1.bram_count),
        _ => eprintln!("Rust: Simulation failed with error code: {}. This should not happen here.\n", result1.error_code),
    }

    // Test Case 2: Simulated DeadlockDetected
    println!("--- Test Case 2: Simulated DeadlockDetected (FIFO depth 0) ---");
    let result2 = unsafe { simulate_hls_module(102, 0, 0x1004, 30) };
    let simulation_error: Result<(), SimulationErrorRust> = match result2.error_code {
        0 => Ok(()),
        1 => Err(SimulationErrorRust::DeadlockDetected),
        _ => Err(SimulationErrorRust::OtherCError(result2.error_code)),
    };
    if let Err(e) = simulation_error {
        eprintln!("Rust: Caught expected simulation error: {}\n", e);
    } else {
        println!("Rust: Unexpectedly successful simulation for deadlock scenario.\n");
    }

    // Test Case 3: Simulated FifoDepthNotProvided
    println!("--- Test Case 3: Simulated FifoDepthNotProvided (FIFO depth -1) ---");
    let result3 = unsafe { simulate_hls_module(103, -1, 0x1008, 40) };
    let simulation_error: Result<(), SimulationErrorRust> = match result3.error_code {
        0 => Ok(()),
        2 => Err(SimulationErrorRust::FifoDepthNotProvided(103)),
        _ => Err(SimulationErrorRust::OtherCError(result3.error_code)),
    };
    if let Err(e) = simulation_error {
        eprintln!("Rust: Caught expected simulation error: {}\n", e);
    } else {
        println!("Rust: Unexpectedly successful simulation for missing FIFO depth.\n");
    }

    // Test Case 4: Simulated AxiDelayNotProvided
    println!("--- Test Case 4: Simulated AxiDelayNotProvided (AXI delay 0) ---");
    let result4 = unsafe { simulate_hls_module(104, 32, 0x100C, 0) };
    let simulation_error: Result<(), SimulationErrorRust> = match result4.error_code {
        0 => Ok(()),
        3 => Err(SimulationErrorRust::AxiDelayNotProvided(0x100C)),
        _ => Err(SimulationErrorRust::OtherCError(result4.error_code)),
    };
    if let Err(e) = simulation_error {
        eprintln!("Rust: Caught expected simulation error: {}\n", e);
    } else {
        println!("Rust: Unexpectedly successful simulation for missing AXI delay.\n");
    }


    // --- Demonstrate C++ Name Mangling ---
    println!("--- Demonstrating C++ Name Mangling ---");

    // This will fail because the name is mangled.
    println!("Rust: Attempting to resolve 'cpp_mangled_function' (unmangled)...");
    match unsafe { lib.get::<Symbol<CppMangledFunctionFn>>(b"cpp_mangled_function\0") } {
        Ok(_) => println!("Rust: Unexpectedly resolved 'cpp_mangled_function'."),
        Err(e) => eprintln!("Rust: Failed to resolve 'cpp_mangled_function' (unmangled) as expected: {}", e),
    }

    // Actual mangled function name
    let mangled_name = "_Z20cpp_mangled_functionif\0";
    println!("Rust: Attempting to resolve 'cpp_mangled_function' with mangled name: {}...", mangled_name);
    match unsafe { lib.get::<Symbol<CppMangledFunctionFn>>(mangled_name.as_bytes()) } {
        Ok(mangled_func) => {
            println!("Rust: Successfully resolved 'cpp_mangled_function' with mangled name.");
            let result = unsafe { mangled_func(5, 7.5f32) };
            println!("Rust: Result from mangled function: {}", result);
        },
        Err(e) => eprintln!("Rust: Failed to resolve 'cpp_mangled_function' with mangled name: {}", e),
    }

    Ok(())
}
