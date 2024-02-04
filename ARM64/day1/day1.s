.align 4
.global _main
_main:
    mov X0, 1
    adrp X1, input@PAGE
    add X1, X1, input@PAGEOFF
    mov X2, #input_len
    mov X16, 4
    svc 80

    mov X0, #0
    mov X16, 1
    svc 80

.const
input: .incbin "../../day1/rsc/input.txt"
input_len = (. - input)
