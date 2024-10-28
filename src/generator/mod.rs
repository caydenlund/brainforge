//! Defines & implements assembly generation types and methods

pub mod amd64;

use crate::instruction::IntermediateInstruction;
use crate::{Architecture, BFResult};

/// Generate assembly for the given program, memory size, and target architecture
pub fn generate(
    src: &[IntermediateInstruction],
    partial_evaluation: bool,
    mem_size: usize,
    arch: Architecture,
) -> BFResult<String> {
    match arch {
        Architecture::AMD64 => amd64::generate(src, partial_evaluation, mem_size),
    }
}
