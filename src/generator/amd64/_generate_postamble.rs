//! Generates the postamble (assembly after the body) for AMD64

/// Generates the postamble (assembly after the body) for AMD64
///
/// Lists used functions from the C standard library
pub(crate) fn generate_postamble(libc_funcs: &[String]) -> String {
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
        libc_funcs
            .iter()
            .map(|func| format!("    .extern {}", func))
            .collect::<Vec<String>>()
            .join("\n")
    )
}
