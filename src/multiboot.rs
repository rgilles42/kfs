// multiboot handling file for x86 and x86_64
/* How many bytes from the start of the file we search for the header. */

use crate::printk;

#[allow(dead_code)]
pub mod flags {
    pub const MULTIBOOT_SEARCH : u32                        = 8192;
    pub const MULTIBOOT_HEADER_ALIGN : u32                  = 4;

    /* The magic field should contain this. */
    pub const MULTIBOOT_HEADER_MAGIC : u32                  = 0x1BADB002;

    /* This should be in %eax. */
    pub const MULTIBOOT_BOOTLOADER_MAGIC : u32              = 0x2BADB002;

    /* Alignment of multiboot modules. */
    pub const MULTIBOOT_MOD_ALIGN : u32                     = 0x00001000;

    /* Alignment of the multiboot info structure. */
    pub const MULTIBOOT_INFO_ALIGN : u32                    = 0x00000004;

    /* Flags set in the ’flags’ member of the multiboot header. */

    /* Align all boot modules on i386 page (4KB) boundaries. */
    pub const MULTIBOOT_PAGE_ALIGN : u32                    = 0x00000001;

    /* Must pass memory information to OS. */
    pub const MULTIBOOT_MEMORY_INFO : u32                   = 0x00000002;

    /* Must pass video information to OS. */
    pub const MULTIBOOT_VIDEO_MODE : u32                    = 0x00000004;

    /* This flag indicates the use of the address fields in the header. */
    pub const MULTIBOOT_AOUT_KLUDGE : u32                   = 0x00010000;

    /* Flags to be set in the ’flags’ member of the multiboot info structure. */

    /* is there basic lower/upper memory information? */
    pub const MULTIBOOT_INFO_MEMORY : u32                   = 0x00000001;
    /* is there a boot device set? */
    pub const MULTIBOOT_INFO_BOOTDEV : u32                  = 0x00000002;
    /* is the command-line defined? */
    pub const MULTIBOOT_INFO_CMDLINE : u32                  = 0x00000004;
    /* are there modules to do something with? */
    pub const MULTIBOOT_INFO_MODS : u32                     = 0x00000008;

    /* These next two are mutually exclusive */

    /* is there a symbol table loaded? */
    pub const MULTIBOOT_INFO_AOUT_SYMS : u32                = 0x00000010;
    /* is there an ELF section header table? */
    pub const MULTIBOOT_INFO_ELF_SHDR : u32                 = 0x00000020;

    /* is there a full memory map? */
    pub const MULTIBOOT_INFO_MEM_MAP : u32                  = 0x00000040;

    /* Is there drive info? */
    pub const MULTIBOOT_INFO_DRIVE_INFO : u32               = 0x00000080;

    /* Is there a config table? */
    pub const MULTIBOOT_INFO_CONFIG_TABLE : u32             = 0x00000100;

    /* Is there a boot loader name? */
    pub const MULTIBOOT_INFO_BOOT_LOADER_NAME : u32         = 0x00000200;

    /* Is there a APM table? */
    pub const MULTIBOOT_INFO_APM_TABLE : u32                = 0x00000400;

    /* Is there video information? */
    pub const MULTIBOOT_INFO_VBE_INFO : u32                 = 0x00000800;
    pub const MULTIBOOT_INFO_FRAMEBUFFER_INFO : u32         = 0x00001000;
}
pub use flags::*;

type Multibootu8 = u8;
type Multibootu16 = u16;
type Multibootu32 = u32;
type Multibootu64 = u64;

#[repr(C)]
#[allow(dead_code)]
struct MultibootHeader
{
  /* Must be MULTIBOOT_MAGIC - see above. */
  magic: Multibootu32,

  /* Feature flags. */
  flags: Multibootu32,

  /* The above fields plus this one must equal 0 mod 2^32. */
  checksum: Multibootu32,

  /* These are only valid if MULTIBOOT_AOUT_KLUDGE is set. */
  header_addr: Multibootu32,
  load_addr: Multibootu32,
  load_end_addr: Multibootu32,
  bss_end_addr: Multibootu32,
  entry_addr: Multibootu32,

  /* These are only valid if MULTIBOOT_VIDEO_MODE is set. */
  mode_type: Multibootu32,
  width: Multibootu32,
  height: Multibootu32,
  depth: Multibootu32,
}

/* The symbol table for a.out. */
#[repr(C)]
#[derive(Copy, Clone)]
struct SymbolTable
{
  tabsize: Multibootu32,
  strsize: Multibootu32,
  addr: Multibootu32,
  reserved: Multibootu32,
}


/* The section header table for ELF. */
#[repr(C)]
#[derive(Copy, Clone)]
struct ElfSectionHeader
{
  num: Multibootu32,
  size: Multibootu32,
  addr: Multibootu32,
  shndx: Multibootu32,
}

// I'v extracted the union types from the C code
#[repr(C)]
pub union multiboot_header_union
{
  aout: SymbolTable,
  elf: ElfSectionHeader,
}

#[repr(C)]
pub struct FramebufferCommon
{
  palette_addr: u32,
  palette_num_colors: u16
}

// TODO : translate this to rust in the structure above
// union
// {
//   struct
//   {
//     multiboot_uint32_t framebuffer_palette_addr;
//     multiboot_uint16_t framebuffer_palette_num_colors;
//   };
//   struct
//   {
//     multiboot_uint8_t framebuffer_red_field_position;
//     multiboot_uint8_t framebuffer_red_mask_size;
//     multiboot_uint8_t framebuffer_green_field_position;
//     multiboot_uint8_t framebuffer_green_mask_size;
//     multiboot_uint8_t framebuffer_blue_field_position;
//     multiboot_uint8_t framebuffer_blue_mask_size;
//   };
// };

#[repr(C)]
pub struct MultibootInfo
{
  /* Multiboot info version number */
  pub flags: Multibootu32,

  /* Available memory from BIOS */
  pub mem_lower: Multibootu32,
  pub mem_upper: Multibootu32,

  /* "root" partition */
  pub boot_device: Multibootu32,

  /* Kernel command line */
  pub cmdline: Multibootu32,

  /* Boot-Module list */
  pub mods_count: Multibootu32,
  pub mods_addr: Multibootu32,

  pub u: multiboot_header_union,

  /* Memory Mapping buffer */
  pub mmap_length: Multibootu32,
  pub mmap_addr: Multibootu32,

  /* Drive Info buffer */
  pub drives_length: Multibootu32,
  pub drives_addr: Multibootu32,

  /* ROM configuration table */
  pub config_table: Multibootu32,

  /* Boot Loader Name */
  pub boot_loader_name: Multibootu32,

  /* APM table */
  pub apm_table: Multibootu32,

  /* Video */
  pub vbe_control_info: Multibootu32,
  pub vbe_mode_info: Multibootu32,
  pub vbe_mode: Multibootu16,
  pub vbe_interface_seg: Multibootu16,
  pub vbe_interface_off: Multibootu16,
  pub vbe_interface_len: Multibootu16,

  pub framebuffer_addr: Multibootu64,
  pub framebuffer_pitch: Multibootu32,
  pub framebuffer_width: Multibootu32,
  pub framebuffer_height: Multibootu32,
  pub framebuffer_bpp: Multibootu8,
// #define MULTIBOOT_FRAMEBUFFER_TYPE_INDEXED 0
// #define MULTIBOOT_FRAMEBUFFER_TYPE_RGB     1
// #define MULTIBOOT_FRAMEBUFFER_TYPE_EGA_TEXT     2
  pub framebuffer_type: Multibootu8,

  pub framebuffer_palette: FramebufferCommon,
}

#[repr(C)]
#[allow(dead_code)]
struct MultiBootColor
{
  red: Multibootu8,
  green: Multibootu8,
  blue: Multibootu8,
}

#[repr(C, packed)]
#[allow(dead_code)]
#[derive(Copy, Clone)]
struct MultibootMmapEntry
{
  pub size: Multibootu32,
  pub addr: Multibootu64,
  pub len: Multibootu64,
// #define MULTIBOOT_MEMORY_AVAILABLE              1
// #define MULTIBOOT_MEMORY_RESERVED               2
// #define MULTIBOOT_MEMORY_ACPI_RECLAIMABLE       3
// #define MULTIBOOT_MEMORY_NVS                    4
// #define MULTIBOOT_MEMORY_BADRAM                 5
  pub type_: Multibootu32
}

#[repr(C)]
#[allow(dead_code)]
struct MultibootModList
{
  /* the memory used goes from bytes ’mod_start’ to ’mod_end-1’ inclusive */
  mod_start: Multibootu32,
  mod_end: Multibootu32,

  /* Module command line */
  cmdline: Multibootu32,

  /* padding to take it to 16 bytes (must be zero) */
  pad: Multibootu32,
}

/* APM BIOS info. */
#[repr(C)]
#[allow(dead_code)]
struct MultibootApmInfo
{
  version: Multibootu16,
  cseg: Multibootu16,
  offset: Multibootu32,
  cseg_16: Multibootu16,
  dseg: Multibootu16,
  flags: Multibootu16,
  cseg_len: Multibootu16,
  cseg_16_len: Multibootu16,
  dseg_len: Multibootu16,
}


pub fn parse_mboot_info(ptr: *const u32)
{
    let info : &MultibootInfo = unsafe {&*(ptr as *const MultibootInfo)};

    printk!("Multiboot info : flags({:b})", info.flags);
    // memory info
    if info.flags & MULTIBOOT_INFO_MEMORY != 0 {
        printk!("Memory lower: {} KB", info.mem_lower);
        printk!("Memory upper: {} KB", info.mem_upper);
    }
    // memory map
    if info.flags & MULTIBOOT_INFO_MEM_MAP != 0 {
        let nentries = info.mmap_length / core::mem::size_of::<MultibootMmapEntry>() as u32;
        printk!("Memory map has {} entries", nentries);
        let mut ptr = info.mmap_addr as *const MultibootMmapEntry;
        for _i in 0..nentries
        {
          unsafe {
            let entry = ptr.read_unaligned();
            printk!("Mmap entry : size({}) addr({:p}) len({}) type({})",
                  {entry.size}, {entry.addr as *const u32}, {entry.len}, {entry.type_});
            ptr = ptr.offset(1);
          }
        }
        printk!("Address of frame buffer : {:p}", info.framebuffer_addr as *const u32);
    }

    // elf or aout
    if info.flags & MULTIBOOT_INFO_AOUT_SYMS != 0 && info.flags & MULTIBOOT_INFO_ELF_SHDR != 0 {
      panic!("This is not possible ! Those two flags are mutually exclusive");
    }

    if info.flags & MULTIBOOT_INFO_AOUT_SYMS != 0 {
      printk!("This is an AOUT format");
    }
    if info.flags & MULTIBOOT_INFO_ELF_SHDR != 0 {
      printk!("This is an ELF format");
    }
    printk!("4bit {} 5bit {}", (info.flags & MULTIBOOT_INFO_AOUT_SYMS), (info.flags & MULTIBOOT_INFO_ELF_SHDR));
}
