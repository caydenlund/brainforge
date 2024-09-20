//! Generates the main program body for AARCH64

use super::AARCH64Generator;

impl AARCH64Generator {
    /// Generates the BF program body as a single function, `bf_prog`
    ///
    /// `bf_prog` accepts 1 argument: a pointer to a tape of memory
    pub(crate) fn bf_prog(&self) -> String {
        format!(
            r#";# <<<Begin BF Program>>>
bf_prog:
    movq %rdi, %r12
{}
    ret
;# <<<End BF Program>>>"#,
            self.bf_instrs.join("\n")
        )
    }
}
