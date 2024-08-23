LIMINE_BRANCH := v7.x

RS_FILES := $(shell find src -name '*.rs')
ASM_FILES := $(shell find src -name '*.s')

default: build

setup:
	mkdir disk/ disk/limine disk/EFI/ disk/EFI/BOOT/
	wget -O disk/EFI/BOOT/BOOTX64.EFI https://github.com/limine-bootloader/limine/raw/$(LIMINE_BRANCH)-binary/BOOTX64.EFI

build: $(RS_FILES) Cargo.toml
	cargo build -Zbuild-std=core,alloc --target x86_64-unknown-linux-gnu

asm: $(ASM_FILES)

release: $(RS_FILES) Cargo.toml
	cargo build -Zbuild-std=core,alloc --target x86_64-unknown-linux-gnu --release

disk: build
	cp limine.cfg disk/limine/limine.cfg
	cp target/x86_64-unknown-linux-gnu/debug/kernel_limine disk/kernel

qemu: disk
	qemu-system-x86_64 -serial file:kernel_serial.log -enable-kvm -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd -drive format=raw,file=fat:rw:disk

clean:
	rm -fr disk/
	cargo clean
