.globl kmain
.globl start

.set MODULEALIGN, (1<<0)
.set MEMINFO, (1<<1)
.set FLAGS, MODULEALIGN | MEMINFO
.set MAGIC, 0x1BADB002
.set CHECKSUM, -(MAGIC + FLAGS)
.set STACKSIZE, 0x4000

.section .multiboot
.align 4
MultiBootHeader:
  .long MAGIC
  .long FLAGS
  .long CHECKSUM

.section .bss
.align 16
stack_bottom:
.skip 16384 # 16 KiB
stack_ptr:

.section .text
.global start
.type start,@function

start:
  movl $stack_ptr, %esp
  push %ebx
  push %eax
  call kmain
  cli
1:hlt
  jmp 1b

.size start, . - start

