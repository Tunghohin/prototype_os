/// size of kernel heap
pub const KERNEL_HEAP_SIZE: usize = 0x02000000;

/// physical memory end address
pub const MEMORY_END: usize = 0x88000000;

/// page size : 4KB
pub const PAGE_SIZE: usize = 0x1000;

/// page size bits: 12
pub const PAGE_SIZE_BITS: usize = 0xc;

/// the virtual address of trapoline
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;

/// the virtual addr of trap context
pub const TRAP_CONTEXT_BASE: usize = TRAMPOLINE - PAGE_SIZE;

/// user app's stack size
pub const USER_STACK_SIZE: usize = PAGE_SIZE;

/// kernel stack size
pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE;
