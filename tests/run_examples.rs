use std::process::Command;
use std::path::Path;

/// Test that runs all examples and verifies they execute successfully
#[test]
fn test_run_all_examples() {
    let examples = vec![
        "01_basic_extraction",
        "02_named_extractions", 
        "03_custom_extract_value",
        "04_namespaces",
        "05_contexts",
        "06_nested_structs",
        "07_raw_xml",
        "08_binary_handling",
    ];

    for example in examples {
        println!("Running example: {}", example);
        
        let output = Command::new("cargo")
            .args(&["run", "--example", example])
            .current_dir(Path::new("."))
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("✅ Example {} ran successfully", example);
                    
                    // Print the output for debugging
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    
                    if !stdout.is_empty() {
                        println!("  stdout: {}", stdout);
                    }
                    if !stderr.is_empty() {
                        println!("  stderr: {}", stderr);
                    }
                } else {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    
                    panic!(
                        "❌ Example {} failed with status {}\nstdout: {}\nstderr: {}",
                        example, output.status, stdout, stderr
                    );
                }
            }
            Err(e) => {
                panic!("❌ Failed to run example {}: {}", example, e);
            }
        }
    }
}

/// Test that verifies all example files exist
#[test]
fn test_all_example_files_exist() {
    let examples = vec![
        "examples/01_basic_extraction.rs",
        "examples/02_named_extractions.rs",
        "examples/03_custom_extract_value.rs", 
        "examples/04_namespaces.rs",
        "examples/05_contexts.rs",
        "examples/06_nested_structs.rs",
        "examples/07_raw_xml.rs",
        "examples/08_binary_handling.rs",
    ];

    for example_path in examples {
        let path = Path::new(example_path);
        assert!(
            path.exists(),
            "Example file {} does not exist",
            example_path
        );
        println!("✅ Example file exists: {}", example_path);
    }
}

/// Test that compiles all examples without errors
#[test]
fn test_compile_all_examples() {
    let examples = vec![
        "01_basic_extraction",
        "02_named_extractions",
        "03_custom_extract_value",
        "04_namespaces", 
        "05_contexts",
        "06_nested_structs",
        "07_raw_xml",
        "08_binary_handling",
    ];

    for example in examples {
        println!("Compiling example: {}", example);
        
        let output = Command::new("cargo")
            .args(&["check", "--example", example])
            .current_dir(Path::new("."))
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("✅ Example {} compiles successfully", example);
                } else {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    
                    panic!(
                        "❌ Example {} failed to compile\nstdout: {}\nstderr: {}",
                        example, stdout, stderr
                    );
                }
            }
            Err(e) => {
                panic!("❌ Failed to compile example {}: {}", example, e);
            }
        }
    }
} 