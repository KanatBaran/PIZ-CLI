/* Imports */
use crate::core::fill::{PiGenerator, RandomGenerator};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;
/* ./Imports */

/* Functions */
pub fn expand_file(
    input_path: &Path,  // source path
    output_path: &Path, // extended path
    bytes_to_add: u64,  // amount of bytes to be added
    fill_method: &str,  // random or pi
) -> Result<(), String> {
    // Open input and create output files
    let input_file = File::open(input_path)
        .map_err(|e| format!("Error! The source file could not be opened: {}", e))?;
    let output_file = File::create(output_path)
        .map_err(|e| format!("Error! The output file could not be generated: {}", e))?;

    // create buffered reader and writer
    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    let chunk_size = 65536; // 64 KB Buffer Size
    let mut remaining = bytes_to_add;

    io::copy(&mut reader, &mut writer)
        .map_err(|e| format!("Error! The file could not be copied: {}", e))?;

    if fill_method == "random" {
        let mut generator = RandomGenerator::new();
        while remaining > 0 {
            let write_len = std::cmp::min(remaining, chunk_size as u64) as usize;
            let buffer: Vec<u8> = (&mut generator).take(write_len).collect();
            writer
                .write_all(&buffer)
                .map_err(|e| format!("Error! The file could not be expanded: {}", e))?;
            remaining -= write_len as u64;
        }
    } else {
        let mut generator = PiGenerator::new();
        while remaining > 0 {
            let write_len = std::cmp::min(remaining, chunk_size as u64) as usize;
            let buffer: Vec<u8> = (&mut generator).take(write_len).collect();
            writer
                .write_all(&buffer)
                .map_err(|e| format!("Error! The file could not be expanded: {}", e))?;
            remaining -= write_len as u64;
        }
    }

    writer
        .flush()
        .map_err(|e| format!("Error! The file write buffer could not be cleared: {}", e))?;
    Ok(())
}
/* ./Functions */

/* Unit Tests */
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /*
    A unique file name is generated for each test using the current process ID
    and an atomic counter. The process ID prevents file name conflicts between
    separate test runs, while the counter prevents conflicts between tests
    running in parallel within the same test process.
    */
    static TEST_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn setup_temp_files(initial_content: &[u8]) -> (PathBuf, PathBuf, Box<dyn FnOnce()>) {
        let counter = TEST_FILE_COUNTER.fetch_add(1, Ordering::SeqCst);
        let process_id = std::process::id();
        let temp_dir = std::env::temp_dir();

        let input_path = temp_dir.join(format!("test_expand_input_{}_{}.txt", process_id, counter));
        let output_path =
            temp_dir.join(format!("test_expand_output_{}_{}.txt", process_id, counter));

        fs::write(&input_path, initial_content).expect("Failed to write temp input file");

        let input_path_clone = input_path.clone();
        let output_path_clone = output_path.clone();

        let cleanup = Box::new(move || {
            let _ = fs::remove_file(input_path_clone);
            let _ = fs::remove_file(output_path_clone);
        });

        (input_path, output_path, cleanup)
    }

    // --------------------
    // expand_file()
    // --------------------

    // Verifies that a file is successfully expanded with pseudo-random bytes.
    #[test]
    fn test_expand_file_random() {
        let initial_data = b"hello";
        let (input_path, output_path, cleanup) = setup_temp_files(initial_data);

        let result = expand_file(&input_path, &output_path, 10, "random");
        assert!(result.is_ok());

        // Check output size
        let metadata = fs::metadata(&output_path).expect("Failed to read output metadata");
        assert_eq!(metadata.len(), 15);

        // Check content
        let content = fs::read(&output_path).expect("Failed to read output content");
        assert_eq!(&content[0..5], initial_data);

        cleanup();
    }

    // Verifies that a file is successfully expanded using the Pi fill method.
    #[test]
    fn test_expand_file_pi() {
        let initial_data = b"world";
        let (input_path, output_path, cleanup) = setup_temp_files(initial_data);

        let result = expand_file(&input_path, &output_path, 5, "pi");
        assert!(result.is_ok());

        // Check output size
        let metadata = fs::metadata(&output_path).expect("Failed to read output metadata");
        assert_eq!(metadata.len(), 10);

        // Check content
        let content = fs::read(&output_path).expect("Failed to read output content");
        assert_eq!(&content[0..5], initial_data);

        cleanup();
    }

    // Verifies that a file is successfully copied with 0 bytes added.
    #[test]
    fn test_expand_file_zero_bytes() {
        let initial_data = b"hello";
        let (input_path, output_path, cleanup) = setup_temp_files(initial_data);

        let result = expand_file(&input_path, &output_path, 0, "random");
        assert!(result.is_ok());

        // Check output size is identical to input
        let metadata = fs::metadata(&output_path).expect("Failed to read output metadata");
        assert_eq!(metadata.len(), 5);

        // Check content is identical to input
        let content = fs::read(&output_path).expect("Failed to read output content");
        assert_eq!(content, initial_data);

        cleanup();
    }

    // Verifies that expanding a file by an amount exceeding the chunk size (65536 bytes) works correctly.
    #[test]
    fn test_expand_file_exceed_chunk_size() {
        let initial_data = b"chunk_test";
        let (input_path, output_path, cleanup) = setup_temp_files(initial_data);
        let bytes_to_add = 65537;

        let result = expand_file(&input_path, &output_path, bytes_to_add, "random");
        assert!(result.is_ok());

        // Check output size is initial_data.len() + bytes_to_add
        let expected_size = (initial_data.len() as u64) + bytes_to_add;
        let metadata = fs::metadata(&output_path).expect("Failed to read output metadata");
        assert_eq!(metadata.len(), expected_size);

        // Check original content is preserved at the beginning
        let content = fs::read(&output_path).expect("Failed to read output content");
        assert_eq!(&content[0..initial_data.len()], initial_data);

        cleanup();
    }

    // Verifies that expanding a non-existent input file returns an error.
    #[test]
    fn test_expand_file_non_existent_input() {
        let temp_dir = std::env::temp_dir();
        let input_path = temp_dir.join("non_existent_input_file_123.txt");
        let output_path = temp_dir.join("non_existent_output_file_123.txt");

        let result = expand_file(&input_path, &output_path, 10, "random");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("source file could not be opened")
        );
    }
}
/* ./Unit Tests */
