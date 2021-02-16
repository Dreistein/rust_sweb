#![no_std]
#![feature(alloc_error_handler)]

use core::panic::PanicInfo;

extern crate alloc;
use alloc::vec::Vec;

mod allocator;
use allocator::SwebAllocator;

mod console;

#[global_allocator]
static ALLOCATOR: SwebAllocator = SwebAllocator;

#[no_mangle]
pub unsafe extern "C" fn add_one(val: u64) -> u64 {
    let mut v = Vec::new();

    println!("Hello from Rust!");

    v.push(val);
    v.push(1);
    //panic!("Panics work too!");
    v.into_iter().sum::<u64>()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    console::raw::printd("Kernel Panic\n");
    println!("[Rust Kernel Panic.]\n{}", info);
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Failed to Allocate: {:?}", layout)
}
