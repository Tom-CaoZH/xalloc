// src/xalloc.rs
//
use tikv_jemalloc_sys;

extern crate nix;
// use libnuma::numa;
use std::os::raw::c_int;

#[cfg(not(target_env = "msvc"))]
use tikv_jemalloc_sys::{mallocx, malloc, free, MALLOCX_ARENA};

use std::mem;
use std::os::raw::c_void;
// use std::sync::Once;
pub mod numa_rs;

use numa_rs::{numa_available, numa_preferred, numa_alloc_onnode, numa_free};

pub enum MemoryType {
    NORMAL,
    EXMEM,
}

pub struct XAllocator {
    memory_type: MemoryType,
    // init_once: Once, 
    // arena_zero: usize,
    // arena_map_mask: usize, 
}


impl XAllocator {
    // fn get_fs_base() -> usize {
        // pthread_self() as usize
    // }

    // fn xalloc_thread_get_arena(arena: &mut u32) {
        // let arena_idx = hash64(get_fs_base()) & self.arena_map_mask;
        // *arena = self.arena_zero + arena_idx;
    // }

    fn jeallocate_ex_mem(size: usize) -> *mut u8 {
        // Implement EX_MEM allocation logic here
        
        let mut arena: c_int = 0;
        // xalloc_thread_get_arena(&arena);
        let flags = MALLOCX_ARENA(arena as usize);
        let mut ptr: *mut c_void = std::ptr::null_mut();
        unsafe {
            ptr = mallocx(size, flags);
        }
        if ptr.is_null() {
            panic!("EXMEM memory allocation failed");
        }
        let ptr = unsafe {
            mem::transmute::<*mut c_void, *mut u8>(malloc(size))
        };
        ptr
    }

    fn allocate_ex_mem(size: usize) -> *mut u8 {
        // Implement EX_MEM allocation logic here
        // Check if NUMA is available
        let available = unsafe { numa_available() };
        if available == -1 {
            panic!("NUMA is not available.");
        }

        // Get the preferred NUMA node
        let preferred_node = unsafe { numa_preferred() };
        println!("Preferred NUMA node: {}", preferred_node);

        // Allocate memory on the preferred NUMA node
        let allocated_memory = unsafe { numa_alloc_onnode(size, preferred_node) };
        if allocated_memory.is_null() {
            panic!("Failed to allocate memory on the preferred NUMA node.");
        }

        // Convert the pointer to a mutable u8 pointer
        allocated_memory as *mut u8

        
    }

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
                    panic!("NORMAL Memory allocation failed");
                }
                let ptr = unsafe {
                    mem::transmute::<*mut c_void, *mut u8>(malloc(size))
                };
                ptr
            }
            MemoryType::EXMEM => {
                Self::allocate_ex_mem(size)
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
                unsafe {
                    numa_free(ptr as *mut c_void, size);
                }
            }
        }
    }
}

