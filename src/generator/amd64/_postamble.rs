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
indices_1:
    .byte  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15
    .byte 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
indices_2:
    .byte  0,  0,  2,  0,  4,  0,  6,  0,  8,  0, 10,  0, 12,  0, 14, 0
    .byte 16,  0, 18,  0, 20,  0, 22,  0, 24,  0, 26,  0, 28,  0, 30, 0
indices_4:
    .byte  0,  0,  0,  0,  4,  0,  0,  0,  8,  0,  0,  0, 12,  0,  0, 0
    .byte 16,  0,  0,  0, 20,  0,  0,  0, 24,  0,  0,  0, 28,  0,  0, 0
indices_neg1:
    .byte 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16
    .byte 15, 14, 13, 12, 11, 10,  9,  8,  7,  6,  5,  4,  3,  2,  1,  0
indices_neg2:
    .byte  0, 30,  0, 28,  0, 26,  0, 24,  0, 22,  0, 20,  0, 18,  0, 16
    .byte  0, 14,  0, 12,  0, 10,  0,  8,  0,  6,  0,  4,  0,  2,  0,  0
indices_neg4:
    .byte  0,  0,  0, 28,  0,  0,  0, 24,  0,  0,  0, 20,  0,  0,  0, 16
    .byte  0,  0,  0, 12,  0,  0,  0,  8,  0,  0,  0,  4,  0,  0,  0,  0
;# <<<End postamble>>>"#,
            self.libc_funcs
                .iter()
                .map(|func| String::from("    .extern ") + func)
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
