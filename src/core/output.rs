/* Imports */
use std::path::{Path, PathBuf};
/* ./Imports */

/* Functions */
pub fn generate_output_path(input_path: &str) -> PathBuf {
    // path object
    let path = Path::new(input_path);

    // It retrieves the file's main name; if it can't find it, it defaults to 'output'.
    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    // It finds the folder, otherwise it says ""
    let parent = path.parent().unwrap_or_else(|| Path::new(""));

    // compound extension list
    const COMPOUND_EXTENSIONS: &[&str] = &[".tar.gz", ".tar.bz2", ".tar.xz", ".tar.zst"];

    // The file extension is checked against the list of known compound extensions. If a match is found, the compound extension is preserved; otherwise, only the last extension is considered.
    let new_filename = if let Some(extension) = COMPOUND_EXTENSIONS
        .iter()
        .find(|extension| file_name.ends_with(*extension))
    {
        let stem = &file_name[..file_name.len() - extension.len()];
        format!("{}_expanded{}", stem, extension)
    } else {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some(extension) => {
                let stem = path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or("output");

                format!("{}_expanded.{}", stem, extension)
            }
            None => format!("{}_expanded", file_name),
        }
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
    }

    // Verifies that a compound file extension is preserved when '_expanded' is appended.
    #[test]
    fn test_generate_output_path_with_compound_extension() {
        assert_eq!(
            generate_output_path("archive.tar.gz"),
            PathBuf::from("archive_expanded.tar.gz")
        );
    }

    // Verifies that dots within the file name are preserved and only the last extension is considered.
    #[test]
    fn test_generate_output_path_with_dots_in_filename() {
        assert_eq!(
            generate_output_path("my.screen.jpg"),
            PathBuf::from("my.screen_expanded.jpg")
        );
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
