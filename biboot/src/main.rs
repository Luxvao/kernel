#![no_main]
#![no_std]

use core::panic::PanicInfo;

use log::info;
use uefi::{
    table::{Boot, SystemTable},
    Handle,
};

#[uefi::entry]
fn entry(handle: Handle, mut table: SystemTable<Boot>) -> uefi::Status {
    uefi_services::init(&mut table).expect("Failed to initialize");

    info!("32 bit application ran!");

    table.boot_services().stall(10000000);

    uefi::Status(0)
}
