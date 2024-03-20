pub trait GenericContext<T: Sized> {
    fn zero_init() -> T {
        unsafe { core::mem::zeroed::<T>() }
    }

    fn goto_trap_return(kstack_ptr: usize) -> T;

    fn switch(current_task_cx_ptr: *mut T, next_task_cx_ptr: *const T) -> !;
}
