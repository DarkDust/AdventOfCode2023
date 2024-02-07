.include "syscall.inc"

.align 4
.global _main
_main:
    adrp X0, input@PAGE
    add X0, X0, input@PAGEOFF
    mov X1, #input_len
    bl _print_n

    mov X0, #0
    mov X16, #SYSCALL_EXIT
    svc 80

.const
input: .incbin "../../day1/rsc/input.txt"
input_len = (. - input)
