#![no_main]
#![no_std]

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn testfunc() -> u32 {
    0x12345678
}


#[no_mangle]
#[allow(unused_results)] // TODO remove and handle correctly
fn kmain() -> ! {
    unsafe {
        asm!("cli");
    }

    let vga_buffer = 0xb8000 as *mut u8;
    unsafe {
        *vga_buffer = 'H' as u8;
        *vga_buffer.offset(1) = 0xb;
    }

    loop {}
}

