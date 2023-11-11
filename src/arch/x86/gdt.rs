// use crate::VGA_INSTANCE;
// use core::fmt::Write;
use crate::println;

#[allow(dead_code)]
pub mod AF // aligned for 4kb
{
    pub const A : u8 = 1 << 0;
    pub const RW : u8 = 1 << 1;
    pub const DC : u8 = 1 << 2;
    pub const E : u8 = 1 << 3;
    pub const S : u8 = 1 << 4;
    pub const DPLH : u8 = 3 << 5;
    pub const DPLM : u8 = 2 << 5;
    pub const DPLL : u8 = 1 << 5;
    pub const P : u8 = 1 << 7;
}

#[allow(dead_code)]
pub mod F
{
    pub const L : u8 = 1 << 1;
    pub const DB : u8 = 1 << 2;
    pub const G : u8 = 1 << 3;
}

#[repr(C,packed)]
pub struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    flags_limit_high: u8,
    base_high: u8,
}

impl Default for GdtEntry {
    fn default() -> Self {
        
        GdtEntry {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: 0,
            flags_limit_high: 0,
            base_high: 0,
        }
    }
}

impl Clone for GdtEntry 
{
    fn clone(&self) -> Self {
        GdtEntry {
            limit_low: self.limit_low,
            base_low: self.base_low,
            base_mid: self.base_mid,
            access: self.access,
            flags_limit_high: self.flags_limit_high,
            base_high: self.base_high,
        }
    }
}


impl Copy for GdtEntry {}

const NENTRIES : usize = 5;


static mut GDT : [GdtEntry; NENTRIES] = [
    GdtEntry { limit_low: 0, base_low: 0, base_mid: 0, access: 0, flags_limit_high: 0, base_high: 0, }; NENTRIES];

#[repr(C,packed)]
struct Gdtr {
    size : u16, // size in bytes - 1
    offset : u32, // offset of GDT (linear address, paging applies)
}

static mut GDTR : Gdtr = Gdtr {size : 0, offset : 0}; // pointer to the gdt table

pub fn format_entry(base: u32, limit : u32, access_byte: u8, flags: u8) -> GdtEntry
{
    let mut entry = GdtEntry::default();
    entry.base_low = (base & 0xffff) as u16;
    entry.base_mid = ((base >> 16) & 0xff) as u8;
    entry.base_high = ((base >> 24) & 0xff) as u8;
    entry.limit_low = (limit & 0xffff) as u16;
    entry.flags_limit_high = (((limit >> 16) & 0xf) as u8) | (flags << 4);
    entry.access = access_byte;

    // print raw struct in hex
// unsafe {
//     println!("New entry");
//     let pointer = &entry as *const _ as *const u8;
//         for i in 0..8 {
//             println!("{:08b} ", *pointer.offset(i as isize));
//         }
//     }
    entry
}

extern "C" {
    // fn load_gdt(gdtr: *const Gdtr);
    fn load_gdt(size: u16, offset: u32);
    fn reload_segments();
}

pub fn load()
{
    // setup basic segments
    unsafe {
        GDT[0] = format_entry(0, 0, 0, 0);
        // kernel code
        GDT[1] = format_entry(0, 0xffffffff,
                                AF::P | AF::S | AF::E| AF::RW,
                                F::DB | F::G);
        // kernel data
        GDT[2] = format_entry(0, 0xffffffff,
                                AF::P | AF::S | AF::RW,
                                F::DB | F::G);
        // user code TODO change permissions
        GDT[3] = format_entry(0, 0xffffffff,
                                AF::P | AF::S | AF::E | AF::RW,
                                F::DB | F::G);
        // user data  TODO change permissions
        GDT[4] = format_entry(0, 0xffffffff,
                                AF::P | AF::S | AF::RW,
                                F::DB | F::G);
        GDTR.size = 8 * NENTRIES as u16 - 1;
        GDTR.offset = &GDT as *const _ as u32;

        println!("GDT poitner : {:x}", &GDT as *const _ as u32);
        println!("GDTR poitner : {:x}", &GDTR as *const _ as u32);
        println!("GDT size : {}", { GDTR.size });
        println!("GDT offset : {:x}", { GDTR.offset });
        // load_gdt(&GDTR);
        load_gdt(GDTR.size, GDTR.offset);
        reload_segments()
    }
    // unsafe {
    //     write!(VGA_INSTANCE.as_mut().unwrap(), "GDT poitner : {:x}\n", &GDT as *const _ as u32).unwrap();
    // }
    
}

