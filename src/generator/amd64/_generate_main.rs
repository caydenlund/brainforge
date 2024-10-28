//! Assembly generation for the `main` function

/// Generates assembly for the `main` function
///
/// Calls `calloc` from the C standard library to allocate program memory.
/// Passes a pointer to the center of the resulting memory tape
/// as an argument to function `bf_prog`
pub(crate) fn generate_main(mem_size: usize) -> String {
    format!(
        r#";# <<<Begin main>>>
main:
    push rbp
    mov rbp, rsp

    mov rdi, {}
    mov rsi, 1
    call calloc

    lea rdi, [rax + {}]
    call bf_prog

    mov rsp, rbp
    pop rbp

    mov rax, 0
    ret
;# <<<End main>>>"#,
        mem_size,
        mem_size / 2,
    )
}
