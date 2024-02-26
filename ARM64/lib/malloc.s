.include "syscall.inc"

//////////////////////////////////////////////////////////////////////////////
// This is a very, very simple `malloc` implementation. Pretty wasteful, only
// supports allocating in page-sized chunks but does support reallocation. If
// you're lucky, the reallocation does not need to copy memory.

// void * malloc(uint64_t size);
// Allocate a memory chunk of at least the given size.
.global _malloc

// uint64_t malloc_capacity(void * chunk);
// Returns the amount of memory left in the allocated chunk before realloc
// needs to fetch another memory region from kernel.
.global _malloc_capacity

// void * realloc(void * chunk, uint64_t new_size);
// Resize the given memory chunk. In the worst case, a new memory area is
// allocated and the old content is copied.
.global _realloc

// void free(void * chunk);
// Deallocate the given memory chunk.
.global _free

// (void *, uint64_t) malloc_wholepage(uint64_t size);
// Allocate a memory chunk of at least the given size. Immediately "consumes"
// the whole capacity and returns a pointer to the region and its size.
.global _malloc_wholepage

// (void *, uint64_t) realloc_wholepage(void * chunk, uint64_t additional_size);
// Resize the given memory chunk by adding at least the given amount of memory.
// Immediately "consumes" the whole capacity and returns a pointer to the new
// region and its new size.
.global _realloc_wholepage

//////////////////////////////////////////////////////////////////////////////

// Offset from memory chunk address: total size of the region.
.equ MEM_CHUNK_SIZE, -16
// Offset from memory chunk address: size asked to allocate.
.equ MEM_CHUNK_USED, -8
// Size of the metadata (size + capacity) per memory chunk.
.equ MEM_RECORD_SIZE, 16

// Size requested from caller(s).
MA_USED .req X6
// Size allocated for the memory chunk.
MA_CHUNK_SIZE .req X7

.balign 4
_malloc:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    // Remember size caller asked to allocate.
    mov MA_USED, X0

    // Get page size.
    adrp X1, _page_size@PAGE
    ldr W2, [X1, _page_size@PAGEOFF]

    // Round the request size up to nearest page size, accounting for the metadata.
    add X3, X0, #(MEM_RECORD_SIZE - 1)
    add X3, X3, X2
    sub X2, X2, #1 // Convert size to mask (size is always 1 << PAGE_SHIFT)
    bic MA_CHUNK_SIZE, X3, X2

    str XZR, [SP, #-16]! // Push null pointer on stack.

    adrp X1, _mach_task_self@PAGE
    ldr X0, [X1, _mach_task_self@PAGEOFF] // Arg 1: mach_task_self
    mov X1, SP // Arg 2: pointer to address (in/out)
    mov X2, MA_CHUNK_SIZE // Arg 3: size to allocate
    mov X3, #1 // Arg 4: VM_FLAGS_ANYWHERE | VM_MAKE_TAG(VM_MEMORY_MALLOC_LARGE)
    movk X3, #0x0300, LSL #16 // value of the flags: 0x3000001
    SYSCALL SYSCALL_MACH_VM_ALLOCATE

    cbz X0, 1f  // Check result.
    mov X0, #0  // On error, return NULL
    b 2f

1:  ldr X0, [SP] // Load pointer to allocated memory region
    add X0, X0, #MEM_RECORD_SIZE // Adjust pointer
    stp MA_CHUNK_SIZE, MA_USED, [X0, #MEM_CHUNK_SIZE] // Save chunk size and what caller wanted

2:  mov SP, FP
    ldp FP, LR, [SP], #16
    ret


.balign 4
_free:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    cbz X0, 1f // addr == NULL? Leave.

    ldr X2, [X0, #MEM_CHUNK_SIZE] // Arg 3: size to deallocate
    sub X1, X0, #MEM_RECORD_SIZE // Arg 2: pointer to region
    adrp X0, _mach_task_self@PAGE
    ldr X0, [X0, _mach_task_self@PAGEOFF] // Arg 1: mach_task_self
    SYSCALL SYSCALL_MACH_VM_DEALLOCATE

1:  mov SP, FP
    ldp FP, LR, [SP], #16
    ret


.balign 4
_malloc_capacity:
    ldp X1, X2, [X0, #MEM_CHUNK_SIZE] // Load chunk size and used space
    add X2, X2, #MEM_RECORD_SIZE // Account for the metadata at start of chunk
    sub X0, X1, X2 // Calculate remaining space.
    ret


RA_PAGE_SIZE .req X9
RA_PAGE_SIZE_W .req W9
RA_USED .req X10
RA_CHUNK_SIZE .req X11
RA_CHUNK_START .req X12
RA_TMP .req X13
RA_NEW_USED .req X14
RA_NEW_CHUNK_SIZE .req X15
RA_NEW_CHUNK_START .req X6

.balign 4
_realloc:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    cbz X0, L_realloc_malloc // addr == NULL? Delegate to malloc.

    ldp RA_CHUNK_SIZE, RA_USED, [X0, #MEM_CHUNK_SIZE] // Load chunk size and what caller wanted so far.
    mov RA_NEW_USED, X1 // Get new used amount.
    sub RA_TMP, RA_CHUNK_SIZE, #MEM_RECORD_SIZE // Available space in chunk
    cmp RA_NEW_USED, RA_TMP // Is new size <= available space?
    b.gt L_do_realloc // If no, do the expensive part.

    str RA_NEW_USED, [X0, #MEM_CHUNK_USED] // Update the used counter and leave
    // X0 is still the original pointer.
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

L_do_realloc:
    sub RA_CHUNK_START, X0, #MEM_RECORD_SIZE // Get start of chunk
    add RA_TMP, RA_CHUNK_START, RA_CHUNK_SIZE // Calculate address of next region
    str RA_TMP, [SP, #-16]! // Save on stack for call to vm_allocate

    // Get page size.
    adrp RA_PAGE_SIZE, _page_size@PAGE
    ldr RA_PAGE_SIZE_W, [RA_PAGE_SIZE, _page_size@PAGEOFF]

    // Round new capacity up to next page size, account for metadata
    add RA_NEW_CHUNK_SIZE, RA_NEW_USED, #(MEM_RECORD_SIZE - 1)
    add RA_NEW_CHUNK_SIZE, RA_NEW_CHUNK_SIZE, RA_PAGE_SIZE
    sub RA_TMP, RA_PAGE_SIZE, #1 // Convert to bitmask (is always 1 << PAGE_SHIFT)
    bic RA_NEW_CHUNK_SIZE, RA_NEW_CHUNK_SIZE, RA_TMP

    adrp X0, _mach_task_self@PAGE // Arg 1: mach_task_self
    ldr X0, [X0, _mach_task_self@PAGEOFF]
    mov X1, SP // Arg 2: pointer to address (in/out)
    sub X2, RA_NEW_CHUNK_SIZE, RA_CHUNK_SIZE // Arg 3: size to allocate
    mov X3, #1 // Arg 4: VM_FLAGS_ANYWHERE | VM_MAKE_TAG(VM_MEMORY_REALLOC)
    movk X3, #0x0600, LSL #16 // value of the flags: 0x6000001
    SYSCALL SYSCALL_MACH_VM_ALLOCATE
    cbnz X0, L_realloc_with_copy // If it failed, allocate new region and copy old

    // Success. Update record.
    stp RA_NEW_CHUNK_SIZE, RA_NEW_USED, [RA_CHUNK_START]
    add X0, RA_CHUNK_START, #MEM_RECORD_SIZE // Return old pointer
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

L_realloc_malloc:
    mov X0, X1  // Move the size to first argument
    bl _malloc  // Call malloc
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

L_realloc_with_copy:
    // Fetch a new memory region of the required size.
    str WZR, [SP, #-16]! // Push null pointer on stack

    adrp X1, _mach_task_self@PAGE
    ldr X0, [X1, _mach_task_self@PAGEOFF] // Arg 1: mach_task_self
    mov X1, SP // Arg 2: pointer to address (in/out)
    mov X2, RA_NEW_CHUNK_SIZE // Arg 3: size to allocate
    mov X3, #1 // Arg 4: VM_FLAGS_ANYWHERE | VM_MAKE_TAG(VM_MEMORY_MALLOC_LARGE)
    movk X3, #0x0300, LSL #16 // value of the flags: 0x3000001
    SYSCALL SYSCALL_MACH_VM_ALLOCATE
    cbnz X0, L_realloc_failure // Leave on failure.

    ldr RA_NEW_CHUNK_START, [SP] // Fetch start of new chunk

    // Load the src and dst pointers, but move 16 bytes in front of it.
    // The `ldr` and `str` use this offset to allow `[X0, #128]!` which saves
    // additional `add` calls.
    sub X0, RA_CHUNK_START, #16
    sub X1, RA_NEW_CHUNK_START, #16
    mov X2, RA_CHUNK_SIZE

    // The memory access is always aligned, and the chunk size is always a
    // power of 2 >= 4096. This makes memcpy easy here.

    // Use NEON registers to avoid shuffing the general-purpose registers 
    // around. Copy 128 bytes each loop iteration.
L_realloc_copy_loop:
    ldr Q0, [X0, #16]
    ldr Q1, [X0, #32]
    ldr Q2, [X0, #48]
    ldr Q3, [X0, #64]
    ldr Q4, [X0, #80]
    ldr Q5, [X0, #96]
    ldr Q6, [X0, #112]
    ldr Q7, [X0, #128]! // updates source pointer
    str Q0, [X0, #16]
    str Q1, [X0, #32]
    str Q2, [X0, #48]
    str Q3, [X0, #64]
    str Q4, [X0, #80]
    str Q5, [X0, #96]
    str Q6, [X0, #112]
    str Q7, [X0, #128]! // updates destination pointer
    subs X2, X2, #128 // Decrease remaining bytes to copy
    b.ne L_realloc_copy_loop

    // Everything is copied. Update the metadata.
    stp RA_NEW_CHUNK_SIZE, RA_NEW_USED, [RA_NEW_CHUNK_START]

    // Deallocate old chunk.
    adrp X0, _mach_task_self@PAGE
    ldr X0, [X0, _mach_task_self@PAGEOFF] // Arg 1: mach_task_self
    mov X1, RA_CHUNK_START // Arg 2: pointer to region
    mov X2, RA_CHUNK_SIZE // Arg 3: size to deallocate
    SYSCALL SYSCALL_MACH_VM_DEALLOCATE
    // Don't care about the result. If deallocation failed, there's nothing to
    // do anyway.

    // Great success! Return "user pointer" to the new chunk.
    add X0, RA_NEW_CHUNK_START, #MEM_RECORD_SIZE
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret    

L_realloc_failure:
    mov X0, #0 // Return null pointer
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret


.balign 4
_malloc_wholepage:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    bl _malloc
    cbz X0, L_mw_return // Leave if null pointer
    ldr X1, [X0, #MEM_CHUNK_SIZE] // Get chunk size
    sub X1, X1, #MEM_RECORD_SIZE  // Account for header
    str X1, [X0, #MEM_CHUNK_USED] // "Consume" it.

L_mw_return:
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret


.balign 4
_realloc_wholepage:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    ldp RA_CHUNK_SIZE, RA_USED, [X0, #MEM_CHUNK_SIZE] // Load chunk size and what caller wanted so far.
    add X2, RA_USED, X1 // Does the requested size still fit in the current chunk?
    add X2, X2, #MEM_RECORD_SIZE
    cmp RA_CHUNK_SIZE, X2
    b.gt L_rw_cheap

    // Get page size.
    adrp RA_PAGE_SIZE, _page_size@PAGE
    ldr RA_PAGE_SIZE_W, [RA_PAGE_SIZE, _page_size@PAGEOFF]

    // Calculate how to call `realloc` to get the excact result wanted.
    add X2, RA_CHUNK_SIZE, X1
    add X2, X2, RA_PAGE_SIZE
    sub X3, RA_PAGE_SIZE, #1 // Convert to bitmask (is always 1 << PAGE_SHIFT)
    bic X3, X2, X3 // Round to page size
    sub X1, X3, #MEM_RECORD_SIZE // Account for header size

    bl _realloc
    cbz X0, L_rw_failure

    ldr X1, [X0, #MEM_CHUNK_USED] // Get the used amount
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

L_rw_cheap:
    // The cheap (and somewhat surprising) case: just consome everything that's left.
    sub X1, RA_CHUNK_SIZE, #MEM_RECORD_SIZE  // Account for header
    str X1, [X0, #MEM_CHUNK_USED] // "Consume" it.

    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

L_rw_failure:
    mov X1, #0
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret
