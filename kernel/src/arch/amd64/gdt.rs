use core::arch::asm;

use crate::arch::amd64::registers::{PrivilegeLevel, SegmentSelector, CS, DS, ES, FS, GS, SS};

static mut GDT: [GdtEntry; 5] = [GdtEntry::null(); 5];

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Gdtr {
    size: u16,
    offset: u64,
}

#[repr(C, align(8))]
#[derive(Clone, Copy, Debug)]
pub struct GdtEntry {
    entry: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct Flags {
    flags: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct AccessByte {
    ab: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct GdtEntryBuilder {
    pub base: u32,
    pub limit: u32,
    pub flags: Flags,
    pub access_byte: AccessByte,
}

#[derive(Clone, Copy, Debug)]
pub struct FlagsBuilder {
    pub granularity: bool,
    pub db: bool,
    pub long: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct AccessByteBuilder {
    pub present: bool,
    pub dpl: PrivilegeLevel,
    pub entry_type: bool,
    pub exec: bool,
    pub dc: bool,
    pub rw: bool,
    pub accessed: bool,
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
    const fn null() -> GdtEntry {
        GdtEntry { entry: 0 }
    }

    const fn new(base: u32, limit: u32, flags: Flags, access_byte: AccessByte) -> GdtEntry {
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
    const fn new(granularity: bool, db: bool, long: bool) -> Flags {
        Flags {
            flags: (granularity as u8) << 3 | (db as u8) << 2 | (long as u8) << 1,
        }
    }
}

impl AccessByte {
    const fn new(
        present: bool,
        dpl: PrivilegeLevel,
        entry_type: bool,
        exec: bool,
        dc: bool,
        rw: bool,
        accessed: bool,
    ) -> AccessByte {
        AccessByte {
            ab: (present as u8) << 7
                | (dpl as u8) << 5
                | (entry_type as u8) << 4
                | (exec as u8) << 3
                | (dc as u8) << 2
                | (rw as u8) << 1
                | accessed as u8,
        }
    }
}

impl GdtEntryBuilder {
    pub const fn build(self) -> GdtEntry {
        GdtEntry::new(self.base, self.limit, self.flags, self.access_byte)
    }
}

impl FlagsBuilder {
    pub const fn build(self) -> Flags {
        Flags::new(self.granularity, self.db, self.long)
    }
}

impl AccessByteBuilder {
    pub const fn build(self) -> AccessByte {
        AccessByte::new(
            self.present,
            self.dpl,
            self.entry_type,
            self.exec,
            self.dc,
            self.rw,
            self.accessed,
        )
    }
}

pub fn init(next_routine: fn()) -> Option<()> {
    setup_gdt()?;
    switch_gdt(next_routine);

    // Never returns
    Some(())
}

fn setup_gdt() -> Option<()> {
    let kernel_code_segment = GdtEntryBuilder {
        base: 0x0,
        limit: 0xfffff,
        flags: FlagsBuilder {
            granularity: true,
            db: false,
            long: true,
        }
        .build(),
        access_byte: AccessByteBuilder {
            present: true,
            dpl: PrivilegeLevel::Ring0,
            entry_type: true,
            exec: true,
            dc: false,
            rw: false,
            accessed: true,
        }
        .build(),
    }
    .build();

    let kernel_data_segment = GdtEntryBuilder {
        base: 0x0,
        limit: 0xfffff,
        flags: FlagsBuilder {
            granularity: true,
            db: false,
            long: true,
        }
        .build(),
        access_byte: AccessByteBuilder {
            present: true,
            dpl: PrivilegeLevel::Ring0,
            entry_type: true,
            exec: false,
            dc: false,
            rw: true,
            accessed: true,
        }
        .build(),
    }
    .build();

    let user_code_segment = GdtEntryBuilder {
        base: 0x0,
        limit: 0xfffff,
        flags: FlagsBuilder {
            granularity: true,
            db: false,
            long: true,
        }
        .build(),
        access_byte: AccessByteBuilder {
            present: true,
            dpl: PrivilegeLevel::Ring3,
            entry_type: true,
            exec: true,
            dc: false,
            rw: false,
            accessed: true,
        }
        .build(),
    }
    .build();

    let user_data_segment = GdtEntryBuilder {
        base: 0x0,
        limit: 0xfffff,
        flags: FlagsBuilder {
            granularity: true,
            db: false,
            long: true,
        }
        .build(),
        access_byte: AccessByteBuilder {
            present: true,
            dpl: PrivilegeLevel::Ring3,
            entry_type: true,
            exec: false,
            dc: false,
            rw: true,
            accessed: true,
        }
        .build(),
    }
    .build();

    unsafe {
        GDT[1] = kernel_code_segment;
        GDT[2] = kernel_data_segment;
        GDT[3] = user_code_segment;
        GDT[4] = user_data_segment;
    }

    Some(())
}

fn switch_gdt(next_routine: fn()) {
    unsafe {
        #[allow(static_mut_refs)]
        let gdtr = Gdtr::new(&GDT);

        asm!(
            "lgdt [{0}]",
            in(reg) &gdtr
        );

        let kernel_data_segment = SegmentSelector::new(2, false, PrivilegeLevel::Ring0);

        DS::set(kernel_data_segment);
        SS::set(kernel_data_segment);
        ES::set(kernel_data_segment);
        FS::set(kernel_data_segment);
        GS::set(kernel_data_segment);

        let kernel_code_segment = SegmentSelector::new(1, false, PrivilegeLevel::Ring0);

        CS::set(kernel_code_segment, next_routine);
    }
}
