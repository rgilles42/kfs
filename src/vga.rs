use core::fmt;
use core::fmt::Write;
use core::mem::transmute;

#[macro_export]
macro_rules! printk {
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	unsafe {
		if let Some(vga) = &mut GLOBAL_VGA {
			vga.write_fmt(args).unwrap();
		}
	};
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Colour {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColourPair(u8);

impl ColourPair {
	fn new(fg: Colour, bg: Colour) -> Self {
		ColourPair((bg as u8) << 4 | (fg as u8))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VGAChar {
	ascii_char: u8,
	colour_pair: ColourPair,
}

const VGA_BUFFER_HEIGHT: usize = 25;
const VGA_BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct VGABuffer {
	chars: [[VGAChar; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}

impl VGABuffer {
	pub fn read(&self, row: usize, col: usize) -> VGAChar {
		unsafe {
			core::ptr::read_volatile(
				&self.chars[row][col] as *const VGAChar,
			)
		}
	}
	pub fn write(&mut self, row: usize, col: usize, vgachar: VGAChar) {
		unsafe {
			core::ptr::write_volatile(
				&mut self.chars[row][col] as *mut VGAChar,
				vgachar
			);
		}
	}
}

static mut GLOBAL_VGA: Option<VGA> = None;
const VGA_BUFFER_ADDR : u32 = 0xC00b8000;

pub fn setup_io() {
	unsafe {
		if GLOBAL_VGA.is_none() {
			GLOBAL_VGA = Some(VGA::new());
			//printk!("/*********************/\n/* VGA Output Loaded */\n/*********************/");
		}
	}
}


pub struct VGA {
	buffer: &'static mut VGABuffer,
	current_row: usize,
	pos_in_row: usize,
	colour_pair: ColourPair
}

impl VGA {
	fn new() -> Self {
		VGA {
			buffer: unsafe {&mut *(VGA_BUFFER_ADDR as *mut VGABuffer)},
			current_row: 0,
			pos_in_row: 0,
			colour_pair: ColourPair::new(Colour::White, Colour::Black)
		}
	}
	fn printstr(&mut self, s: &str) {
		let mut is_ansiing: Option<[u8; 2]> = None;
		for byte in s.bytes() {
			match byte {
				0x1B => is_ansiing = Some([255, 255]),
				0x00..=0x0F if is_ansiing.is_some() => {
					let dest: usize = if is_ansiing.as_ref().unwrap()[0] == 255 {0} else {1};
					is_ansiing.as_mut().unwrap()[dest] = byte;
					if dest == 1 {
						self.colour_pair = unsafe {ColourPair::new(transmute(is_ansiing.unwrap()[0] % 16), transmute(is_ansiing.unwrap()[1] % 16))};
						is_ansiing = None;
					}
				}
				0x20..=0xfe | b'\n' => {
					if is_ansiing.is_some() {
						is_ansiing = None;
					}
					self.write_byte(byte)
				},
				_ => self.write_byte(0xfe),
			}
		}
	}
	fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.pos_in_row >= VGA_BUFFER_WIDTH {
					self.new_line();
				}
				self.buffer.write(self.current_row, self.pos_in_row,
					VGAChar {
						ascii_char: byte,
						colour_pair: self.colour_pair,
					}
				);
				self.pos_in_row += 1;
			}
		}
	}
	fn new_line(&mut self) {
		if self.current_row < VGA_BUFFER_HEIGHT - 1 {
			self.current_row += 1;
		} else {
			let blank = VGAChar {
				ascii_char: b' ',
				colour_pair: ColourPair::new(Colour::White, Colour::Black),
			};
			for row in 0..VGA_BUFFER_HEIGHT - 1 {
				for col in 0..VGA_BUFFER_WIDTH {
					self.buffer.write(row, col, self.buffer.read(row + 1, col));
				}
			}
			for col in 0..VGA_BUFFER_WIDTH {
				self.buffer.write(VGA_BUFFER_HEIGHT - 1, col, blank);
			}
		}
		self.pos_in_row = 0;
	}
}

impl fmt::Write for VGA {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.printstr(s);
		Ok(())
	}
}
