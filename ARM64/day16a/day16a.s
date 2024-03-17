.include "syscall.inc"
.include "utils.inc"

.equ CONTRAPTION_FIELDS, 0
.equ CONTRAPTION_WIDTH, 8
.equ CONTRAPTION_HEIGHT, 16
.equ CONTRAPTION_ENERGIZED, 24
.equ CONTRAPTION_CYCLE, 32
CONTRAPTION_TOTAL_SIZE = 40

.equ FIELD_EMPTY, 0
.equ FIELD_MIRROR_SLASH, 1
.equ FIELD_MIRROR_BACKSLASH, 2
.equ FIELD_SPLIT_VERTICAL, 3
.equ FIELD_SPLIT_HORIZONTAL, 4

// For the directions, omit 0 so functions can return 0 for "nil".
// The values are used as offsets in jump tables!
.equ DIR_NORTH, 1
.equ DIR_EAST, 2
.equ DIR_SOUTH, 3
.equ DIR_WEST, 4

// A beam consists of three parts: X coordinate, Y coordinate, direction.
// The contraption's dimension is actually "small", 110x110. So 8 bits per coordinate is enough,
// let's be generous and use 16 bits. For consistency, also use 16 bits for the direction.
.equ BEAM_SHIFT_X, 32
.equ BEAM_SHIFT_Y, 16
.equ BEAM_SHIFT_DIR, 0
.equ BEAM_PART_MASK, 0xFFFF
.equ BEAM_PART_BITS, 16

// Main data structure.
.comm contraption, CONTRAPTION_TOTAL_SIZE


// Pack a beam into a single value. If any of the values is larger than 16 bits, it gets cut off.
// Can use the `dir` as `target`.
.macro PACK_BEAM target, x, y, dir
    and \target, \dir, #BEAM_PART_MASK
    bfi \target, \x, #BEAM_SHIFT_X, #BEAM_PART_BITS
    bfi \target, \y, #BEAM_SHIFT_Y, #BEAM_PART_BITS
.endmacro

// Unpack a beam value into components.
// Can use the `dir` as `target`.
.macro UNPACK_BEAM source, x, y, dir
    ubfx \x, \source, #BEAM_SHIFT_X, #BEAM_PART_BITS
    ubfx \y, \source, #BEAM_SHIFT_Y, #BEAM_PART_BITS
    ubfx \dir, \source, #BEAM_SHIFT_DIR, #BEAM_PART_BITS
.endmacro


.text
.global _main
.balign 4
_main:
    bl _mach_init

    // Setup time and remember current timestamp.
    bl _setup_time
    bl _time_nanoseconds
    str X0, [SP, #-16]!

    bl _part1

    // Get "now" and calculate elapsed time.
    bl _time_nanoseconds
    ldr X1, [SP]
    str X0, [SP]
    sub X0, X0, X1
    bl _print_elapsed

    bl _part2

    // Get "now" and calculate elapsed time.
    bl _time_nanoseconds
    ldr X1, [SP]
    sub X0, X0, X1
    bl _print_elapsed

    mov X0, #0
    SYSCALL SYSCALL_EXIT


.balign 4
_part1:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    PRINT_STRING str_part1

    // Initialize data structures.
    bl init_contraption

    mov X1, #0
    mov X2, #0
    mov X3, #DIR_EAST
    bl trace_from // trace_from(contraption, 0, 0, EAST)

    bl _print_uint64
    bl _print_newline

    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

.balign 4
_part2:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    PRINT_STRING str_part2

    // Initialize data structures.
    bl init_contraption

    mov X1, #0
    mov X2, #0
    mov X3, #DIR_EAST
    bl trace_beams_from_all_sides

    bl _print_uint64
    bl _print_newline

    mov SP, FP
    ldp FP, LR, [SP], #16
    ret


.balign 4
init_contraption:
    stp FP, LR, [SP, #-16]!
    mov FP, SP
    stp X20, X21, [SP, #-16]!

    // Load the input string.
    adrp X0, str_input@PAGE
    add X0, X0, str_input@PAGEOFF
    mov X1, #str_input_len

    // Load contraption address.
    adrp X20, contraption@PAGE
    add X20, X20, contraption@PAGEOFF

    bl determine_field_size

    str X0, [X20, #CONTRAPTION_WIDTH]
    str X1, [X20, #CONTRAPTION_HEIGHT]

    mul X21, X0, X1 // Calculate number of fields.
    mov X0, X21 // Allocate fields array
    bl _array1_create_with_length
    str X0, [X20, #CONTRAPTION_FIELDS] // Saves loading the input string address again.

    // "Allocate" some space on the stack for parse_contraption_handler and place some values.
    // Offset 0: X (0)
    // Offset 4: Y (0)
    // Offset 8: Pointer to contraption.
    // Ofset 16: width
    stp XZR, X0, [SP, #-16]!
    stp XZR, X20, [SP, #-16]!

    // Parse the contraption.
    // Load the input string again.
    adrp X0, str_input@PAGE
    add X0, X0, str_input@PAGEOFF
    mov X1, #str_input_len
    mov X2, SP // Pointer to data for parse_contraption_handler
    adrp X3, parse_contraption_handler@PAGE
    add X3, X3, parse_contraption_handler@PAGEOFF
    bl _iterate_chars

    // Allocate array for the "energized" fields.
    mov X0, X21
    bl _array1_create_with_length
    str X0, [X20, #CONTRAPTION_ENERGIZED]

    lsl X0, X21, #2 // For cycle detector, combine coordinates with direction
    bl _array1_create_with_length
    str X0, [X20, #CONTRAPTION_CYCLE]

    mov X0, X20
    ldp X20, X21, [SP], #16
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret


// Determine the size of the input.
// X0 = Input string, X1 = Input string length.
// Returns X dimension in X0, and Y dimension in X1
.equ FS_X_OFFSET, 0
.equ FS_Y_OFFSET, 4
.balign 4
determine_field_size:
    stp FP, LR, [SP, #-16]!
    mov FP, SP
    // Allocate on the stack
    stp XZR, XZR, [SP, #-16]!

    // Iterate over the input lines.
    mov X2, SP
    adrp X3, determine_field_size_handler@PAGE
    add X3, X3, determine_field_size_handler@PAGEOFF
    bl _iterate_lines

    // Get return values.
    ldr W0, [SP, #FS_X_OFFSET]
    ldr W1, [SP, #FS_Y_OFFSET]

    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

.balign 4
determine_field_size_handler:
    str W1, [X2, #FS_X_OFFSET]  // Store X dimension. Unnecessary to do it every iteration, but is
                                // it worth trying to get rid of it? Might be faster doing the
                                // store every time.
    ldr W3, [X2, #FS_Y_OFFSET]  // Get current Y dimension.
    add W3, W3, #1              // y += 1
    str W3, [X2, #FS_Y_OFFSET]  // Store it.
    ret

.equ PC_X_OFFSET, 0
.equ PC_Y_OFFSET, 4
.equ PC_CONTRAPTION_OFFSET, 8
.balign 4
parse_contraption:

PC_X .req W10
PC_Y .req W11
PC_CONTRAPTION .req X12
PC_WIDTH .req X13
PC_WIDTH_W .req W13
.balign 4
parse_contraption_handler:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    // Parse the field.
    cmp X0, '.'
    b.eq 1f
    cmp X0, '\\'
    b.eq 2f
    cmp X0, '/'
    b.eq 3f
    cmp X0, '-'
    b.eq 4f
    cmp X0, '|'
    b.eq 5f
    cmp X0, '\n'
    b.eq L_pc_newline
    RUNTIME_ERROR str_invalid_contraption_char

1:  mov X2, #FIELD_EMPTY
    b L_pc_push
2:  mov X2, #FIELD_MIRROR_BACKSLASH
    b L_pc_push
3:  mov X2, #FIELD_MIRROR_SLASH
    b L_pc_push
4:  mov X2, #FIELD_SPLIT_HORIZONTAL
    b L_pc_push
5:  mov X2, #FIELD_SPLIT_VERTICAL
    b L_pc_push

L_pc_push:
    // Load some values.
    ldr PC_X, [X1, #PC_X_OFFSET]
    ldr PC_Y, [X1, #PC_Y_OFFSET]
    ldr PC_CONTRAPTION, [X1, #PC_CONTRAPTION_OFFSET]
    ldr PC_WIDTH, [PC_CONTRAPTION, #CONTRAPTION_WIDTH]

    // Calculate index.
    madd W4, PC_WIDTH_W, PC_Y, PC_X
    // Increase X and save it.
    add PC_X, PC_X, #1
    str PC_X, [X1, #PC_X_OFFSET]

    // Arg 1: The field array.
    add X0, PC_CONTRAPTION, #CONTRAPTION_FIELDS // Pointer to pointer!
    // Arg 2: Index in array.
    mov X1, X4
    // Arg 3: Field value (set above).
    bl _array1_set

    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

L_pc_newline:
    ldr PC_Y, [X1, #PC_Y_OFFSET]

    // Increase Y and save it.
    add PC_Y, PC_Y, #1
    str PC_Y, [X1, #PC_Y_OFFSET]
    // Reset X.
    str WZR, [X1, #PC_X_OFFSET]

    mov SP, FP
    ldp FP, LR, [SP, #-16]!
    ret


// ******************* Deliberate AArch64 ABI violation **********************
//
// To optimize some calls, all op_* functions expect a few registers to be set
// which are usually volatile ("temporariy registers" in "Procedure Call
// Standard for the Arm® 64-bit Architecture (AArch64)"), which means the
// caller is supposed to save them if needed. But I'm not doing that and rely
// on the fact that all subfunctions only use these registers for reading.
//
// This way, several stack load/saves and several moves can be omitted.
OP_X .req X9
OP_Y .req X10
OP_DIR .req X11
OP_WIDTH .req X12
OP_HEIGHT .req X13
OP_BEAM_LIST_P .req X14
OP_CYCLE_RAW .req X15
OP_ENERGIZED_RAW .req X16 // !!! aka IP0, used by dynamic linker
OP_FIELDS_RAW .req X17 // aka IP1, used by dynamic linker
// Use of X16 (IP0) and X17 (IP1) should be save since no functions of
// dynamically linked frameworks are called, only our own functions. But I'm
// not 100% sure.


TF_CONTRAPTION .req X19
TF_BEAM_LIST .req X20
TF_BEAM_LIST_P .req X21
TF_BEAM_LIST_ALT_P .req X22
TF_BEAM .req X23
TF_INDEX .req X24

// Trace a beam
// X0: Contraption address
// X1: X position
// X2: Y position
// X3: direction
.balign 4
trace_from:
    stp FP, LR, [SP, #-16]!
    mov FP, SP
    stp TF_CONTRAPTION, TF_BEAM_LIST_ALT_P, [SP, #-16]!
    stp TF_BEAM_LIST, TF_BEAM_LIST_P, [SP, #-16]!
    stp TF_BEAM, TF_INDEX, [SP, #-16]!

    mov TF_CONTRAPTION, X0
    PACK_BEAM TF_BEAM, X1, X2, X3

    // Set up most of the OP_* registers.
    ldp OP_WIDTH, OP_HEIGHT, [X0, #CONTRAPTION_WIDTH]
    
    ldr X0, [TF_CONTRAPTION, #CONTRAPTION_FIELDS]
    bl _array1_get_raw
    mov OP_FIELDS_RAW, X0

    ldr X0, [TF_CONTRAPTION, #CONTRAPTION_CYCLE]
    bl _array1_get_raw
    mov OP_CYCLE_RAW, X0

    ldr X0, [TF_CONTRAPTION, #CONTRAPTION_ENERGIZED]
    bl _array1_get_raw
    mov OP_ENERGIZED_RAW, X0

    mov X0, #1 // Create a list for the beams to process
    bl _array8_create
    str X0, [SP, #-16]!
    mov TF_BEAM_LIST, X0
    mov TF_BEAM_LIST_P, SP // Store pointer-to-pointer!

    mov X0, TF_BEAM_LIST_P
    mov X1, TF_BEAM // Push start beam to list
    bl _array8_push

    mov X0, #1 // Create a second list for the beams to process
    bl _array8_create
    str X0, [SP, #-16]!
    mov TF_BEAM_LIST_ALT_P, SP // Store pointer-to-ponter!

L_tf_loop:
    // Get current list length
    mov X0, TF_BEAM_LIST
    bl _array8_count
    cbz X0, L_tf_done // Leave if empty

    sub TF_INDEX, X0, #1

L_tf_inner_loop:
    // Get next beam.
    mov X0, TF_BEAM_LIST
    mov X1, TF_INDEX
    bl _array8_get

    // Process it.
    UNPACK_BEAM X0, OP_X, OP_Y, OP_DIR
    mov OP_BEAM_LIST_P, TF_BEAM_LIST_ALT_P
    bl op_beam_step

    // if (index-- > 0) goto inner_loop
    cbz TF_INDEX, 1f
    sub TF_INDEX, TF_INDEX, #1
    b L_tf_inner_loop

1:  // Clear the old list
    mov X0, TF_BEAM_LIST
    bl _array8_remove_all
    // Swap the lists.
    mov X0, TF_BEAM_LIST_P
    mov TF_BEAM_LIST_P, TF_BEAM_LIST_ALT_P
    mov TF_BEAM_LIST_ALT_P, X0
    ldr TF_BEAM_LIST, [TF_BEAM_LIST_P]
    // Loop
    b L_tf_loop

L_tf_done:
    // Free up allocated memory
    ldr X0, [TF_BEAM_LIST_P]
    bl _array8_free
    ldr X0, [TF_BEAM_LIST_ALT_P]
    bl _array8_free

   
    // Get number of energized fields (return value of this function).
    // Calculate number of 16 byte blocks to iterate (rounded up). Relies on
    // the fact that memory is 16 byte aligned and unused parts are 0 (due to
    // mach_vm_allocate).
    mul X0, OP_WIDTH, OP_HEIGHT
    add X0, X0, #15
    lsr TF_INDEX, X0, #4

    // Get the raw buffer.
    mov X0, OP_ENERGIZED_RAW

    // Initialize accumulator (NEON register 0) with 0.
    fmov D0, #0

1:  ld1 { V1.16B }, [X0], #16 // Load 16 bytes into NEON register 1.
    addv B2, V1.16B // Add all 16 bytes and save in NEON register 2.
    add D0, D0, D2 // Add to accumulator.
    subs TF_INDEX, TF_INDEX, #1 // Count down and loop if index > 0.
    b.ne 1b

    fmov X0, D0 // Get result from NEON register.

    // Pop and return
    add SP, SP, #32 // The two pointer-to-pointers
    ldp TF_BEAM, TF_INDEX, [SP], #16
    ldp TF_BEAM_LIST, TF_BEAM_LIST_P, [SP], #16
    ldp TF_CONTRAPTION, TF_BEAM_LIST_ALT_P, [SP], #16
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret


BS_CYCLE_INDEX .req X5
BS_FIELD_INDEX .req X6

// Arguments: OP_* registers
.balign 4
op_beam_step:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    // Calculate index for the cycle detector.
    madd BS_FIELD_INDEX, OP_WIDTH, OP_Y, OP_X // (width * y) + x
    sub X4, OP_DIR, #1 // !!! Relies on direction being in range of 1 to 4
    lsl BS_CYCLE_INDEX, BS_FIELD_INDEX, #2 // Shift by two bits for the direction
    orr BS_CYCLE_INDEX, BS_CYCLE_INDEX, X4 // Merge with direction

    // Check whether we've hit a cycle.
    ldrb W0, [OP_CYCLE_RAW, BS_CYCLE_INDEX]
    cbnz X0, L_bs_done // Leave if it's a cycle.

    // Mark for cycle detection.
    mov X0, #1
    strb W0, [OP_CYCLE_RAW, BS_CYCLE_INDEX]

    // Mark in energized array.
    strb W0, [OP_ENERGIZED_RAW, BS_FIELD_INDEX]

    // Read field.
    ldrb W0, [OP_FIELDS_RAW, BS_FIELD_INDEX]

    // Combine the field value with direction to get an index into a jumptable.
    // Field is a value 0 – 4, so 3 bits. Direction is a value 1 – 4, reduced
    // by one we get 2 bits. So index is thus a 5 bits = 32 value field.
    sub X1, OP_DIR, #1
    bfi X0, X1, #3, #2
    // Use a jumptable to switch on the field. This will crash if the field
    // value is garbage.
    adr X1, L_bs_jumptable
    add X1, X1, X0, lsl #2
    br x1

L_bs_jumptable:
    b L_bs_go_north             // DIR_NORTH, FIELD_EMPTY               00_000
    b L_bs_go_east              // DIR_NORTH, FIELD_SLASH               00_001
    b L_bs_go_west              // DIR_NORTH, FIELD_BACKSLASH           00_010
    b L_bs_go_north             // DIR_NORTH, FIELD_SPLIT_VERTICAL      00_011
    b L_bs_split_horizontal     // DIR_NORTH, FIELD_SPLIT_HORIZONTAL    00_100
    nop                         //                                      00_101
    nop                         //                                      00_110
    nop                         //                                      00_111
    b L_bs_go_east              // DIR_EAST, FIELD_EMPTY                01_000
    b L_bs_go_north             // DIR_EAST, FIELD_SLASH                01_001
    b L_bs_go_south             // DIR_EAST, FIELD_BACKSLASH            01_010
    b L_bs_split_vertical       // DIR_EAST, FIELD_SPLIT_VERTICAL       01_011
    b L_bs_go_east              // DIR_EAST, FIELD_SPLIT_HORIZONTAL     01_100
    nop                         //                                      01_101
    nop                         //                                      01_110
    nop                         //                                      01_111
    b L_bs_go_south             // DIR_SOUTH, FIELD_EMPTY               10_000
    b L_bs_go_west              // DIR_SOUTH, FIELD_SLASH               10_001
    b L_bs_go_east              // DIR_SOUTH, FIELD_BACKSLASH           10_010
    b L_bs_go_south             // DIR_SOUTH, FIELD_SPLIT_VERTICAL      10_011
    b L_bs_split_horizontal     // DIR_SOUTH, FIELD_SPLIT_HORIZONTAL    10_100
    nop                         //                                      10_101
    nop                         //                                      10_110
    nop                         //                                      10_111
    b L_bs_go_west              // DIR_WEST, FIELD_EMPTY                11_000
    b L_bs_go_south             // DIR_WEST, FIELD_SLASH                11_001
    b L_bs_go_north             // DIR_WEST, FIELD_BACKSLASH            11_010
    b L_bs_split_vertical       // DIR_WEST, FIELD_SPLIT_VERTICAL       11_011
    b L_bs_go_west              // DIR_WEST, FIELD_SPLIT_HORIZONTAL     11_100
    nop                         //                                      11_101
    nop                         //                                      11_110
    nop                         //                                      11_111

L_bs_go_north:
    mov X0, #DIR_NORTH
    bl op_advance_beam
    b L_bs_done

L_bs_go_east:
    mov X0, #DIR_EAST
    bl op_advance_beam
    b L_bs_done

L_bs_go_south:
    mov X0, #DIR_SOUTH
    bl op_advance_beam
    b L_bs_done

L_bs_go_west:
    mov X0, #DIR_WEST
    bl op_advance_beam
    b L_bs_done

L_bs_split_horizontal:
    mov X0, #DIR_EAST
    bl op_advance_beam
    mov X0, #DIR_WEST
    bl op_advance_beam
    b L_bs_done

L_bs_split_vertical:
    mov X0, #DIR_NORTH
    bl op_advance_beam
    mov X0, #DIR_SOUTH
    bl op_advance_beam
    // b L_bs_done

L_bs_done:
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret


// Arguments: OP_* registers
// X0: Direction
.balign 4
op_advance_beam:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    mov X1, OP_X
    mov X2, OP_Y
    bl op_next_pos
    cbz X0, 1f

    PACK_BEAM X5, X1, X2, X0

    mov X0, OP_BEAM_LIST_P
    mov X1, X5
    bl _array8_push

1:  mov SP, FP
    ldp FP, LR, [SP], #16
    ret


// Arguments: OP_* registers
// X0: Direction
// X1: X pos
// X2: Y pos
// Returns:
// X0: == 0 if no next position, direction if there is one.
// X1: X pos
// X2: Y pos
.balign 4
op_next_pos:
    // Use a jump table to "switch" on the direction. Of course, this will crash if you feed a
    // garbage direction.
    adr X4, L_np_jumptable
    add X4, X4, X0, lsl #2
    br X4

L_np_jumptable:
    nop
    b L_np_north
    b L_np_east
    b L_np_south
    // b L_np_west // Skip the branch for the last case, directly start the case.
L_np_west:
    // if X1 > 0 { Some(X1 - 1, X2) } else { None }
    subs X1, X1, #1
    csel X0, X0, XZR, pl // X1 >= 0? If so, pass direction.
    ret

L_np_north:
    // if X2 > 0 { Some(X1, X2 - 1) } else { None }
    subs X2, X2, #1
    csel X0, X0, XZR, pl // X2 >= 0? If so, pass direction.
    ret

L_np_east:
    add X1, X1, #1
    cmp X1, OP_WIDTH
    csel X0, X0, XZR, lt // Is result within bounds? If so, pass direction.
    ret

L_np_south:
    add X2, X2, #1
    cmp X2, OP_HEIGHT
    csel X0, X0, XZR, lt // Is result within bounds? If so, pass direction.
    ret


.balign 4
accumulate_handler:
    ldr X0, [X2]
    add X0, X0, X1
    str X0, [X2]
    mov X0, #1
    ret


AS_CONTRAPTION .req X20
AS_MAX_ENERGIZED .req X21
AS_INDEX .req X22
AS_MAX_X .req X23
AS_MAX_Y .req X24

.macro AS_TRACE_BEAM x, y, dir
    // Trace the beam.
    mov X0, AS_CONTRAPTION
    mov X1, \x
    mov X2, \y
    mov X3, \dir
    bl trace_from

    // Store max value (no `umax` on M1, unfortunately)
    cmp AS_MAX_ENERGIZED, X0
    csel AS_MAX_ENERGIZED, AS_MAX_ENERGIZED, X0, hi

    // Reset the contraption.
    ldr X0, [AS_CONTRAPTION, #CONTRAPTION_ENERGIZED]
    bl _array1_set_all_to_zero
    ldr X0, [AS_CONTRAPTION, #CONTRAPTION_CYCLE]
    bl _array1_set_all_to_zero
.endmacro


// X0: Pointer to contraption
.balign 4
trace_beams_from_all_sides:
    stp FP, LR, [SP, #-16]!
    mov FP, SP
    stp AS_CONTRAPTION, AS_MAX_ENERGIZED, [SP, #-16]!
    str AS_INDEX, [SP, #-16]!
    stp AS_MAX_X, AS_MAX_Y, [SP, #-16]!

    mov AS_CONTRAPTION, X0
    mov AS_MAX_ENERGIZED, XZR
    ldr AS_MAX_X, [AS_CONTRAPTION, #CONTRAPTION_WIDTH]
    ldr AS_MAX_Y, [AS_CONTRAPTION, #CONTRAPTION_HEIGHT]

    sub AS_MAX_X, AS_MAX_X, #1
    sub AS_MAX_Y, AS_MAX_Y, #1

    mov AS_INDEX, AS_MAX_X
L_as_loop1:
    // Trace from top
    AS_TRACE_BEAM AS_INDEX, #0, #DIR_SOUTH
    // Trace from bottom
    AS_TRACE_BEAM AS_INDEX, AS_MAX_Y, #DIR_NORTH

    subs AS_INDEX, AS_INDEX, #1
    b.pl L_as_loop1 // Loop while index >= 0

    mov AS_INDEX, AS_MAX_Y
L_as_loop2:
    // Trace from left
    AS_TRACE_BEAM #0, AS_INDEX, #DIR_EAST
    // Trace from right
    AS_TRACE_BEAM AS_MAX_X, AS_INDEX, #DIR_WEST

    subs AS_INDEX, AS_INDEX, #1
    b.pl L_as_loop2 // Loop while index >= 0

    // Return the maximum
    mov X0, AS_MAX_ENERGIZED

    ldp AS_MAX_X, AS_MAX_Y, [SP], #16
    ldr AS_INDEX, [SP], #16
    ldp AS_CONTRAPTION, AS_MAX_ENERGIZED, [SP], #16
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret


.const
str_input: .incbin "../../day16/rsc/input.txt"
str_input_len = (. - str_input)

DEFINE_STRING str_part1, "Part 1: "
DEFINE_STRING str_part2, "Part 2: "
DEFINE_STRING str_invalid_contraption_char, "Encountered invalid character in input."