//////////////////////////////////////////////////////////////////////////////

// uint64_t strlen(void const * const str);
// Get length of a null-terminated string.
.global _strlen

//////////////////////////////////////////////////////////////////////////////

COUNT .req X0
ALIGNED .req X1
INDEX .req X2
TMP .req X3
BUFFER .req X5

// Semi-naive version assuming little-endian. Fetches aligned 8-byte chunks, but then simply
// iterates over the 8 fetched bytes.
.align 4
_strlen:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    cmp X0, XZR                     // Is the input a NULL pointer?
    b.eq L_return                   // If so, just leave. X0 = return value = already 0.

    bic ALIGNED, COUNT, #7          // Round down to nearest aligned address.
    ldr BUFFER, [ALIGNED]           // Load 8 bytes
    subs INDEX, X0, ALIGNED         // By how many bytes was the pointer misaligned?
    mov COUNT, #0                   // COUNT = 0

    b.eq L_aligned                  // Skip shifting if pointer was already aligned.
    lsl TMP, INDEX, #3              // Misaligned in bits: TMP = INDEX * 8
    lsrv BUFFER, BUFFER, TMP        // Shift to offset the misalignment.

L_aligned:
    mov TMP, #8
    sub INDEX, TMP, INDEX           // Start loop: convert misaligned offset into number of bytes
                                    // left to process: INDEX = 8 - INDEX, 

L_loop:
    tst BUFFER, #0xFF               // Is the rightmost byte zero?
    b.eq L_return                   // If so, we're done.
    add COUNT, COUNT, #1            // COUNT += 1
    subs INDEX, INDEX, #1           // INDEX -= 1
    b.eq L_load_next                // if INDEX > 0 load next 8 bytes
    lsr BUFFER, BUFFER, #8          // Check next byte
    b L_loop

L_load_next:
    ldr BUFFER, [ALIGNED, #8]!       // Fetch next 8 bytes
    mov INDEX, #8
    b L_loop

L_return:
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret
