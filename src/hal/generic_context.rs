pub trait GenericContext<T: Sized> {
    fn zero_init() -> T {
        unsafe { core::mem::zeroed::<T>() }
    }

    fn goto_trap_return(kstack_ptr: usize) -> T;
}
