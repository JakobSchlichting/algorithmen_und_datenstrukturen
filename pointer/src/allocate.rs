use std::alloc::{GlobalAlloc, System};

const CAPACITY: u8 = 10;

pub struct CustomAllocater<T> {
    buffer: Option<[T, 10]>
}

unsafe impl GlobalAlloc for CustomAllocater {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}
