pub fn generate_main(_mem_size: usize) -> String {
    r#"
; <<<Begin main>>>
main:
    pushq %rbp
    movq %rsp, %rbp

    movl $4, %edi
    call malloc
    movq %rax, %rsi

    movl $5, (%rsi)

    movl $format_string, %edi
    movl 4(%rsi), %eax
    movl %eax, %esi
    call printf

    movq %rbp, %rsp
    popq %rbp
    ret
; <<<End main>>>
    "#.trim().to_string()
}
