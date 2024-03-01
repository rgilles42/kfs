mod bitmap;

use bitmap::BitMap;
use core::fmt;
use crate::memory::{PhysicalMemory, RegionType, PAGE_SIZE};
use crate::printk;

/// Abstract representation of Frame
/// It does not represent an address because the PMM shouldn't be aware of the current
/// architecture, beyond that it is a page based system
pub struct Frame(pub usize);

impl fmt::Display for Frame
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.0)
    }
}

pub struct FrameRange
{
    pub start: Frame,
    pub size: usize
}

/// The PMM instance
static mut PMM: BitMap = BitMap::default_const();

// TODO proper non contiguous area management

/// Physical Memory Manager trait
/// Every physical memory manager should implement this trait
/// We expose a safe api below directly under the crate::pmm namespace
/// That way we don't have to make the PMM instance public, it is cleaner
pub trait PageManager {
    fn alloc_page(&mut self) -> Option<Frame>;
    fn alloc_contiguous_pages(&mut self, n: usize) -> Option<FrameRange>;
    fn free_page(&mut self, f: Frame);
    fn free_contiguous_pages(&mut self, r: FrameRange);
    fn fill_range(&mut self, r: FrameRange) -> ();
}

/// Initialize
#[inline(always)]
pub fn init(memmap: &PhysicalMemory)
{   
    for entry in memmap.regions {
        if entry.rtype == RegionType::Available {
            free_contiguous_pages(FrameRange {
                start: Frame(entry.start / PAGE_SIZE),
                size: entry.size / PAGE_SIZE
            })
        }
    }
    unsafe {printk!("{}", PMM)};
}

/// Allocate a single physical page
#[inline(always)]
pub fn alloc_page() -> Option<Frame>
{   
    unsafe {PMM.alloc_page()}
}

/// Allocate n contiguous pages
#[inline(always)]
pub fn alloc_contiguous_pages(n: usize) -> Option<FrameRange>
{
    unsafe {PMM.alloc_contiguous_pages(n)}
}

#[inline(always)]
pub fn free_page(f: Frame)
{
    unsafe {PMM.free_page(f)}
}

#[inline(always)]
pub fn free_contiguous_pages(f: FrameRange)
{
    unsafe {PMM.free_contiguous_pages(f)}
}

#[inline(always)]
pub fn fill_range(f: FrameRange) -> ()
{
    unsafe {PMM.fill_range(f)}
}
