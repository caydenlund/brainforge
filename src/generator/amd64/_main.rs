use super::AMD64Generator;

impl AMD64Generator {
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
