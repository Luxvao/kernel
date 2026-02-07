#![no_std]
#![no_main]
#![feature(c_size_t)]
#![feature(allocator_api)]
#![feature(const_closures)]
#![feature(const_trait_impl)]
#![feature(abi_x86_interrupt)]

// extern crate alloc;

mod arch;
mod drivers;
mod libc;
mod limine;
mod mem;
mod utils;

#[allow(unused_imports)]
use core::panic::PanicInfo;

use crate::{arch::amd64::gdt, utils::halt};

#[export_name = "_start"]
extern "C" fn kernel_main() {
    gdt::init(post_gdt);
    halt();
}

fn post_gdt() {
    halt();
}

#[panic_handler]
#[cfg(not(test))]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
