extern crate libc;
use libc::*;

#[link(name = "numa")]
extern {
    pub fn numa_available() -> i32;
    pub fn numa_alloc_local(size: usize) -> *mut c_void;
    pub fn numa_alloc_onnode(size: usize, node: i32) -> *mut c_void;
    pub fn numa_free(ptr: *mut c_void, size: usize);
    pub fn numa_preferred() -> i32;
    // Add other libnuma function definitions here as needed
}

// Wrapper functions

// Wrapper for numa_available()
pub fn numa_available_wrapper() -> i32 {
    unsafe {
        numa_available()
    }
}

// Wrapper for numa_alloc_local()
pub fn numa_alloc_local_wrapper(size: usize) -> *mut c_void {
    unsafe {
        numa_alloc_local(size)
    }
}

// Wrapper for numa_alloc_onnode()
pub fn numa_alloc_onnode_wrapper(size: usize, node: i32) -> *mut c_void {
    unsafe {
        numa_alloc_onnode(size, node)
    }
}

// Wrapper for numa_free()
pub fn numa_free_wrapper(ptr: *mut c_void, size: usize) {
    unsafe {
        numa_free(ptr, size)
    }
}

// Wrapper for numa_preferred()
pub fn numa_preferred_wrapper() -> i32 {
    unsafe {
        numa_preferred()
    }
}

