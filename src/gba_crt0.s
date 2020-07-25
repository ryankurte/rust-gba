@ GBA minimal boot script
@ ryankurte/rust-gba

.section .text.boot
.global _boot
.cpu arm7tdmi
.align
.arm

_boot:
    mov r0, #0x4000000      @ REG_BASE
    str r0, [r0, #0x208]
    mov r0, #0x12           @ Switch to IRQ Mode
    msr cpsr, r0
    ldr sp, =__sp_irq       @ Set IRQ stack
    mov r0, #0x1f           @ Switch to System Mode
    msr cpsr, r0
    ldr sp, =__sp_usr       @ Set user stack
    ldr r3, =reset          @ Load reset address
    bx r3                   @ Jump!
