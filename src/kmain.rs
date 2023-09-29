#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;
use crate::vga::VGA;
use crate::vga::ColourPair;

static mut GLOBAL_VGA: Option<VGA> = None;

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
	println!("22 km a pied, ca use, ca use, 22 km a pied, ca use les souliers");
	println!("23 km a pied, ca use, ca use, 23 km a pied, ca use les souliers");
	println!("24 km a pied, ca use, ca use, 24 km a pied, ca use les souliers");
	println!("25 km a pied, ca use, ca use, 25 km a pied, ca use les souliers");
	println!("26 km a pied, ca use, ca use, 26 km a pied, ca use les souliers");
	println!("27 km a pied, ca use, ca use, 27 km a pied, ca use les souliers");
	println!("28 km a pied, ca use, ca use, 28 km a pied, ca use les souliers");
	println!("29 km a pied, ca use, ca use, 29 km a pied, ca use les souliers");
	println!("30 km a pied, ca use, ca use, 30 km a pied, ca use les souliers");
	println!("bite");
	unsafe{GLOBAL_VGA.as_mut()}.unwrap().printstr("bite mais en jaune", ColourPair::new(vga::Colour::Yellow, vga::Colour::Black));
	loop {}
}

