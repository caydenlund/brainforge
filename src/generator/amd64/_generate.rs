use super::{generate_bf_prog, generate_main, generate_postamble, generate_preamble};
use crate::instruction::IntermediateInstruction;
use crate::BFResult;

pub fn generate(
    src: &[IntermediateInstruction],
    _partial_evaluation: bool,
    mem_size: usize,
) -> BFResult<String> {
    let libc_funcs = vec!["getchar".into(), "putchar".into()];
    Ok(vec![
        generate_preamble(),
        generate_main(mem_size),
        generate_bf_prog(src),
        generate_postamble(&libc_funcs),
    ]
    .join("\n\n")
        + "\n")
}
