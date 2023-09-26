#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
	loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
	println!("Hello {}!", "World");
	loop {}
}

