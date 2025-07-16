fn main() {
    // Tell Cargo to link the 'box_blur_filter' dynamic library.
    println!("cargo:rustc-link-lib=dylib=box_blur_filter");

    // Tell Cargo where to search for the library.
    println!("cargo:rustc-link-search=native=.");

    // Tell the OS linker to look for shared libraries in the same directory as the executable.
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
}
