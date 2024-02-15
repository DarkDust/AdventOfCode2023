.include "syscall.inc"

//////////////////////////////////////////////////////////////////////////////

// uint64_t mach_task_self(void);
// Get the Mach port number of the current task.
.global _mach_task_self

// int64_t mach_vm_allocate(uint64_t mach_port, void ** addr, uint64_t size, int64_t flags);
// Allocate a region of memory.
.global _mach_vm_allocate

// int64_t mach_vm_deallocate(uint64_t mach_port, void * addr);
// Deallocate a region of memory.
.global _mach_vm_allocate

// void * mach_alloc(uint64_t size);
// Convenience function: allocate a region of memory. On success, a non-NULL
// pointer is returned.
.global _mach_alloc

// void * mach_free(void * addr);
// Convenience function: deallocate a region of memory. On success, 0 is
// returned.
.global _mach_free

//////////////////////////////////////////////////////////////////////////////

.text
.balign 4
_mach_task_self:
    // TODO: Can we cache the return value?
    SYSCALL SYSCALL_MACH_TASK_SELF
    ret

.balign 4
_mach_vm_allocate:
    SYSCALL SYSCALL_MACH_VM_ALLOCATE
    ret

.balign 4
_mach_alloc:
    stp FP, LR, [SP, #-16]!
    mov FP, SP
    str XZR, [SP, #-16]!

    mov X2, X0  // Arg 3 (used later): size.

    SYSCALL SYSCALL_MACH_TASK_SELF
                // Arg 1: mach_task_self
    mov X1, SP  // Arg 2: Pointer to pointer
                // Arg 3: size, already set above.
    mov X3, #1  // Arg 4: VM_FLAGS_ANYWHERE
    SYSCALL SYSCALL_MACH_VM_ALLOCATE
    cbnz X0, 1f // On failure, return NULL.
    ldr X0, [SP]// Load pointer
    b 2f

1:  mov X0, #0
2:  mov SP, FP
    ldp FP, LR, [SP], #16
    ret

.balign 4
_mach_free:
    cbz X0, 1f
    mov X2, X1  // Arg 3 (used later): size to free.
    mov X1, X0  // Arg 2 (used later): pointer to free.
 
    SYSCALL SYSCALL_MACH_TASK_SELF
    SYSCALL SYSCALL_MACH_VM_DEALLOCATE
1:  ret