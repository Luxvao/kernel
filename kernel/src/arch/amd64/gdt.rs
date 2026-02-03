use core::arch::asm;

use crate::arch::amd64::registers::{PrivilegeLevel, CS, DS, ES, FS, GS, SS};

static mut GDT: [GdtEntry; 5] = [GdtEntry::null(); 5];

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct Gdtr {
    size: u16,
    offset: u64,
}

#[repr(C, align(8))]
#[derive(Clone, Copy, Debug)]
pub struct GdtEntry {
    entry: u64,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct Flags {
    flags: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct AccessByte {
    ab: u8,
}

impl Gdtr {
    pub fn new(gdt: &[GdtEntry]) -> Gdtr {
        Gdtr {
            size: gdt.len() as u16 * 8 - 1,
            offset: gdt.as_ptr() as u64,
        }
    }
}

impl GdtEntry {
    pub const fn null() -> GdtEntry {
        GdtEntry { entry: 0 }
    }

    pub fn new(base: u32, limit: u32, flags: Flags, access_byte: AccessByte) -> GdtEntry {
        GdtEntry {
            entry: (base as u64 & 0xff000000) << 32
                | (flags.flags as u64) << 52
                | (limit as u64 & 0xf0000) << 32
                | (access_byte.ab as u64) << 40
                | (base as u64 & 0xffffff) << 16
                | (limit as u64 & 0xffff),
        }
    }
}

impl Flags {
    pub fn new(granularity: bool, db: bool, long: bool) -> Flags {
        Flags {
            flags: (granularity as u8) << 3 | (db as u8) << 2 | (long as u8) << 1,
        }
    }
}

impl AccessByte {
    pub fn new(
        present: bool,
        dpl: PrivilegeLevel,
        system: bool,
        exec: bool,
        dc: bool,
        rw: bool,
        accessed: bool,
    ) -> AccessByte {
        AccessByte {
            ab: (present as u8) << 7
                | (dpl as u8) << 5
                | (system as u8) << 4
                | (exec as u8) << 3
                | (dc as u8) << 2
                | (rw as u8) << 1
                | accessed as u8,
        }
    }
}

pub fn init(next_routine: fn()) {
    setup_gdt();
    switch_gdt(next_routine);
}

fn setup_gdt() {
    let kernel_code_segment = GdtEntry::new(
        0,
        0xfffff,
        Flags::new(true, false, true),
        AccessByte::new(true, PrivilegeLevel::Ring0, true, true, false, true, true),
    );

    let kernel_data_segment = GdtEntry::new(
        0,
        0xfffff,
        Flags::new(true, false, true),
        AccessByte::new(true, PrivilegeLevel::Ring0, true, false, false, true, true),
    );

    let user_code_segment = GdtEntry::new(
        0,
        0xfffff,
        Flags::new(true, false, true),
        AccessByte::new(true, PrivilegeLevel::Ring3, true, true, true, true, true),
    );

    let user_data_segment = GdtEntry::new(
        0,
        0xfffff,
        Flags::new(true, false, true),
        AccessByte::new(true, PrivilegeLevel::Ring3, true, false, false, true, true),
    );

    unsafe {
        GDT[1] = kernel_code_segment;
        GDT[2] = kernel_data_segment;
        GDT[3] = user_code_segment;
        GDT[4] = user_data_segment;
    }
}

fn switch_gdt(next_routine: fn()) {
    unsafe {
        #[allow(static_mut_refs)]
        let gdtr = Gdtr::new(&GDT);

        asm!(
            "lgdt [{0}]",
            in(reg) &gdtr
        );

        DS::set(2, false, PrivilegeLevel::Ring0);
        SS::set(2, false, PrivilegeLevel::Ring0);
        ES::set(2, false, PrivilegeLevel::Ring0);
        FS::set(2, false, PrivilegeLevel::Ring0);
        GS::set(2, false, PrivilegeLevel::Ring0);

        CS::set(1, false, PrivilegeLevel::Ring0, next_routine);
    }
}
