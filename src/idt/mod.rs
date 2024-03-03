mod interrupts;

use x86::{
    dtables::{lidt, DescriptorTablePointer},
    segmentation::{
        BuildDescriptor, Descriptor, DescriptorBuilder, GateDescriptorBuilder, SegmentSelector,
    },
};

pub fn init() {
    idt();
}

fn idt() {
    let mut idt: [Descriptor; 256] = [Descriptor::NULL; 256];

    idt[0] = DescriptorBuilder::interrupt_descriptor(
        SegmentSelector::new(1, x86::Ring::Ring0),
        interrupts::div_by_zero as u32,
    )
    .dpl(x86::Ring::Ring0)
    .db()
    .present()
    .finish();

    idt[69] = DescriptorBuilder::interrupt_descriptor(
        SegmentSelector::new(1, x86::Ring::Ring0),
        interrupts::syscall as u32,
    )
    .dpl(x86::Ring::Ring3)
    .db()
    .present()
    .finish();

    let idt_ptr = DescriptorTablePointer::new(&idt);

    unsafe {
        lidt(&idt_ptr);
    }
}
