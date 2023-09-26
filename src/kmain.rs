#![no_main]
#![no_std]

use core::panic::PanicInfo;
use core::arch::asm;
use core::ffi::c_void;

static VGA_PTR : u32 = 0xb8000;

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn strlen(s : *const u8) -> usize {
    let mut len = 0;
    let mut ptr = s;
    unsafe {
        while *ptr != 0 {
            len += 1;
            ptr = ptr.offset(1);
        }
    }
    len
}

#[no_mangle]
fn memcpy(dst: *const c_void, src: *const c_void, n : usize) -> *mut u8 {
    let dst = dst as *mut u8;
    let src = src as *mut u8; 
    for i in 0..n {
        unsafe {
            *dst.offset(i as isize) = *src.offset(i as isize);
        }
    }
    dst
}

#[no_mangle]
fn memset(dst: *const c_void, val:  u8, n : usize) -> *mut u8 {
    let dst = dst as *mut u8;
    for i in 0..n {
        unsafe {
            *dst.offset(i as isize) = val;
        }
    }
    dst
}

#[no_mangle]
fn memcmp(s1: *const c_void, s2: *const c_void, n : isize) -> i32 {
    let s1 = s1 as *mut i8;
    let s2 = s2 as *mut i8;
    for i in 0..n {
        unsafe {
            let v1 : i32 = s1.offset(i as isize) as i32;
            let v2 : i32 = s2.offset(i as isize) as i32;
            if v1 != v2 {
                return v1 - v2;
            }
        }
    }
    0
}

fn print(s : &str)
{
    let vga_buffer = VGA_PTR as *mut u8;
    for (i, c) in s.chars().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = c as u8;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

#[no_mangle]
#[allow(unused_results)] // TODO remove and handle correctly
fn kmain() -> ! {
    unsafe {
        asm!("cli");
    }
    print("Hello, world!");
    loop {}
}

