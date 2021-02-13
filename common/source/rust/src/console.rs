
use alloc::vec::Vec;
use alloc::str;

extern "C" {
    fn _kprint_c(c: u8);
    fn _dprint_c(c: u8);
}

fn make_printable(c: u8) -> u8 {
    match c {
        0x20..=0x7f | b'\n' => c, // printable
        _ => b'?',
    }
}

pub fn printk(msg: &str) {
    for c in msg.as_bytes() {
        unsafe {
            _kprint_c(make_printable(*c));
        }
    }
}

pub fn printd(msg: &str) {
    for c in msg.as_bytes() {
        unsafe {
            _dprint_c(make_printable(*c));
        }
    }
}

