// src/xalloc.rs
use tikv_jemalloc_sys;

extern crate nix;
// use libnuma::numa;
use std::process;
use nix::sys::pthread::pthread_self;
use std::os::raw::c_int;

#[cfg(not(target_env = "msvc"))]
use tikv_jemalloc_sys::{mallocx, malloc, free, MALLOCX_ARENA};

use std::mem;
use std::os::raw::c_void;
// use std::sync::Once;

pub enum MemoryType {
    NORMAL,
    EXMEM,
}

pub struct XAllocator {
    memory_type: MemoryType,
    // init_once: Once, 
    arena_zero: usize,
    arena_map_mask: usize, 
}


impl XAllocator {
    fn get_fs_base() -> usize {
        pthread_self() as usize
    }

    fn xalloc_thread_get_arena(arena: &mut u32) {
        let arena_idx = hash64(get_fs_base()) & self.arena_map_mask;
        *arena = self.arena_zero + arena_idx;
    }

    fn allocate_ex_mem(size: usize) -> *mut u8 {
        // Implement EX_MEM allocation logic here
        
        let mut arena: c_int = 0;
        xalloc_thread_get_arena(&arena);
        let flags = MALLOCX_ARENA(arena as usize);
        let mut ptr: *mut c_void = std::ptr::null_mut();
        unsafe {
            ptr = mallocx(size, flag);
        }
        if ptr.is_null() {
            panic!("EXMEM memory allocation failed");
        }
        let ptr = unsafe {
            mem::transmute::<*mut c_void, *mut u8>(malloc(size))
        };
        ptr
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
                allocate_ex_mem(size)
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

