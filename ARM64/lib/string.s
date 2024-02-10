//////////////////////////////////////////////////////////////////////////////

// uint64_t strlen(char const * const str);
// Get length of a null-terminated string.
.global _strlen

// void iterate_chars(char const * const str, uint64_t len, void * context, void (*handler) (char c, void * context));
// Iterate over the characters in a string of given length. For each character,
// the given handler is called with an additional context.
.global _iterate_chars

// void iterate_lines(char const * const str, uint64_t len, void * context, void (*handler) (char const * const line, uint64_t len, void * context));
// Iterate over the lines in a string of a given length. For each line (start
// address + length), the given handler is called with an additional context.
.global _iterate_lines

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


IL_COUNT .req X20
IL_ALIGNED .req X21
IL_INDEX .req X22
IL_TMP .req X23
IL_BUFFER .req X24
IL_HANDLER .req X25
IL_CONTEXT .req X26
IL_LINESTART .req X27

// Assumes little-endian. Fetches aligned 8-byte chunks.
.balign 4
_iterate_lines:
    stp FP, LR, [SP, #-16]!
    mov FP, SP
    stp X20, X21, [SP, #-16]!
    stp X22, X23, [SP, #-16]!
    stp X24, X25, [SP, #-16]!
    stp X26, X27, [SP, #-16]!
    str X28, [SP, #-16]!

    cmp X0, XZR                     // Is the input a NULL pointer?
    b.eq L_il_return                // If so, just leave. X0 = return value = already 0.

    cmp X1, XZR                     // Is length 0?
    b.eq L_il_return0               // If so, just leave. X0 = return value = already 0.

    mov IL_COUNT, X1                // Save some values from the volatile registers
    mov IL_HANDLER, X2
    mov IL_CONTEXT, X3

    mov IL_LINESTART, X0            // Set first line start
    bic IL_ALIGNED, X0, #7          // Round down to nearest aligned address.
    ldr IL_BUFFER, [IL_ALIGNED]     // Load 8 bytes
    subs IL_INDEX, X0, IL_ALIGNED   // By how many bytes was the pointer misaligned?

    b.eq L_il_aligned               // Skip shifting if pointer was already aligned.
    lsl IL_TMP, IL_INDEX, #3        // Misaligned in bits: IL_TMP = IL_INDEX * 8
    lsrv IL_BUFFER, IL_BUFFER, IL_TMP // Shift to offset the misalignment.

L_il_aligned:
    mov IL_TMP, #8
    sub IL_INDEX, IL_TMP, IL_INDEX  // Start loop: convert misaligned offset into number of bytes
                                    // left to process: INDEX = 8 - INDEX, 

L_il_loop:
    and X0, IL_BUFFER, #0xFF        // Extract byte for handler.
    cmp X0, #'\n'                   // Is it a newline?
    b.ne L_il_next_char             // No? Fetch next char.

    mov X0, IL_LINESTART            // Arg 1: pointer to line start.
    sub X1, IL_TMP, IL_INDEX        // Arg 2: line length. Calculate offset from aligned.
    add X1, X1, IL_ALIGNED          // Add offset to aligned. Now we have a pointer to the line end.
    add IL_LINESTART, X1, #1        // Save next line start.
    sub X1, X1, X0                  // Calculate length.
    mov X2, IL_CONTEXT              // Arg 3: Context.
    blr IL_HANDLER                  // Call handler

L_il_next_char:
    subs IL_COUNT, IL_COUNT, #1     // Decrement length.
    b.eq L_il_end_of_string         // All bytes processed.

    subs IL_INDEX, IL_INDEX, #1     // INDEX -= 1
    b.eq L_il_load_next             // if INDEX == 0 load next 8 bytes
    lsr IL_BUFFER, IL_BUFFER, #8    // Check next byte
    b L_il_loop

L_il_load_next:
    ldr IL_BUFFER, [IL_ALIGNED, #8]! // Fetch next 8 bytes
    mov IL_INDEX, #8
    b L_il_loop

L_il_end_of_string:
    sub IL_INDEX, IL_INDEX, #1      // Need to do the skipped INDEX -= 1
    sub X1, IL_TMP, IL_INDEX        // Calculate offset from aligned.
    add X1, X1, IL_ALIGNED          // Add offset to aligned. Now we have a pointer to the line end.
    cmp X1, IL_LINESTART            // Did the string end with a newline?
    b.eq L_il_return0               // If so, we're done.

    // Otherwise, the handler needs to be called one more time.
    mov X0, IL_LINESTART            // Arg 1: pointer to line start.
    sub X1, X1, IL_LINESTART        // Arg 2: Calculate length.
    mov X2, IL_CONTEXT              // Arg 3: Context.
    blr IL_HANDLER                  // Call handler

L_il_return0:
    mov X0, #0
L_il_return:
    ldr X28, [SP], #16
    ldp X26, X27, [SP], #16
    ldp X24, X25, [SP], #16
    ldp X22, X23, [SP], #16
    ldp X20, X21, [SP], #16
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret
