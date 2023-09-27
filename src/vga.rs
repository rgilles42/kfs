use core::fmt;
use core::fmt::Write;

#[macro_export]
macro_rules! println {
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	let mut writer = Writer::new(ColourPair::new(Colour::White, Colour::Black));
	writer.write_fmt(args).unwrap();
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
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
	pub fn new(fg: Colour, bg: Colour) -> Self {
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
struct Buffer {
	chars: [[VGAChar; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}

const VGA_BUFFER_ADDR : u32 = 0xb8000;

struct Writer {
	current_colour_pair: ColourPair,
	buffer: &'static mut Buffer,
	current_col: usize,
	pos_in_col: usize,
}

impl Writer {
	pub fn new(current_colour_pair: ColourPair) -> Self {
		Writer {
			current_colour_pair,
			buffer: unsafe { &mut *(VGA_BUFFER_ADDR as *mut Buffer)},
			current_col: 0,
			pos_in_col: 0,
		}
	}
	fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.pos_in_col >= VGA_BUFFER_WIDTH {
					self.new_line();
				}
				self.buffer.chars[self.current_col][self.pos_in_col] = VGAChar {
					ascii_char: byte,
					colour_pair: self.current_colour_pair,
				};
				self.pos_in_col += 1;
			}
		}
	}
	fn new_line(&mut self) {
		if self.current_col < VGA_BUFFER_HEIGHT - 1 {
			self.current_col += 1;
		} else {
			let blank = VGAChar {
				ascii_char: b' ',
				colour_pair: self.current_colour_pair,
			};
			for row in 0..VGA_BUFFER_HEIGHT - 1 {
				for col in 0..VGA_BUFFER_WIDTH {
					self.buffer.chars[row][col] = self.buffer.chars[row + 1][col];
				}
			}
			for col in 0..VGA_BUFFER_WIDTH {
				self.buffer.chars[VGA_BUFFER_HEIGHT - 1][col] = blank;
			}
		}
		self.pos_in_col = 0;
	}
}

impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		for byte in s.bytes() {
			match byte {
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				_ => self.write_byte(0xfe),
			}

		}
		Ok(())
	}
}
