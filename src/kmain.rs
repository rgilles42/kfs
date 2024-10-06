#![no_std]
#![no_main]

mod vga;
mod multiboot;
mod arch;
mod memory;
mod utils;
mod serial;

use core::panic::PanicInfo;
use crate::multiboot::parse_mboot_info;
use crate::arch::gdt;
use crate::arch::paging;

use memory::pmm;
use memory::pmm::{FrameRange, Frame};
use memory::vmm;
use alloc::vec;
extern crate alloc;

#[panic_handler]
fn panic(pinfo: &PanicInfo) -> ! {
        unsafe {core::arch::asm!("cli")};
        dbg!("KERNEL PANIC");
	printk!("KERNEL PANIC");
	printk!("panic occured at {}", pinfo.location().unwrap());
	dbg!("panic occured at {}", pinfo.location().unwrap());
	loop {}
}

extern "C" {
    static kernel_image_start : u32;
    static kernel_image_end : u32;
}

#[no_mangle]
pub extern "C" fn kmain(magic: u32, mboot: *const u32) -> ! {
	vga::setup_io();
	gdt::load();
	dbg!("Mutilboot: magic({:x}) mboot({:p})", magic, mboot);
	dbg!("/*******************/\n/* VGA Output Demo */\n/*******************/");
	printk!("According to \x1b\x06\x00all laws \x1b\x01\x00of aviation, \x1b\x04\x00there is \x1b\x0d\x00no way \x1b\x0c\x00that a \x1b\x00\x0fbee should \x1b\x04\x0ebe able \x1b\x07\x03to fly\x1b\x0f\x00");
	// demo_stackframe();
    printk!("VGA initialized");


    let kstart: usize;
    let kend: usize;
    unsafe {
        dbg!("Kernel start {:p}", &kernel_image_start);
        dbg!("Kernel end {:p}", &kernel_image_end);
        kstart = &kernel_image_start as *const u32 as usize;
        kend = &kernel_image_end as *const u32 as usize;
    }
    // TODO should I calulate this before jumping to kstart, as it might require identity mapping more pages
    // at the start ?
    let ksize = (kend - kstart)/1024; 
    dbg!("Kernel size : {}KB", ksize);
    dbg!("Multiboot: magic({:x}) mboot({:p})", magic, mboot);

    // setting the first 4MB of PMM bitmap TODO api seems dirty


    // Figuring out the physical memory layout
    // Here we assume the kernel is booted using multiboot
    use multiboot::MbootError;
    match multiboot::parse_mboot_info(mboot)
    {
            Err(MbootError::InvalidFlags) => {panic!("Multiboot flags malformed")},
            Err(MbootError::NoMemoryMap) => {panic!("No memory map")}, // TODO BIOS functions ?
            Ok(()) => (),
    }
    dbg!("Physical Memory regions:");
    for entry in memory::phys_mem().regions  {
        dbg!("- {entry:?}");
    }

    // This will filter out unusable pages
    dbg!("Start init pmm");
    pmm::init(memory::phys_mem());
    // Blocking out the first 4MB as they will always be mapped
    pmm::fill_range(FrameRange{start: Frame(0), size: (kend - arch::KERNEL_OFFSET) / arch::PAGE_SIZE});
    dbg!("End init pmm");

    dbg!("Setup paging post jump");
    paging::init_post_jump();

    dbg!("Setting up the memory manager");
    // Sets up the virtual memory manager
    let memstart = ROUND_PAGE_UP!(kend);
    vmm::init(memstart, arch::KERNEL_PAGE_TABLES_START - kend);

    {
        dbg!("========================== MEMORY TESTING"); 
        let a = alloc::string::String::from("Moi je suis en pleine forme");
        let _vec_test = vec![1;100];
        {
            let _vec_test2 = vec![1;100];
            let b = alloc::string::String::from("Bonjour tout le monde");
            printk!("{}", b);
        }
        {
                let _bebou = alloc::boxed::Box::new([1 as u8;400]);
        }
        let _bebou = alloc::boxed::Box::new([1 as u8;400]);
        printk!("{}", a);
        for i in 0..10 {
            printk!("{:x}", _vec_test[i]);
        }
        let _bebou2 = alloc::boxed::Box::new([1 as u8;400]);
        {
            let _vec_test3 = vec![1;100000];
        }
    }

    printk!("=========================== PANIC TEST"); 
    dbg!("=========================== PANIC TEST"); 
    panic!("Oh no we are panicking !");
    loop {}
}
