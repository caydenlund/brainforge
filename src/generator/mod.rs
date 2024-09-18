mod bf_prog;
pub use bf_prog::*;

mod main;
pub use main::*;
mod preamble;
pub use preamble::*;
mod postamble;
pub use postamble::*;

use crate::instruction::*;

pub fn generate(src: &[Instruction], mem_size: usize) -> String {
    vec![
        generate_preamble(),
        generate_main(mem_size),
        generate_bf_prog(src),
        generate_postamble(),
    ]
    .join("\n\n")
        + "\n"
}
