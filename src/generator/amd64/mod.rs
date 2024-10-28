//! Assembly generation for the AMD64 architecture

mod _generate;
pub use _generate::*;

mod _generate_bf_prog;
pub(crate) use _generate_bf_prog::*;
mod _generate_instrs;
pub(crate) use _generate_instrs::*;
mod _generate_main;
pub(crate) use _generate_main::*;
mod _generate_postamble;
pub(crate) use _generate_postamble::*;
mod _generate_preamble;
pub(crate) use _generate_preamble::*;

/// Assembly generator for the AMD64 architecture
pub struct AMD64Generator {
    /// The size of the tape to allocate at runtime
    mem_size: usize,

    /// A list of instructions that make up the `bf_body` function body
    bf_instrs: Vec<String>,

    /// A list of C standard library functions used
    libc_funcs: Vec<String>,
}
