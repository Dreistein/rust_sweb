
use alloc::vec::Vec;
use alloc::str;
use core::fmt;

pub mod raw {
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
}

pub struct Writer {
    print_function: fn(&str),
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        (self.print_function)(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = Writer { print_function: raw::printk };
    writer.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! dbg {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        $crate::println!("[{}:{}]", file!(), line!());
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::println!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}