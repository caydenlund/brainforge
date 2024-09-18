    .global _start
    .section .data
format_string:
    .string "%d\n"

    .section .text
    .globl main

main:
    pushq   %rbp
    movq    %rsp, %rbp

    movl    $4, %edi
    call    malloc
    movq    %rax, %rsi

    movl    $5, (%rsi)

    movl    $format_string, %edi
    movl    (%rsi), %eax
    movl    %eax, %esi
    call    printf

    movq    %rbp, %rsp
    popq    %rbp
    ret

    .extern malloc
    .extern printf
