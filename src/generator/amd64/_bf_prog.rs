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
    leaq {}(%rdi), %r12
    vmovdqu mask_1(%rip), %ymm1
    vmovdqu mask_2(%rip), %ymm2
    vmovdqu mask_4(%rip), %ymm4
    vpxor %ymm0, %ymm0, %ymm0
{}
    ret
;# <<<End BF Program>>>"#,
            self.mem_size >> 1,
            self.bf_instrs.join("\n")
        )
    }
}
