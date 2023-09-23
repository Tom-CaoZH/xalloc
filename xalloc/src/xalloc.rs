// src/xalloc.rs
use tikv_jemalloc_sys;

extern crate nix;
use linux_support::libnuma;
use std::process;
use nix::sys::pthread::pthread_self;
use std::os::raw::c_int;

#[cfg(not(target_env = "msvc"))]
use tikv_jemalloc_sys::{mallocx, malloc, free, MALLOCX_ARENA, MALLOCX_TCACHE, MALLOCX_TCACHE_NONE};

use std::mem;
use std::os::raw::c_void;
use std::sync::Once;

pub enum MemoryType {
    NORMAL,
    EXMEM,
}

pub struct XAllocator {
    memory_type: MemoryType,
    init_once: Once, 
}


impl XAllocator {
    fn get_fs_base() -> usize {
        pthread_self() as usize
    }

    fn xalloc_thread_get_arena(arena: &mut u32, size: usize) -> i32 {
        let arena_idx = hash64(get_fs_base()) & kind.arena_map_mask;
        *arena = kind.arena_zero + arena_idx;
        0
    }

    fn xalloc_ex_mem_init() {
        println!("Initializing ex_mem");

        if let Err(e) = numa::available() {
            eprintln!("Error: {:?}", e);
            process::abort(); // Abort the program in case of any error
        }

        println!("NUMA available");
    }

    fn init_xallocator(&self) {
        self.init_once.call_once(|| {
            xalloc_ex_mem_init();
        });
    }

    fn allocate_ex_mem(size: usize) -> *mut u8 {
        // Implement EX_MEM allocation logic here
        
        
        let mut arena: c_int = 0;
        let flags = MALLOCX_ARENA(arena as usize) | get_tcache_flag((*kind).partition, size);
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

