pub mod amd64;

mod _architecture;
pub use _architecture::*;

mod _generator;
pub use _generator::*;

use crate::instruction::Instruction;

pub fn generate(src: &[Instruction], mem_size: usize, arch: Architecture) -> String {
    match arch {
        Architecture::AMD64 => {
            let generator = amd64::AMD64Generator::new(src, mem_size);
            generator.text()
        }
    }
}
