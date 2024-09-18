pub fn generate_preamble() -> String {
    r#"
;# <<<Begin preamble>>>
    .section .text
    .globl main
;# <<<End preamble>>>
    "#
    .trim()
    .to_string()
}
