/*
 * A Basic heap allocator which cannot deallocate memory.
 * It allocates memory by simply increasing a pointer and is only intended for learning and testing purposes.
 *
 * Author: Philipp Oppermann, https://os.phil-opp.com/allocator-designs/
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-01-13
 */

use crate::allocator::global::{align_up, dump_free_list, Locked};
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

/// A simple bump allocator that allocates memory in a linear fashion.
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    /// Create a new empty bump allocator.
    pub const fn new() -> BumpAllocator {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    /// Initialize the allocator and set the whole heap as unused.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        //todo!("bump::init() is not implemented yet.")

        // The heap range is half-open: heap_start is included,
        // while heap_end points one byte past the usable memory.
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        // next always points to the first byte that has not been allocated yet.
        // Reinitializing the allocator discards all previous allocation state.
        self.next = heap_start;
        self.allocations = 0;
    }

    /// Dump free memory for debugging purposes.
    /// Print the current heap bounds, next free address, and remaining memory.
    pub fn dump_free_list(&mut self) {
        //todo!("bump::dump_free_list() is not implemented yet.")
        println!("Bump allocator:");
        println!("\tHeap_start: {:#x}, Heap end: {:#x}", self.heap_start, self.heap_end);
        println!("\tFree block(s):");
        println!("\t\tBlock at {:#x} with size {}", self.next, self.heap_end - self.next);
    }

    /// Allocate memory of the given size and alignment.
    /// Align the next free address and move it forward by the allocation size.
    /// Returns a null pointer if the allocation does not fit.
    pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        //todo!("bump::alloc() is not implemented yet.")

        // The current cursor may not satisfy the requested alignment.
        // Any skipped bytes become unused alignment padding.
        let alloc_start = align_up(self.next, layout.align());

        // Saturating addition prevents an overflowing size from wrapping around
        // and incorrectly appearing to lie inside the heap.
        let alloc_end = alloc_start.saturating_add(layout.size());
        
        if alloc_end <= self.heap_end {
            // Only update the allocator state after confirming that the complete
            // allocation fits into the heap.
            self.next = alloc_end;
            self.allocations += 1;
            alloc_start as *mut u8
        } else {
            null_mut()
        }
    }

    /// Deallocate memory (not supported by bump allocator because no metadata is stored describing
    /// previous allocations).
    /// Freed memory is therefore not reused.
    pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        //todo!("bump::dealloc() is not implemented yet.")
        let _ = ptr;
        let _ = layout;
    }
}

// Trait required by the Rust runtime for heap allocations
unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            self.lock().alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            self.lock().dealloc(ptr, layout);
        }
    }
}
