//! Assembly generation for the AMD64 architecture

use super::Generator;
use crate::instruction::IntermediateInstruction;

mod _bf_prog;
mod _generate_instrs;
mod _main;
mod _postamble;
mod _preamble;

/// Assembly generator for the AMD64 architecture
pub struct AMD64Generator {
    /// The size of the tape to allocate at runtime
    mem_size: usize,

    /// A list of instructions that make up the `bf_body` function body
    bf_instrs: Vec<String>,

    /// A list of C standard library functions used
    libc_funcs: Vec<String>,
}

impl Generator for AMD64Generator {
    /// Instantiates a new [`AMD64Generator`] [`Generator`]
    ///
    /// This is where most of the generation logic lives; fills out the `bf_body` function body
    fn new(src: &[IntermediateInstruction], mem_size: usize) -> Self {
        let libc_funcs = vec!["malloc".into(), "getchar".into(), "putchar".into()];
        let mem_size = mem_size.next_power_of_two();

        Self {
            mem_size,
            bf_instrs: Self::generate_instrs(src, &mut Box::new(0), mem_size),
            libc_funcs,
        }
    }

    /// Generates a comprehensive assembly program as a single string
    fn text(&self) -> String {
        vec![
            self.preamble(),
            self.main(),
            self.bf_prog(),
            self.postamble(),
        ]
        .join("\n\n")
            + "\n"
    }
}
