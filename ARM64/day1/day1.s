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
    // Setup time and remember current timestamp.
    bl _setup_time
    bl _time_nanoseconds
    str X0, [SP, #-16]!

    bl _part1

    // Get "now" and calculate elapsed time.
    bl _time_nanoseconds
    ldr X1, [SP]
    str X0, [SP]
    sub X0, X0, X1
    bl _print_elapsed

    bl _part2

    // Get "now" and calculate elapsed time.
    bl _time_nanoseconds
    ldr X1, [SP]
    sub X0, X0, X1
    bl _print_elapsed

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

1:  adrp X0, str_part1@PAGE             // Print "Part 1:"
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
_part2:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    // Reserve some space for our variables and initialize them.
    sub SP, SP, #16
    str WZR, [SP, #OFFSET_TOTAL]        // Set 'total' = 0

    adrp X0, str_input@PAGE             // Load input string address
    add X0, X0, str_input@PAGEOFF
    mov X1, #str_input_len              // Set input string length
    adrp X2, _process_part2@PAGE        // Load handler address
    add X2, X2, _process_part2@PAGEOFF
    mov X3, SP                          // "Context": address for our variables
    bl _iterate_lines

    adrp X0, str_part2@PAGE             // Print "Part 1:"
    add X0, X0, str_part2@PAGEOFF
    mov X1, #str_part2_len
    bl _print_n

    ldr W0, [SP, #OFFSET_TOTAL]         // Load 'total'.
    bl _print_uint64                    // Print 'total'.
    bl _print_newline

    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

P2_LINE .req X20
P2_LINELEN .req X21
P2_CONTEXT .req X22
P2_FIRST .req X24
P2_LAST .req X25

.balign 4
_process_part2:
    stp FP, LR, [SP, #-16]!
    mov FP, SP
    stp X20, X21, [SP, #-16]!
    stp X22, X23, [SP, #-16]!
    stp X24, X25, [SP, #-16]!

    // Save a few values.
    mov P2_LINE, X0
    mov P2_LINELEN, X1
    mov P2_CONTEXT, X2
    mov P2_FIRST, #-1
    mov P2_LAST, #-1

1:  mov X0, P2_LINE
    mov X1, P2_LINELEN
    bl _part2_get_num // Try to parse the number at current location
    cbz X0, 2f

    cmp P2_FIRST, #-1               // Is 'first' still -1?
    csel P2_FIRST, X0, P2_FIRST, eq // If so, save number in X0, otherwise retain previous value
    mov P2_LAST, X0

2:  add P2_LINE, P2_LINE, #1        // Advance pointer …
    sub P2_LINELEN, P2_LINELEN, #1  // … and decrease length.
    cbnz P2_LINELEN, 1b             // Loop if length != 0

    // End of line reached.
    mov W1, #10
    mul X0, P2_FIRST, X1                    // X0 = 'first' * 10
    add X0, X0, P2_LAST                     // X0 += 'last'
    ldr W1, [P2_CONTEXT, #OFFSET_TOTAL]     // Load 'total'
    add W1, W1, W0                          // total += W0
    str W1, [P2_CONTEXT, #OFFSET_TOTAL]     // Save 'total'

    ldp X24, X25, [SP], #16
    ldp X22, X23, [SP], #16
    ldp X20, X21, [SP], #16
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret


.macro P2_CHECKPREFIX prefix, value
    mov X0, X20                     // Arg 1: (remaining) line pointer
    mov X1, X21                     // Arg 2: (remaining) line length
    adrp X2, \prefix\()@PAGE        // Arg 3: address of prefix
    add X2, X2, \prefix\()@PAGEOFF
    mov X3, #\prefix\()_len         // Arg 4: length of prefix
    bl _has_prefix                  // Call "has_prefix"
    mov X1, #\value                 // Load the value the prefix should represent
    mul X0, X0, X1                  // X0 is either 0 or 1 here; multiply with value
    cbnz X0, L_p2_return            // If X0 contains a value, return
.endmacro

.balign 4
_part2_get_num:
    stp FP, LR, [SP, #-16]!
    mov FP, SP
    // Save the string pointer and length
    stp X20, X21, [SP, #-16]!

    ldrb W3, [X0]           // Load the first byte
    cmp W3, #'e'            // Starts with 'e'?
    b.eq L_p2_nondigit_e
    cmp W3, #'f'            // Starts with 'f'?
    b.eq L_p2_nondigit_f
    cmp W3, #'n'            // Starts with 'n'?
    b.eq L_p2_nondigit_n
    cmp W3, #'o'            // Starts with 'o'?
    b.eq L_p2_nondigit_o
    cmp W3, #'s'            // Starts with 's'?
    b.eq L_p2_nondigit_s
    cmp W3, #'t'            // Starts with 't'?
    b.eq L_p2_nondigit_t
    sub W4, W3, #'0'        // Is it a number?
    cmp W4, #9
    b.gt L_p2_return_false  // Nothing matched, ignore.
    mov X0, X4              // It's a number! Return it.
    b L_p2_return

L_p2_nondigit_e:
    P2_CHECKPREFIX str_eight, 8
    b L_p2_return_false

L_p2_nondigit_f:
    P2_CHECKPREFIX str_four, 4
    P2_CHECKPREFIX str_five, 5
    b L_p2_return_false

L_p2_nondigit_n:
    P2_CHECKPREFIX str_nine, 9
    b L_p2_return_false

L_p2_nondigit_o:
    P2_CHECKPREFIX str_one, 1
    b L_p2_return_false

L_p2_nondigit_s:
    P2_CHECKPREFIX str_six, 6
    P2_CHECKPREFIX str_seven, 7
    b L_p2_return_false

L_p2_nondigit_t:
    P2_CHECKPREFIX str_two, 2
    P2_CHECKPREFIX str_three, 3
    b L_p2_return_false

    // Nothing found.
L_p2_return_false:
    mov X0, 0
L_p2_return:
    ldp X20, X21, [SP], #16
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

.const
str_input: .incbin "../../day1/rsc/input.txt"
str_input_len = (. - str_input)

.macro DEFINE_STRING name, text
\name : .ascii "\text"
\name\()_len = (. - \name)
.endmacro

DEFINE_STRING str_part1, "Part 1: "
DEFINE_STRING str_part2, "Part 2: "
DEFINE_STRING str_one, "one"
DEFINE_STRING str_two, "two"
DEFINE_STRING str_three, "three"
DEFINE_STRING str_four, "four"
DEFINE_STRING str_five, "five"
DEFINE_STRING str_six, "six"
DEFINE_STRING str_seven, "seven"
DEFINE_STRING str_eight, "eight"
DEFINE_STRING str_nine, "nine"
