//! Assembly generation for the `main` function for AMD64

use super::AMD64Generator;

impl AMD64Generator {
    /// Generates assembly for the `main` function
    ///
    /// Calls `malloc` from the C standard library to allocate program memory.
    /// Passes a pointer to the center of the resulting memory tape
    /// as an argument to function `bf_prog`
    pub(crate) fn main(&self) -> String {
        format!(
            r#";# <<<Begin main>>>
main:
    push rbp
    mov rbp, rsp

    mov edi, {}
    call malloc

    lea rdi, [rax + {}]
    call bf_prog

    mov rsp, rbp
    pop rbp

    mov rax, 0
    ret
;# <<<End main>>>"#,
            self.mem_size,
            self.mem_size / 2,
        )
    }
}
