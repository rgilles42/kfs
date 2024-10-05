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

// extern "C" {
// 	fn get_sp() -> usize;
// 	fn get_bp() -> usize;
// }

// macro_rules! sp {
// 	() => (unsafe {get_sp()});
// }

// macro_rules! bp {
// 	() => (unsafe {get_bp()});
// }

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
	printk!("{}", _panic);
	loop {}
}


// fn print_stack(mut sp: usize, bp: usize) {
// 	while sp < bp {
// 		let stack_word: &[u32; 4] = unsafe {&*(sp as *const [u32; 4])};
// 		printk!("\x1b\x04\x000x{:08x}:\x1b\x0f\x00        0x{:08x}    0x{:08x}    0x{:08x}    0x{:08x}", sp, stack_word[0], stack_word[1], stack_word[2], stack_word[3]);
// 		sp += 4;
// 	}
// }

// fn demo_stackframe() {
// 	printk!("/***************/\n/* Stack frame */\n/***************/");
// 	printk!("Current values of sp: 0x{:08x} and bp: 0x{:08x}, size is of 0x{:08x}", sp!(), bp!(), bp!() - sp!());
// 	print_stack(sp!(), bp!());
// }

extern "C" {
    static kernel_image_start : u32;
    static kernel_image_end : u32;
}

#[no_mangle]
pub extern "C" fn kmain(magic: u32, mboot: *const u32) -> ! {
	vga::setup_io();
	gdt::load();
	printk!("Mutilboot: magic({:x}) mboot({:p})", magic, mboot);
	printk!("/*******************/\n/* VGA Output Demo */\n/*******************/");
	printk!("According to \x1b\x06\x00all laws \x1b\x01\x00of aviation, \x1b\x04\x00there is \x1b\x0d\x00no way \x1b\x0c\x00that a \x1b\x00\x0fbee should \x1b\x04\x0ebe able \x1b\x07\x03to fly\x1b\x0f\x00");
	// demo_stackframe();
    printk!("VGA initialized");


    let kstart: usize;
    let kend: usize;
    unsafe {
        printk!("Kernel start {:p}", &kernel_image_start);
        printk!("Kernel end {:p}", &kernel_image_end);
        kstart = &kernel_image_start as *const u32 as usize;
        kend = &kernel_image_end as *const u32 as usize;
    }
    // TODO should I calulate this before jumping to kstart, as it might require identity mapping more pages
    // at the start ?
    let ksize = (kend - kstart)/1024; 
    printk!("Kernel size : {}KB", ksize);
    printk!("Multiboot: magic({:x}) mboot({:p})", magic, mboot);

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
    printk!("Physical Memory regions:");
    for entry in memory::phys_mem().regions  {
        printk!("- {entry:?}");
    }

    // This will filter out unusable pages
    printk!("Start init pmm");
    pmm::init(memory::phys_mem());
    // Blocking out the first 4MB as they will always be mapped
    pmm::fill_range(FrameRange{start: Frame(0), size: (kend - arch::KERNEL_OFFSET) / arch::PAGE_SIZE});
    printk!("End init pmm");

    printk!("Setup paging post jump");
    paging::init_post_jump();

    printk!("Setting up the memory manager");
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
    dbg!("========================== END"); 
    loop {}
}
