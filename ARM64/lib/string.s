//////////////////////////////////////////////////////////////////////////////

// uint64_t strlen(char const * const str);
// Get length of a null-terminated string.
.global _strlen

// void iterate_chars(char const * const str, uint64_t len, void * context, void (*handler) (char c, void * context));
// Iterate over the characters in a string of given length. For each character,
// the given handler is called with an additional context.
.global _iterate_chars

//////////////////////////////////////////////////////////////////////////////

SL_COUNT .req X0
SL_ALIGNED .req X1
SL_INDEX .req X2
SL_TMP .req X3
SL_BUFFER .req X5

// Semi-naive version assuming little-endian. Fetches aligned 8-byte chunks, but then simply
// iterates over the 8 fetched bytes.
.balign 4
_strlen:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    cmp X0, XZR                     // Is the input a NULL pointer?
    b.eq L_sl_return                // If so, just leave. X0 = return value = already 0.

    bic SL_ALIGNED, X0, #7          // Round down to nearest aligned address.
    ldr SL_BUFFER, [SL_ALIGNED]     // Load 8 bytes
    subs SL_INDEX, X0, SL_ALIGNED   // By how many bytes was the pointer misaligned?
    mov SL_COUNT, #0                // SL_COUNT = 0

    b.eq L_sl_aligned               // Skip shifting if pointer was already aligned.
    lsl SL_TMP, SL_INDEX, #3        // Misaligned in bits: SL_TMP = SL_INDEX * 8
    lsrv SL_BUFFER, SL_BUFFER, SL_TMP // Shift to offset the misalignment.

L_sl_aligned:
    mov SL_TMP, #8
    sub SL_INDEX, SL_TMP, SL_INDEX  // Start loop: convert misaligned offset into number of bytes
                                    // left to process: SL_INDEX = 8 - SL_INDEX, 

L_sl_loop:
    tst SL_BUFFER, #0xFF            // Is the rightmost byte zero?
    b.eq L_sl_return                // If so, we're done.
    add SL_COUNT, SL_COUNT, #1      // COUNT += 1
    subs SL_INDEX, SL_INDEX, #1     // INDEX -= 1
    b.eq L_sl_load_next             // if INDEX == 0 load next 8 bytes
    lsr SL_BUFFER, SL_BUFFER, #8    // Check next byte
    b L_sl_loop

L_sl_load_next:
    ldr SL_BUFFER, [SL_ALIGNED, #8]! // Fetch next 8 bytes
    mov SL_INDEX, #8
    b L_sl_loop

L_sl_return:
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret


IC_COUNT .req X20
IC_ALIGNED .req X21
IC_INDEX .req X22
IC_TMP .req X23
IC_BUFFER .req X24
IC_HANDLER .req X25
IC_CONTEXT .req X26

// Assumes little-endian. Fetches aligned 8-byte chunks.
.balign 4
_iterate_chars:
    stp FP, LR, [SP, #-16]!
    mov FP, SP
    stp X20, X21, [SP, #-16]!
    stp X22, X23, [SP, #-16]!
    stp X24, X25, [SP, #-16]!
    str X26, [SP, #-16]!

    cmp X0, XZR                     // Is the input a NULL pointer?
    b.eq L_ic_return                // If so, just leave. X0 = return value = already 0.

    cmp X1, XZR                     // Is length 0?
    b.eq L_ic_return0               // If so, just leave. X0 = return value = already 0.

    mov IC_COUNT, X1                // Save some values from the volatile registers
    mov IC_HANDLER, X2
    mov IC_CONTEXT, X3

    bic IC_ALIGNED, X0, #7          // Round down to nearest aligned address.
    ldr IC_BUFFER, [IC_ALIGNED]     // Load 8 bytes
    subs IC_INDEX, X0, IC_ALIGNED   // By how many bytes was the pointer misaligned?

    b.eq L_ic_aligned               // Skip shifting if pointer was already aligned.
    lsl IC_TMP, IC_INDEX, #3        // Misaligned in bits: IC_TMP = IC_INDEX * 8
    lsrv IC_BUFFER, IC_BUFFER, IC_TMP // Shift to offset the misalignment.

L_ic_aligned:
    mov IC_TMP, #8
    sub IC_INDEX, IC_TMP, IC_INDEX  // Start loop: convert misaligned offset into number of bytes
                                    // left to process: INDEX = 8 - INDEX, 

L_ic_loop:
    and X0, IC_BUFFER, #0xFF        // Extract byte for handler.
    mov X1, IC_CONTEXT              // Pass context.
    blr IC_HANDLER                  // Call handler.

    subs IC_COUNT, IC_COUNT, #1     // Decrement length.
    b.eq L_ic_return0               // All bytes processed.

    subs IC_INDEX, IC_INDEX, #1     // INDEX -= 1
    b.eq L_ic_load_next             // if INDEX == 0 load next 8 bytes
    lsr IC_BUFFER, IC_BUFFER, #8    // Check next byte
    b L_ic_loop

L_ic_load_next:
    ldr IC_BUFFER, [IC_ALIGNED, #8]! // Fetch next 8 bytes
    mov IC_INDEX, #8
    b L_ic_loop

L_ic_return0:
    mov X0, #0
L_ic_return:
    ldr X26, [SP], #16
    ldp X24, X25, [SP], #16
    ldp X22, X23, [SP], #16
    ldp X20, X21, [SP], #16
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret
