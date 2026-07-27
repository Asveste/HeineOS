/*
 * A heap allocator that uses a linked list to manage free memory blocks.
 * It allows for dynamic memory allocation and deallocation.
 *
 * Author: Philipp Oppermann, https://os.phil-opp.com/allocator-designs/
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-01-13
 */

use crate::allocator::global::{align_up, Locked};
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

/// Header of a free block in the list allocator.
struct ListNode {
    /// Size of the memory block
    size: usize,

    /// &'static mut type semantically describes an owned object behind a pointer.
    /// Basically, it’s a Box without a destructor that frees the object at the end of the scope.
    /// Its lifetime is static, meaning it will live for the entire duration of the program.
    /// Of course, this is not true in reality, as we might delete the list node at some point.
    /// But the compiler does not know this.
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    /// Create a new ListNode with the given size and no next node.
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    /// Get the start address of the memory block.
    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    /// Get the end address of the memory block.
    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

/// A linked list allocator that uses a free list to manage memory.
pub struct LinkedListAllocator {
    head: ListNode,
    heap_start: usize,
    heap_end: usize,
}

impl LinkedListAllocator {
    /// Create a new empty linked list allocator.
    pub const fn new() -> LinkedListAllocator {
        LinkedListAllocator {
            head: ListNode::new(0),
            heap_start: 0,
            heap_end: 0,
        }
    }

    /// Initialize the allocator with the heap bounds given in the constructor.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        //todo!("list::init() is not implemented yet.")
        self.heap_start = heap_start;
        self.heap_end = heap_start.saturating_add(heap_size);
        
        unsafe {
            self.add_free_block(heap_start, heap_size);
        }
    }

    /// Adds the given free memory block 'addr' to the front of the free list.
    unsafe fn add_free_block(&mut self, addr: usize, size: usize) {
        //todo!("list::add_free_block() is not implemented yet.")
        if size >= size_of::<ListNode>() && align_up(addr, align_of::<ListNode>()) == addr {
            let old = self.head.next.take();
            let new_node_ptr = addr as *mut ListNode;
            
            unsafe {
                new_node_ptr.write(ListNode::new(size));
                let new_node: &'static mut ListNode = &mut *new_node_ptr;

                new_node.next = old;
                self.head.next = Some(new_node);
            }
        }
    }

    /// Search a free block with the given size and alignment and remove it from the list.
    fn find_free_block(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
        //todo!("list::find_free_block() is not implemented yet.")
        let mut current = &mut self.head;
        
        while let Some(ref mut block) = current.next {
            if let Ok(alloc_start) = Self::check_block_for_alloc(block, size, align) {
                let next = block.next.take();
                let free_block = current.next.take().unwrap();
                
                current.next = next;

                return Some((free_block, alloc_start));
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        
        None
    }

    /// Check if the given block is large enough for an allocation with `size` and `align`.
    fn check_block_for_alloc(block: &ListNode, size: usize, align: usize) -> Result<usize,()> {
        //todo!("list::check_block_for_alloc() is not implemented yet.")
        let alloc_start = align_up(block.start_addr(), align);
        let alloc_end = alloc_start.saturating_add(size);

        if alloc_end <= block.end_addr() {
            return Ok(alloc_start)
        }
        
        Err(())
    }

    /// Adjust the given layout so that the resulting allocated memory
    /// block is also capable of storing a `ListNode`.
    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(align_of::<ListNode>())
            .expect("adjusting alignment failed")
            .pad_to_align();
        let size = layout.size().max(size_of::<ListNode>());

        (size, layout.align())
    }

    /// Dump the free list for debugging purposes.
    pub fn dump_free_list(&mut self) {
        //todo!("list::dump_free_list() is not implemented yet.")
        println!("Heap allocator:");
        println!("\tHeap_start: {:#x}, Heap end: {:#x}", self.heap_start, self.heap_end);
        println!("\tFree block(s):");

        let mut current = self.head.next.as_ref();
        
        while let Some(node) = current {
            println!("\t\tBlock at {:#x} with size {}", node.start_addr(), node.size);
            current = node.next.as_ref();
        }
    }

    /// Allocate memory of the given size and alignment.
    pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        //todo!("list::alloc() is not implemented yet.")
        let (size, align) = Self::size_align(layout);
        let block = self.find_free_block(size, align);

        if block.is_none() {
            return null_mut()
        }
        
        let (free_block, alloc_start) = block.unwrap();

        let alloc_end = alloc_start.saturating_add(size);
        let excess_size = free_block.end_addr() - alloc_end;

        if excess_size > 0 {
            unsafe {
                self.add_free_block(alloc_end, excess_size);
            }
        }

        alloc_start as *mut u8
    }

    /// Free the memory block at the given pointer with the given layout.
    pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let (size, _) = LinkedListAllocator::size_align(layout);

        unsafe {
            self.add_free_block(ptr as usize, size)
        }
    }
}

// Trait required by the Rust runtime for heap allocations
unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
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