.include "syscall.inc"

//////////////////////////////////////////////////////////////////////////////

// void mach_init(void);
// Initialize Mach support. Call this early.
.global _mach_init

// extern uint64_t mach_task_self;
// The Mach port number of the current task. Unlike in real C code, this is a
// variable and not a function.
.global _mach_task_self

// extern uint32_t page_size;
// Size of a memory page.
.global _page_size

//////////////////////////////////////////////////////////////////////////////

// From xnu/osfmk/arm/cpu_abilities.h
_COMM_PAGE64_RO_ADDRESS = 0x0000000FFFFF4000
_COMM_PAGE_USER_PAGE_SHIFT_64 = (_COMM_PAGE64_RO_ADDRESS + 0x025)

// The calculated page size.
.balign 4
.comm _page_size, 4

// Cached mach_task_self value.
.balign 8
.comm _mach_task_self, 8

.text
.balign 4
_mach_init:
    // Read the page size shift used by kernel from the comm page.
    mov X0, #(_COMM_PAGE_USER_PAGE_SHIFT_64 & 0xFFFF)
    movk X0, #((_COMM_PAGE_USER_PAGE_SHIFT_64 >> 16) & 0xFFFF), LSL #16
    movk X0, #((_COMM_PAGE_USER_PAGE_SHIFT_64 >> 32) & 0xFFFF), LSL #32
    movk X0, #((_COMM_PAGE_USER_PAGE_SHIFT_64 >> 48) & 0xFFFF), LSL #48
    ldrb W1, [X0]
    // Calculate the page size using the shift value.
    mov W2, #1
    lslv W3, W2, W1
    // Store it.
    adrp X4, _page_size@PAGE
    str W3, [X4, _page_size@PAGEOFF]

    // Get and cache mach_task_self
    SYSCALL SYSCALL_MACH_TASK_SELF
    adrp X1, _mach_task_self@PAGE
    str X0, [X1, _mach_task_self@PAGEOFF]

    ret
