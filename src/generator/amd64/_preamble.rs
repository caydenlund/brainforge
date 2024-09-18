use super::AMD64Generator;

impl AMD64Generator {
    pub(crate) fn preamble(&self) -> String {
        r#";# <<<Begin preamble>>>
    .section .text
    .globl main
;# <<<End preamble>>>"#
            .trim()
            .to_string()
    }
}
