//! A compiler for the Brainfuck language
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

#![warn(missing_docs)]

mod _error;
pub use _error::*;
mod _input;
pub use _input::*;

pub mod generator;
pub mod instruction;
pub mod interpreter;
