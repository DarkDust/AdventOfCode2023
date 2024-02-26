.include "utils.inc"
.include "syscall.inc"

//////////////////////////////////////////////////////////////////////////////

// typedef struct array array;

// array4 * array4_create(uint64_t minimumCapacityInBytes);
// Create a new dynamically sized array with given initial capacity
// (but no elements). On error, NULL is returned.
.global _array4_create

// void array4_free(array4 * array);
// Deallocate an array.
.global _array4_free

// uint64_t array4_count(array4 * array);
// Return current number of elements in the array.
.global _array4_size

// uint32_t array4_get(array4 * array, uint64_t index);
// Get an element from the array.
.global _array4_get

// void array4_set(array4 ** array, uint64_t index, uint32_t value);
// Set an element in the array. If the index == array4_count(array), the value
// is appended. Otherwise, if the index is out of bounds, the process is
// terminated.
//
// Note that the first argument is a pointer-to-pointer. The location of the
// array in memory may change.
.global _array4_set

// void array4_push(array4 ** array, uint32_t value);
// Appends an element to the array.
//
// Note that the first argument is a pointer-to-pointer. The location of the
// array in memory may change.
.global _array4_push

// uint32_t array4_pop(array4 * array);
// Removes the last element from the array and returns it. If the array is
// empty, -1 is returned.
.global _array4_pop

//////////////////////////////////////////////////////////////////////////////

.equ A4_COUNT, 0
.equ A4_CAPACITY, 8
.equ A4_RECORD_SIZE, 16

.balign 4
_array4_create:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    // Array starts with a count, and a minimum capacity.
    add X0, X0, #A4_RECORD_SIZE
    bl _malloc_wholepage // Allocate space.
    cbz X0, L_a4c_return // Leave if allocation failed (X0 == NULL)

    // X0 now is pointer to region, X1 its size.
    sub X1, X1, #A4_RECORD_SIZE // Account for array header
    lsr X1, X1, #2 // Capacity in element = bytes / 4
    stp XZR, X1, [X0] // Save count (0) and capacity

L_a4c_return:
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret
    
.balign 4
_array4_free:
    bl _free // Not much to do, just call `free`
    ret

.balign 4
_array4_get:
    // Do a range check first.
    ldr X3, [X0, #A4_COUNT]
    cmp X1, X3
    b.ge L_a4g_out_of_range 

    lsl X2, X1, #2 // byte offset = index * 4
    add X2, X2, #A4_RECORD_SIZE // Skip record
    ldr W0, [X0, X2] // Load the entry
    ret

L_a4g_out_of_range:
    RUNTIME_ERROR str_out_of_range

// These registers are _shared_ between _array4_set and _array_push so that the
// former can jump into the later.
REG_A4_COUNT .req X4
REG_A4_CAPACITY .req X5
REG_A4_START .req X6

.balign 4
_array4_set:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    ldr REG_A4_START, [X0] // Load actual array location (arg 1 is pointer-to-pointer)
    ldp REG_A4_COUNT, REG_A4_CAPACITY, [REG_A4_START] // Get count and capacity
    cmp X1, REG_A4_COUNT
    b.gt L_a4s_out_of_range
    b.eq L_a4s_push

    // Overwrite of an existing item. Calculate offset.
    lsl X3, X1, #2 // offset = index * 4
    add X3, X3, #A4_RECORD_SIZE // offset += header-size
    str W2, [REG_A4_START, X3] // Store

    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

L_a4s_push:
    mov X1, X2 // Prepare jump inoto _array4_push
    b L_a4p_push

L_a4s_out_of_range:
    RUNTIME_ERROR str_out_of_range


.balign 4
_array4_push:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    ldr REG_A4_START, [X0] // Load actual array location (arg 1 is pointer-to-pointer)
    ldp REG_A4_COUNT, REG_A4_CAPACITY, [REG_A4_START] // Get count and capacity

L_a4p_push:
    cbz REG_A4_CAPACITY, L_a4p_grow // Grow if no space is left.
L_a4p_do_push:
    lsl X3, REG_A4_COUNT, #2 // X3 = count * 4
    add X3, X3, #A4_RECORD_SIZE // X3 += header-size
    str W2, [REG_A4_START, X3] // Store value
    add REG_A4_COUNT, REG_A4_COUNT, #1 // Accounting
    sub REG_A4_CAPACITY, REG_A4_CAPACITY, #1
    stp REG_A4_COUNT, REG_A4_CAPACITY, [REG_A4_START] // Store updated accounting

    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

L_a4p_grow:
    // Not enough space left, reallocate. Save important stuff on the stack first.
    stp X0, X1, [SP, #-16]!
    stp X2, REG_A4_COUNT, [SP, #-16]!

    // Want at least 4 more bytes
    mov X0, REG_A4_START
    mov X1, #4
    bl _realloc_wholepage
    cbz X0, L_a4p_out_of_memory

    // Have new pointer in X0, its size in X1
    mov REG_A4_START, X0
    mov REG_A4_CAPACITY, X1 // !!! It's wrong now, get's fixed below
    
    // Reload saved infos
    ldp X2, REG_A4_COUNT, [SP], #16
    ldp X0, X1, [SP], #16

    // Calculate new array capacity.
    lsl X3, REG_A4_COUNT, #2 // bytes used = count * 4
    add X3, X3, #A4_RECORD_SIZE // bytes used += header-size
    sub REG_A4_CAPACITY, REG_A4_CAPACITY, X3 // capacity bytes -= used so far
    lsr REG_A4_CAPACITY, REG_A4_CAPACITY, #2 // capacity elements = capacity bytes / 4

    // Update the array caller's pointer-to-pointer with the new array address.
    str REG_A4_START, [X0]

    // Continue with push
    b L_a4p_do_push

L_a4p_out_of_memory:
    RUNTIME_ERROR str_out_of_memory


.balign 4
_array4_pop:
    ldp REG_A4_COUNT, REG_A4_CAPACITY, [REG_A4_START]
    cbz REG_A4_COUNT, L_a4pop_empty
    sub REG_A4_COUNT, REG_A4_COUNT, #1
    add REG_A4_CAPACITY, REG_A4_CAPACITY, #1
    lsl X1, REG_A4_COUNT, #2 // offset = count * 4
    add X1, X1, #A4_RECORD_SIZE // offset += header-size
    stp REG_A4_COUNT, REG_A4_CAPACITY, [X0]
    ldr W0, [X0, X1]
    ret

L_a4pop_empty:
    mov X0, #-1
    ret

.const
DEFINE_STRING str_out_of_range, "Array get/set out of range!"
DEFINE_STRING str_out_of_memory, "Out of memory while growing array!"
