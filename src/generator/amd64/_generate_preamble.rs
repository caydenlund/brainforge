//! Generates the preamble (assembly before the main function) for AMD64

/// Generates the preamble (assembly before the main function) for AMD64
pub(crate) fn generate_preamble() -> String {
    r#";# <<<Begin preamble>>>
    .section .text
    .globl main
    .align 32
    .intel_syntax noprefix
;# <<<End preamble>>>"#
        .to_string()
}
