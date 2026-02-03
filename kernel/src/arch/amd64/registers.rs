use core::{arch::asm, fmt::Write};

use crate::drivers::io::serial::{Uart, UartPort};

#[derive(Clone, Copy, Debug)]
pub enum PrivilegeLevel {
    Ring0 = 0b00,
    Ring1 = 0b01,
    Ring2 = 0b10,
    Ring3 = 0b11,
}

macro_rules! setup_seg_reg {
    ( $( $x:ident ),* ) => {
        $(
            pub struct $x;

            impl $x {
                pub unsafe fn set(idx: u16, ti: bool, rpl: PrivilegeLevel) {
                    let selector = idx << 3 | (ti as u16) << 2 | rpl as u16;

                    asm!(
                        concat!("mov ", stringify!($x), ", {0:x}"),
                        in(reg) selector
                    );
                }

                pub unsafe fn get() -> (u16, bool, PrivilegeLevel) {
                    let mut selector: u16;

                    asm!(
                        concat!("mov {0:x}, ", stringify!($x)),
                        out(reg) selector
                    );

                    let dpl = match selector & 0b11 {
                        0b00 => PrivilegeLevel::Ring0,
                        0b01 => PrivilegeLevel::Ring1,
                        0b10 => PrivilegeLevel::Ring2,
                        0b11 => PrivilegeLevel::Ring3,
                        _ => unreachable!(),
                    };

                    (selector >> 3, (selector >> 2) & 0b1 == 1, dpl)
                }
            }
        )*
    };
}

pub struct CS;

impl CS {
    pub unsafe fn set(idx: u16, ti: bool, rpl: PrivilegeLevel, next_routine: fn()) {
        let selector = (idx << 3) | (ti as u16) << 2 | rpl as u16;
        let next_routine_pointer = next_routine as usize;

        let mut uart = Uart::init(UartPort::COM1);

        writeln!(uart, "\nSelector: {:x}", selector).unwrap();

        asm!(
            "push {0:x}",
            "push {1}",
            "retfq",
            in(reg) selector,
            in(reg) next_routine_pointer
        )
    }

    pub unsafe fn get() -> (u16, bool, PrivilegeLevel) {
        let selector: u16;

        asm!("mov {0:x}, cs", out(reg) selector);

        let dpl = match selector & 0b11 {
            0b00 => PrivilegeLevel::Ring0,
            0b01 => PrivilegeLevel::Ring1,
            0b10 => PrivilegeLevel::Ring2,
            0b11 => PrivilegeLevel::Ring3,
            _ => unreachable!(),
        };

        (selector >> 3, (selector >> 2 & 0b1) == 1, dpl)
    }
}

setup_seg_reg!(DS, SS, ES, FS, GS);
