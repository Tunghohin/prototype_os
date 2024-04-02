pub trait GenericTrap<T: Sized> {
    fn init();
    fn task_init_cx(entry: usize, user_sp: usize, kernel_sp: usize) -> T;
}
