#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
	println!("{}", _panic);
	loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
	vga::setup_io();
	println!("1 km a pied, ca use, ca use, 1 km a pied, ca use les souliers");
	println!("2 km a pied, ca use, ca use, 2 km a pied, ca use les souliers");
	println!("3 km a pied, ca use, ca use, 3 km a pied, ca use les souliers");
	println!("4 km a pied, ca use, ca use, 4 km a pied, ca use les souliers");
	println!("5 km a pied, ca use, ca use, 5 km a pied, ca use les souliers");
	println!("6 km a pied, ca use, ca use, 6 km a pied, ca use les souliers");
	println!("7 km a pied, ca use, ca use, 7 km a pied, ca use les souliers");
	println!("8 km a pied, ca use, ca use, 8 km a pied, ca use les souliers");
	println!("9 km a pied, ca use, ca use, 9 km a pied, ca use les souliers");
	println!("10 km a pied, ca use, ca use, 10 km a pied, ca use les souliers");
	println!("11 km a pied, ca use, ca use, 11 km a pied, ca use les souliers");
	println!("12 km a pied, ca use, ca use, 12 km a pied, ca use les souliers");
	println!("13 km a pied, ca use, ca use, 13 km a pied, ca use les souliers");
	println!("14 km a pied, ca use, ca use, 14 km a pied, ca use les souliers");
	println!("15 km a pied, ca use, ca use, 15 km a pied, ca use les souliers");
	println!("16 km a pied, ca use, ca use, 16 km a pied, ca use les souliers");
	println!("17 km a pied, ca use, ca use, 17 km a pied, ca use les souliers");
	println!("18 km a pied, ca use, ca use, 18 km a pied, ca use les souliers");
	println!("19 km a pied, ca use, ca use, 19 km a pied, ca use les souliers");
	println!("20 km a pied, ca use, ca use, 20 km a pied, ca use les souliers");
	println!("21 km a pied, ca use, ca use, 21 km a pied, ca use les souliers");
	println!("Voir \x1b\x06\x00la \x1b\x01\x00vie \x1b\x04\x00en \x1b\x0d\x00couleur, \x1b\x0c\x00c'est \x1b\x00\x0fle \x1b\x04\x0esecret \x1b\x07\x03du \x1b\x03\x07bonheur");
	print!("\x1b\x0e\x0042...");
	loop {}
}

