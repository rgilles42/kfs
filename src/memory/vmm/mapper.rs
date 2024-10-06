use crate::memory::pmm::{Frame, FrameRange};
use crate::arch::paging::kernel_mapper;


// TODO error enum ?

#[allow(dead_code)] // TODO
pub trait MapperInterface
{
    /// Map a single frame
    fn map_single(&mut self, f: Frame, address: usize) -> Result<(), ()>;
    /// Unmap a single frame
    fn unmap_single(&mut self, address: usize) -> Result<(), ()>;
    /// Contiguous mapping of the FrameRange from the address and up
    fn map_range(&mut self, f: FrameRange, address: usize) -> Result<(), ()>;
    /// Unmap contiguous frame
    fn unmap_range(&mut self, address: usize, npages: usize) -> Result<(), ()>;
    /// Return the physical address of a virtual and mapped address
    fn virt_to_phys(&mut self, address: usize) -> Option<usize>;
}

/// Map a single frame
#[inline(always)]
pub fn map_single_kernel(f: Frame, address: usize) -> Result<(), ()> {
    kernel_mapper().map_single(f, address)
}

/// Contiguous mapping of the FrameRange from the address and up
#[inline(always)]
pub fn map_range_kernel(r: FrameRange, address: usize) -> Result<(), ()> {
    kernel_mapper().map_range(r, address)
}

/// Return the physical address of a virtual and mapped address
#[inline(always)]
pub fn virt_to_phys_kernel(address: usize) -> Option<usize> {
    kernel_mapper().virt_to_phys(address)
}

/// Unmap a single frame
#[inline(always)]
#[allow(dead_code)]
pub fn unmap_single_kernel(address: usize) -> Result<(), ()> {
    kernel_mapper().unmap_single(address)
}

/// Unmap a virtual frame range
#[inline(always)]
#[allow(dead_code)]
pub fn unmap_range_kernel(address: usize, n: usize) -> Result<(), ()> {
    kernel_mapper().unmap_range(address, n)
}
