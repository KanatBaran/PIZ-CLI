/* Imports */
use std::fs;
use std::path::Path;
use std::process::Command;
/* ./Imports */

#[test]
fn test_exe_compatibility_expansion() {
    // Define file paths
    let input_file = "tests/fixtures/sample.exe";
    let output_file = "tests/fixtures/sample_expanded.exe";

    // Expand an EXE file by 100KB using PIZ-CLI.
    let status = Command::new("cargo")
        .args([
            "run",
            "--",
            "expand",
            input_file,
            "--add",
            "100KB",
            "--output",
            output_file,
        ])
        .status()
        .expect("The CLI command could not be executed.");

    assert!(status.success(), "CLI command failed: {:?}", status);

    // Verify that the output file has been created.
    let output_path = Path::new(output_file);

    assert!(
        output_path.exists(),
        "The output file was not found on disk: {}",
        output_file
    );

    // Get original and expanded file sizes.
    let input_len = fs::metadata(input_file)
        .expect("The input file could not be read")
        .len();

    let output_len = fs::metadata(output_path)
        .expect("The output file could not be read.")
        .len();

    // 100 KB = 102,400 byte
    let expected_added_bytes = 100 * 1024;

    // Verify that the file has grown by exactly 100 KB.
    assert_eq!(
        output_len,
        input_len + expected_added_bytes,
        "The expanded file size is not the expected size!"
    );

    // Read original and expanded binary content.
    let original_content = fs::read(input_file).expect("Could not read original EXE file");
    let expanded_content = fs::read(output_path).expect("Could not read expanded EXE file");

    // Verify that the original executable header and binary data are preserved at the beginning of the expanded file.
    assert_eq!(
        &expanded_content[..original_content.len()],
        original_content.as_slice(),
        "Original EXE binary content changed after expansion!"
    );

    // Execute the expanded executable file to verify it still runs properly without crashing.
    let run_status = Command::new(output_file)
        .arg("--test")
        .status()
        .expect("Expanded EXE file could not be executed");

    assert!(
        run_status.success(),
        "Expanded EXE failed during execution: {:?}",
        run_status
    );

    // Clean up the output file created during test.
    fs::remove_file(output_path).expect("Could not delete test output file");
}
