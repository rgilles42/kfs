use crate::arch;
use crate::memory::pmm::{PageManager, Frame, FrameRange};
use core::fmt;

/// WARNING : this is extremely slow and not intended to be the final PMM
/// Ultimately will be replaced by a buddy system

// TODO this is a naive page manager, only for bootstraping the kernel development
// TODO locking
pub const BITMAP_SIZE: usize = arch::N_PAGES; // enough to manage 4GB

pub struct BitMap {
    pub data: [u8; BITMAP_SIZE/8],
    pub size: usize,
    free_pages: usize
}

// TODO add max address for physical memory

impl BitMap {
    pub const fn default_const() -> Self {
        BitMap {
            data: [0xff; BITMAP_SIZE/8],
            size: BITMAP_SIZE,
            free_pages: 0
        }
    }
}

impl fmt::Display for BitMap
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BitMap(size: {}, free_pages: {})", self.size, self.free_pages)
    }
}

macro_rules! notset {
    ($self:ident, $i:expr) => {
        $self.data[$i / 8] & (1 << ($i % 8)) == 0
    };
}

macro_rules! set {
    ($self:ident, $i:expr) => {
        $self.data[$i / 8] |= 1 << ($i % 8);
    };
}

macro_rules! unset {
    ($self:ident, $i:expr) => {
        $self.data[$i / 8] &= !(1 << ($i % 8));
        $self.free_pages += 1;
    };
}

impl PageManager for BitMap {

    fn alloc_page(&mut self) -> Option<Frame> { // TODO limit and out of memory error
        for i in 0..self.size {
            if notset!(self, i){
                // TODO could I speed up this module with & ?
                set!(self, i);
                return Some(Frame(i));
            }
        }
        None
    }

    fn alloc_contiguous_pages(&mut self, n: usize) -> Option<FrameRange> {
        let mut j = 0;
        for i in 0..self.size {
            if notset!(self, i) {
                j += 1;
                if j == n {
                    for a in i-j..i {
                        set!(self, a);
                    }
                    return Some(FrameRange{start: Frame(i-j), size: n});
                }
            }
            else if j > 0
            {
                j = 0;
            }
        }
        None
    }

    fn free_page(&mut self, f: Frame) {
        unset!(self, f.0);
    }

    fn free_contiguous_pages(&mut self, r: FrameRange) {
        for i in r.start.0..r.start.0 + r.size {
            unset!(self, i);
        }
    }

    fn fill_range(&mut self, r: FrameRange) {
        for i in r.start.0..r.start.0 + r.size {
            set!(self, i);
        }
    }
}
