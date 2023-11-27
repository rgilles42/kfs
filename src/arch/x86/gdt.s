.section .text

.global load_gdt
.global reload_segments

load_gdt:
  mov 4(%esp), %eax
  lgdtl (%eax)
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
