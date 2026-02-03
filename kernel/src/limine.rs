use limine::{
    memory_map::{Entry, EntryType},
    request::{FramebufferRequest, MemoryMapRequest, StackSizeRequest},
    BaseRevision,
};

use crate::mem::{MMap, MMapInfo};

pub static BASE_REVISION: BaseRevision = BaseRevision::new();

pub const STACK_SIZE: u64 = 0x100000;

pub static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new().with_size(STACK_SIZE);

pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

// Implement the limine Entry -> MMap conversion
impl From<&&Entry> for MMap {
    fn from(value: &&Entry) -> Self {
        let info = MMapInfo {
            base: value.base,
            length: value.length,
        };

        match value.entry_type {
            EntryType::USABLE => MMap::Usable(info),
            EntryType::RESERVED => MMap::Reserved(info),
            EntryType::ACPI_RECLAIMABLE => MMap::ACPIReclaimable(info),
            EntryType::ACPI_NVS => MMap::ACPINVS(info),
            EntryType::BAD_MEMORY => MMap::BadMemory(info),
            EntryType::BOOTLOADER_RECLAIMABLE => MMap::BootloaderReclaimable(info),
            EntryType::EXECUTABLE_AND_MODULES => MMap::ExecutableAndModules(info),
            EntryType::FRAMEBUFFER => MMap::Framebuffer(info),
            _ => MMap::Other(info),
        }
    }
}
