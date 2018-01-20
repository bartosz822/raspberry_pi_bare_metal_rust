# Requires arm-none-eabi toolchain and xargo in PATH

ARMGNU ?= arm-none-eabi

COPS = -O2 -nostdlib -ffreestanding -T linker.ld

kernel.img : blinker.elf
	$(ARMGNU)-objcopy blinker.elf -O binary kernel.img

blinker.elf : target/arm-none-eabihf/debug/libkernel.rlib 
	$(ARMGNU)-gcc $(COPS) -o blinker.elf -O2 -ffreestanding -nostdlib boot.o greenled.o target/arm-none-eabihf/debug/libkernel.rlib

target/arm-none-eabihf/debug/libkernel.rlib : src/lib.rs
	xargo build --target arm-none-eabihf    

.PHONY: clean
clean :
	rm -f *.bin
	rm -f *.hex
	rm -f *.elf
	rm -f *.list
	rm -f *.img
	rm -rf target