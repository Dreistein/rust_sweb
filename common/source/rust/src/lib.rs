#![no_std]
#![no_main]

#![feature(alloc_error_handler)]

extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use alloc::str;
use alloc::alloc::{GlobalAlloc, Layout};


extern "C" {
    fn kmalloc(size: usize) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn krealloc(ptr: *mut u8, size: usize) -> *mut u8;
    fn kprintfd(format: *const u8, ...);
}

struct SwebAllocator;

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


#[global_allocator]
static ALLOCATOR: SwebAllocator = SwebAllocator;


use core::panic::PanicInfo;

fn printk(msg: &str) {
    let mut msg = Vec::from(msg.as_bytes());
    //msg.push(0);
    unsafe {
        kprintfd(&msg[0]);
    }
}

#[no_mangle]
pub unsafe extern "C" fn add_one(val: u64) -> u64 {
    let mut v = Vec::new();
    printk("Hello from Rust!\n");
    v.push(val);
    v.push(1);
    panic!("lel no");
    v.into_iter().sum::<u64>()
}


/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    printk("PANIC in kernel!\n");
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}