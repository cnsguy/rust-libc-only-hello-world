#![feature(lang_items, alloc_error_handler, core_c_str)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate libc;

mod allocator;
mod lang_items;

use core::ffi::CStr;

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    unsafe {
        libc::puts(CStr::from_bytes_with_nul_unchecked(b"hello world\0").as_ptr());
    }

    0
}
