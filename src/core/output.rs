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

/* Unit Tests */
#[cfg(test)]
mod tests {
    use super::*;

    // --------------------
    // generate_output_path()
    // --------------------

    // Verifies that a file path with an extension correctly appends '_expanded' before the extension.
    #[test]
    fn test_generate_output_path_with_extension() {
        assert_eq!(
            generate_output_path("test.txt"),
            PathBuf::from("test_expanded.txt")
        );

        // TODO: Enable this test after generate_output_path() is updated
        // to support compound file extensions such as .tar.gz.
        //
        // assert_eq!(
        //     generate_output_path("archive.tar.gz"),
        //     PathBuf::from("archive_expanded.tar.gz")
        // );
    }

    // Verifies that a file path without an extension correctly appends '_expanded'.
    #[test]
    fn test_generate_output_path_without_extension() {
        assert_eq!(generate_output_path("test"), PathBuf::from("test_expanded"));
    }

    // Verifies that a file path containing directories correctly preserves the parent directory structure.
    #[test]
    fn test_generate_output_path_with_directories() {
        let input = Path::new("dir").join("subdir").join("test.txt");
        let input_str = input.to_str().unwrap();
        let expected = Path::new("dir").join("subdir").join("test_expanded.txt");
        assert_eq!(generate_output_path(input_str), expected);
    }

    // Verifies that an empty path input defaults to 'output_expanded'.
    #[test]
    fn test_generate_output_path_empty_input() {
        assert_eq!(generate_output_path(""), PathBuf::from("output_expanded"));
    }
}
/* ./Unit Tests */
