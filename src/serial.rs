
use core::fmt;
use core::fmt::Write;
use core::arch::asm;

#[macro_export]
macro_rules! dbg_print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! dbg {
        () => {
            ($crate::dbg_print!("\n"))
        };
        ($($arg:tt)*) => {
            ($crate::dbg_print!("{}\n", format_args!($($arg)*)))
        };
}

static mut COM1: SerialDriver = SerialDriver { com1: Pio::new(0x3f8) };

pub fn _print(args: fmt::Arguments) {
    unsafe {
        COM1.write_fmt(args).unwrap();
    }
}

struct SerialDriver {
     com1: Pio<u8>
}

impl SerialDriver {
    pub fn puts(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.com1.write(*b);
        }
    }
}

impl Write for SerialDriver {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.puts(&s.as_bytes());
        Ok(())
    }
}

use core::marker::PhantomData;
#[derive(Copy, Clone)]
pub struct Pio<Op> {
    port: u16,
    _value: PhantomData<Op>,
}


pub trait PortIO {
    type Op;

    fn read(&self) -> Self::Op;
    fn write(&self, op: Self::Op);
}


impl<T> Pio<T> {
    pub const fn new(port: u16) -> Self {
        Pio {
            port,
            _value: PhantomData,
        }
    }
}

impl PortIO for Pio<u8> {
    type Op = u8;

    #[inline]
    fn read(&self) -> Self::Op {
        inb(self.port)
    }

    #[inline]
    fn write(&self, op: Self::Op) {
        outb(self.port, op);
    }
}

pub fn outb(port: u16, byte: u8) {
    unsafe {
        asm!(
        "out dx, al",
        in("dx") port,
        in("al") byte
        );
    }
}

pub fn inb(port: u16) -> u8 {
    let byte: u8;
    unsafe {
        asm!(
        "in al, dx",
        in("dx") port,
        out("al") byte
        );
    }
    byte
}


