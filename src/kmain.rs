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
	println!(
"1 km a pied, ca use, ca use, 1 km a pied, ca use les souliers
2 km a pied, ca use, ca use, 2 km a pied, ca use les souliers
3 km a pied, ca use, ca use, 3 km a pied, ca use les souliers
4 km a pied, ca use, ca use, 4 km a pied, ca use les souliers
5 km a pied, ca use, ca use, 5 km a pied, ca use les souliers
6 km a pied, ca use, ca use, 6 km a pied, ca use les souliers
7 km a pied, ca use, ca use, 7 km a pied, ca use les souliers
8 km a pied, ca use, ca use, 8 km a pied, ca use les souliers
9 km a pied, ca use, ca use, 9 km a pied, ca use les souliers
10 km a pied, ca use, ca use, 10 km a pied, ca use les souliers
11 km a pied, ca use, ca use, 11 km a pied, ca use les souliers
12 km a pied, ca use, ca use, 12 km a pied, ca use les souliers
13 km a pied, ca use, ca use, 13 km a pied, ca use les souliers
14 km a pied, ca use, ca use, 14 km a pied, ca use les souliers
15 km a pied, ca use, ca use, 15 km a pied, ca use les souliers
16 km a pied, ca use, ca use, 16 km a pied, ca use les souliers
17 km a pied, ca use, ca use, 17 km a pied, ca use les souliers
18 km a pied, ca use, ca use, 18 km a pied, ca use les souliers
19 km a pied, ca use, ca use, 19 km a pied, ca use les souliers
20 km a pied, ca use, ca use, 20 km a pied, ca use les souliers
21 km a pied, ca use, ca use, 21 km a pied, ca use les souliers
22 km a pied, ca use, ca use, 22 km a pied, ca use les souliers
23 km a pied, ca use, ca use, 23 km a pied, ca use les souliers
24 km a pied, ca use, ca use, 24 km a pied, ca use les souliers
25 km a pied, ca use, ca use, 25 km a pied, ca use les souliers
26 km a pied, ca use, ca use, 26 km a pied, ca use les souliers
27 km a pied, ca use, ca use, 27 km a pied, ca use les souliers
28 km a pied, ca use, ca use, 28 km a pied, ca use les souliers
29 km a pied, ca use, ca use, 29 km a pied, ca use les souliers
30 km a pied, ca use, ca use, 30 km a pied, ca use les souliers");
	loop {}
}

