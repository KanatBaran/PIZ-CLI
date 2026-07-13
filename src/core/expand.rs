/* Imports */
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;
use crate::core::fill::{PiGenerator, RandomGenerator};
/* ./Imports */


/* Functions */
pub fn expand_file(
    input_path: &Path,  // source path
    output_path: &Path, // extended path
    bytes_to_add: u64,  // amount of bytes to be added
    fill_method: &str,  // random or pi
) -> Result<(), String> 
{
 
    // Open input and create output files
    let input_file = File::open(input_path).map_err(|e| format!("Error! The source file could not be opened: {}", e))?;
    let output_file = File::create(output_path).map_err(|e| format!("Error! The output file could not be generated: {}", e))?;
    
    // create buffered reader and writer
    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    let chunk_size = 65536; // 64 KB Buffer Size
    let mut remaining = bytes_to_add;

    io::copy(&mut reader, &mut writer).map_err(|e| format!("Error! The file could not be copied: {}", e))?;
    
    if fill_method == "random" {
        let mut generator = RandomGenerator::new();
        while remaining > 0 {
            let write_len = std::cmp::min(remaining, chunk_size as u64) as usize;
            let buffer: Vec<u8> = (&mut generator).take(write_len).collect();
            writer.write_all(&buffer).map_err(|e| format!("Error! The file could not be expanded: {}", e))?;
            remaining -= write_len as u64;
        }
    } else {
        let mut generator = PiGenerator::new();
        while remaining > 0 {
            let write_len = std::cmp::min(remaining, chunk_size as u64) as usize;
            let buffer: Vec<u8> = (&mut generator).take(write_len).collect();
            writer.write_all(&buffer).map_err(|e| format!("Error! The file could not be expanded: {}", e))?;
            remaining -= write_len as u64;
        }
    }
    
    writer.flush().map_err(|e| format!("Error! The file write buffer could not be cleared: {}", e))?;
    Ok(())
}
/* ./Functions */