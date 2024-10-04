QEMU=qemu-system-i386
CARGO=cargo

NAME=kfs.elf

AS=gcc
ASFLAGS=-m32 -masm=intel -g

LD=ld
LDFLAGS=-n -nostdlib -m elf_i386

ASM_OBJS =	target/x86/boot.o target/x86/gdt.o target/x86/mapping.o
KLIB =		target/x86/debug/libkfs.a # TODO debug ? release ?

ISO =		kfs.iso


target/%.o : src/arch/%.s
	$(AS) $(ASFLAGS) -c $< -o $@

all: $(NAME)

$(NAME): $(KLIB) $(ASM_OBJS) # TODO debug ? # TODO figure out no relink
	$(LD) $(LDFLAGS) -T src/arch/x86/x86.ld -o $(NAME) $(ASM_OBJS) $(KLIB) 

$(KLIB): # TODO debug ?
	$(CARGO) build  

iso: $(NAME)
	cp $(NAME) isodir/boot/kfs.bin
	grub-mkrescue -d ./i386-pc -o $(ISO) isodir

clean:
	$(CARGO) clean
	rm -f $(ASM_OBJS)
	rm -f $(NAME)
	rm -f $(ISO)

re: clean all

run: iso
	$(QEMU) -cdrom kfs.iso -no-reboot -serial stdio

run_kernel: $(NAME)
	$(QEMU) -kernel $(NAME) -no-reboot

run_debug: $(NAME)
	$(QEMU) -kernel $(NAME) -s -S -no-reboot -d int,cpu_reset

.PHONY: clean run run_kernel run_debug $(KLIB)
