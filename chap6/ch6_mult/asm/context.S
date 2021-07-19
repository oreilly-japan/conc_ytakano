#ifdef __APPLE__ // Macの場合は、関数名のはじめにアンダースコアが必要
    #define SET_CONTEXT _set_context
    #define SWITCH_CONTEXT _switch_context
#else
    #define SET_CONTEXT set_context
    #define SWITCH_CONTEXT switch_context
#endif

.global SET_CONTEXT // <1>
.global SWITCH_CONTEXT

SET_CONTEXT: // <2>
    // callee保存レジスタを保存
    stp  d8,  d9, [x0] // <3>
    stp d10, d11, [x0, #16] // <4>
    stp d12, d13, [x0, #16 * 2]
    stp d14, d15, [x0, #16 * 3]
    stp x19, x20, [x0, #16 * 4]
    stp x21, x22, [x0, #16 * 5]
    stp x23, x24, [x0, #16 * 6]
    stp x25, x26, [x0, #16 * 7]
    stp x27, x28, [x0, #16 * 8]

    // スタックポインタとリンクレジスタを保存
    mov x1, sp
    stp x30, x1, [x0, #16 * 9]

    // return 0 <5>
    mov x0, 0
    ret

SWITCH_CONTEXT: // <6>
    // callee保存レジスタを復元
    ldp  d8,  d9, [x0] // <7>
    ldp d10, d11, [x0, #16] // <8>
    ldp d12, d13, [x0, #16 * 2]
    ldp d14, d15, [x0, #16 * 3]
    ldp x19, x20, [x0, #16 * 4]
    ldp x21, x22, [x0, #16 * 5]
    ldp x23, x24, [x0, #16 * 6]
    ldp x25, x26, [x0, #16 * 7]
    ldp x27, x28, [x0, #16 * 8]

    // スタックポインタとリンクレジスタを復元
    ldp x30, x2, [x0, #16 * 9]
    mov sp, x2

    // return 1 <9>
    mov x0, 1
    ret