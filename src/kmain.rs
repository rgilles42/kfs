#![no_std]
#![no_main]

mod vga;
mod multiboot;
mod arch;

use core::panic::PanicInfo;
use crate::multiboot::parse_mboot_info;
use crate::arch::x86::gdt;


#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
	println!("{}", _panic);
	loop {}
}

#[no_mangle]
pub extern "C" fn kmain(magic: u32, mboot: *const u32) -> ! {
	vga::setup_io();
	gdt::load();
	println!("Mutilboot: magic({:x}) mboot({:p})", magic, mboot);
	parse_mboot_info(mboot);
	println!("Voir \x1b\x06\x00la \x1b\x01\x00vie \x1b\x04\x00en \x1b\x0d\x00couleur, \x1b\x0c\x00c'est \x1b\x00\x0fle \x1b\x04\x0esecret \x1b\x07\x03du \x1b\x03\x07bonheur");
	println!("\x1b\x0e\x0042...\x1b\x0f\x00");
	loop {}
}

