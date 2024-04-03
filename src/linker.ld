OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;
MEMORY_END = 0x88000000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        . = ALIGN(4K);
        strampoline = .;
        *(.text.trampoline);
        . = ALIGN(4K);
        *(.text .text.*)
    }
    . = ALIGN(4K);
    etext = .;
    
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    . = ALIGN(4K);
    erodata = .;
    
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    . = ALIGN(4K);
    edata = .;
    
    bss_with_stack = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.sbss .sbss.*)
        *(.bss .bss.*)
    }
    . = ALIGN(4K);
    ebss = .;
    ekernel = .;
    . = MEMORY_END;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
