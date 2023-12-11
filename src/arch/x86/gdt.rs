use core::usize;

use crate::printk;

#[allow(dead_code)]
mod segment_access // aligned for 4kb
{
	pub const P : u8 = 1 << 7;
	pub const DPL1 : u8 = 1 << 5;
	pub const DPL2 : u8 = 2 << 5;
	pub const DPL3 : u8 = 3 << 5;
	pub const S : u8 = 1 << 4;
	pub const E : u8 = 1 << 3;
	pub const DC : u8 = 1 << 2;
	pub const RW : u8 = 1 << 1;
	pub const A : u8 = 1 << 0;
}

#[allow(dead_code)]
mod segment_flags
{
	pub const G : u8 = 1 << 3;
	pub const DB : u8 = 1 << 2;
	pub const L : u8 = 1 << 1;
}

#[repr(C,packed)]
#[derive(Clone, Default)]
struct GdtEntry {
	limit_low: u16,
	base_low: u16,
	base_mid: u8,
	access: u8,
	flags_limit_high: u8,
	base_high: u8,
}

#[repr(C,packed)]
struct Gdtr {
	size: u16,								// size in bytes - 1
	gdt: &'static mut [GdtEntry; NENTRIES]	// offset of GDT in current address space
}

impl Gdtr {
	fn new() -> Self {						//TODO? Clear table before use
		Gdtr {
			size: 8 * NENTRIES as u16 - 1,
			gdt: unsafe {&mut *(GDT_LOCATION as *mut [GdtEntry; NENTRIES])}
		}
	}

	fn add_entry(&mut self, i: usize, entry: &Entry) {
		if i >= NENTRIES { panic!() }
		self.gdt[i] = GdtEntry {
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

const GDT_LOCATION : u32 = 0x00000800;
const GDT_ENTRIES : [Entry; 7] = [
	// Null Descriptor
	Entry { base: 0, limit: 0, access_byte: 0, flags: 0  },
	// Kernel Code
	Entry { base: 0x00000000, limit: 0xfffff,
		access_byte: segment_access::P | segment_access::S | segment_access::E | segment_access::RW,
		flags: segment_flags::DB | segment_flags::G
	},
	// Kernel Data
	Entry { base: 0x00000000, limit: 0xfffff,
		access_byte: segment_access::P | segment_access::S | segment_access::RW,
		flags: segment_flags::DB | segment_flags::G
	},
	// Kernel Stack
	Entry { base: 0x00000000, limit: 0xfffff,
		access_byte: segment_access::P | segment_access::S | segment_access::RW,
		flags: segment_flags::DB | segment_flags::G
	},
	// User Code
	Entry { base: 0x00000000, limit: 0xfffff,
		access_byte: segment_access::P | segment_access::DPL3 | segment_access::S | segment_access::E | segment_access::RW,
		flags: segment_flags::DB | segment_flags::G
	},
	// User Data
	Entry { base: 0x00000000, limit: 0xfffff,
		access_byte: segment_access::P | segment_access::DPL3 | segment_access::S | segment_access::RW,
		flags: segment_flags::DB | segment_flags::G
	},
	// User Stack
	Entry { base: 0x00000000, limit: 0xfffff,
		access_byte: segment_access::P | segment_access::DPL3 | segment_access::S | segment_access::RW,
		flags: segment_flags::DB | segment_flags::G
	},
];
const NENTRIES : usize = GDT_ENTRIES.len();

extern "C" {
	fn reload_gdt(gdtr: *const Gdtr);
}

pub fn load()
{
	printk!("/***************/\n/* Loading GDT */\n/***************/");
	
	let mut gdtr = Gdtr::new();
	for (index, entry) in GDT_ENTRIES.iter().enumerate() {
		gdtr.add_entry(index, entry);
	}
	printk!("Loading GDTR at location : 0x{:08x}", &gdtr as *const _ as u32);
	printk!("GDT at location : 0x{:08x}", gdtr.gdt.as_ptr() as *const _ as u32);
	printk!("GDT size : {}", {gdtr.size});
	printk!("Loading base segment + {} segments...", NENTRIES - 1);
	unsafe {reload_gdt(&gdtr as *const _);}
}
