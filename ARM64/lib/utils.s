.include "utils.inc"
.include "syscall.inc"

//////////////////////////////////////////////////////////////////////////////

// void runtime_error(char const * const str, uint64_t strlength);
// Prints the message to stderr and exits the process with an error code. The
// message should not contain a newline.
.global _runtime_error

//////////////////////////////////////////////////////////////////////////////

.balign 4
_runtime_error:
    mov X10, X0
    mov X11, X1

    mov X0, #FD_STDERR
    adrp X1, str_error_prefix@PAGE
    add X1, X1, str_error_prefix@PAGEOFF
    mov X2, #str_error_prefix_len
    SYSCALL SYSCALL_WRITE

    mov X0, #FD_STDERR
    mov X1, X10
    mov X2, X11
    SYSCALL SYSCALL_WRITE

    mov X0, #'\n'
    str X0, [SP, #-16]!
    mov X0, #FD_STDERR
    mov X1, SP
    mov X2, #1
    SYSCALL SYSCALL_WRITE

    mov X0, #1
    SYSCALL SYSCALL_EXIT

.const
DEFINE_STRING str_error_prefix, "Runtime error: "