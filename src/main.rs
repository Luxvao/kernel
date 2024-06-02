#![feature(c_size_t)]
#![feature(c_str_module)]
#![feature(abi_x86_interrupt)]
#![feature(ptr_as_ref_unchecked)]
#![no_std]
#![no_main]

mod framebuffer;
mod fs;
mod gdt;
mod idt;
mod io;
mod lazy;
mod libc;
mod memory;
mod process;
mod sync;
mod tss;
mod tty;
mod uefi;
mod utils;
mod serial;

#[allow(unused_imports)]
use core::panic::PanicInfo;

use framebuffer::{Color, FRAMEBUFFER};
use limine::BaseRevision;
use serial::SERIAL;
use utils::hcf;

pub static BASE_REVISION: BaseRevision = BaseRevision::new();

#[no_mangle]
extern "C" fn _start() -> ! {
    FRAMEBUFFER.lock().clrscr(Color::new());

    FRAMEBUFFER
        .lock()
        .drawrect(0, 0, 100, 100, Color::from(0x82d2f5));

    SERIAL.send_str("leni njok klobasko");

    hcf();
}

#[panic_handler]
#[cfg(any(rust_analyzer, not(test)))]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
