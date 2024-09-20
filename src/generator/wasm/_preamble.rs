//! Generates the preamble (assembly before the main function) for WASM

use super::WASMGenerator;

impl WASMGenerator {
    /// Generates the preamble (assembly before the main function) for WASM
    pub(crate) fn preamble(&self) -> String {
        r#";# <<<Begin preamble>>>
    .section .text
    .globl main
;# <<<End preamble>>>"#
            .trim()
            .to_string()
    }
}
