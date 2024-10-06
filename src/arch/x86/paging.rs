use crate::{printk, print};
use core::ops::DerefMut;
use core::arch::asm;
// use bitflags::bitflags;
use crate::utils::rawbox::RawBox;
use crate::memory::pmm;
use crate::memory::pmm::{Frame, FrameRange};
use crate::arch::{KERNEL_PAGE_TABLES_START, PAGE_SIZE};
use crate::memory::vmm::mapper;

pub static mut MAPPER : RawBox<PageDir> = RawBox {data: 0 as *mut PageDir};

#[macro_export]
macro_rules!  ROUND_PAGE_UP{
    ($a:expr) => {
           ($a + crate::arch::PAGE_SIZE) & !(0xfff as usize)
    };
}


#[macro_export]
macro_rules!  ROUND_PAGE_DOWN{
    ($a:expr) => {
           ROUND_PAGE_UP!($a) - PAGE_SIZE
    };
}

extern "C" {
    static EPD_PHYS: PageDir;
    static EARLY_PAGE_DIRECTORY: PageDir;
}

#[allow(dead_code)]
mod pdef {
    pub const PRESENT: usize = 1;
    pub const WRITE: usize = 1 << 1;
    pub const USER: usize = 1 << 2;
    pub const WRITETHROUGH: usize = 1 << 3;
    pub const CACHEDISABLE: usize = 1 << 4;
    pub const ACCESSED: usize = 1 << 5;
    pub const AVAILABLE: usize = 1 << 6;
    pub const PAGESIZE: usize = 1 << 7;
}

#[allow(dead_code)]
mod ptef {
    pub const PRESENT: usize = 1;
    pub const WRITE: usize = 1 << 1;
    pub const USER: usize = 1 << 2;
    pub const WRITETHROUGH: usize = 1 << 3;
    pub const CACHEDISABLE: usize = 1 << 4;
    pub const ACCESSED: usize = 1 << 5;
    pub const DIRTY: usize = 1 << 6;
    pub const PAT: usize = 1 << 7;
    pub const GLOBAL: usize = 1 << 8;
}


type PDE = usize;
type PTE = usize;

#[repr(C)]
pub struct PageDir
{
        entries: [PDE; 1024]
}

impl PageDir {
    #[inline(always)]
    pub fn set_entry(&mut self, i: usize, address: usize, flags: usize)
    {
        self.entries[i] = (address & !0xfff) | flags; // TODO convert to flags 
    }

}

impl Default for PageDir
{
    fn default() -> Self
    {
        PageDir {
            entries: [0; 1024]
        }
    }
}

fn flush_tlb()
{
    unsafe {
        asm!("push eax",
        "mov eax, cr3",
        "mov cr3, eax",
        "pop eax");
    }
}

#[repr(C, packed)]
struct PageTable
{
        pub entries: [PTE; 1024]
}

#[inline(always)]
pub fn kernel_mapper() -> &'static mut PageDir
{
    unsafe {MAPPER.deref_mut()}
}

macro_rules! pde_index {
    ($a: expr) => {
       $a >> 22
    };
}

macro_rules! pte_index {
    ($a: expr) => {
       $a >> 12 & (2 << 10 - 1)
    };
}

macro_rules! offset {
    ($a: expr) => {
       $a & (2 << 12 - 1)
    };
}

macro_rules! is_page_aligned {
    ($a: expr) => {
       ($a & (PAGE_SIZE - 1) == 0)
    };
}

// TODO the most abhorrent code i've written in my life so far
fn get_kernel_pt(index: usize) -> *mut PageTable
{
    let ptr: *mut PageTable;
    ptr = (KERNEL_PAGE_TABLES_START + index * core::mem::size_of::<PageTable>()) as *mut PageTable;
    ptr
}

impl mapper::MapperInterface for PageDir
{

    fn map_single(&mut self, f: Frame, address: usize) -> Result<(), ()> {
        if !is_page_aligned!(address) { return Err(()) }
        let phys_address = f.0 * PAGE_SIZE;
        let index = pde_index!(address);
        let pt = unsafe {&mut(*get_kernel_pt(index))};
        if self.entries[index] == 0 {
            self.entries[index] = self.virt_to_phys(pt as *const PageTable as usize).unwrap() | pdef::PRESENT | pdef::WRITE;
        }
        pt.entries[pte_index!(address)] = phys_address | ptef::PRESENT | ptef::WRITE ;
        flush_tlb();
        Ok(())
    }

    fn unmap_single(&mut self, address: usize) -> Result<(), ()> {
        let address = self.virt_to_phys(address).expect("Trying to unmap unmapped page");
        pmm::free_page(Frame(address/PAGE_SIZE));

        let pde_index = address >> 22;
        let pte_offset = ((address >> 12) & 0x3ff) * core::mem::size_of::<PDE>();
        let special = (0x3ff << 22) | pde_index << 12 | pte_offset;
        unsafe { 
            *(special as *mut usize) = 0;
        }
        flush_tlb();
        Ok(())
    }

    fn unmap_range(&mut self, address: usize, npages: usize) -> Result<(), ()> {
        let mut ptr = address;
        for _i in 0..npages {
            let _ = self.unmap_single(ptr);
            ptr += PAGE_SIZE;
        }
        Ok(())
    }

    fn map_range(&mut self, r: FrameRange, address: usize) -> Result<(), ()> {
        if !is_page_aligned!(address) { return Err(()) }
        // TODO assert aligned to page 
        // TODO more generic api
        for i in 0..r.size {
            self.map_single(Frame(r.start.0 + i), address + i * PAGE_SIZE).unwrap();
        }
        Ok(())
    }

    /// Will use the last entry of the page directory for recursive mapping
    fn virt_to_phys(&mut self, address: usize) -> Option<usize>
    {
        let pde_index = address >> 22;
        if self.entries[pde_index] == 0 {
            return None;
        }
        let offset = address & 0xfff;
        let pte_offset = ((address >> 12) & 0x3ff) * core::mem::size_of::<u32>(); // TODO refactor types

        let special = (0x3ff << 22) | pde_index << 12 | pte_offset;
        let pte : usize;
        unsafe { 
            pte = *(special as *const usize) & !0xfff;
        }
        if pte == 0 { return None }
        Some(pte + offset)
    }

}

// TODO might put this in the assembly
static mut KERNEL_PT_TEMP : [usize; 1024] = [0; 1024];

pub fn init_post_jump()
{
        unsafe { 
            // We will use the static early page dir at first TODO should we change it ?
            // TODO oh my god do a macro for getting symbols's address I
            // shot myself in the foot multiple times already it hurts so bad
            MAPPER = RawBox::from_ptr(&EARLY_PAGE_DIRECTORY);
            let dir: &mut PageDir = MAPPER.deref_mut();

            // setting the recursive mapping entry at the last entry of the table
            // We lose 4MB of virtual space, but we gain happiness
            // TODO this is very naky, EPD_PHYS is the load address
            dir.set_entry(0x3ff, &EPD_PHYS as *const PageDir as usize, 3);


            // remove identity mapping
            dir.set_entry(0, 0, 0);

            // Allocating 4MB in high memory to store the kernel page tables
            // We are making sure that the virtual address is well aligned and within the last
            // index of the directory
            assert!(pde_index!(KERNEL_PAGE_TABLES_START) == 0x3fe);
            assert!(is_page_aligned!(KERNEL_PAGE_TABLES_START));
            let address = mapper::virt_to_phys_kernel(&KERNEL_PT_TEMP as *const usize as usize).expect("Cannot map KERNEL_PT_TMP");
            dir.set_entry(0x3fe, address, 3);
            
            // flush the tlb so we can map in the new table
            flush_tlb();

            // allocating 4MB of pages to store kernel pages TODO might be a bit overkill and
            // unoptimized
            let range = pmm::alloc_contiguous_pages(1024).expect("Cannot allocate page tables memory space");

            // Manually map the range, as mapper::map_range_kernel requires the kernel allocator to
            // be initialized, which itself needs paging (what we are doing right now you dingus)
            let start = range.start.0;
            for i in 0..range.size { 
                KERNEL_PT_TEMP[i] = start + i * PAGE_SIZE | ptef::PRESENT | ptef::WRITE; // TODO better flags
            }
            // flush the tlb one last time so that the new table is updated
            flush_tlb();
        }
}

// TODO is this code archiecture specific ?
pub fn page_fault_handler(instruction_pointer: u32, code: u32)
{

    let address : u32;
    printk!("PAGE FAULT EXCEPTION");
    unsafe {asm!("mov {0}, cr2", out(reg) address);}
    printk!("Virtual address : {:p}", (address as *const u32));
    print!("Error code: "); // TODO reformat in the future
    // let flags = PF::from_bits(code).unwrap();
    // print!("{} ", if flags.contains(PF::P) {"PAGE_PROTECTION"} else {"PAGE_NOT_PRESENT"});
    // print!("{} ", if flags.contains(PF::W) {"WRITE"} else {"READ"});
    // if flags.contains(PF::U) { print!("CPL_3 ") };
    // if flags.contains(PF::R) { print!("RESERVED_WRITE_BITS ") };
    // if flags.contains(PF::I) { print!("INSTRUCTION_FETCH ") };
    // if flags.contains(PF::PK) { print!("KEY_PROTECTION ") };
    // if flags.contains(PF::SS) { print!("SHADOW STACK ") };
    // if flags.contains(PF::SGX) { print!("SGX_VIOLATION ") };
    print!("\n");
    loop{}
}

