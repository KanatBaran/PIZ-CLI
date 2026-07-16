/* Imports */
use std::path::Path;
/* ./Imports */

/* Functions */
// Converts size expressions to bytes.
pub fn parse_size(size_str: &str) -> Result<u64, String> {
    let clean_str = size_str.trim().to_uppercase();

    let num_part: String = clean_str
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    let unit_part: String = clean_str
        .chars()
        .skip(num_part.len())
        .filter(|c| !c.is_whitespace())
        .collect();

    if num_part.is_empty() {
        return Err(format!("Error! Invalid size format: {}", size_str));
    }

    let value: f64 = num_part
        .parse()
        .map_err(|e| format!("Erorr! The number could not be deciphered: {}", e))?;

    let multiplier: u64 = match unit_part.as_str() {
        "B" | "" => 1,
        "KB" | "K" => 1024,
        "MB" | "M" => 1024 * 1024,
        "GB" | "G" => 1024 * 1024 * 1024,
        "TB" | "T" => 1024 * 1024 * 1024 * 1024,
        _ => return Err(format!("Error! Invalid Format: {}", unit_part)),
    };

    Ok((value * multiplier as f64) as u64)
}

// Verifies parameters given via CLI.
pub fn validate_inputs(
    file_path: &str,
    add: &Option<String>,
    size: &Option<String>,
) -> Result<(), String> {
    if !Path::new(file_path).exists() {
        return Err(format!("Error! File isn't found: {}", file_path));
    }

    if add.is_none() && size.is_none() {
        return Err("Error! Please specify either the '--add' or '--size' parameter:".to_string());
    }

    Ok(())
}
/* ./Functions */

/* Unit Tests */
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /*
        A separate file with a different name is created for each test. This prevents file names from conflicting for unit tests running in parallel or unit tests that have been interrupted.
    */
    static TEST_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn create_temp_file() -> (String, Box<dyn FnOnce()>) {
        let counter = TEST_FILE_COUNTER.fetch_add(1, Ordering::SeqCst);

        let mut temp_path = std::env::temp_dir();
        temp_path.push(format!("test_validator_temp_{}.txt", counter));

        let path_str = temp_path.to_string_lossy().into_owned();

        fs::write(&temp_path, "temp content").expect("The temp file was not created.");

        let temp_path_clone = temp_path.clone();
        let cleanup = Box::new(move || {
            let _ = fs::remove_file(temp_path_clone);
        });

        (path_str, cleanup)
    }

    // --------------------
    // parse_size()
    // --------------------

    // Verifies that plain numbers and byte units are parsed correctly.
    #[test]
    fn test_parse_size_bytes() {
        assert_eq!(parse_size("100").unwrap(), 100);
        assert_eq!(parse_size("100B").unwrap(), 100);
        assert_eq!(parse_size(" 100 b ").unwrap(), 100);
    }

    // Verifies that kilobyte values, including decimal values, are parsed correctly.
    #[test]
    fn test_parse_size_kilobytes() {
        assert_eq!(parse_size("2KB").unwrap(), 2048);
        assert_eq!(parse_size("2K").unwrap(), 2048);
        assert_eq!(parse_size("1.5KB").unwrap(), 1536);
    }

    // Verifies that megabyte values are converted to the correct number of bytes.
    #[test]
    fn test_parse_size_megabytes() {
        assert_eq!(parse_size("5MB").unwrap(), 5 * 1024 * 1024);
        assert_eq!(parse_size("5M").unwrap(), 5 * 1024 * 1024);
        assert_eq!(parse_size("0.5M").unwrap(), 512 * 1024);
    }

    // Verifies that gigabyte values are converted correctly, including decimal values.
    #[test]
    fn test_parse_size_gigabytes() {
        assert_eq!(parse_size("1GB").unwrap(), 1024 * 1024 * 1024);
        assert_eq!(parse_size("1G").unwrap(), 1024 * 1024 * 1024);
        assert_eq!(
            parse_size("2.5G").unwrap(),
            (2.5 * 1024.0 * 1024.0 * 1024.0) as u64
        );
    }

    // Verifies that terabyte values are converted to bytes correctly.
    #[test]
    fn test_parse_size_terabytes() {
        assert_eq!(parse_size("1TB").unwrap(), 1024 * 1024 * 1024 * 1024);
        assert_eq!(parse_size("1T").unwrap(), 1024 * 1024 * 1024 * 1024);
    }

    // Verifies that invalid size formats return an error.
    #[test]
    fn test_parse_size_invalid_formats() {
        assert!(parse_size("").is_err());
        assert!(parse_size("   ").is_err());
        assert!(parse_size("abc").is_err());
        assert!(parse_size("KB").is_err());
        assert!(parse_size("1.2.3MB").is_err());
    }

    // Verifies that unsupported size units return an error.
    #[test]
    fn test_parse_size_invalid_units() {
        assert!(parse_size("100XB").is_err());
        assert!(parse_size("100foo").is_err());
        assert!(parse_size("100GBB").is_err());
    }

    // --------------------
    // validate_inputs()
    // --------------------

    // Verifies that validation succeeds when a valid file and the '--add' parameter are provided.
    #[test]
    fn test_validate_inputs_success_with_add() {
        let (file_path, cleanup) = create_temp_file();

        let result = validate_inputs(&file_path, &Some("10MB".to_string()), &None);
        assert!(result.is_ok());

        cleanup();
    }

    // Verifies that validation succeeds when a valid file and the '--size' parameter are provided.
    #[test]
    fn test_validate_inputs_success_with_size() {
        let (file_path, cleanup) = create_temp_file();

        let result = validate_inputs(&file_path, &None, &Some("20MB".to_string()));
        assert!(result.is_ok());

        cleanup();
    }

    // Verifies that validation fails when the specified file does not exist.
    #[test]
    fn test_validate_inputs_file_not_found() {
        let result = validate_inputs(
            "non_existent_file_xyz.txt",
            &Some("10MB".to_string()),
            &None,
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("File isn't found"));
    }

    // Verifies that validation fails when neither '--add' nor '--size' is provided.
    #[test]
    fn test_validate_inputs_missing_both_params() {
        let (file_path, cleanup) = create_temp_file();

        let result = validate_inputs(&file_path, &None, &None);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Please specify either the '--add' or '--size' parameter")
        );

        cleanup();
    }
}
/* ./Unit Tests */
