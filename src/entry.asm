    .section .text.entry
    .globl _start
_start:
    la sp, eboot_stack
    j rust_main

    .section .bss.stack
    .globl sboot_stack
sboot_stack:
    .space 4096 * 16
    .globl eboot_stack
eboot_stack:

