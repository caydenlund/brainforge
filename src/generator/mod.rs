//! Defines & implements assembly generation types and methods

pub mod aarch64;
pub mod amd64;
pub mod wasm;

mod _architecture;
pub use _architecture::*;

mod _generator;
pub use _generator::*;

use crate::instruction::Instruction;

/// Generate assembly for the given program, memory size, and target architecture
pub fn generate(src: &[Instruction], mem_size: usize, arch: Architecture) -> String {
    match arch {
        Architecture::AARCH64 => {
            let generator = aarch64::AARCH64Generator::new(src, mem_size);
            generator.text()
        }
        Architecture::AMD64 => {
            let generator = amd64::AMD64Generator::new(src, mem_size);
            generator.text()
        }
        Architecture::WASM => {
            let generator = wasm::WASMGenerator::new(src, mem_size);
            generator.text()
        }
    }
}
