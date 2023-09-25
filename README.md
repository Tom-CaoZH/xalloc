# xalloc

*Still under construction.*

This lib is used to allocate normal DRAM-based memory and CXL-based memory using Rust.

Generally, for normal DRAM-based memory, we add a wrapper above [jemalloc](https://github.com/tikv/jemallocator). For CXL-based memory, because CXL-based memory can be transfered into cpuless numa-node memory, we enable specific numa node memory allocation. 

## APIs

``
fn new(memory_type: MemoryType) -> Self

fn allocate(&self, size: usize) -> *mut u8

fn allocate_cxl_mem(&self, size: usize, preferred_node: i32) -> *mut u8

fn deallocate(&self, ptr: *mut u8, size: usize)

``

## Examples

You can see examples [here](https://github.com/Tom-CaoZH/xalloc/blob/master/xalloc/src/lib.rs)

Referrence:

[memkind](https://github.com/memkind/memkind)

[Simplified Interface to Complex Memory](https://github.com/lanl/SICM)

[SMDK, Scalable Memory Development Kit](https://github.com/OpenMPDK/SMDK)
