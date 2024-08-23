#![feature(c_size_t)]
#![no_std]
#![no_main]

mod drivers;
mod libc;
mod utils;
mod x86;

#[allow(unused_imports)]
use core::panic::PanicInfo;

use utils::halt;

#[export_name = "_start"]
extern "C" fn kernel_main() -> ! {
    halt();
}

#[panic_handler]
#[cfg(not(test))]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
