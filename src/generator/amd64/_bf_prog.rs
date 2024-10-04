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
    movq %rdi, %r12
    vpxor %ymm1, %ymm1, %ymm1
    vmovdqu indices_neg4(%rip), %ymm2
    vmovdqu indices_neg2(%rip), %ymm3
    vmovdqu indices_neg1(%rip), %ymm4
    vmovdqu indices_1(%rip), %ymm5
    vmovdqu indices_2(%rip), %ymm6
    vmovdqu indices_4(%rip), %ymm7
{}
    ret
;# <<<End BF Program>>>"#,
            self.bf_instrs.join("\n")
        )
    }
}
