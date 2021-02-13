
use alloc::alloc::{GlobalAlloc, Layout};

extern "C" {
    fn kmalloc(size: usize) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn krealloc(ptr: *mut u8, size: usize) -> *mut u8;
}

pub struct SwebAllocator;

unsafe impl GlobalAlloc for SwebAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        kmalloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        kfree(ptr)
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        krealloc(ptr, new_size)
    }
}
