//! A compiler for the Brainfuck language
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

#![warn(missing_docs)]

mod error;
pub use error::*;

pub mod generator;
pub mod instruction;
pub mod interpreter;
