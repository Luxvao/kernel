use x86_64::{
    registers::segmentation::{Segment, CS, SS},
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
    PrivilegeLevel,
};

use crate::tss::TSS;

pub fn init() {
    static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

    unsafe {
        GDT.append(Descriptor::kernel_code_segment());
        GDT.append(Descriptor::user_code_segment());
        GDT.append(Descriptor::user_data_segment());

        GDT.append(Descriptor::tss_segment(&TSS));

        GDT.load();
    }

    set_segm();
}

fn set_segm() {
    unsafe {
        CS::set_reg(SegmentSelector::new(1, PrivilegeLevel::Ring0));
        SS::set_reg(SegmentSelector::new(3, PrivilegeLevel::Ring3));
    }
}
