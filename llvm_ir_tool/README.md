# **LLVM IR Analysis Tool**

This project provides a simple command-line tool written in Rust to analyze LLVM Intermediate Representation (IR) files (.ll). It can count instruction opcodes within functions, display the source code of specific functions, and even offer an explanation for individual LLVM IR instructions.

## **Backstory**

As I was trying to become more familiar with LLVM IR syntax, I quickly realized that understanding the nuances of each instruction and how they translate from high-level code could be challenging. The existing documentation is thorough, but I found myself wishing for a more interactive and immediate way to dissect and learn from actual LLVM IR output. This led me to create this tool, aimed at providing a clearer, line-by-line understanding of LLVM IR, making the learning process more intuitive and convenient.

## **Getting Started**

To use this tool, you'll need to have Rust and Clang installed on your system.

### **1\. Install Rust**

If you don't have Rust installed, the recommended way is through rustup.  
curl \--proto '=https' \--tlsv1.2 \-sSf https://sh.rustup.rs | sh

Follow the on-screen instructions. You might need to restart your terminal or run source $HOME/.cargo/env to update your PATH.

### **2\. Install Clang**

Clang is a C, C++, Objective-C, and Objective-C++ compiler which can emit LLVM IR.

* **On macOS:** Clang is usually installed with Xcode Command Line Tools.  
  xcode-select \--install

* **On Linux (Debian/Ubuntu):**  
  sudo apt update  
  sudo apt install clang

* **On Windows:** You can install Clang via LLVM Installer or through Chocolatey/Scoop. Refer to the official LLVM website for detailed instructions.

### **3\. Clone the Repository**

First, you'll need to get the project code. Assuming this code is in a Git repository:  
git clone \<your-repository-url\>  
cd \<your-project-directory\>

*(Replace \<your-repository-url\> and \<your-project-directory\> with the actual values)*

### **4\. Generate LLVM IR from C/C++ Code**

To analyze LLVM IR, you first need some. You can generate it from C/C++ source files using Clang. It's important to include debug information (-g) if you want to see the original source line numbers referenced in the LLVM IR, and to emit LLVM IR (-emit-llvm).
If you already have your .ll file synthesized (from HLS or others), you can skip to step 5.

I created a simple  program that can be found in the project (example.c):  
// example.c  
int add\_and\_multiply(int a, int b) {  
    int sum \= a \+ b;  
    int product \= sum \* 5;  
    return product;  
}

int main() {  
    int x \= 10;  
    int y \= 20;  
    int result \= add\_and\_multiply(x, y);  
    return result;  
}

Now, compile it to LLVM IR with debug statements:  
clang \-g \-S \-emit-llvm example.c \-o example.ll

This command will create an example.ll file in your current directory, which you can then analyze with the Rust tool. The “-g” is important for creating the debug lines needed for line by line analysis by this tool.

### **5\. Run the Analysis Tool**

Navigate to your project directory (where Cargo.toml is located) and run the tool using cargo run.

## **Usage**

The tool supports several command-line arguments to control its behavior:  
cargo run \<filename.ll\> \[function\_name\] \[line\_number\]. Alternatively, you can run cargo build too compile the project and run the executable with the same flags.

* \<filename.ll\>: **Required.** The path to the LLVM IR file you want to analyze.  
* \[function\_name\]: **Optional.** If provided, the tool will filter its output to only show statistics for this function and will print its source lines.  
* \[line\_number\]: **Optional.** Requires function\_name to be provided. If given, the tool will attempt to explain the LLVM IR instruction on this specific line within the specified function.

### **Examples:**

1. **Analyze the entire LLVM IR file and show instruction counts for all functions:**  
   cargo run example.ll

\!\[Default Usage\](images/default.png)

2. **Show instruction counts and source lines for a specific function:**  
   cargo run example.ll main

\!\[Function usage\](images/function.png)

3. **Explain a specific line within a function:**  
   cargo run example.ll main 57

\!\[Line Usage\](images/line.png)  
*(Note: Replace 57 with the actual line number of the instruction you want explained within the specified function in your .ll file.)*

## **Future expansions**

Here are some of the future expansions that I would like to explore.

1. **Deeper Analysis & Information Extraction:** Enhance the tool to provide richer insights into LLVM IR by analyzing operands, control flow, data flow, and memory access patterns.  
2. **Enhanced User Experience & Output:** Improve usability with interactive modes, more detailed explanations, flexible output formatting, and robust search capabilities, and perhaps even a GUI for improved visualization.  
3. **LLVM IR Code Generation/Modification:** Extend the tool's functionality to directly modify existing `.ll` files or generate new LLVM IR code based on the user’s needs.

## **Resources**

- [https://github.com/cdisselkoen/llvm-ir](https://github.com/cdisselkoen/llvm-ir)  
- [https://crates.io/crates/llvm-ir](https://crates.io/crates/llvm-ir)
- [https://docs.rs/regex/latest/regex/](https://docs.rs/regex/latest/regex/)
- [https://doc.rust-lang.org/std/io/struct.BufReader.html](https://doc.rust-lang.org/std/io/struct.BufReader.html)

