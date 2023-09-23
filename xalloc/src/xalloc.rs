// src/xalloc.rs

use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

pub enum MemoryType {
    NORMAL,
    EX_MEM,
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
                let ptr = unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align(size, 1).unwrap()) };
                if ptr.is_null() {
                    panic!("Memory allocation failed");
                }
                ptr
            }
            MemoryType::EX_MEM => {
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
                    std::alloc::dealloc(ptr, std::alloc::Layout::from_size_align(size, 1).unwrap());
                }
            }
            MemoryType::EX_MEM => {
                // Implement EX_MEM deallocation logic here
                unimplemented!("EX_MEM deallocation not implemented");
            }
        }
    }
}

