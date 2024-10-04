//! Generates the postamble (assembly after the body) for AMD64

use super::AMD64Generator;

impl AMD64Generator {
    /// Generates the postamble (assembly after the body) for AMD64
    ///
    /// Lists used functions from the C standard library
    pub(crate) fn postamble(&self) -> String {
        format!(
            r#";# <<<Begin postamble>>>
{}
    .section ".note.GNU-stack","",@progbits

.section .rodata
.align 32
indices:
    .byte 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
    .byte 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
;# <<<End postamble>>>"#,
            self.libc_funcs
                .iter()
                .map(|func| String::from("    .extern ") + func)
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
