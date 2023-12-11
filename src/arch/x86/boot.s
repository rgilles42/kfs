.globl kmain
.globl start
.globl get_sp
.globl get_bp

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
.global get_sp
.type get_sp,@function
.global get_bp
.type get_bp,@function
start:
  movl $stack_ptr, %esp
  movl $stack_ptr, %ebp
  push %ebx
  push %eax
  call kmain
  cli
1:hlt
  jmp 1b

get_sp:
  movl %esp, %eax
  ret

get_bp:
  movl %ebp, %eax
  ret

.size start, . - start

