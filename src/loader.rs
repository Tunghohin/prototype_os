pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

pub fn get_app_data_addr(app_id: usize) -> usize {
    assert!(app_id < get_num_app());
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).add(1) as usize }
}

pub fn get_app_data(app_id: usize) {
    let app_addr = get_app_data_addr(app_id);
}
