#![no_std]
#![no_main]

mod vga;
mod multiboot;

use core::panic::PanicInfo;
use crate::multiboot::{parse_mboot_info};


#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
	println!("{}", _panic);
	loop {}
}

#[no_mangle]
pub extern "C" fn kmain(magic: u32, mboot: *const u32) -> ! {
	vga::setup_io();
	println!("Mutilboot: magic({:x}) mboot({:p})", magic, mboot);
	parse_mboot_info(mboot);
	println!("Voir \x1b\x06\x00la \x1b\x01\x00vie \x1b\x04\x00en \x1b\x0d\x00couleur, \x1b\x0c\x00c'est \x1b\x00\x0fle \x1b\x04\x0esecret \x1b\x07\x03du \x1b\x03\x07bonheur");
	print!("\x1b\x0e\x0042...");
	loop {}
}

