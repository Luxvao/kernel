#![feature(c_size_t)]
#![feature(allocator_api)]
#![no_std]
#![no_main]

// extern crate alloc;

mod arch;
mod drivers;
mod libc;
mod limine;
mod mem;
mod utils;

use core::fmt::Write;
#[allow(unused_imports)]
use core::panic::PanicInfo;

use crate::{
    arch::amd64::{
        gdt,
        registers::{CS, SS},
    },
    drivers::io::serial::Uart,
    utils::halt,
};

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
