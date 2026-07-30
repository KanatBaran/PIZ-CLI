/* Imports */
use std::fs;
use std::path::Path;
use std::process::Command;
/* ./Imports */

#[test]
fn test_txt_compatibility_expansion() {
    // Define file paths
    let input_file = "tests/fixtures/sample.txt";
    let output_file = "tests/fixtures/sample_expanded.txt";

    // Expand a TXT file by 100KB using PIZ-CLI.
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

    // Read original and expanded content.
    let original_content = fs::read(input_file).expect("Could not read original TXT file");
    let expanded_content = fs::read(output_path).expect("Could not read expanded TXT file");

    // Verify that the original text is preserved at the beginning of the expanded file.
    assert_eq!(
        &expanded_content[..original_content.len()],
        original_content.as_slice(),
        "Original TXT content changed after expansion!"
    );

    // Verify that the original text is still valid UTF-8 and readable.
    let original_text =
        std::str::from_utf8(&original_content).expect("Original TXT fixture is not valid UTF-8");
    let preserved_text = std::str::from_utf8(&expanded_content[..original_content.len()])
        .expect("Preserved TXT prefix is not valid UTF-8 after expansion");

    assert_eq!(
        preserved_text, original_text,
        "Readable TXT content changed after expansion!"
    );

    // Verify that appended fill data is ASCII digits only ('0'..='9').
    let appended = &expanded_content[original_content.len()..];
    assert!(
        appended.iter().all(|&b| (b'0'..=b'9').contains(&b)),
        "Appended TXT fill contained a non-digit byte"
    );

    // Verify that the entire expanded file remains valid UTF-8.
    std::str::from_utf8(&expanded_content)
        .expect("Expanded TXT file is not valid UTF-8 after expansion");

    // Clean up the output file created during test.
    fs::remove_file(output_path).expect("Could not delete test output file");
}
