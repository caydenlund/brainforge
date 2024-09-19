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
{}
    ret
;# <<<End BF Program>>>"#,
            self.bf_instrs.join("\n")
        )
    }
}
