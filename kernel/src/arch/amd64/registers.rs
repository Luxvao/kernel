use core::arch::asm;

#[derive(Clone, Copy, Debug)]
pub enum PrivilegeLevel {
    Ring0 = 0b00,
    Ring1 = 0b01,
    Ring2 = 0b10,
    Ring3 = 0b11,
}

#[derive(Clone, Copy)]
pub struct SegmentSelector {
    pub entry: u16,
}

impl SegmentSelector {
    pub fn new(idx: u16, ti: bool, rpl: PrivilegeLevel) -> SegmentSelector {
        SegmentSelector {
            entry: idx << 3 | (ti as u16) << 2 | rpl as u16,
        }
    }
}

macro_rules! setup_seg_reg {
    ( $( $x:ident ),* ) => {
        $(
            pub struct $x;

            impl $x {
                pub unsafe fn set(selector: SegmentSelector) {
                    asm!(
                        concat!("mov ", stringify!($x), ", {0:x}"),
                        in(reg) selector.entry
                    );
                }

                pub unsafe fn get() -> SegmentSelector {
                    let mut selector: u16;

                    asm!(
                        concat!("mov {0:x}, ", stringify!($x)),
                        out(reg) selector
                    );

                    SegmentSelector {
                        entry: selector
                    }
                }
            }
        )*
    };
}

pub struct CS;

impl CS {
    pub unsafe fn set(selector: SegmentSelector, next_routine: fn()) {
        let next_routine_pointer = next_routine as usize;

        asm!(
            "push {0:x}",
            "push {1}",
            "retfq",
            in(reg) selector.entry,
            in(reg) next_routine_pointer
        )
    }

    pub unsafe fn get() -> SegmentSelector {
        let selector: u16;

        asm!("mov {0:x}, cs", out(reg) selector);

        SegmentSelector { entry: selector }
    }
}

setup_seg_reg!(DS, SS, ES, FS, GS);
