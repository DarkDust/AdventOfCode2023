.include "syscall.inc"

//////////////////////////////////////////////////////////////////////////////

// void print_newline(void);
// Prints a single newline.
.global _print_newline

// void print_hex64(uint64_t num);
// Print a 64 bit number in hex.
.global _print_hex64

// void print_uint64(uint64_t num);
// Print a 64 bit unsigned number in decimal.
.global _print_uint64

// void print_n(const char * str, uint64_t count);
// Print `count` number of characters of the given string.
.global _print_n

//////////////////////////////////////////////////////////////////////////////

.equ BUFFER_SIZE, 16

.const
.balign 8
log2_10: .double 3.321928094887362 // log2(10)

.text

.balign 4
_print_newline:
	mov W0, #'\n'
	strb W0, [SP, #-16]! // Push '\n' on the stack
	mov X0, #FD_STDOUT
	mov X1, SP // Address of the pushed '\n'
	mov X2, #1 // Length to write
	SYSCALL #SYSCALL_WRITE
	add SP, SP, #16
	ret

.balign 4
_print_hex64:
	stp FP, LR, [SP, #-16]!
	mov FP, SP

	sub SP, SP, #BUFFER_SIZE	// "Allocate" 16 bytes on the stack
	mov X2, #(BUFFER_SIZE - 1)

L_next_digit:
	and X4, X0, #0xF
	cmp X4, #0x9
	b.hi L_hex
	add X4, X4, #'0'
	b L_cont
L_hex:
	add X4, X4, #('A' - 10)
L_cont:
	strb W4, [SP, X2]
	
	cmp X2, 0
	b.eq L_done
	lsr X0, X0, #4
	sub X2, X2, #1
	b L_next_digit

L_done:
	mov X0, #FD_STDOUT
	mov X1, SP
	mov	X2, #16 // length of hex digit
	SYSCALL #SYSCALL_WRITE

	mov SP, FP
	ldp FP, LR, [SP], #16
	ret


.balign 4
_print_uint64:
	stp FP, LR, [SP, #-16]!
	mov FP, SP

	// The number of digits required can be calculated using:
	//     floor(log10(num) + 1)
	// Unfortunately, there's no floating point logarithm on ARM64, and even
	// FLOGB (which produces an integer result) is only available in the SVE2
	// extension, which Apple's M1 and M2 don't implement.
	//
	// Luckily, there's a bit-twiddling hack that works nicely on ARM64.
	// The integer result of log2(x) can be calculated by counting the number
	// of leading 0's, then 64 - count gives us the result.
	//
	// Then, log10(x) = log2(x) / log2(10). The later is a constant. We do need
	// the fraction part here to get correct results, it looks like we can get
	// away with log2(x) being rounded down.

	mov X1, #10
	mov X2, #64

	clz X3, X0 // See above, count leading zeros to get log2(X0)
	sub X3, X2, X3
	ucvtf D3, X3 // D3 = floor(log2(X0))

	adrp X4, log2_10@PAGE // Load log2(10) constant.
	ldr D4, [X4, log2_10@PAGEOFF] // D4 = log2(10)

	fdiv D5, D3, D4
	fcvtzu X5, D5 // X5 = log10(X0) [log2(X0) / log2(10)]
	add X5, X5, #1 // X5 (digits) = floor(log10(num) + 1)

	// Now that we know the number of digits (X5), we can start.

	// Allocate 32 byte on the stack as buffer. The largest 64 bit number is
	// 18,446,744,073,709,551,615 which has 20 digits.
	sub SP, SP, #32

	// Finally, do the conversion.
	// X0 is still the input, X5 is the number of digits.
	mov X1, X0
	mov X2, #10
	sub X6, X5, #1

1:	udiv X3, X1, X2
	msub X4, X3, X2, X1 // X4 = X1 % 10
	add W4, W4, #'0' // Convert to ASCII
	strb W4, [SP, X6] // Save to buffer (back to front)

	mov X1, X3 // X1 = X1 / 10
	subs X6, X6, #1 // X6 -= 1
	b.pl 1b // If X >= 0, loop

	// We're done! We can print it.

	mov X1, SP	// Start of our buffer
	mov X2, X5	// Still number of digits

	cmp W4, #'0'	// Account for imprecision due to rounding above: if first digit is 0,
	b.ne 2f			// skip the first character ...
	cmp X5, #1		// ... except if it's the only character.
	b.eq 2f

	add X1, X1, #1
	sub X2, X2, #1

2:	mov X0, #FD_STDOUT
	SYSCALL #SYSCALL_WRITE

	mov SP, FP // Restore the stack, undoing our "allocation"
	ldp FP, LR, [SP], #16
	ret


.balign 4
_print_n:
	mov X2, X1	// Move length to correct register
	mov X1, X0	// Move string pointer to correct register
	mov X0, #FD_STDOUT
	SYSCALL #SYSCALL_WRITE
	ret