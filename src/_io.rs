//! Gets the source BF program

use crate::{BFError, BFResult};
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
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

/// Gets the target output as a writable stream, either to a file or stdout
pub fn output(path: &PathBuf) -> BFResult<Box<dyn Write>> {
    if *path == PathBuf::from("-") {
        Ok(Box::new(stdout()))
    } else {
        let Ok(file) = File::create(path) else {
            return Err(BFError::FileWriteError(PathBuf::from(path)));
        };
        Ok(Box::new(file))
    }
}
