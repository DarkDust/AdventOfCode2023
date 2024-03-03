.include "utils.inc"
.include "syscall.inc"

//////////////////////////////////////////////////////////////////////////////
// A set of array functions for dynamic arrays of 1, 2, 4 and 8 byte entry
// size. Replace uppercase X in the definitions below with one of these byte
// sizes.
//
// typedef struct arrayX arrayX;
//
//
// arrayX * arrayX_create(uint64_t minimumEntryCapacity);
// Create a new dynamically sized array with given initial capacity (but no
// elements). On error, NULL is returned.
//
//
// void arrayX_free(arrayX * array);
// Deallocate an array.
//
//
// uint64_t arrayX_count(arrayX * array);
// Return current number of elements in the array.
//
//
// uint{8,16,32,64}_t arrayX_get(arrayX * array, uint64_t index);
// Get an element from the array.
//
//
// void arrayX_set(arrayX ** array, uint64_t index, uint{8,16,32,64}_t value);
// Set an element in the array. If the index == arrayX_count(array), the value
// is appended. Otherwise, if the index is out of bounds, the process is
// terminated.
//
//   Note that the first argument is a pointer-to-pointer. The location of the
//   array in memory may change.
//
//
// void arrayX_push(arrayX ** array, uint{8,16,32,64}_t value);
// Appends an element to the array.
//
//   Note that the first argument is a pointer-to-pointer. The location of the
//   array in memory may change.
//
//
// uint{8,16,32,64}_t arrayX_pop(arrayX * array);
// Removes the last element from the array and returns it. If the array is
// empty, -1 is returned.
//
//
// void arrayX_set_all_to_zero(arrayX * array);
// Set all elements in the array to 0.
//
//
// void arrayX_remove_all(arrayX * array);
// Remove all elements, setting the array's size to 0.
//
//
// arrayX * arrayX_clone(arrayX * array);
// Creates an independent copy of the array.
//
//////////////////////////////////////////////////////////////////////////////

.equ AX_OFFSET_COUNT, 0
.equ AX_OFFSET_CAPACITY, 8
.equ AX_RECORD_SIZE, 16

// Most of this file is one large macro. The arguments are:
// - name: Number of bytes per entry.
// - shift: Shift amount for each entry (e.g. 3 for 8 byte entries).
// - load: Mnemonic to load from memory (e.g. ldrh for 2 byte entries).
// - store: Mnemonic to store to memory (e.g. strh for 2 byte entries).
// - regsize: Size of the operand register. Either W or X.
.macro ARRAY_DEFINE name, shift, load, store, regsize

.balign 4
.global _array\name\()_create
_array\name\()_create:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    // Array starts with a count, and a minimum capacity.
    add X0, X0, #AX_RECORD_SIZE
    bl _malloc_wholepage // Allocate space.
    cbz X0, L_a\name\()c_return // Leave if allocation failed (X0 == NULL)

    // X0 now is pointer to region, X1 its size.
    sub X1, X1, #AX_RECORD_SIZE // Account for array header
    lsr X1, X1, #\shift // Capacity in element = bytes / (1 << shift)
    stp XZR, X1, [X0] // Save count (0) and capacity

L_a\name\()c_return:
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret


.balign 4
.global _array\name\()_free
_array\name\()_free:
    bl _free // Not much to do, just call `free`
    ret


.balign 4
.global _array\name\()_count
 _array\name\()_count:
    ldr X0, [X0, #AX_OFFSET_COUNT]
    ret


.balign 4
.global _array\name\()_get
_array\name\()_get:
    // Do a range check first.
    ldr X3, [X0, #AX_OFFSET_COUNT]
    cmp X1, X3
    b.ge L_out_of_range 

    lsl X2, X1, #\shift // byte offset = index * (1 << shift)
    add X2, X2, #AX_RECORD_SIZE // Skip record
    \load \regsize\()0, [X0, X2] // Load the entry
    ret


// These registers are _shared_ between _array4_set and _array_push so that the
// former can jump into the later.
REG_AX_COUNT .req X4
REG_AX_CAPACITY .req X5
REG_AX_START .req X6

.balign 4
.global _array\name\()_set
_array\name\()_set:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    ldr REG_AX_START, [X0] // Load actual array location (arg 1 is pointer-to-pointer)
    ldp REG_AX_COUNT, REG_AX_CAPACITY, [REG_AX_START] // Get count and capacity
    cmp X1, REG_AX_COUNT
    b.gt L_out_of_range
    b.eq L_a\name\()s_push

    // Overwrite of an existing item. Calculate offset.
    lsl X3, X1, #\shift // offset = index * (1 << shift)
    add X3, X3, #AX_RECORD_SIZE // offset += header-size
    \store \regsize\()2, [REG_AX_START, X3] // Store

    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

L_a\name\()s_push:
    mov X1, X2 // Prepare jump inoto _array4_push
    b L_a4p_push


.balign 4
.global _array\name\()_push
_array\name\()_push:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    ldr REG_AX_START, [X0] // Load actual array location (arg 1 is pointer-to-pointer)
    ldp REG_AX_COUNT, REG_AX_CAPACITY, [REG_AX_START] // Get count and capacity

L_a\name\()p_push:
    cbz REG_AX_CAPACITY, L_a\name\()p_grow // Grow if no space is left.
L_a\name\()p_do_push:
    lsl X3, REG_AX_COUNT, #\shift // X3 = count * (1 << shift)
    add X3, X3, #AX_RECORD_SIZE // X3 += header-size
    \store \regsize\()2, [REG_AX_START, X3] // Store value
    add REG_AX_COUNT, REG_AX_COUNT, #1 // Accounting
    sub REG_AX_CAPACITY, REG_AX_CAPACITY, #1
    stp REG_AX_COUNT, REG_AX_CAPACITY, [REG_AX_START] // Store updated accounting

    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

L_a\name\()p_grow:
    // Not enough space left, reallocate. Save important stuff on the stack first.
    stp X0, X1, [SP, #-16]!
    stp X2, REG_AX_COUNT, [SP, #-16]!

    // Want at least 4 more bytes
    mov X0, REG_AX_START
    mov X1, #4
    bl _realloc_wholepage
    cbz X0, L_out_of_memory

    // Have new pointer in X0, its size in X1
    mov REG_AX_START, X0
    mov REG_AX_CAPACITY, X1 // !!! It's wrong now, get's fixed below
    
    // Reload saved infos
    ldp X2, REG_AX_COUNT, [SP], #16
    ldp X0, X1, [SP], #16

    // Calculate new array capacity.
    lsl X3, REG_AX_COUNT, #\shift // bytes used = count * (1 << shift)
    add X3, X3, #AX_RECORD_SIZE // bytes used += header-size
    sub REG_AX_CAPACITY, REG_AX_CAPACITY, X3 // capacity bytes -= used so far
    lsr REG_AX_CAPACITY, REG_AX_CAPACITY, #\shift // capacity elements = capacity bytes / (1 << shift)

    // Update the array caller's pointer-to-pointer with the new array address.
    str REG_AX_START, [X0]

    // Continue with push
    b L_a\name\()p_do_push

.balign 4
.global _array\name\()_pop
_array\name\()_pop:
    ldp REG_AX_COUNT, REG_AX_CAPACITY, [REG_AX_START]
    cbz REG_AX_COUNT, L_a\name\()pop_empty
    sub REG_AX_COUNT, REG_AX_COUNT, #1
    add REG_AX_CAPACITY, REG_AX_CAPACITY, #1
    lsl X1, REG_AX_COUNT, #2 // offset = count * 4
    add X1, X1, #AX_RECORD_SIZE // offset += header-size
    stp REG_AX_COUNT, REG_AX_CAPACITY, [X0]
    ldr W0, [X0, X1]
    ret

L_a\name\()pop_empty:
    mov X0, #-1
    ret


.balign 4
.global _array\name\()_set_all_to_zero
_array\name\()_set_all_to_zero:
    stp FP, LR, [SP, #-16]!
    mov FP, SP

    ldr X2, [X0, #AX_OFFSET_COUNT] // Get number of elements
    lsl X2, X2, #\shift // Convert to byte size; arg 3 of memset
    mov X1, #0 // Null byte; arg 2 of memset
    add X0, X0, #AX_RECORD_SIZE // Start of content; arg 1 of memset
    bl _memset

    mov SP, FP
    ldp FP, LR, [SP], #16


.balign 4
.global _array\name\()_remove_all
_array\name\()_remove_all:
    ldp REG_AX_COUNT, REG_AX_CAPACITY, [X0] // Get number of elements and capacity.
    add REG_AX_CAPACITY, REG_AX_CAPACITY, REG_AX_COUNT // Adjust capacity.
    stp XZR, REG_AX_CAPACITY, [X0] // Save count (0) and capacity back.
    ret


.balign 4
.global _array\name\()_clone
_array\name\()_clone:
    stp FP, LR, [SP, #-16]!
    mov FP, SP
    stp X20, X21, [SP, #-16]!
    str X22, [SP, #-16]!

    mov X20, X0 // Save the array pointer
    ldr X21, [X20, #AX_OFFSET_COUNT] // Get number of elements

    mov X0, X21
    bl _array\name\()_create // Allocate a new array
    mov X22, X0 // Save pointer to new array

    ldr X2, [X22, #AX_OFFSET_CAPACITY] // Get new array capacity
    sub X2, X2, X21 // Adjust the capacity
    stp X21, X2, [X22] // Save new count and capacity

    add X0, X22, #AX_RECORD_SIZE // Arg 1: Start of destination array content
    add X1, X20, #AX_RECORD_SIZE // Arg 2: Start of source array content
    lsl X2, X21, #\shift // Arg 3: Number of bytes to copy
    bl _memcpy

    mov X0, X22 // Return value: pointer to new array
    ldr X22, [SP], #16
    ldp X20, X21, [SP], #16
    mov SP, FP
    ldp FP, LR, [SP], #16
    ret

.endmacro


L_out_of_range:
    RUNTIME_ERROR str_out_of_range

L_out_of_memory:
    RUNTIME_ERROR str_out_of_memory


ARRAY_DEFINE 1, 0, ldrb, strb, W
ARRAY_DEFINE 2, 1, ldrh, strh, W
ARRAY_DEFINE 4, 2, ldr, str, W
ARRAY_DEFINE 8, 3, ldr, str, X

.const
DEFINE_STRING str_out_of_range, "Array get/set out of range!"
DEFINE_STRING str_out_of_memory, "Out of memory while growing array!"
