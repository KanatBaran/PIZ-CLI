/* Imports */
use std::fs;
use std::path::Path;
use std::process::Command; // to run terminal commands
/* ./Imports */

#[test]
fn test_jpg_compatibility_expansion() {
    // Define file paths
    let input_file = "tests/fixtures/sample.jpg";
    let output_file = "tests/fixtures/sample_expanded.jpg";

    // Expand a JPEG file by 100KB using PIZ-CLI.
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

    // Decode the original JPG file.
    let original_image = image::open(input_file)
        .expect("Could not open original JPEG file")
        .to_rgb8();

    // Decode the expanded JPEG file.
    let expanded_image = image::open(output_path)
        .expect("Expanded JPEG file is corrupted and cannot be opened")
        .to_rgb8();

    // Verify that image dimensions have not changed.
    assert_eq!(
        original_image.dimensions(),
        expanded_image.dimensions(),
        "Image dimensions changed after expansion!"
    );

    // Verify that decoded pixel data is identical.
    assert_eq!(
        original_image, expanded_image,
        "Image content changed after expansion!"
    );

    // Clean up the output file created during test.
    fs::remove_file(output_path).expect("Could not delete test output file");
}
