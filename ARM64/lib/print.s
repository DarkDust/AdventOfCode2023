.include "syscall.inc"

//////////////////////////////////////////////////////////////////////////////

// void print_newline(void);
// Prints a single newline.
.global _print_newline

// void print_hex64(uint64_t num);
// Print a 64 bit number in hex.
.global _print_hex64

// void print_n(const char * str, uint64_t count);
// Print `count` number of characters of the given string.
.global _print_n

//////////////////////////////////////////////////////////////////////////////

.equ BUFFER_SIZE, 16
// 16 byte, all zeros.
.lcomm buffer, BUFFER_SIZE

.align 4
_print_newline:
	mov W0, #'\n'
	strb W0, [SP, #-4] // Push '\n' on the stack
	mov X0, #FD_STDOUT
	sub X1, SP, #4 // Address of the pushed '\n'
	mov X2, #1 // Length to write
	mov X16, #SYSCALL_WRITE
	svc 80
	ret

.align 4
_print_hex64:
	stp FP, LR, [SP, #-16]!
	mov FP, SP

	mov X2, #(BUFFER_SIZE - 1)
	adrp X3, buffer@PAGE
	add	X3, X3, buffer@PAGEOFF
	add X3, X3, X2 // end of buffer

L_next_digit:
	and X4, X0, #0xF
	cmp X4, #0x9
	b.hi L_hex
	add X4, X4, #'0'
	b L_cont
L_hex:
	add X4, X4, #('A' - 10)
L_cont:
	strb W4, [X3, X2]
	
	cmp X2, 0
	b.eq L_done
	lsr X0, X0, #4
	sub X2, X2, #1
	b L_next_digit

L_done:
	mov X0, #FD_STDOUT
	mov X1, X3
	mov	X2, #16 // length of hex digit
	mov	X16, #SYSCALL_WRITE
	svc	#0x80

	mov SP, FP
	ldp FP, LR, [SP], #16
	ret

_print_n:
	mov X2, X1	// Move length to correct register
	mov X1, X0	// Move string pointer to correct register
	mov X0, #FD_STDOUT
	mov X16, #SYSCALL_WRITE
	svc #0x80
	ret