// The goal here is to map the first 4MB of the physical memory in two places in the virtual space
// one identity mapped, and one mapped in the higher half
// this code is one of the first things to run
// the kernel symbols, being linked as if it were loaded physically in the higher half, but it's not
// thus in order for all the adress to be valid, we must first create the double mapping and enable paging

.globl setup_early_paging
.globl EPD_PHYS
.globl EARLY_PAGE_DIRECTORY

.set HIGH_MAPPING_START, 0xC0000000
// only the 10 leftmost bits are used to index into the PDE
.set PDE_INDEX, (HIGH_MAPPING_START >> 22) 

.section .text

.set EPT_PHYS, (EARLY_PAGE_TABLE - HIGH_MAPPING_START)
.set EPD_PHYS, (EARLY_PAGE_DIRECTORY - HIGH_MAPPING_START)

setup_early_paging:

  push %eax
  push %ebx
  push %ecx

  mov $0x0, %eax
  // base physical address
  mov $0x0, %ebx
  // Maybe there is a better way than to use this register for indexs TODO
  push %edx
  fill_table:
      mov %ebx, %ecx
      or $3, %ecx
      // calculating index TODO I don't know the better GAS syntax
      mov %eax, %edx
      imul $4, %edx
      add $EPT_PHYS, %edx
      mov %ecx, (%edx)
      // increment 4KB everytime
      add $4096, %ebx
      inc %eax
      cmp $1024, %eax
      jne fill_table
 

  mov $EPT_PHYS, %eax
  or $0x83, %eax
  // first entry (identity mapping first 4MB)
  mov %eax, EPD_PHYS
  // same table for kernel address space space
  mov %eax, EPD_PHYS + PDE_INDEX * 4

  pop %edx

  mov $EPD_PHYS, %eax
  mov %eax, %cr3

  mov %cr0, %eax
  // paging bit
  or $0x80000000, %eax
  mov %eax, %cr0
  

  pop %ecx
  pop %ebx
  pop %eax

  ret


.section .data
.align 4096
EARLY_PAGE_TABLE:
  .rept 1024
      .long 0
  .endr

.align 4096
EARLY_PAGE_DIRECTORY:
  .rept 1024
      .long 0
  .endr

