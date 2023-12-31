// src/xalloc.rs
//
use tikv_jemalloc_sys;

extern crate nix;

#[cfg(not(target_env = "msvc"))]
use tikv_jemalloc_sys::{malloc, free,};

use std::mem;
use std::os::raw::c_void;
pub mod numa_rs;

use numa_rs::{numa_available_wrapper, numa_alloc_onnode_wrapper, numa_free_wrapper, numa_num_configured_cpus_wrapper, numa_max_node_wrapper, numa_node_of_cpu_wrapper, numa_alloc_local_wrapper};

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
                Self::jeallocate(size)
            }
            MemoryType::EXMEM => {
                Self::allocate_ex_mem(size)
            }
        }
    }

    pub fn allocate_cxl_mem(&self, size: usize, preferred_node: i32) -> *mut u8 {
        // Implement EX_MEM allocation logic here
        match self.memory_type {
            MemoryType::NORMAL => {
                panic!("allocate_cxl_mem should not be used for NORMAL memory type");
            }
            MemoryType::EXMEM => {
                let available =  numa_available_wrapper();
                if available == -1 {
                    panic!("NUMA is not available.");
                }

                let is_cpuless = {
                    let max_cpus = numa_num_configured_cpus_wrapper();
                    let mut found_cpus = false;

                    for cpu in 0..max_cpus {
                        if numa_node_of_cpu_wrapper(cpu) == preferred_node {
                            found_cpus = true;
                            break;
                        }
                    }

                    !found_cpus
                };

                if !is_cpuless {
                    panic!("Not a cpuless numa node");
                }

                // Allocate memory on the preferred NUMA node
                let allocated_memory = numa_alloc_onnode_wrapper(size, preferred_node);
                if allocated_memory.is_null() {
                    panic!("Failed to allocate memory on the preferred NUMA node.");
                }

                // Convert the pointer to a mutable u8 pointer
                allocated_memory as *mut u8
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
                numa_free_wrapper(ptr as *mut c_void, size);
            }
        }
    }

    fn jeallocate(size: usize) -> *mut u8 {
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

    fn allocate_nuam_local(size: usize) -> *mut u8 {
        let allocated_memory = numa_alloc_local_wrapper(size);
        if allocated_memory.is_null() {
            panic!("Failed to allocate memory on the preferred NUMA node.");
        }

        // Convert the pointer to a mutable u8 pointer
        allocated_memory as *mut u8
    }

    fn find_cpuless_numa_nodes() -> Vec<i32> {
        let max_cpus = numa_num_configured_cpus_wrapper();
        let num_nodes = numa_max_node_wrapper() + 1;

        let mut cpu_less_nodes = vec![]; 

        for node in 0..num_nodes {
            let mut found_cpus = false;

            for cpu in 0..max_cpus {
                if numa_node_of_cpu_wrapper(cpu) == node {
                    found_cpus = true;
                    break;
                }
            }

            if !found_cpus {
                cpu_less_nodes.push(node);
            }
        }

        cpu_less_nodes
    }


    fn allocate_ex_mem(size: usize) -> *mut u8 {
        // Implement EX_MEM allocation logic here

        let cpuless_nodes = Self::find_cpuless_numa_nodes();

        if cpuless_nodes.is_empty() {
            println!("No CPU-less NUMA nodes found.");
        } else {
            println!("CPU-less NUMA nodes found: {:?}", cpuless_nodes);
        }

        // Allocate memory on the preferred NUMA node
        let preferred_node = *cpuless_nodes.iter().min().unwrap();
        let allocated_memory = numa_alloc_onnode_wrapper(size, preferred_node);
        if allocated_memory.is_null() {
            panic!("Failed to allocate memory on the preferred NUMA node.");
        }

        // Convert the pointer to a mutable u8 pointer
        allocated_memory as *mut u8
    }
}

