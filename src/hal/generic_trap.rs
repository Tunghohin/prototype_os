pub fn trap_return() {}

pub trait GenericTrap<T: Sized> {
    fn task_init() -> T;
}
