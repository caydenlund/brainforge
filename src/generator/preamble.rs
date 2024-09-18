pub fn generate_preamble() -> String {
    r#"
; <<<Begin preamble>>>
    .global _start
    .section .data
format_string:
    .string "%d\n"

    .section .text
    .globl main
; <<<End preamble>>>
    "#
    .trim()
    .to_string()
}
