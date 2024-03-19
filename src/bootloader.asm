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

    .align 4
    .section .data
    .global _num_app
_num_app: 
    .quad 2
    .quad app0_start
    .quad app1_start
    
.global _app_names
_app_names:
    .string "initproc"
    .string "console_out"

    .section .data
    .global app0_start
    .global app0_end
    .align 4
app0_start:
    .incbin "../prototype_lib/target/riscv64gc-unknown-none-elf/release/initproc"
app0_end:

    .section .data
    .global app1_start
    .global app1_end
    .align 4
app1_start:
    .incbin "../prototype_lib/target/riscv64gc-unknown-none-elf/release/console_out"
app1_end:
