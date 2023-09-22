ENTRY(start)
OUTPUT(elf32-i386)

SECTIONS
{
    . = 0x00100000;
  .text : ALIGN(CONSTANT(MAXPAGESIZE)) {
	  . = ALIGN(8);
	  KEEP(*(.multiboot))
	_TEXT_START_ = .;
	    *(.text)
	_TEXT_END_ = .;
  }

  .data : ALIGN(CONSTANT(MAXPAGESIZE)) {
	_DATA_START_ = .;
	    *(.data)
	_DATA_END_ = .;
  }

  .bss : ALIGN(CONSTANT(MAXPAGESIZE)) {
	_BSS_START_ = .;
	    *(.bss)
	_BSS_END_ = .;
  }
}
