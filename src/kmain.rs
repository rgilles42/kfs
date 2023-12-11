#![no_std]
#![no_main]

mod vga;
mod multiboot;
mod arch;

use core::panic::PanicInfo;
use crate::multiboot::parse_mboot_info;
use crate::arch::x86::gdt;

extern "C" {
	fn get_sp() -> usize;
	fn get_bp() -> usize;
}

macro_rules! sp {
	() => (unsafe {get_sp()});
}

macro_rules! bp {
	() => (unsafe {get_bp()});
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
	printk!("{}", _panic);
	loop {}
}



fn print_stack(mut sp: usize, bp: usize) {
	while sp < bp {
		let stack_word: &[u32; 4] = unsafe {&*(sp as *const [u32; 4])};
		printk!("\x1b\x04\x000x{:08x}:\x1b\x0f\x00        0x{:08x}    0x{:08x}    0x{:08x}    0x{:08x}", sp, stack_word[0], stack_word[1], stack_word[2], stack_word[3]);
		sp += 4;
	}
}

fn demo_stackframe() {
	printk!("/***************/\n/* Stack frame */\n/***************/");
	printk!("Current values of sp: 0x{:08x} and bp: 0x{:08x}, size is of 0x{:08x}", sp!(), bp!(), bp!() - sp!());
	//print_stack(sp!(), bp!());
}

#[no_mangle]
pub extern "C" fn kmain(magic: u32, mboot: *const u32) -> ! {
	vga::setup_io();
	gdt::load();
	parse_mboot_info(mboot);
	printk!("Mutilboot: magic({:x}) mboot({:p})", magic, mboot);
	printk!("/*******************/\n/* VGA Output Demo */\n/*******************/");
	printk!("According to \x1b\x06\x00all laws \x1b\x01\x00of aviation, \x1b\x04\x00there is \x1b\x0d\x00no way \x1b\x0c\x00that a \x1b\x00\x0fbee should \x1b\x04\x0ebe able \x1b\x07\x03to fly\x1b\x0f\x00");
	demo_stackframe();
	loop {}
}

