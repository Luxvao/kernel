use core::arch::asm;

macro_rules! setup_seg_reg {
    ( $( $x:ident ),* ) => {
        $(
            pub struct $x;

            impl $x {
                pub unsafe fn set(selector: u16) {
                    let gdt_offset = selector * 8;

                    asm!(
                        concat!("mov ", stringify!($x), ", {0:x}"),
                        in(reg) gdt_offset
                    );
                }

                pub unsafe fn get() -> u16 {
                    let mut gdt_offset: u16;

                    asm!(
                        concat!("mov {0:x}, ", stringify!($x)),
                        out(reg) gdt_offset
                    );

                    gdt_offset / 8
                }
            }
        )*
    };
}

pub struct CS;

impl CS {
    pub unsafe fn set(selector: u16, next_routine: fn()) {
        let gdt_offset = selector * 8;
        let next_routine_pointer = next_routine as usize;

        asm!(
            "push {0:x}",
            "push {1}",
            "retfq",
            in(reg) gdt_offset,
            in(reg) next_routine_pointer
        )
    }
}

setup_seg_reg!(DS, SS, ES, FS, GS);
