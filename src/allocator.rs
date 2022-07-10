use core::ffi::c_void;
use alloc::alloc::{GlobalAlloc, Layout};

struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        libc::malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        libc::free(ptr as *mut c_void)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        libc::calloc(1, layout.size()) as *mut u8
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        libc::realloc(ptr as *mut c_void, new_size) as *mut u8
    }
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;
