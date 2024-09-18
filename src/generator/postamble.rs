pub fn generate_postamble() -> String {
    r#"
;# <<<Begin postamble>>>
    .extern malloc
    .extern putchar
;# <<<End postamble>>>
    "#.trim().to_string()
}
