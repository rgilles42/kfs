.section .text

.global reload_gdt

reload_gdt:
  mov 4(%esp), %eax
  lgdtl (%eax)

reload_segments:
  ljmp $0x8, $reload_CS

reload_CS:
  movw $0x10, %ax
  movw $0x18, %bx
  movw %ax, %ds
  movw %ax, %es
  movw %ax, %fs
  movw %ax, %gs
  movw %bx, %ss
  ret
