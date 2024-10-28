//! Generates the main BF program body as a callable function

use super::generate_instrs;
use crate::instruction::IntermediateInstruction;

/// Generates the BF program body as a single function, `bf_prog`
///
/// `bf_prog` accepts 1 argument: a pointer to the center of a tape of memory
pub(crate) fn generate_bf_prog(src: &[IntermediateInstruction]) -> String {
    format!(
        r#";# <<<Begin BF Program>>>
bf_prog:
    mov r12, rdi
    vmovdqu ymm1, [rip + mask_1]
    vmovdqu ymm2, [rip + mask_2]
    vmovdqu ymm4, [rip + mask_4]
    vpxor ymm0, ymm0, ymm0
{}
    ret
;# <<<End BF Program>>>"#,
        generate_instrs(src).join("\n")
    )
}
