
// Define a non-null-terminated string of the given name, plus a symbol of the name plus "_len"
// with the length of the string.
.macro DEFINE_STRING name, text
\name : .ascii "\text"
\name\()_len = (. - \name)
.endmacro

// Print a named string.
// Modifies X0 and X1, and does a function call.
.macro PRINT_STRING name
    adrp X0, \name\()@PAGE
    add X0, X0, \name\()@PAGEOFF
    mov X1, \name\()_len
    bl _print_n
.endmacro

// Round up to nearest multiple.
.macro ROUND_UP_CONST reg, value, multiple
    mov \reg, #(((\value + \multiple - 1) / \multiple) * \multiple)
.endmacro

// Bit shift for memory page size.
.equ PAGE_SHIFT, 12

// Size of a memory page in bytes.
.equ PAGE_SIZE, (1 << PAGE_SHIFT)