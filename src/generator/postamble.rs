pub fn generate_postamble() -> String {
    r#"
; <<<Begin postamble>>>
    .extern malloc
    .extern printf
; <<<End postamble>>>
    "#.trim().to_string()
}
