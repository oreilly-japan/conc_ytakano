#define SET_CONTEXT set_context
#define SWITCH_CONTEXT switch_context

.global SET_CONTEXT
.global SWITCH_CONTEXT

.text
.align 4

SET_CONTEXT:
        pop     %rbp
        xor     %eax, %eax      /* Direct invocation returns 0 */

        movq    %rbx, (%rdi)
        movq    %rbp, 8(%rdi)
        movq    %r12, 16(%rdi)
        movq    %r13, 24(%rdi)
        movq    %r14, 32(%rdi)
        movq    %r15, 40(%rdi)
        lea     8(%rsp), %rdx
        movq    %rdx, 48(%rdi)
        push    %rbp
        movq    (%rsp), %rdx
        movq    %rdx, 56(%rdi)
        ret

.text
.align 4

SWITCH_CONTEXT:
        xor     %eax, %eax
        inc     %eax            /* Return 1 instead */

        pop     %rsi
        movq    (%rdi), %rbx
        movq    8(%rdi), %rbp
        movq    16(%rdi), %r12
        movq    24(%rdi), %r13
        movq    32(%rdi), %r14
        movq    40(%rdi), %r15
        movq    48(%rdi), %rdx
        movq    %rdx, %rsp
        addq    $0x8, %rsp
        push    %rbp
        push    %rsi
        movq    56(%rdi), %rdx
        jmpq    *%rdx
