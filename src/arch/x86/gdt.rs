use core::usize;

use crate::println;

#[allow(dead_code)]
mod segment_access // aligned for 4kb
{
	pub const A : u8 = 1 << 0;
	pub const RW : u8 = 1 << 1;
	pub const DC : u8 = 1 << 2;
	pub const E : u8 = 1 << 3;
	pub const S : u8 = 1 << 4;
	pub const DPLL : u8 = 1 << 5;
	pub const DPLM : u8 = 2 << 5;
	pub const DPLH : u8 = 3 << 5;
	pub const P : u8 = 1 << 7;
}

#[allow(dead_code)]
mod segment_flags
{
	pub const L : u8 = 1 << 1;
	pub const DB : u8 = 1 << 2;
	pub const G : u8 = 1 << 3;
}

#[repr(C,packed)]
#[derive(Clone, Copy, Default)]
struct GdtEntry {
	limit_low: u16,
	base_low: u16,
	base_mid: u8,
	access: u8,
	flags_limit_high: u8,
	base_high: u8,
}

#[repr(C,packed)]
struct Gdt {
	size: u16,									// size in bytes - 1
	table: &'static mut [GdtEntry; NENTRIES]	// offset of GDT in current address space
}

impl Gdt {
	fn new() -> Self {	//TODO? Clear table before use
		Gdt {
			size: 8 * NENTRIES as u16 - 1,
			table: unsafe {&mut *(GDT_LOCATION as *mut [GdtEntry; NENTRIES])}
		}
	}

	fn add_entry(&mut self, i: usize, entry: &Entry) {
		if i >= NENTRIES { panic!() }
		self.table[i] = GdtEntry {
			base_low: (entry.base & 0xffff) as u16,
			base_mid: ((entry.base >> 16) & 0xff) as u8,
			base_high: ((entry.base >> 24) & 0xff) as u8,
			limit_low: (entry.limit & 0xffff) as u16,
			flags_limit_high: (((entry.limit >> 16) & 0xf) as u8) | (entry.flags << 4),
			access: entry.access_byte,
		}
	}
}

struct Entry {
	base: u32, limit: u32, access_byte: u8, flags: u8
}

const GDT_ENTRIES : [Entry; 5] = [
	// Null Descriptor
	Entry { base: 0, limit: 0, access_byte: 0, flags: 0  },
	// Kernel Code
	Entry { base: 0x00000000, limit: 0xffffffff,
		access_byte: segment_access::P | segment_access::S | segment_access::E| segment_access::RW,
		flags: segment_flags::DB | segment_flags::G
	},
	// Kernel Data
	Entry { base: 0x00000000, limit: 0xffffffff,
		access_byte: segment_access::P | segment_access::S | segment_access::RW,
		flags: segment_flags::DB | segment_flags::G
	},
	// User Code TODO change perms
	Entry { base: 0x00000000, limit: 0xffffffff,
		access_byte: segment_access::P | segment_access::S | segment_access::E| segment_access::RW,
		flags: segment_flags::DB | segment_flags::G
	},
	// User Data TODO change perms
	Entry { base: 0x00000000, limit: 0xffffffff,
		access_byte: segment_access::P | segment_access::S | segment_access::RW,
		flags: segment_flags::DB | segment_flags::G
	},
];
const NENTRIES : usize = GDT_ENTRIES.len();
const GDT_LOCATION : u32 = 0x00000800;

extern "C" {
	// fn load_gdt(gdtr: *const Gdtr);
	fn load_gdt(size: u16, offset: u32);
	fn reload_segments();
}

pub fn load()
{
	let mut gdt = Gdt::new();

	for (index, entry) in GDT_ENTRIES.iter().enumerate() {
		gdt.add_entry(index, entry);
	}
	
	println!("GDTR pointer : {:x}", &gdt as *const _ as u32);
	println!("GDT size : {}", { gdt.size });
	println!("GDT pointer : {:x}", gdt.table.as_ptr() as *const _ as u32);
	unsafe {
		// load_gdt(&GDTR);
		load_gdt(gdt.size, gdt.table.as_ptr() as *const _ as u32);
		reload_segments()
	}
}
