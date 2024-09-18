pub fn generate_main(mem_size: usize) -> String {
    format!(
        r#"
;# <<<Begin main>>>
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
;# <<<End main>>>
    "#,
        mem_size
    )
    .trim()
    .to_string()
}
