/* Imports */
use crate::cli::ExpandArgs; // cli.rs
use crate::core::expand::expand_file;
use crate::core::output::generate_output_path; // core/output.rs
use crate::core::validator::{parse_size, validate_inputs}; // core/validator.rs
use std::path::{Path, PathBuf}; // core/expand.rs
/* ./Imports */

/* Functions */
pub fn exec(args: ExpandArgs) -> Result<(), String> {
    // Check the rules.
    validate_inputs(&args.file, &args.add, &args.size)?;

    // Determine the output path.
    let output_path = match &args.output {
        Some(out) => PathBuf::from(out),
        None => generate_output_path(&args.file),
    };

    // Get the present file size.
    let current_size = std::fs::metadata(&args.file)
        .map_err(|e| format!("Error! The file info couldn't be read: {}", e))?
        .len();

    // Calculate the amount of bytes to be added.
    let bytes_to_add = if let Some(add_str) = &args.add {
        parse_size(add_str)?
    } else if let Some(size_str) = &args.size {
        let target_size = parse_size(size_str)?;
        if target_size < current_size {
            return Err(format!(
                "The target size ({}) cannot be smaller than the current file size ({}).",
                target_size, current_size
            ));
        }
        target_size - current_size
    } else {
        0
    };

    println!("Expand process is starting...");
    println!("Source File: {}", args.file);
    println!("Target File: {}", output_path.to_string_lossy());
    println!("Size to Add: {} byte", bytes_to_add);
    println!("Fill Method: {}", args.fill);

    // Extend the file.
    expand_file(
        Path::new(&args.file),
        &output_path,
        bytes_to_add,
        &args.fill,
    )?;

    println!("\nProcess completed successfully!");
    println!("\nTechnology is more meaningful when it helps people.");
    println!("If you'd like to make a difference, visit https://piz.world");
    Ok(())
}
/* ./Functions */
