QEMU=qemu-system-i386
CARGO=cargo

NAME=kfs.elf

AS=gcc
ASFLAGS=-m32 -masm=intel -g

LD=ld
LDFLAGS=-n -nostdlib -m elf_i386

KLIB =		target/x86/debug/libkfs.a # TODO debug ? release ?
BOOT_ASM =	target/x86/boot.o

ISO =		kfs.iso


all: $(NAME)

$(NAME): klib_build $(BOOT_ASM)
	$(LD) $(LDFLAGS) -T src/arch/x86/x86.ld -o $(NAME) $(BOOT_ASM) $(KLIB) # TODO debug ?

klib_build:
	$(CARGO) build  # TODO debug ?

$(BOOT_ASM): # TODO doesn't relink if the assembly file has changed
	$(AS) $(ASFLAGS) -c src/arch/x86/boot.s  -o $(BOOT_ASM)

iso: $(NAME)
	cp $(NAME) isodir/boot/kfs.bin
	grub-mkrescue -d ./i386-pc -o $(ISO) isodir

clean:
	$(CARGO) clean
	rm -f $(BOOT_ASM)
	rm -f $(NAME)
	rm -f $(ISO)

run: iso
	$(QEMU) -cdrom kfs.iso -no-reboot

run_kernel: $(NAME)
	$(QEMU) -kernel $(NAME) -no-reboot

run_debug: $(NAME)
	$(QEMU) -kernel $(NAME) -s -S -no-reboot -d int,cpu_reset

.PHONY: clean run run_kernel run_debug
