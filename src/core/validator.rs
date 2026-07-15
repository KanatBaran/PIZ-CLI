/* Imports */
use std::path::Path;
/* ./Imports */

/* Functions */
// Converts size expressions to bytes.
pub fn parse_size(size_str: &str) -> Result<u64, String> {
    let clean_str = size_str.trim().to_uppercase();

    let num_part: String = clean_str
        .chars()
        .take_while(|c| c.is_digit(10) || *c == '.')
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
        _ => return Err(format!("Geçersiz birim: {}", unit_part)),
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
