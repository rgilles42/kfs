use core::borrow::BorrowMut;
use super::{KernelAllocator, AllocError};
use crate::memory::{pmm, PAGE_SIZE};
use crate::memory::vmm::mapper;
use crate::ROUND_PAGE_UP;
use alloc::alloc::{Layout, GlobalAlloc};
use core::mem::size_of;
use crate::is_aligned;
use super::Lock;
use core::fmt;
use crate::dbg;

pub struct ListAllocator
{
    head: BlockInfo,
    // Virtual address of the heap's start
    memstart: usize,
    /// Current size of heap
    heapsize: usize,
    /// Max size of heap
    heapmax: usize,
}

impl KernelAllocator for ListAllocator
{
    /// Init list allocator parameters
    fn init(&mut self, memstart: usize, heapmax: usize) {
        self.memstart = memstart; 
        self.heapmax = heapmax;
    }
}

/// Metadata about free block
/// Attached to every free block
#[repr(C)]
struct BlockInfo {
    size: usize,
    next: Option<&'static mut BlockInfo>
}

impl fmt::Debug for BlockInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("PhysicalRegion")
            .field("size", &format_args!("{}KB ({}B)", self.size/1024, self.size))
            .field("next", &format_args!("{:p}", self as *const BlockInfo))
            .finish()
    }
}
 
macro_rules! binfo_size{
    () => {
       core::mem::size_of::<BlockInfo>()
    };
}

impl BlockInfo {
    #[inline(always)]
    pub fn data(&self) -> *mut u8
    {
        (self as *const BlockInfo as usize + size_of::<Self>()) as *mut u8
    }
    #[inline(always)]
    fn addr(&self) -> usize {
        self as *const BlockInfo as usize
    }
}

impl ListAllocator
{
    pub fn default() -> Self {
        ListAllocator { 
            head: BlockInfo {size: 0, next: None},
            memstart: 0,
            heapmax: 0,
            heapsize: 0,
        }
    }

    // Release block and insert it in free list, coalesce with neighbours, unmap if last 
    fn free_block(&mut self, block: &'static mut BlockInfo)
    {
        // we will loop throught the nodes to find the neigbhours
        let head_address = self.head.addr();
        let mut current = &mut self.head;
        // TODO temp fix until I get the courage to clean up all the code from this allocator
        if current.next.is_none() {
            current.next = Some(block);
            return;
        }
        while !current.next.is_none() {
            let ca = current.addr();
            // TODO confused about the ownership situation here
            let next = current.next.as_mut().unwrap();
            // let next = current.next.unwrap();
            // this tells us if the block to free is between current and current.next
            if next.addr() > block.addr() {
                if ca == head_address {
                    if block.addr() + block.size == next.addr() {
                        block.size += next.size;
                        block.next = next.next.take();
                        current.next = Some(block);
                    }
                    else {
                        block.next = current.next.take();
                        // Goto first block
                        current.next = Some(block);
                        current = current.next.as_mut().unwrap();
                    }

                }
                else if ca + current.size == block.addr() {
                    current.size += block.size;
                    // coalesce with next block if touching
                    if ca + current.size == next.addr() {
                        current.size += next.size;
                        current.next = next.next.take(); 
                    }
                }
                else {
                    let tmp = current.next.take();
                    block.next = tmp;
                    current.next = Some(block);
                }
                // // If the last free block is at the heap's end, unmap pages if we can
                // if current.next.is_none() && current.addr() + current.size == self.memstart + self.heapsize {
                //     let start = if is_aligned!(current.addr(), PAGE_SIZE) { current.addr() } else {ROUND_PAGE_UP!(current.addr())};
                //     let npages = current.size / PAGE_SIZE;
                //     let _ = mapper::unmap_range_kernel(start, npages);
                //     current.size -= npages * PAGE_SIZE;
                // }
                return;
            }
            current = current.next.as_mut().unwrap(); // TODO don't really understand this line
        }
        panic!("Invalid free pointer {:x}", block.addr());
    }

    /// Tries to find an existing free block of sufficient enough size
    /// This function will expand the heap if it doesn't find one
    fn alloc_block(&mut self, new_size: usize) -> Result<*const BlockInfo, AllocError>
    {
        let mut current = self.head.borrow_mut();
        // Loop through existing free blocks to find match
        while let Some(ref mut b) = current.next { // TODO how is b type legal
            // If exact match
            let fbsize = b.size;
            if fbsize == new_size {
                // Removing node from list
                let next = b.next.take();
                let ret = current.next.take().unwrap();
                current.next = next;
                // b.next = None;
                return Ok(ret as *const BlockInfo);
            }
            // if the requested size is smaller than available, and the difference can contain a new blockinfo
            else if fbsize > new_size
                && is_aligned!(b.size - new_size, core::mem::align_of::<BlockInfo>()) {// TODO is this alignment check rational ?
                unsafe {
                    let left = BlockInfo {size: b.size - new_size, next: b.next.take()};
                    // TODO this line makes me sad
                    let left_ptr = (*b as *mut BlockInfo as *mut u8).offset(new_size as isize);
                    // TODO might crash on large blocks
                    (left_ptr as *mut BlockInfo).write(left);
                    // Downsizing newly allocated block size
                    b.size = new_size;
                    // Getting its address
                    let ret = *b as *const BlockInfo;
                    // setting the current next to the leftovers node
                    current.next = (left_ptr as *mut BlockInfo).as_mut();
                    return Ok(ret);
                }
            }
            current = current.next.as_mut().unwrap();
        }

        // If we arrive here, we have to map more pages for the new block
        // we need to align in case the leftover allocation needs another block
        let total = ROUND_PAGE_UP!(new_size + core::mem::align_of::<BlockInfo>());
        let start = self.memstart + self.heapsize;
        let range = pmm::alloc_contiguous_pages(total/PAGE_SIZE).ok_or(AllocError::ENOMEM)?;
        
        if mapper::map_range_kernel(range, start).is_err() {
            return Err(AllocError::EMAPPING);
        }

        self.heapsize += total;

        let allocated_ptr = start as *const BlockInfo as *mut u8;
        unsafe {
            // leftovers from allocating the pages
            // we need to allocate a new free block at the end of our list
            if total - new_size > 0 {
                let left = &mut *((allocated_ptr.offset(new_size.try_into().unwrap())) as *mut BlockInfo);
                *left = BlockInfo {
                    next : None,
                    size : total - new_size
                };
                // point last unused block next to leftovers
                current.next = Some(left);
            }
            let nb = &mut*(allocated_ptr as *mut BlockInfo);
            nb.size = new_size;
            // The newly allocated block isn't in the free list, no neighbours
            nb.next = None;
        }
        Ok(allocated_ptr as *const BlockInfo)
    }

    /// Decrease current heap size by a number of pages
    #[allow(dead_code)]
    fn decrease_heap(&mut self, npages: usize)
    {
        mapper::unmap_range_kernel(self.memstart + self.heapsize - npages * PAGE_SIZE, npages).expect("Failed to decrease kernel heap");
        self.heapsize -= npages * PAGE_SIZE;
    }

    fn adjust_layout(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(core::mem::align_of::<BlockInfo>())
            .expect("adjusting alignment failed")
            .pad_to_align();
        let size = layout.size().max(core::mem::size_of::<BlockInfo>());
        (size, layout.align())
    }

    pub fn print_list(&self)
    {
        dbg!("Block list, heapsize({})", self.heapsize);
        dbg!("head {:^10p}", &self.head as *const BlockInfo);
        let mut current = &self.head.next;
        let mut i = 0;
        loop {
            match current {
                Some(b) => {
                    dbg!("B{:^4} {:^4} {:p} to 0x{:x}", i, b.size, *b as *const BlockInfo, *b as *const BlockInfo as usize + b.size);
                    current = &b.next; 
                }
                None => break
            }
            i += 1;
        }
        dbg!("");
    }

}


unsafe impl GlobalAlloc for Lock<ListAllocator>
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8
    {
        let alloc = self.get();
        dbg!("*** alloc : {}", layout.size());
        let (size, _align) = ListAllocator::adjust_layout(layout);
        match alloc.alloc_block(size + size_of::<BlockInfo>()) { // TODO better alignment
            // management
            Ok(b) => {
                let address = b as *mut u8 as usize;
                let ptr = (address + binfo_size!()) as *mut u8;
                alloc.print_list();
                return ptr
            }
            Err(_e) => return core::ptr::null_mut()
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
    {
        let alloc = self.get();
        dbg!("*** dealloc : {}", layout.size());
        // TODO check aligntment and use layout
        // TODO Add a mechanism to check if the pointer is valid ?
        let block_address = ptr as usize - binfo_size!();
        let block: &mut BlockInfo =  &mut*(block_address as *mut BlockInfo);
        alloc.free_block(block);
        alloc.print_list();
    }
}


impl fmt::Debug for ListAllocator {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("ListAlloc")
            .field("memstart", &format_args!("0x{:x}", self.memstart))
            .field("heapsize", &format_args!("{}KB ({}B)", self.heapsize/1024, self.heapsize))
            .field("heapmax", &format_args!("0x{:x}",self.heapmax))
            .finish()
    }
}
