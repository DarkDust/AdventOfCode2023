.include "syscall.inc"

.equ OFFSET_TOTAL, 0
.equ OFFSET_FIRST, 4
.equ OFFSET_LAST, 8

// Reset the per-line infos. Overwrites W3.
.macro reset_part1 pointer
    mov W3, #-1
    stp W3, W3, [\pointer, #OFFSET_FIRST]
.endmacro


.balign 4
.global _main
_main:
    bl _part1

    mov X0, #0
    mov X16, #SYSCALL_EXIT
    svc 80

.balign 4
_part1:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    // Reserve some space for our variables and initialize them.
    sub SP, SP, #16
    str WZR, [SP, #OFFSET_TOTAL]        // Set 'total' = 0
    reset_part1 SP                      // Set 'first' and 'last' to -1

    adrp X0, str_input@PAGE             // Load input string address
    add X0, X0, str_input@PAGEOFF
    mov X1, #str_input_len              // Set input string length
    adrp X2, _process_part1@PAGE        // Load handler address
    add X2, X2, _process_part1@PAGEOFF
    mov X3, SP                          // "Context": address for our variables
    bl _iterate_chars

    ldp W3, W4, [SP, #OFFSET_FIRST]     // Check the first/last values
    cmp W3, #-1                         // If first is not -1, input is missing a newline
    b.eq 1f

    mov X0, #'\n'                       // Simulate missing '\n' and call the processor
    mov X1, SP                          // "Context"
    bl _process_part1

1:  adrp X0, str_part1@PAGE             // Print the `Part1:` string
    add X0, X0, str_part1@PAGEOFF
    mov X1, #str_part1_len
    bl _print_n

    ldr W0, [SP, #OFFSET_TOTAL]         // Load 'total'.
    bl _print_uint64                    // Print 'total'.
    bl _print_newline

    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

.balign 4
_process_part1:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    cmp W0, #'\n'           // Check for newline
    b.eq L_p1_newline
    sub W0, W0, #'0'        // Shift the input character so that digit 0 gets value 0
    cmp W0, #9              // Check: value >= 0 and <= 9?
    b.gt L_p1_return        // Leave if out of range

    ldp W3, W4, [X1, #OFFSET_FIRST] // Load 'first' and 'last' in one go.
    cmp W3, #-1             // Check whether 'first' is already set.
    csel W3, W0, W3, eq     // W3 = (W3 == -1) ? W0 : W3 , only set the value if it wasn't set
    mov W4, W0              // 'last' = W0
    stp W3, W4, [X1, #OFFSET_FIRST] // Save 'first' and 'last'

    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

L_p1_newline:
    ldp W3, W4, [X1, #OFFSET_FIRST] // Load 'first' and 'last' in one go.
    ldr W5, [X1, #OFFSET_TOTAL] // Load 'total'

    mov W6, #10
    mul W0, W3, W6          // W0 = 'first' * 10
    add W0, W0, W4          // W0 += 'last'
    add W5, W5, W0          // total += W0
    str W5, [X1, #OFFSET_TOTAL] // Save 'total'

    reset_part1 X1          // Reset for next line.

L_p1_return:
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

.balign 4
_handler:
    strb W0, [SP, #-16]! // Push the byte on the stack

    mov X0, #FD_STDOUT
    mov X1, SP // Adress of pushed byte
    mov X2, #1
    SYSCALL #SYSCALL_WRITE

    add SP, SP, 16 // Pop
    ret


.const
str_input: .incbin "../../day1/rsc/input.txt"
str_input_len = (. - str_input)

str_part1: .ascii "Part 1: "
str_part1_len = (. - str_part1)