//! A compiler for the Brainfuck language
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

#![warn(missing_docs)]

mod _architecture;
pub use _architecture::*;
mod _error;
pub use _error::*;
mod _io;
pub use _io::*;

pub mod assembly;
pub mod generator;
pub mod instruction;
pub mod interpreter;
pub mod jit;
pub mod optimizer;
