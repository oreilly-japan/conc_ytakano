    .text
    .global _start ; <1>

    .align 4 ; <2>
_start: ; <3>
    mov    x19, #1 ; <4>
.Lloop:
    cmp    x19, #100
    b.hi   .Lend0
    mov    x0, x19
    bl     FizzBuzz ; <5>
    add    x19, x19, 1
    b      .Lloop
.Lend0:
    mov    x0, xzr ; <6>
    mov    x8, #93 ; exit
    svc    #0

FizzBuzz: ; <7>
    stp    x19, x20, [sp, #-16]!

    ; if x0 % 15 == 0 then go to .LFB <8>
    mov    x20, #15
    udiv   x19, x0, x20
    msub   x19, x19, x20, x0
    cmp    x19, xzr ; x0 % 15 == 0
    b.eq   .LFB

    ; if x0 % 3 == 0 then go to .LF
    mov    x20, #3
    udiv   x19, x0, x20
    msub   x19, x19, x20, x0
    cmp    x19, xzr ; x0 % 3 == 0
    b.eq   .LF

    ; if x0 % 5 != 0 then go to .Lend1
    mov    x20, #5
    udiv   x19, x0, x20
    msub   x19, x19, x20, x0
    cmp    x19, xzr ; x0 % 3 == 0
    b.ne   .Lend1
    adr    x1, buzzStr ; <9>
    mov    x2, #5   ; legth of "Buzz\n"
    b      .LWrite
.LF:
    adr    x1, fizzStr
    mov    x2, #5   ; legth of "Fizz\n"
    b     .LWrite
.LFB:
    adr    x1, fizzBuzzStr
    mov    x2, #9   ; legth of "FizzBuzz\n"
.LWrite:
    mov    x0, xzr
    mov    x8, #64  ; write
    svc    #0
.Lend1:
    ldp    x19, x20, [sp], #-16 ; <10>
    ret

.data ; <11>
fizzBuzzStr:
    .string "FizzBuzz\n"

fizzStr:
    .string "Fizz\n"

buzzStr:
    .string "Buzz\n"
