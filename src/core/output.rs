/* Imports */
use std::path::{Path, PathBuf};
/* ./Imports */

/* Functions */
pub fn generate_output_path(input_path: &str) -> PathBuf {
    // path object
    let path = Path::new(input_path);

    // It retrieves the file's main name; if it can't find it, it defaults to 'output'.
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    // It retrieves the file extension, if it exists.
    let extension = path.extension().and_then(|e| e.to_str());
    // It finds the folder, otherwise it says ""
    let parent = path.parent().unwrap_or_else(|| Path::new(""));

    let new_filename = match extension {
        Some(ext) => format!("{}_expanded.{}", file_stem, ext),
        None => format!("{}_expanded", file_stem),
    };

    // result
    return parent.join(new_filename);
}
/* ./Functions */
