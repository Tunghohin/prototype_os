pub mod console;
pub mod sbi;
pub mod trap;

pub fn init() {
    trap::init();
}
