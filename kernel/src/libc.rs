use core::ffi::{c_size_t, c_void};

#[no_mangle]
unsafe extern "C" fn memcpy(
    dest: *mut c_void,
    src: *const c_void,
    count: c_size_t,
) -> *const c_void {
    let dest = dest as *mut u8;
    let src = src as *const u8;

    for i in 0..count {
        *(dest.add(i)) = *(src.add(i));
    }

    dest as *const c_void
}
