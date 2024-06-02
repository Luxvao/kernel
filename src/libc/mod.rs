use core::ffi::{c_char, c_int, c_size_t, c_str, c_void, CStr};

#[no_mangle]
unsafe extern "C" fn memset(dest: *mut c_void, ch: c_int, count: c_size_t) -> *mut c_void {
    for i in 0..count {
        *(dest.add(i) as *mut u8) = ch as u8;
    }

    dest
}

#[no_mangle]
unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, count: c_size_t) -> *mut c_void {
    for i in 0..count {
        *(dest.add(i) as *mut u8) = *(src.add(i) as *const u8);
    }

    dest
}

#[no_mangle]
unsafe extern "C" fn strlen(str: *const c_char) -> c_size_t {
    let mut str_cursor = 0;

        while *str.add(str_cursor) != 0 {
            str_cursor += 1;
        }

    str_cursor
}
