//! Gets the source BF program

use crate::{BFError, BFResult};
use std::fs::File;
use std::io::{stdin, Read};
use std::path::PathBuf;

/// Gets the source BF input as a vector of bytes, either from a filename or stdin
pub fn input(filename: Option<PathBuf>) -> BFResult<Vec<u8>> {
    let mut src: Vec<u8> = vec![];
    if let Some(filename) = filename {
        // Read program from file.
        if let Ok(mut file) = File::open(&filename) {
            let Ok(_) = file.read_to_end(&mut src) else {
                return Err(BFError::FileReadError(filename));
            };
        } else {
            return Err(BFError::FileReadError(filename));
        }
    } else {
        // Read program from stdin.
        src = stdin()
            .bytes()
            .filter(|result| result.is_ok())
            .map(|result| result.unwrap())
            .collect();
    }

    Ok(src)
}
