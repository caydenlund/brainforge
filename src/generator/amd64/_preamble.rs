//! Generates the preamble (assembly before the main function) for AMD64

use super::AMD64Generator;

impl AMD64Generator {
    /// Generates the preamble (assembly before the main function) for AMD64
    pub(crate) fn preamble(&self) -> String {
        r#";# <<<Begin preamble>>>
    .section .text
    .globl main
    .align 32
;# <<<End preamble>>>"#
            .trim()
            .to_string()
    }
}
