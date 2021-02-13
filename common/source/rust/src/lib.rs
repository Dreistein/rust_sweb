#![no_std]
#![feature(alloc_error_handler)]


use core::panic::PanicInfo;

extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc, format};

mod allocator;
use allocator::SwebAllocator;

mod console;

#[global_allocator]
static ALLOCATOR: SwebAllocator = SwebAllocator;

#[no_mangle]
pub unsafe extern "C" fn add_one(val: u64) -> u64 {
    let mut v = Vec::new();
    console::printk("Hello from Rust!\n");
    console::printd("Hello from Rust to the console!\n");
    v.push(val);
    v.push(1);
    //panic!("Panics work too!");
    v.into_iter().sum::<u64>()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    console::printd("Kernel Panic\n");
    console::printk("[Rust Kernel Panic.]\n");
    console::printk(&format!("{}", info));
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Failed to Allocate: {:?}", layout)
}