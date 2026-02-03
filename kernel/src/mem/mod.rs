// pub mod salloc;

// Enum describing memory maps
#[derive(Clone, Copy, Debug)]
pub enum MMap {
    Usable(MMapInfo),
    Reserved(MMapInfo),
    ACPIReclaimable(MMapInfo),
    ACPINVS(MMapInfo),
    BadMemory(MMapInfo),
    BootloaderReclaimable(MMapInfo),
    ExecutableAndModules(MMapInfo),
    Framebuffer(MMapInfo),
    Other(MMapInfo),
}

// Extra metadata from memory maps
#[derive(Clone, Copy, Debug)]
pub struct MMapInfo {
    pub base: u64,
    pub length: u64,
}
