use x86::{
    dtables::{lgdt, DescriptorTablePointer},
    segmentation::{
        load_cs, load_ds, load_es, load_fs, load_gs, load_ss, BuildDescriptor, Descriptor,
        DescriptorBuilder, SegmentDescriptorBuilder, SegmentSelector,
    },
};

pub fn init() {
    gdt();
    reg();
}

fn gdt() {
    let mut gdt: [Descriptor; 4] = [Descriptor::NULL; 4];

    gdt[1] = DescriptorBuilder::code_descriptor(
        0,
        0xfffff,
        x86::segmentation::CodeSegmentType::ExecuteAccessed,
    )
    .db()
    .limit_granularity_4kb()
    .dpl(x86::Ring::Ring0)
    .present()
    .finish();

    gdt[2] = DescriptorBuilder::data_descriptor(
        0,
        0xffffff,
        x86::segmentation::DataSegmentType::ReadWriteAccessed,
    )
    .db()
    .limit_granularity_4kb()
    .dpl(x86::Ring::Ring0)
    .present()
    .finish();

    let gdt_ptr = DescriptorTablePointer::new(&gdt);

    unsafe {
        lgdt(&gdt_ptr);
    }
}

fn reg() {
    unsafe {
        load_ss(SegmentSelector::new(2, x86::Ring::Ring0));
        load_ds(SegmentSelector::new(2, x86::Ring::Ring0));
        load_es(SegmentSelector::new(2, x86::Ring::Ring0));
        load_fs(SegmentSelector::new(2, x86::Ring::Ring0));
        load_gs(SegmentSelector::new(2, x86::Ring::Ring0));
        load_cs(SegmentSelector::new(1, x86::Ring::Ring0));
    }
}
