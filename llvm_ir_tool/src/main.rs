use llvm_ir::{Module}; //LLVM IR Library for handling .ll files
use std::collections::HashMap;
use std::env; // Command line arguments
use std::fs::File;
use std::io::{BufRead, BufReader}; //Turn lines into vectors
use regex::Regex; // Search annd manipulate strings

fn main() -> Result<(), String> {

    let args: Vec<String> = env::args().collect();
    //Expect at least the executable and the filename
    if args.len() < 2 {
        return Err("Usage: cargo run <filename.ll> [function_name(opt)] [line_number(opt)]".to_string());
    }
    let filename = &args[1];
    let function_filter = args.get(2).map(|s| s.as_str());
    let explain_line_number: Option<usize> = args.get(3).and_then(|s| s.parse().ok());

    // Load the module from file in IR text)
    let llvm_module = Module::from_ir_path(filename)
        .map_err(|e| format!("Failed to parse LLVM module: {}", e))?;

    // Read all lines of the input .ll file into a vector for line referencing
    let file = File::open(filename).map_err(|e| format!("Failed to open file: {}", e))?;
    let lines: Vec<String> = BufReader::new(file).lines().collect::<Result<_, _>>()
        .map_err(|e| format!("Failed reading file lines: {}", e))?;

    // Map function name -> (start line, end line) in the .ll file
    let func_line_spans = find_function_line_spans(&lines);
    // Initialize a Hashmap to store instruction counts per function
    let mut func_instr_counts: HashMap<&str, HashMap<String, usize>> = HashMap::new();
    //Iterate through each function in the LLVM Module to find the matching one
    for func in &llvm_module.functions {
        let name = func.name.as_str();
        if let Some(filter) = function_filter {
            if filter != name {
                continue;
            }
        }
        let instr_counts = func_instr_counts.entry(name).or_default();
        //Iterate through each basic blocks and extract the information
        for bb in &func.basic_blocks {
            for instr in &bb.instrs {
                // Extract opcode name (e.g. "Load", "Store", etc)
                let op_name = format!("{:?}", instr);
                let op_code_name = op_name.split_whitespace().next().unwrap_or("Unknown");
                *instr_counts.entry(op_code_name.to_string()).or_insert(0) += 1; //Increment the count in hashmap
            }
        }
    }

    // Print instruction counts per function
    for (func_name, counts) in &func_instr_counts {
        println!("Function: {}", func_name);
        let mut total = 0;
        // Sort by opcode name
        let mut sorted: Vec<_> = counts.iter().collect();
        sorted.sort_by_key(|&(k, _)| k);
        for (op_name, count) in sorted {
            println!("  {:<15} {}", op_name, count);
            total += count;
        }
        println!("  Total: {}\n", total);
    }

    // If function_filter is specified, print the lines of that function with line numbers
    if let Some(func_name) = function_filter {
        if let Some(&(start_line, end_line)) = func_line_spans.get(func_name) {
            println!("Lines for function '{}' (lines {} to {}):", func_name, start_line + 1, end_line + 1);
            for (i, line) in lines.iter().enumerate().take(end_line + 1).skip(start_line) {
                println!("{:4} | {}", i + 1, line);
            }

            // If a line number to explain is passed and in range
            if let Some(explain_line) = explain_line_number {
                if explain_line >= start_line + 1 && explain_line <= end_line + 1 {
                    let code_line = &lines[explain_line - 1];
                    println!("{}",explain_llvm_line(code_line));
                } else {
                    println!("\nLine number {} is out of range for function '{}'.", explain_line, func_name);
                }
            }
        } else {
            println!("Function '{}' not found in source file.", func_name);
        }
    }

    Ok(())
}

/// Finds function start and end line numbers in the LLVM IR source lines.
/// Returns a map from function name -> (start_line_index, end_line_index).
/// Line numbers are zero-based indexes.
fn find_function_line_spans(lines: &[String]) -> HashMap<&str, (usize, usize)> {
    let mut spans = HashMap::new();
    let mut current_func: Option<(&str, usize)> = None;

    for (i, line) in lines.iter().enumerate() {
        // Detect function start lines, e.g.: define i32 @main(...) {
        if line.trim_start().starts_with("define") {
            // Parse function name from line
            // This is naive: function name is after '@' and before '('
            if let Some(at_pos) = line.find('@') {
                if let Some(paren_pos) = line[at_pos..].find('(') {
                    let name = &line[at_pos + 1..at_pos + paren_pos];
                    // Close previous function span if any
                    if let Some((prev_name, start)) = current_func.take() {
                        spans.insert(prev_name, (start, i - 1));
                    }
                    current_func = Some((name, i));
                }
            }
        }
        // Detect function end line: a line with only `}` (possibly spaces)
        if line.trim() == "}" {
            if let Some((name, start)) = current_func.take() {
                spans.insert(name, (start, i));
            }
        }
    }
    // If file ended while inside function (no trailing '}'), close it at last line
    if let Some((name, start)) = current_func.take() {
        spans.insert(name, (start, lines.len() - 1));
    }

    spans
}

//Function to explain a single LLVM line in readable format by parsing common instructions based on regex matching
fn explain_llvm_line(line: &str) -> String {
    // Strip trailing debug metadata to simplify matching
    let clean_line = line.split("!dbg").next().unwrap_or(line).trim();

    // Regex patterns for common instructions, allowing optional trailing stuff
    //%14 = load i32, ptr %5, align 4, !dbg !44
    let load_re = Regex::new(r"^(?P<dest>%[\w\d]+)\s*=\s*load\s+(?P<ty>[\w\d]+),\s*ptr\s+(?P<src>%[\w\d]+)(?:,.*)?$").unwrap();
    //store i32 %0, ptr %3, align 4
    let store_re = Regex::new(r"^store\s+(?P<val_ty>[\w\d\s\*]+)\s+(?P<val>[\w\d%\.\-]+),\s*ptr\s+(?P<dest>%[\w\d]+)(?:,.*)?$").unwrap();
    // %13 = add nsw i32 %11, %12, !dbg !41
    let add_re = Regex::new(r"^(?P<dest>%[\w\d]+)\s*=\s*add\s+(?:[^,]+?)\s+(?P<op1>[^,]+?),\s*(?P<op2>[^,]+?)(?:,.*)?$").unwrap();
    //  %16 = mul nsw i32 %14, %15, !dbg !46
    let mul_re = Regex::new(r"^(?P<dest>%[\w\d]+)\s*=\s*mul\s+(?:[^,]+?)\s+(?P<op1>[^,]+?),\s*(?P<op2>[^,]+?)(?:,.*)?$").unwrap();
    //  call void @llvm.dbg.declare(metadata ptr %3, metadata !27, metadata !DIExpression()), !dbg !28
    let call_re = Regex::new(r"^(?P<dest>%[\w\d]+)?\s*=?\s*call\s+.*@(?P<func>[\w\d_]+)(?:\(.*\))?(?:,.*)?$").unwrap();
    //  %3 = alloca i32, align 4
    let alloca_re = Regex::new(r"^(?P<dest>%[\w\d]+)\s*=\s*alloca\s+(?P<ty>[\w\d]+)(?:,.*)?$").unwrap();
    //  ret i32 %22, !dbg !53
    let ret_re = Regex::new(r"^ret\s+(?P<ty>[\w\d]+)\s+(?P<val>[\w\d%]+)(?:,.*)?$").unwrap();
    let br_re = Regex::new(r"^br\s+(?P<cond>.+)$").unwrap(); // Simplified, can improve
    let icmp_re = Regex::new(r"^(?P<dest>%[\w\d]+)\s*=\s*icmp\s+(?P<pred>\w+)\s+(?P<ty>[\w\d]+)\s+(?P<op1>[\w\d%]+),\s*(?P<op2>[\w\d%]+)(?:,.*)?$").unwrap();
    let phi_re = Regex::new(r"^(?P<dest>%[\w\d]+)\s*=\s*phi\s+(?P<ty>[\w\d]+)\s+(?P<vals>.+)$").unwrap();
    let bitcast_re = Regex::new(r"^(?P<dest>%[\w\d]+)\s*=\s*bitcast\s+(?P<from_ty>[\w\d]+)\s+(?P<from_val>[\w\d%]+)\s+to\s+(?P<to_ty>[\w\d]+)(?:,.*)?$").unwrap();

    // Match each pattern and return explanation
    if let Some(caps) = load_re.captures(clean_line) {
        format!(
            "Load the value pointed to by register {} into register {} (type {}).",
            &caps["src"], &caps["dest"], &caps["ty"]
        )
    } else if let Some(caps) = store_re.captures(clean_line) {
        format!(
            "Store the value {} into the memory pointed to by register {}.",
            &caps["val"], &caps["dest"]
        )
    } else if let Some(caps) = add_re.captures(clean_line) {
        format!(
            "Add the values of registers {} and {}, store the result in register {}.",
            &caps["op1"], &caps["op2"], &caps["dest"]
        )
    } else if let Some(caps) = mul_re.captures(clean_line) {
        format!(
            "Multiply the values of registers {} and {}, store the result in register {}.",
            &caps["op1"], &caps["op2"], &caps["dest"]
        )
    } else if let Some(caps) = call_re.captures(clean_line) {
        if let Some(dest) = caps.name("dest") {
            format!(
                "Call function '{}' and store the return value in register {}.",
                &caps["func"], dest.as_str()
            )
        } else {
            format!("Call function '{}'.", &caps["func"])
        }
    } else if let Some(caps) = alloca_re.captures(clean_line) {
        format!(
            "Allocate space on the stack for type {} and assign its pointer to register {}.",
            &caps["ty"], &caps["dest"]
        )
    } else if let Some(caps) = ret_re.captures(clean_line) {
        format!(
            "Return the value {} of type {} from the function.",
            &caps["val"], &caps["ty"]
        )
    } else if let Some(caps) = br_re.captures(clean_line) {
        format!("Branch instruction: {}.", &caps["cond"])
    } else if let Some(caps) = icmp_re.captures(clean_line) {
        format!(
            "Integer compare ({}) between registers {} and {}, result stored in {}.",
            &caps["pred"], &caps["op1"], &caps["op2"], &caps["dest"]
        )
    } else if let Some(caps) = phi_re.captures(clean_line) {
        format!(
            "PHI node in register {} selecting among values: {}.",
            &caps["dest"], &caps["vals"]
        )
    } else if let Some(caps) = bitcast_re.captures(clean_line) {
        format!(
            "Bitcast value in register {} from type {} to type {}, store in {}.",
            &caps["from_val"], &caps["from_ty"], &caps["to_ty"], &caps["dest"]
        )
    } else {
        // Debug print unmatched lines - comment out if not needed
        // eprintln!("No detailed explanation for line: {}", line);

        // Fallback simple message:
        format!("No detailed explanation available for this instruction: {}", clean_line)
    }
}


