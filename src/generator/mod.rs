//! Defines & implements assembly generation types and methods

pub mod amd64;

mod _architecture;
pub use _architecture::*;

mod _generator;
pub use _generator::*;

use crate::instruction::IntermediateInstruction;

/// Generate assembly for the given program, memory size, and target architecture
pub fn generate(src: &[IntermediateInstruction], mem_size: usize, arch: Architecture) -> String {
    match arch {
        Architecture::AMD64 => {
            let generator = amd64::AMD64Generator::new(src, mem_size);
            generator.text()
        }
    }
}
