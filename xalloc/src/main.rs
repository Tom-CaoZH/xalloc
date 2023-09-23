// src/main.rs

mod xalloc;

use xalloc::{XAllocator, MemoryType};
use std::ptr;

fn main() {
    let normal_allocator = XAllocator::new(MemoryType::NORMAL);
    let ex_mem_allocator = XAllocator::new(MemoryType::EX_MEM);

    // Allocate memory
    let normal_ptr = normal_allocator.allocate(100);
    let ex_mem_ptr = ex_mem_allocator.allocate(200);

    // Write data to the memory
    let normal_data: &[u8] = b"Hello, NORMAL memory!";
    unsafe {
        ptr::copy(non_ex_data.as_ptr(), normal_ptr, normal_data.len());
    }

    let ex_mem_data: &[u8] = b"Hello, EX_MEM memory!";
    unsafe {
        ptr::copy(ex_mem_data.as_ptr(), ex_mem_ptr, ex_mem_data.len());
    }

    // Read and print data from the memory
    let normal_str = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(normal_ptr, normal_data.len())) };
    let ex_mem_str = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(ex_mem_ptr, ex_mem_data.len())) };

    println!("Data from NORMAL memory: {}", normal_str);
    println!("Data from EX_MEM memory: {}", ex_mem_str);

    // Deallocate memory
    normal_allocator.deallocate(normal_ptr, 100);
    ex_mem_allocator.deallocate(ex_mem_ptr, 200);
}

