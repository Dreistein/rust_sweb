#![no_std]

use core::panic::PanicInfo;


#[no_mangle]
pub unsafe extern "C" fn add_one(val: u64) -> u64 {
    val + 1
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
