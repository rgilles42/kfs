.section .text

.global load_gdt
.global reload_segments

gdtr:
  .word 0          # Size of GDT - 1
  .long 0          # Offset of GDT

load_gdt:
  mov 4(%esp), %ax
  movw %ax, gdtr
  movl 8(%esp), %eax
  movl %eax, gdtr + 2
  lgdt gdtr
  ret

reload_segments:
  ljmp $0x8, $reload_CS

reload_CS:
  movw $0x10, %ax
  movw %ax, %ds
  movw %ax, %es
  movw %ax, %fs
  movw %ax, %gs
  movw %ax, %ss
  ret
