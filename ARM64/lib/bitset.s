.include "utils.inc"

//////////////////////////////////////////////////////////////////////////////

// void bitset_reset(void * bs, uint64_t size);
// Resets the bitset to all 0s.
.global _bitset_reset

// void bitset_set(void * bs, uint64_t bit);
// Set bit value.
.global _bitset_set

// void bitset_clear(void * bs, uint64_t bit);
// Clear bit value.
.global _bitset_clear

// bool bitset_get(void * bs, uint64_t bit);
// Get bit value.
.global _bitset_get

//////////////////////////////////////////////////////////////////////////////

.balign 4
_bitset_reset:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    ROUND_UP_REG X2, X1, X3, 8
    mov X1, #0
    lsr X2, X2, #3
    bl _memset

    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

.balign 4
_bitset_set:
    lsr X4, X1, #3      // Calculate byte offset.
    lsl X5, X4, #3      // Calculate bit offset in byte.
    sub X6, X1, X5
    ldrb W7, [X0, X4]   // Fetch byte.
    mov W2, #1          
    lsl W5, W2, W6      // Shift bit as needed.
    orr W7, W7, W5      // Set bit
    strb W7, [X0, X4]   // Store byte.
    ret

.balign 4
_bitset_clear:
    lsr X4, X1, #3      // Calculate byte offset.
    lsl X5, X4, #3      // Calculate bit offset in byte.
    sub X6, X1, X5
    ldrb W7, [X0, X4]   // Fetch byte.
    mov W2, #1
    lsl W5, W2, W6      // Shift bit as needed.
    mvn W5, W5          // Invert mask.
    and W7, W7, W5      // Clear bit.
    strb W7, [X0, X4]   // Store byte.
    ret

.balign 4
_bitset_get:
    lsr X4, X1, #3      // Calculate byte offset.
    lsl X5, X4, #3      // Calculate bit offset in byte.
    sub X6, X1, X5
    ldrb W7, [X0, X4]   // Fetch byte.
    lsr W7, W7, W6      // Extract requested bit.
    and W0, W7, #1
    ret
