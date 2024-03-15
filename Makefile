TARGET_DIR := ./target/riscv64gc-unknown-none-elf/debug

QEMU := qemu-system-riscv64
QEMU_FLAG := -machine virt \
			 -nographic \
			 -bios ./rustsbi-qemu.bin \
			 -device loader,file=target/riscv64gc-unknown-none-elf/debug/prototype_os.bin,addr=0x80200000

GDB := gdb-multiarch

.DEFAULT_GOAL: build
.PHONY: build
build:
	cargo build
	
.PHONY: objcopy
objcopy: build
	rust-objcopy ${TARGET_DIR}/prototype_os --strip-all -O binary ${TARGET_DIR}/prototype_os.bin

.PHONY: objdump
objdump: build
	rust-objdump -dw ${TARGET_DIR}/prototype_os

.PHONY: debug
debug: build
	${QEMU} ${QEMU_FLAG} -s -S	

.PHONY: gdb
gdb:
	${GDB} -x ./.gdbinit -q

.PHONY: check
check:
	cargo check


.PHONY: qemu
qemu: build objcopy
	${QEMU} ${QEMU_FLAG}
