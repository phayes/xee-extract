use std::path::Path;
use std::process::Command;

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
        "09_multi_document_extraction",
        "10_bound_variables",
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
                    println!("Example {} ran successfully", example);

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
                        "Example {} failed with status {}\nstdout: {}\nstderr: {}",
                        example, output.status, stdout, stderr
                    );
                }
            }
            Err(e) => {
                panic!("Failed to run example {}: {}", example, e);
            }
        }
    }
}
