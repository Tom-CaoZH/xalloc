// src/xalloc.rs
use tikv_jemalloc_sys;

#[cfg(not(target_env = "msvc"))]
use tikv_jemalloc_sys::{mallocx, malloc, free, MALLOCX_ARENA, MALLOCX_TCACHE, MALLOCX_TCACHE_NONE};

use std::mem;
use std::os::raw::c_void;


pub enum MemoryType {
    NORMAL,
    EXMEM,
}

pub struct XAllocator {
    memory_type: MemoryType,
}

impl XAllocator {
    pub fn new(memory_type: MemoryType) -> Self {
        Self { memory_type }
    }

    pub fn allocate(&self, size: usize) -> *mut u8 {
        match self.memory_type {
            MemoryType::NORMAL => {
                // Allocate memory using the jemalloc allocator for NORMAL type
                let mut ptr: *mut c_void = std::ptr::null_mut();
                unsafe {
                    ptr = malloc(size);
                }
                if ptr.is_null() {
                    panic!("Memory allocation failed");
                }
                let ptr = unsafe {
                    mem::transmute::<*mut c_void, *mut u8>(malloc(size))
                };
                ptr
            }
            MemoryType::EXMEM => {
                // Implement EX_MEM allocation logic here
                unimplemented!("EX_MEM allocation not implemented");
            }
        }
    }

    pub fn deallocate(&self, ptr: *mut u8, size: usize) {
        match self.memory_type {
            MemoryType::NORMAL => {
                // Deallocate memory using the jemalloc allocator for NORMAL type
                unsafe {
                    let ptr = ptr as *mut c_void;
                    free(ptr);
                }
            }
            MemoryType::EXMEM => {
                // Implement EX_MEM deallocation logic here
                unimplemented!("EX_MEM deallocation not implemented");
            }
        }
    }
}

