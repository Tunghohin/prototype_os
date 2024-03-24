use alloc::collections::BTreeMap;
use lazy_static::*;

pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

pub fn get_app_data_addr(app_id: usize) -> usize {
    assert!(app_id <= get_num_app());
    extern "C" {
        fn _num_app();
    }
    unsafe { *((_num_app as usize as *const usize).add(app_id + 1)) as usize }
}

pub fn get_app_data(app_id: usize) -> &'static [u8] {
    let app_start = get_app_data_addr(app_id);
    unsafe {
        core::slice::from_raw_parts(
            app_start as *const u8,
            get_app_data_addr(app_id + 1) - app_start,
        )
    }
}

pub fn get_app_id_by_name(name: &'static str) -> usize {
    APPS_LIST.get(name).expect("App not found!").clone()
}

pub fn get_app_data_by_name(name: &'static str) -> &'static [u8] {
    get_app_data(get_app_id_by_name(name))
}

lazy_static! {
    pub static ref APPS_LIST: BTreeMap<&'static str, usize> = {
        let mut map = BTreeMap::new();

        extern "C" {
            fn _app_names();
        }
        let mut start = _app_names as *const u8;
        for i in 0..get_num_app() {
            let mut end = start;
            unsafe {
                while end.read_volatile() != b'\0' {
                    end = end.add(1);
                }
                let str = core::str::from_utf8(core::slice::from_raw_parts(
                    start,
                    end as usize - start as usize,
                ))
                .unwrap();
                start = end.add(1);
                map.insert(str, i);
            }
        }

        map
    };
}
