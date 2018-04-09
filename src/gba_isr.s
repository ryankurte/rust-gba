
.section .text.isr
.global  _isr_stub
.cpu arm7tdmi
.align
.arm

@ isr stub in arm (32-bit) assembly to jump to actual ISR handler in 16-bit mode
_isr_stub:
    @ Push stack frame
	mrs		r2, spsr				@ Move SPSR to r2
	stmfd	sp!, {r2-r3, ip, lr}	@ Push sprs, IME, (IE,IF), lr_irq to stack

    @ Set mode to usr
	mrs		r3, cpsr				@ Move CPSR to R3
	bic		r3, r3, #0xDF
	orr		r3, r3, #0x1F
	msr		cpsr, r3				@ Load CPSR from R3

	@ Launch isr
	stmfd	sp!, {r0,lr}			@ Push &REG_IE, lr_sys to stack
    ldr 	r3, = isr_master  		@ Load thumb address
    bx		r3                		@ Jump!
	ldmfd	sp!, {r0,lr}			@ Pop &REG_IE, lr_sys from stack

    @ Reset mode to irq
	mrs		r3, cpsr            	@ Move CPSR to R3
	bic		r3, r3, #0xDF
	orr		r3, r3, #0x92
	msr		cpsr, r3            	@ Load CPSR from R3

    @ Pop stack frame
	ldmfd	sp!, {r2-r3, ip, lr}	@ Pop sprs, IME, (IE,IF), lr_irq from stack
	msr		spsr, r2				@ Load SPSR from r2
	bx 		lr						@ Return to caller (bios?)

