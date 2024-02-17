use alloc_rs::vec::Vec;

struct VideoMemory {
    resx: usize,
    resy: usize,
    vmem: *mut Vec<Vec<(u8, u8)>>,
    buffer: Vec<(u8, u8)>,
    pos: i32,
}
