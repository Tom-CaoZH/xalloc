extern crate libc;

pub mod numa_rs {
    // Module content goes here
    use libc::*;

    #[repr(C)]
    pub struct bitmask
    {
        pub size: c_ulong,
        pub maskp: *mut c_ulong,
    }

    // Call numa_available() from libnuma
    pub fn numa_available() -> i32 {
        unsafe {
            numa_available()
        }
    }

    // Call numa_alloc_local() from libnuma
    pub fn numa_alloc_local(size: usize) -> *mut c_void {
        unsafe {
            numa_alloc_local(size)
        }
    }

    // Call numa_alloc_onnode() from libnuma
    pub fn numa_alloc_onnode(size: usize, node: i32) -> *mut c_void {
        unsafe {
            numa_alloc_onnode(size, node)
        }
    }

    // Call numa_free() from libnuma
    pub fn numa_free(ptr: *mut c_void, size: usize) {
        unsafe {
            numa_free(ptr, size)
        }
    }

    // Call numa_preferred() from libnuma
    pub fn numa_preferred() -> i32 {
        unsafe {
            numa_preferred()
        }
    }
}
