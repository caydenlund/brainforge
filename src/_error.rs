//! Defines different types of BrainForge errors
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

use std::path::PathBuf;

/// Errors raised when parsing a program
#[derive(Debug)]
pub enum BFParseError {
    /// When an `[` doesn't have a matching `]`
    UnmatchedLBrace(usize),

    /// When an `]` doesn't have a matching `[`
    UnmatchedRBrace(usize),
}

/// All types of BrainForge errors
#[derive(Debug)]
pub enum BFError {
    /// Errors raised when reading from stdin
    InputReadError,

    /// Errors raised when reading a file
    FileReadError(PathBuf),

    /// Errors raised when writing to a file
    FileWriteError(PathBuf),

    /// Errors raised when parsing a program
    ParseError(BFParseError),
}

/// Wrapper around [`Result`], specialized for a [`BFError`]
pub type BFResult<T> = Result<T, BFError>;
