use core::ffi::{c_int, c_uchar, c_uint};

// memcpy implementation
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, count: c_uint) {
    for i in 0..count {
        *(dest.add(i as usize)) = *(src.add(i as usize));
    }
}

// memset implementation
#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut u8, ch: u8, count: c_uint) {
    for i in 0..count {
        *(dest.add(i as usize)) = ch;
    }
}
