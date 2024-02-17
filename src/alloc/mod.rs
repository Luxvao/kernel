use alloc_rs::alloc::GlobalAlloc;

#[global_allocator]
struct PageAlloc {}

unsafe impl GlobalAlloc for PageAlloc {

}

