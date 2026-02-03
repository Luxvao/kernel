use core::ffi::{c_int, c_size_t, c_void};

#[no_mangle]
unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, count: c_size_t) -> *mut c_void {
    let dest = dest as *mut u8;
    let src = src as *const u8;

    for i in 0..count {
        *(dest.add(i)) = *(src.add(i));
    }

    dest as *mut c_void
}

#[no_mangle]
unsafe extern "C" fn memcmp(lhs: *const c_void, rhs: *const c_void, count: c_size_t) -> c_int {
    let lhs = lhs as *const u8;
    let rhs = rhs as *const u8;

    for i in 0..count {
        let l = *(lhs.add(i)) as c_int;
        let r = *(rhs.add(i)) as c_int;

        if l != r {
            return l - r;
        }
    }

    0
}

#[no_mangle]
unsafe extern "C" fn memset(dest: *mut c_void, ch: c_int, count: c_size_t) -> *mut c_void {
    let dest = dest as *mut u8;

    for i in 0..count {
        *(dest.add(i)) = ch as u8;
    }

    dest as *mut c_void
}
