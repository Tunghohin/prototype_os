pub fn trap_return() {}

pub trait GenericTrap<T: Sized> {
    fn task_init(entry: usize, user_sp: usize, kernel_sp: usize) -> T;
}
