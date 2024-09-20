//! Generates the preamble (assembly before the main function) for AArch64

use super::AArch64Generator;

impl AArch64Generator {
    /// Generates the preamble (assembly before the main function) for AArch64
    pub(crate) fn preamble(&self) -> String {
        r#";# <<<Begin preamble>>>
    .section .text
    .globl main
;# <<<End preamble>>>"#
            .trim()
            .to_string()
    }
}
