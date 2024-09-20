//! Assembly generation for the `main` function for AARCH64

use super::AARCH64Generator;

impl AARCH64Generator {
    /// Generates assembly for the `main` function
    ///
    /// Calls `malloc` from the C std. library to allocate program memory.
    /// Passes the resulting memory tape as an argument to function `bf_prog`
    pub(crate) fn main(&self) -> String {
        format!(
            r#";# <<<Begin main>>>
main:
    pushq %rbp
    movq %rsp, %rbp

    movl ${}, %edi
    call malloc

    movq %rax, %rdi
    call bf_prog

    movq %rbp, %rsp
    popq %rbp
    ret
;# <<<End main>>>"#,
            self.mem_size
        )
    }
}
