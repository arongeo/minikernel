.macro ADR_REL register, symbol
	adrp \register, \symbol
	add \register, \symbol, #:lo12:\symbol
.endm

.section .text._start

_start:
	mrs 	x0, MPIDR_EL1
	and 	x0, x0, #0b11
	ldr 	x1, #0
	cmp 	x0, x1
	b.ne	.L_park_core

	ADR_REL x0, __bss_start
	ADR_REL x1, __bss_end

.L_bss_init:
	cmp 	x0, x1
	b.eq	.L_start_rust
	stp	xzr, xzr, [x0], #16
	b	.L_bss_init

.L_start_rust:
	ADR_REL x0, __boot_core_stack_end
	mov	sp, x0

	b	_rust_entry

.L_park_core:
	wfe
	b	.L_park_core

.size .start, . - _start
.type _start, function
.global _start
