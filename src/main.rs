#![feature(link_cfg)]
#![no_main]
#![no_std]

extern crate panic_abort;
use libc::{c_void, write, STDOUT_FILENO};

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        write(
            STDOUT_FILENO,
            b"hello world\n".as_ptr() as *const c_void,
            12,
        );
    }
}

#[link(name = "c", kind = "static", cfg(target_feature = "crt-static"))]
#[link(name = "c", cfg(not(target_feature = "crt-static")))]
extern "C" {}
