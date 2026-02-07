use core::sync::atomic::AtomicU64;
use core::{mem::transmute, sync::atomic::Ordering};

use seq_macro::seq;

use crate::arch::amd64::registers::{PrivilegeLevel, SegmentSelector};

// Generate the IDT and the handlers
type InterruptHandler = extern "x86-interrupt" fn(InterruptStackFrame);

static mut IDT: [GateDescriptor; 256] = [GateDescriptor::null(); 256];

seq!(N in 0..256 {
    extern "x86-interrupt" fn trampoline~N(mut isf: InterruptStackFrame) {
        match IDT_CALLBACKS[N].get() {
            0 => panic!("IDT callback not present!"),
            c => unsafe {
                transmute::<u64, InterruptCallback>(c)(&mut isf);
            }
        }
    }
});

// Publicly accessible IDT callback structure. Makes hotswappable callbacks easy
pub type InterruptCallback = fn(&mut InterruptStackFrame);

pub static IDT_CALLBACKS: [InterruptCallbackWrapper; 256] = seq!(_ in 0..256 {
    [
        #(
            InterruptCallbackWrapper::null(),
        )*
    ]
});

#[derive(Debug)]
pub struct InterruptCallbackWrapper {
    callback: AtomicU64,
}

#[derive(Clone, Copy)]
pub enum GateType {
    Interrupt = 0xe,
    Trap = 0xf,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct GateDescriptor {
    offset_0_15: u16,
    segment_selector: u16,
    ist_reserved: u8,
    gate_type_dpl_p: u8,
    offset_16_31: u16,
    offset_32_63: u32,
    reserved: u32,
}

pub struct GateDescriptorBuilder {
    present: bool,
    handler: InterruptHandler,
    dpl: PrivilegeLevel,
    segment_selector: SegmentSelector,
    gate_type: GateType,
    ist: u8,
}

impl GateDescriptor {
    const fn null() -> GateDescriptor {
        GateDescriptor {
            offset_0_15: 0,
            segment_selector: 0,
            ist_reserved: 0,
            gate_type_dpl_p: 0,
            offset_16_31: 0,
            offset_32_63: 0,
            reserved: 0,
        }
    }

    const fn new(
        present: bool,
        handler_offset: u64,
        dpl: PrivilegeLevel,
        segment_selector: SegmentSelector,
        gate_type: GateType,
        ist: u8,
    ) -> GateDescriptor {
        GateDescriptor {
            offset_0_15: (handler_offset & 0xffff) as u16,
            segment_selector: segment_selector.entry,
            ist_reserved: ist & 0x7,
            gate_type_dpl_p: gate_type as u8 & 0xf | (dpl as u8) << 5 | (present as u8) << 7,
            offset_16_31: ((handler_offset >> 16) & 0xffff) as u16,
            offset_32_63: ((handler_offset >> 32) & 0xffffffff) as u32,
            reserved: 0,
        }
    }
}

impl GateDescriptorBuilder {
    pub fn build(self) -> GateDescriptor {
        GateDescriptor::new(
            self.present,
            self.handler as u64,
            self.dpl,
            self.segment_selector,
            self.gate_type,
            self.ist,
        )
    }
}

#[repr(C)]
pub struct InterruptStackFrame {
    rip: usize,
    cs: usize,
    rflags: usize,
    rsp: usize,
    ss: usize,
}

impl InterruptCallbackWrapper {
    pub const fn null() -> InterruptCallbackWrapper {
        InterruptCallbackWrapper {
            callback: AtomicU64::new(0),
        }
    }

    pub fn get(&self) -> u64 {
        self.callback.load(Ordering::SeqCst)
    }

    pub fn set(&self, callback: InterruptCallback) {
        self.callback.store(callback as u64, Ordering::SeqCst);
    }
}

pub fn init() {
    // Setup IDT entries linking back to the trampolines
    let trampolines: [InterruptHandler; 256] = seq!(N in 0..256 {
       [
           #(
               trampoline~N,
           )*
       ]
    });

    let mut idt_setup = [GateDescriptor::null(); 256];

    let kernel_code_segment_selector = SegmentSelector::new(1, false, PrivilegeLevel::Ring0);

    for i in 0..=1 {
        idt_setup[i] = GateDescriptorBuilder {
            present: true,
            handler: trampolines[i],
            dpl: PrivilegeLevel::Ring0,
            segment_selector: kernel_code_segment_selector,
            gate_type: GateType::Trap,
            ist: 0,
        }
        .build();
    }

    idt_setup[2] = GateDescriptorBuilder {
        present: true,
        handler: trampolines[2],
        dpl: PrivilegeLevel::Ring0,
        segment_selector: kernel_code_segment_selector,
        gate_type: GateType::Interrupt,
        ist: 0,
    }
    .build();

    for i in 3..=7 {
        idt_setup[i] = GateDescriptorBuilder {
            present: true,
            handler: trampolines[i],
            dpl: PrivilegeLevel::Ring0,
            segment_selector: kernel_code_segment_selector,
            gate_type: GateType::Trap,
            ist: 0,
        }
        .build();
    }

    idt_setup[8] = GateDescriptorBuilder {
        present: true,
        handler: trampolines[8],
        dpl: PrivilegeLevel::Ring0,
        segment_selector: kernel_code_segment_selector,
        gate_type: GateType::Interrupt,
        ist: 0,
    }
    .build();

    for i in 9..=14 {
        idt_setup[i] = GateDescriptorBuilder {
            present: true,
            handler: trampolines[i],
            dpl: PrivilegeLevel::Ring0,
            segment_selector: kernel_code_segment_selector,
            gate_type: GateType::Trap,
            ist: 0,
        }
        .build();
    }

    idt_setup[15] = GateDescriptorBuilder {
        present: true,
        handler: trampolines[15],
        dpl: PrivilegeLevel::Ring0,
        segment_selector: kernel_code_segment_selector,
        gate_type: GateType::Interrupt,
        ist: 0,
    }
    .build();

    for i in 16..=17 {
        idt_setup[i] = GateDescriptorBuilder {
            present: true,
            handler: trampolines[i],
            dpl: PrivilegeLevel::Ring0,
            segment_selector: kernel_code_segment_selector,
            gate_type: GateType::Trap,
            ist: 0,
        }
        .build();
    }

    idt_setup[18] = GateDescriptorBuilder {
        present: true,
        handler: trampolines[18],
        dpl: PrivilegeLevel::Ring0,
        segment_selector: kernel_code_segment_selector,
        gate_type: GateType::Interrupt,
        ist: 0,
    }
    .build();

    idt_setup[19] = GateDescriptorBuilder {
        present: true,
        handler: trampolines[19],
        dpl: PrivilegeLevel::Ring0,
        segment_selector: kernel_code_segment_selector,
        gate_type: GateType::Trap,
        ist: 0,
    }
    .build();

    for i in 20..=31 {
        idt_setup[i] = GateDescriptorBuilder {
            present: true,
            handler: trampolines[i],
            dpl: PrivilegeLevel::Ring0,
            segment_selector: kernel_code_segment_selector,
            gate_type: GateType::Trap,
            ist: 0,
        }
        .build();
    }

    for i in 32..=47 {
        idt_setup[i] = GateDescriptorBuilder {
            present: true,
            handler: trampolines[i],
            dpl: PrivilegeLevel::Ring0,
            segment_selector: kernel_code_segment_selector,
            gate_type: GateType::Interrupt,
            ist: 0,
        }
        .build();
    }

    for i in 48..=255 {
        idt_setup[i] = GateDescriptorBuilder {
            present: true,
            handler: trampolines[i],
            dpl: PrivilegeLevel::Ring0,
            segment_selector: kernel_code_segment_selector,
            gate_type: GateType::Trap,
            ist: 0,
        }
        .build();
    }

    // I hope you like the above. That's real rust gymnastics right there

    // TODO Switch to the IDT
}
