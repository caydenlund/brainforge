//! Generates the preamble (assembly before the main function) for AARCH64

use super::AARCH64Generator;

impl AARCH64Generator {
    /// Generates the preamble (assembly before the main function) for AARCH64
    pub(crate) fn preamble(&self) -> String {
        r#";# <<<Begin preamble>>>
    .section .text
    .globl main
;# <<<End preamble>>>"#
            .trim()
            .to_string()
    }
}
