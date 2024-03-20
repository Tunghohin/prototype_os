    .section .text.entry
    .globl _start
_start:
    la sp, eboot_stack
    j rust_main

    .section .bss.stack
    .globl sboot_stack
sboot_stack:
    .space 4096 * 32
    .globl eboot_stack
eboot_stack:

    .align 4
    .section .data
    .global _num_app
_num_app: 
    .quad 2
    .quad _app0_start
    .quad _app1_start
    .quad _app1_end
    
.global _app_names
_app_names:
    .string "initproc"
    .string "console_out"

    .section .data
    .global _app0_start
    .global _app0_end
    .align 4
_app0_start:
    .incbin "../prototype_lib/target/riscv64gc-unknown-none-elf/release/initproc"
_app0_end:

    .section .data
    .global _app1_start
    .global _app1_end
    .align 4
_app1_start:
    .incbin "../prototype_lib/target/riscv64gc-unknown-none-elf/release/console_out"
_app1_end:
