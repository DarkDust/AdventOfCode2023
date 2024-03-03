//////////////////////////////////////////////////////////////////////////////

// void * memcpy(void * dst, void * src, uint64_t len);
// Copy content from one memory region to another. Handles overlap (unlike
// POSIX memcpy).
.global _memcpy

//////////////////////////////////////////////////////////////////////////////

// Ensure the original destination stays in X0, as return value.
MC_SRC .req X1
MC_COUNT .req X2
MC_DST .req X3
MC_SRCEND .req X4
MC_DSTEND .req X5
MC_ACC_X .req X6
MC_ACC_W .req W6
MC_COUNT_CHUNK .req X7

.balign 4
_memcpy:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    // Save destination so that X0 remains untouched for return.
    mov MC_DST, X0

    // Calculate the end adresses for backwards copies.
    add MC_SRCEND, MC_SRC, MC_COUNT
    add MC_DSTEND, MC_DST, MC_COUNT

    cmp MC_COUNT, #128
    b.cs L_mc_large // if count >= 128, do large copy

    // Small copy. Copy backwards to handle overlap.
L_mc_small:
    tbz MC_COUNT, #6, 1f // Skip if (count & 64) == 0
    ldp Q0, Q1, [MC_SRCEND, #-32]!
    ldp Q2, Q3, [MC_SRCEND, #-32]!
    stp Q0, Q1, [MC_DSTEND, #-32]!
    stp Q2, Q3, [MC_DSTEND, #-32]!

1:  tbz MC_COUNT, #5, 2f // Skip if (count & 32) == 0
    ldp Q0, Q1, [MC_SRCEND, #-32]!
    stp Q0, Q1, [MC_DSTEND, #-32]!

2:  tbz MC_COUNT, #4, 3f // Skip if (count & 16) == 0
    ldr Q0, [MC_SRCEND, #-16]!
    str Q0, [MC_DSTEND, #-16]!

3:  tbz MC_COUNT, #3, 4f // Skip if (count & 8) == 0
    ldr MC_ACC_X, [MC_SRCEND, #-8]!
    str MC_ACC_X, [MC_DSTEND, #-8]!

4:  tbz MC_COUNT, #2, 5f // Skip if (count & 4) == 0
    ldr MC_ACC_W, [MC_SRCEND, #-4]!
    str MC_ACC_W, [MC_DSTEND, #-4]!

5:  tbz MC_COUNT, #1, 6f // Skip if (count & 2) == 0
    ldrh MC_ACC_W, [MC_SRCEND, #-2]!
    strh MC_ACC_W, [MC_DSTEND, #-2]!

6:  tbz MC_COUNT, #0, L_mc_done // Skip if (count & 1) == 0
    ldrb MC_ACC_W, [MC_SRCEND, #-1]!
    strb MC_ACC_W, [MC_DSTEND, #-1]!

L_mc_done:
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

L_mc_large:
    // Check if we can do a forward copy
    cmp MC_SRCEND, MC_DST // If end of source is past start of destination, need top copy backwards
    b.lo L_mc_large_backward

    // We can only align either the source or destination. Let's align the destination to 64 bit.
    tst MC_DST, #63
    b.eq L_mc_large_forward_aligned // Skip if already aligned

    tbz MC_DST, #5, 1f // Skip if (dst & 32) == 0
    ldp Q0, Q1, [MC_SRC], #32
    stp Q0, Q1, [MC_DST], #32

1:  tbz MC_DST, #4, 2f // Skip if (dst & 16) == 0
    ldr Q0, [MC_SRC], #16
    str Q0, [MC_DST], #16

2:  tbz MC_DST, #3, 3f // Skip if (dst & 8) == 0
    ldr MC_ACC_X, [MC_SRC], #8
    str MC_ACC_X, [MC_DST], #8

3:  tbz MC_DST, #2, 4f // Skip if (dst & 4) == 0
    ldr MC_ACC_W, [MC_SRC], #4
    str MC_ACC_W, [MC_DST], #4

4:  tbz MC_DST, #1, 5f // Skip if (dst & 2) == 0
    ldrh MC_ACC_W, [MC_SRC], #2
    strh MC_ACC_W, [MC_DST], #2

5:  tbz MC_DST, #0, L_mc_large_forward_aligned // Skip if (dst & 1) == 0
    ldrb MC_ACC_W, [MC_SRC], #1
    strb MC_ACC_W, [MC_DST], #1

L_mc_large_forward_aligned:
    sub MC_COUNT_CHUNK, MC_DST, X0 // Calculate how many bytes have been copied so far.
    sub MC_COUNT_CHUNK, MC_COUNT, MC_COUNT_CHUNK // Adjust count accordingly.
    cmp MC_COUNT_CHUNK, #128 // Remaining count < 128?
    b.lo L_mc_large_forward_remaining // Oh no, worst case: cannot copy 128 byte chunks.
    lsr MC_COUNT_CHUNK, MC_COUNT_CHUNK, #7 // Number of 128 byte loops = adjusted count / 128

L_mc_large_forward_loop:
    // Copy a 128 byte chunk
    ldp Q0, Q1, [MC_SRC], #32
    ldp Q2, Q3, [MC_SRC], #32
    ldp Q4, Q5, [MC_SRC], #32
    ldp Q6, Q7, [MC_SRC], #32
    stp Q0, Q1, [MC_DST], #32
    stp Q2, Q3, [MC_DST], #32
    stp Q4, Q5, [MC_DST], #32
    stp Q6, Q7, [MC_DST], #32
    subs MC_COUNT_CHUNK, MC_COUNT_CHUNK, #1
    b.ne L_mc_large_forward_loop

L_mc_large_forward_remaining:
    // Adjust the count by the total copied bytes, then delegate to the "small" handling for the
    // remainder.
    sub MC_ACC_X, MC_DST, X0
    subs MC_COUNT, MC_COUNT, MC_ACC_X
    b.eq L_mc_done // Count == 0? Exit!
    b L_mc_small

L_mc_large_backward:
    // Align the destination to 64 bit
    tst MC_DSTEND, #63
    b.eq L_mc_large_backward_aligned

    tbz MC_DSTEND, #0, 1f // Skip if (dstend & 1) == 0
    ldrb MC_ACC_W, [MC_SRCEND, #-1]!
    strb MC_ACC_W, [MC_DSTEND, #-1]!

1:  tbz MC_DSTEND, #1, 2f // Skip if (dstend & 2) == 0
    ldrh MC_ACC_W, [MC_SRCEND, #-2]!
    strh MC_ACC_W, [MC_DSTEND, #-2]!

2:  tbz MC_DSTEND, #2, 3f // Skip if (dstend & 4) == 0
    ldr MC_ACC_W, [MC_SRCEND, #-4]!
    str MC_ACC_W, [MC_DSTEND, #-4]!

3:  tbz MC_DSTEND, #3, 4f // Skip if (dstend & 8) == 0
    ldr MC_ACC_X, [MC_SRCEND, #-8]!
    str MC_ACC_X, [MC_DSTEND, #-8]!

4:  tbz MC_DSTEND, #4, 5f // Skip if (dstend & 16) == 0
    ldr Q0, [MC_SRCEND, #-16]!
    str Q0, [MC_DSTEND, #-16]!

5:  tbz MC_DSTEND, #5, L_mc_large_backward_aligned // Skip if (dstend & 32) == 0
    ldp Q0, Q1, [MC_SRCEND, #-32]!
    stp Q0, Q1, [MC_DSTEND, #-32]!

L_mc_large_backward_aligned:
    add MC_COUNT_CHUNK, MC_DST, MC_COUNT // Calculate original end
    sub MC_COUNT_CHUNK, MC_COUNT_CHUNK, MC_DSTEND // Calculate how many bytes have been copied so far.
    sub MC_COUNT_CHUNK, MC_COUNT, MC_COUNT_CHUNK // Adjust count accordingly.
    cmp MC_COUNT_CHUNK, #128 // Count < 128?
    b.lo L_mc_large_backward_remaining // Oh no, worst case: cannot copy 128 byte chunks
    lsr MC_COUNT_CHUNK, MC_COUNT_CHUNK, #7 // Number of 128 byte loops = adjusted count / 128

L_mc_large_backward_loop:
    // Copy a 128 byte chunk
    ldp Q0, Q1, [MC_SRCEND, #-32]!
    ldp Q2, Q3, [MC_SRCEND, #-32]!
    ldp Q4, Q5, [MC_SRCEND, #-32]!
    ldp Q6, Q7, [MC_SRCEND, #-32]!
    stp Q0, Q1, [MC_DSTEND, #-32]!
    stp Q2, Q3, [MC_DSTEND, #-32]!
    stp Q4, Q5, [MC_DSTEND, #-32]!
    stp Q6, Q7, [MC_DSTEND, #-32]!
    subs MC_COUNT_CHUNK, MC_COUNT_CHUNK, #1
    b.ne L_mc_large_backward_loop

L_mc_large_backward_remaining:
    // Adjust the count by the total copied bytes, then delegate to the "small" handling for the
    // remainder.
    add MC_ACC_X, MC_DST, MC_COUNT // Calculate original dstend
    sub MC_ACC_X, MC_DSTEND, MC_ACC_X // Calculate amount of bytes copied so far
    subs MC_COUNT, MC_COUNT, MC_ACC_X // Adjust count
    b.eq L_mc_done // Count == 0? Exit!
    b L_mc_small
