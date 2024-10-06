mod listalloc;
pub mod mapper;

use listalloc::ListAllocator;


enum AllocError {
    ENOMEM,
    EMAPPING,
}


use core::cell::UnsafeCell;

struct Lock<T> {
   obj: Option<UnsafeCell<T>>
}

impl<T> Lock<T> { // TODO actual locking

    fn new(obj: T) -> Self {
        Self {obj: Some(UnsafeCell::new(obj))}
    }
     

    fn get(&self) -> &mut T 
    {
        unsafe {&mut *self.obj.as_ref().unwrap().get()}
    }

    const fn default() -> Self {
        Self {obj: None}
    }
}

// TODO actually implement thread safety
unsafe impl<T> Sync for Lock<T> {}

// TODO remove the mut ?
#[global_allocator]
static mut ALLOCATOR : Lock<ListAllocator> = Lock::default(); 
pub trait KernelAllocator 
{
    /// Initialize the allocator on a virtual region starting at @memstart, with @size, and a
    /// @base_pages number to allocate to start with
    fn init(&mut self, memstart: usize, heapsize: usize);
    fn get_alloc_size(&mut self, ptr: *const u8) -> usize;
}

pub fn get_alloc_size(ptr: *const u8) -> usize
{
    unsafe {ALLOCATOR.get().get_alloc_size(ptr)}
}

#[inline(always)]
pub fn init(memstart: usize, size: usize)
{
    unsafe { 
        ALLOCATOR = Lock {obj: Some(UnsafeCell::new(ListAllocator::default()))};
        ALLOCATOR.get().init(memstart, size);
    };
}
