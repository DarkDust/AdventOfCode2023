// Various defines

// Kernel: "exit"
.equ SYSCALL_EXIT, 1

// Kernel: "write"
.equ SYSCALL_WRITE, 4

// File descriptor: stdin
.equ FD_STDIN, 0

// File descriptor: stdout
.equ FD_STDOUT, 1

// File descriptor: stderr
.equ FD_STDERR, 2

.macro SYSCALL num
    str X16, [SP, #-16]!    // Save X16
    mov X16, \num           // Put syscall number in X16
    svc 80                  // Call kernel
    ldr X16, [SP], #16      // Restore X16
.endmacro