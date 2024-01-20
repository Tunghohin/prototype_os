    .section .text
    .globl __trapin
    .globl __restore
    .align 4
__trapin:
    call trap_handler

__restore:
    sret
