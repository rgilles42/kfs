pub mod pmm;
pub mod vmm;

use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RegionType
{
    Unknown = 0,
    Available = 1,
    Reserved = 2,
    ACPI = 3,
    NVS = 4,
    BadRAM = 5,
}

#[derive(Clone, Copy)]
pub struct PhysicalRegion
{
        pub start: usize,
        pub size: usize,
        pub rtype: RegionType,
}

impl PhysicalRegion
{
    pub fn new(start: usize, size: usize, rtype: usize) -> Self
    {
        PhysicalRegion{
            start,
            size,
            rtype: match rtype {
                1 => RegionType::Available,
                2 => RegionType::Reserved,
                3 => RegionType::ACPI,
                4 => RegionType::NVS,
                5 => RegionType::BadRAM,
                _ => RegionType::Unknown
            }
        }
    }
}

impl fmt::Debug for PhysicalRegion
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("PhysicalRegion")
            .field("start", &format_args!("{:p}({})", self.start as *const usize, self.start))
            .field("size", &format_args!("{}KB", self.size/1024))
            .field("type", &self.rtype)
            .finish()
    }
}

const MAX_PHYS_REGIONS: usize = 10;
/// Holds list of physical memory regions
pub struct PhysicalMemory {
    pub regions: [PhysicalRegion; MAX_PHYS_REGIONS],
    /// Number of entries
    pub size: usize
}

impl PhysicalMemory {
    pub fn add_entry(&mut self, region: PhysicalRegion) {
        assert!(self.size < MAX_PHYS_REGIONS, "Too many physical regions");
        self.regions[self.size] = region;
        self.size += 1;
    }
}

// Constants
pub const PAGE_SIZE : usize = crate::arch::PAGE_SIZE;

/// Static structure to hold information about memroy regions
static mut PHYS_MEM : PhysicalMemory = PhysicalMemory{
    regions:[PhysicalRegion {start : 0, size: 0, rtype: RegionType::Unknown}; MAX_PHYS_REGIONS],
    size: 0
};

#[inline(always)]
pub fn phys_mem() -> &'static mut PhysicalMemory
{
    unsafe { &mut PHYS_MEM }
}

