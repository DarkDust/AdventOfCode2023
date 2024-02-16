//////////////////////////////////////////////////////////////////////////////

// void * memset(void * start, uint8_t c, uint64_t len);
// Fills a memory region with a given character/byte. Returns its first
// argument (since that's what the C `memset` is doing).
.global _memset

//////////////////////////////////////////////////////////////////////////////

MS_CHAR .req X1
MS_CHAR_W .req W1
MS_LEN .req X2
MS_POINTER .req X3
MS_TMP .req X4
MS_COUNTER .req X5

.balign 4
_memset:
    stp FP, LR, [SP, #-16]!
    mov FP, SP
    mov MS_POINTER, X0      // Save the argument

    // Copy the byte so the register contains 8 copies of it.
    and MS_CHAR_W, MS_CHAR_W, #0xFF                 // Clip to byte.
    orr MS_CHAR_W, MS_CHAR_W, MS_CHAR_W, LSL #8     // Copy the byte to fill all 8 bytes in register.
    orr MS_CHAR_W, MS_CHAR_W, MS_CHAR_W, LSL #16
    orr MS_CHAR, MS_CHAR, MS_CHAR, LSL #32

    cmp MS_LEN, #16                         // Is the length >= 16?
    b.lo L_ms_remainder                     // If not, skip to the remainder.

    // First, do an unaligned write, then align the pointer. Might write a few bytes twice.
    stp MS_CHAR, MS_CHAR, [MS_POINTER], #16 // Write 16 bytes in one go.
    sub MS_LEN, MS_LEN, #16
    and MS_TMP, MS_POINTER, #0xF            // By how many bytes to shift for alignment?
    bic MS_POINTER, MS_POINTER, #0xF        // Align to 16 bytes
    add MS_LEN, MS_LEN, MS_TMP              // Adjust remaining length.

    // Now the pointer is 16-byte aligned. Try to copy 64 byte blocks. Like with Duff's Device,
    // jump into the loop body to account for the remainder of the 16-byte chunks.
    // TODO: Might be able to optimize a tiny bit using RMIF
    lsr MS_COUNTER, MS_LEN, #4              // counter = len / 16
    ands MS_TMP, MS_COUNTER, #3             // tmp = counter % 4 (for jump into main loop)
    sub MS_COUNTER, MS_COUNTER, MS_TMP      // Adjust counter. Its value is now 4-chunks aligned.
    b.eq 1f                                 // tmp == 0?
    cmp MS_TMP, #2
    b.hi 2f                                 // tmp == 3?
    b.eq 3f                                 // tmp == 2?
    b 4f                                    // tmp == 1

L_ms_loop:
    sub MS_COUNTER, MS_COUNTER, #4          // counter -= 4
1:  stp MS_CHAR, MS_CHAR, [MS_POINTER], #16 // Copy 16 bytes
2:  stp MS_CHAR, MS_CHAR, [MS_POINTER], #16 // Copy 16 bytes
3:  stp MS_CHAR, MS_CHAR, [MS_POINTER], #16 // Copy 16 bytes
4:  stp MS_CHAR, MS_CHAR, [MS_POINTER], #16 // Copy 16 bytes
    cbnz MS_COUNTER, L_ms_loop              // Loop until all large chunk are copied.

L_ms_remainder:
    // Here, MS_LEN is either <= 15, or we came here via the "chunks" path above. In the later
    // case, it's also the last four bits that matter for the remainder handling.
    // Just need to test for the bits in the length and skip over portions if the bit is set.
    tbz MS_LEN, #3, 1f              // MS_LEN & 0b1000? If not, skip.
    str MS_CHAR, [MS_POINTER], #8   // Store and advance by 8
1:  tbz MS_LEN, #2, 2f              // MS_LEN & 0b0100? If not, skip.
    str MS_CHAR_W, [MS_POINTER], #4 // Store and advance by 4
2:  tbz MS_LEN, #1, 3f              // MS_LEN & 0b0010? If not, skip.
    strh MS_CHAR_W, [MS_POINTER], #2// Store and avancde by 2
3:  tbz MS_LEN, #0, 4f              // MS_LEN & 0xb0001? If not, skip.
    strb MS_CHAR_W, [MS_POINTER]
4:

    // Return. X0 is still unchanged.
    ldp FP, LR, [SP], #16
    ret
