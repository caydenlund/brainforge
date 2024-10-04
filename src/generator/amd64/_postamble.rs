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
mask_1:
    .byte 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    .byte 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
mask_2:
    .byte 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255
    .byte 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255
mask_4:
    .byte 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255
    .byte 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255
;# <<<End postamble>>>"#,
            self.libc_funcs
                .iter()
                .map(|func| String::from("    .extern ") + func)
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
