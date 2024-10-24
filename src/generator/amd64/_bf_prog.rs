//! Generates the main program body for AMD64

use super::AMD64Generator;

impl AMD64Generator {
    /// Generates the BF program body as a single function, `bf_prog`
    ///
    /// `bf_prog` accepts 1 argument: a pointer to a tape of memory
    pub(crate) fn bf_prog(&self) -> String {
        format!(
            r#";# <<<Begin BF Program>>>
bf_prog:
    mov r12, rdi
    vmovdqu ymm1, [rip + mask_1]
    vmovdqu ymm2, [rip + mask_2]
    vmovdqu ymm4, [rip + mask_4]
    vpxor ymm0, ymm0, ymm0
    {}
    ret
;# <<<End BF Program>>>"#,
            self.bf_instrs.join("\n    ")
        )
    }
}
