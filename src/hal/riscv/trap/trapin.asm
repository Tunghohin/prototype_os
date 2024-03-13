    .section .text.trampoline
    .globl __trapin
    .globl __restore
    .align 2
__trapin:
    csrrw sp, sscratch, sp
    call trap_handler

__restore:
    sret
