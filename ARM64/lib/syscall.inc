// Various defines for system calls.
// The sources are relative to the XNU kernel sources root directory.

// Kernel: "exit"
// Source: bsd/kern/syscalls.master
.equ SYSCALL_EXIT, 1

// Kernel: "write"
// Source: bsd/kern/syscalls.master
.equ SYSCALL_WRITE, 4

// Kernel: "mach_timebase_info"
// Source: osfmk/mach/syscall_sw.h
.equ SYSCALL_MACH_TIMEBASE_INFO, -89

// Kernel: "MACH_ARM_TRAP_CONTTIME" aka "mach_continuous_time".
// Source: osfmk/mach/arm/traps.h
.equ SYSCALL_MACH_CONTINUOUS_TIME, -4

// Kernel: "mach_task_self"
// Source: osfmk/mach/syscall_sw.h
.equ SYSCALL_MACH_TASK_SELF, -28

// Kernel: "_kernelrpc_mach_vm_allocate_trap"
// Source: osfmk/mach/syscall_sw.h
.equ SYSCALL_MACH_VM_ALLOCATE, -10

// Kernel: "_kernelrpc_mach_port_deallocate_trap"
// Source: osfmk/mach/syscall_sw.h
.equ SYSCALL_MACH_VM_DEALLOCATE, -12

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