.weak kmain
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

.section .text

start:
  movl $stack_ptr, %esp
  push %eax
  push %ebx
  //call kmain
  cli

hang:
  hlt
  jmp hang

stack:
  .skip STACKSIZE
stack_ptr:

