file target/riscv64gc-unknown-none-elf/debug/prototype_os
set arch riscv:rv64
target remote localhost:1234
layout src
layout regs
b rust_main
c
