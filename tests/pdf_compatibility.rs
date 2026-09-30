/* Imports */
use std::fs;
use std::path::Path;
use std::process::Command;
/* ./Imports */

#[test]
fn test_pdf_compatibility_expansion() {
    // Define file paths
    let input_file = "tests/fixtures/sample.pdf";
    let output_file = "tests/fixtures/sample_expanded.pdf";

    // Expand a PDF file by 100KB using PIZ-CLI.
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

    // 100 KB = 102,400 bytes
    let expected_added_bytes = 100 * 1024;

    // Verify that the file has grown by exactly 100 KB.
    assert_eq!(
        output_len,
        input_len + expected_added_bytes,
        "The expanded file size is not the expected size!"
    );

    // Read original and expanded content.
    let original_content = fs::read(input_file).expect("Could not read original PDF file");
    let expanded_content = fs::read(output_path).expect("Could not read expanded PDF file");

    // Verify PDF header magic bytes (%PDF-).
    assert!(
        original_content.starts_with(b"%PDF-"),
        "Original file is missing %PDF- header magic bytes"
    );
    assert!(
        expanded_content.starts_with(b"%PDF-"),
        "Expanded PDF lost its %PDF- header magic bytes"
    );

    // Verify that original PDF binary content is strictly preserved at the beginning.
    assert_eq!(
        &expanded_content[..original_content.len()],
        original_content.as_slice(),
        "Original PDF content changed after expansion!"
    );

    // Verify that the original content contains %%EOF marker before appended data.
    let has_eof = original_content.windows(5).any(|w| w == b"%%EOF");
    assert!(has_eof, "PDF fixture is missing %%EOF marker");

    // Verify appended data starts with two newlines and contains only ASCII digits.
    let appended = &expanded_content[original_content.len()..];
    assert!(
        appended.starts_with(b"\n\n"),
        "PDF fill should start with two newlines"
    );
    assert!(
        appended[2..].iter().all(|&b| (b'0'..=b'9').contains(&b)),
        "Appended PDF fill contained a non-digit byte"
    );

    // Clean up the output file created during test.
    fs::remove_file(output_path).expect("Could not delete test output file");
}
