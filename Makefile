QEMU=qemu-system-i386
CARGO=cargo

NAME=kfs.elf

AS=gcc
ASFLAGS=-m32 -masm=intel -g

LD=ld
LDFLAGS=-n -nostdlib -m elf_i386

KLIB=libkfs.a # TODO debug ? release ?


all: $(NAME)

$(NAME): $(KLIB) asm link

$(KLIB):
	$(CARGO) build  # TODO debug ?

asm:
	$(AS) $(ASFLAGS) -c src/arch/x86/boot.s  -o target/boot.o

link:
	$(LD) $(LDFLAGS) -T src/arch/x86/x86.ld -o $(NAME) target/boot.o target/x86/debug/$(KLIB) # TODO debug ?

mkiso: $(NAME)
	cp $(NAME) isodir/boot/kfs.bin
	grub-mkrescue -d ./i386-pc -o kfs.iso isodir

clean:
	$(CARGO) clean
	rm -f target/boot.o
	rm -f $(NAME)
	rm -f kfs.iso

run: mkiso
	$(QEMU) -cdrom kfs.iso -no-reboot

run_kernel: $(NAME)
	$(QEMU) -kernel $(NAME) -no-reboot

run_debug: $(NAME)
	$(QEMU) -kernel $(NAME) -s -S -no-reboot -d int,cpu_reset


