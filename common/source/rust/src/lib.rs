#![no_std]
#![feature(alloc_error_handler)]
#![feature(asm)]

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
    
    // cache_size();
    // panic!("Panics work too!");
    // dbg!(v[2]); // this will panic
    
    v.push(val);
    v.push(1);
    v.into_iter().sum::<u64>()
}


// see https://doc.rust-lang.org/unstable-book/library-features/asm.html
#[no_mangle]
pub unsafe extern "C" fn cache_size() -> u32 {
    let ebx: u32;
    let ecx: u32;

    asm!(
        "cpuid",
        // EAX 4 selects the "Deterministic Cache Parameters" CPUID leaf
        inout("eax") 4 => _,
        // ECX 0 selects the L0 cache information.
        inout("ecx") 0 => ecx,
        lateout("ebx") ebx,
        lateout("edx") _,
    );

    let size = ((ebx >> 22) + 1) * (((ebx >> 12) & 0x3ff) + 1) * ((ebx & 0xfff) + 1) * (ecx + 1);
    println!("L1 Cache: {}", size);
    
    size
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
