//! Defines different types of BrainForge errors
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

use crate::assembly::Instruction;
use crate::instruction::IntermediateInstruction;
use std::path::PathBuf;

/// Errors raised when parsing a program
#[derive(Copy, Clone, Debug)]
pub enum BFParseError {
    /// When a `[` doesn't have a matching `]`
    UnmatchedLBrace(usize),

    /// When a `]` doesn't have a matching `[`
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

    /// Errors raised when generating assembly from an intermediate instruction
    GenerateError(IntermediateInstruction),

    /// Errors raised when encoding assembly instructions
    EncodeError(Box<dyn Instruction>),
}

/// Wrapper around [`Result`], specialized for a [`BFError`]
pub type BFResult<T> = Result<T, BFError>;
