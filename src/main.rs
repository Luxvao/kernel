#![no_std]
#![no_main]
#![allow(unused)]

mod interrupts;
mod std;

#[cfg(any(not(test), rust_analyzer))]
use core::panic::PanicInfo;

use interrupts::div_by_zero;
use uefi::table::{boot::MemoryMap, Runtime, SystemTable};
use x86::{
    dtables::DescriptorTablePointer,
    segmentation::{
        load_cs, load_ds, load_es, load_fs, load_gs, load_ss, BuildDescriptor, Descriptor,
        DescriptorBuilder, GateDescriptorBuilder, SegmentDescriptorBuilder, SegmentSelector,
    },
};

fn init_gdt() {
    let mut gdt = [Descriptor::NULL; 3];

    // Setting up entry 1
    gdt[1] = DescriptorBuilder::code_descriptor(
        0,
        0xfffff,
        x86::segmentation::CodeSegmentType::ExecuteAccessed,
    )
    .db()
    .dpl(x86::Ring::Ring0)
    .limit_granularity_4kb()
    .finish();

    // Setting up entry 2
    gdt[2] = DescriptorBuilder::data_descriptor(
        0,
        0xfffff,
        x86::segmentation::DataSegmentType::ReadWriteAccessed,
    )
    .db()
    .dpl(x86::Ring::Ring0)
    .limit_granularity_4kb()
    .finish();

    let gdt_ptr = DescriptorTablePointer::new(&gdt);

    unsafe {
        x86::dtables::lgdt(&gdt_ptr);
    }
}

fn init_seg() {
    unsafe {
        load_ss(SegmentSelector::new(2, x86::Ring::Ring0));
        load_cs(SegmentSelector::new(1, x86::Ring::Ring0));
        load_ds(SegmentSelector::new(2, x86::Ring::Ring0));
        load_es(SegmentSelector::new(2, x86::Ring::Ring0));
        load_fs(SegmentSelector::new(2, x86::Ring::Ring0));
        load_gs(SegmentSelector::new(2, x86::Ring::Ring0));
    }
}

unsafe fn init_idt() {
    let mut idt = [Descriptor::NULL; 256];

    idt[0] = DescriptorBuilder::interrupt_descriptor(
        SegmentSelector::new(1, x86::Ring::Ring0),
        core::mem::transmute::<_, u32>(div_by_zero as *const ()),
    )
    .finish();

    let idt_ptr = DescriptorTablePointer::new(&idt);

    unsafe {
        x86::dtables::lidt(&idt_ptr);
    }
}

#[no_mangle]
extern "C" fn _start() {
    // Setting up GDT
    init_gdt();

    // Setting registers
    init_seg();

    // Setting up IDT
    unsafe {
        init_idt();
    }
}

#[panic_handler]
#[cfg(any(not(test), rust_analyzer))]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
