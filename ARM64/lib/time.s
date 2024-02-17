.include "syscall.inc"

//////////////////////////////////////////////////////////////////////////////

// void setup_time(void);
// Setup time services.
.global _setup_time

// uint64_t time_nanoseconds(void);
// Get current system timestamp in nanoseconds. Counting starts at an
// arbitrary point, so it's only useful to calculate elapsed time.
.global _time_nanoseconds

// void print_elapsed(uint64_t nanoseconds);
// Print the elapsed time.
.global _print_elapsed

//////////////////////////////////////////////////////////////////////////////

// struct mach_timebase_info {
//         uint32_t        numer;
//         uint32_t        denom;
// };
.lcomm timebase_info, 8 

.text
.balign 4
_setup_time:
    adrp X0, timebase_info@PAGE
    add X0, X0, timebase_info@PAGEOFF
    SYSCALL SYSCALL_MACH_TIMEBASE_INFO
    ret

.balign 4
_time_nanoseconds:
    SYSCALL SYSCALL_MACH_CONTINUOUS_TIME
    // X0 is in ticks. Need to convert using the timebase info.
    adrp X1, timebase_info@PAGE
    add X1, X1, timebase_info@PAGEOFF
    ldp W2, W3, [X1] // Fetch "numer" and "denom" in one go.
    mul X0, X0, X2
    udiv X0, X0, X3
    ret

.balign 4
_print_elapsed:
    stp FP, LR, [SP, #-16]!
    mov FP, SP
    str X0, [SP, #-16]!

    adrp X0, str_elapsed_1@PAGE
    add X0, X0, str_elapsed_1@PAGEOFF
    mov X1, #str_elapsed_1_len
    bl _print_n

    ldr X0, [SP]
    bl _print_uint64

    adrp X0, str_elapsed_2@PAGE
    add X0, X0, str_elapsed_2@PAGEOFF
    mov X1, #str_elapsed_2_len
    bl _print_n

    ldr X0, [SP]
    mov X1, #0x4240         // X1 = (1000 * 1000)
    movk X1, #0xF, LSL 16
    udiv X0, X0, X1
    bl _print_uint64

    adrp X0, str_elapsed_3@PAGE
    add X0, X0, str_elapsed_3@PAGEOFF
    mov X1, #str_elapsed_3_len
    bl _print_n

    mov SP, FP
    ldp FP, LR, [SP], #16
    ret


.const
str_elapsed_1: .ascii "Elapsed: "
str_elapsed_1_len = (. - str_elapsed_1)
str_elapsed_2: .ascii " nanoseconds, "
str_elapsed_2_len = (. - str_elapsed_2)
str_elapsed_3: .ascii " milliseconds\n"
str_elapsed_3_len = (. - str_elapsed_3)