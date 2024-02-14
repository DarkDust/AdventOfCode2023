
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