#! /bin/sh

./build.sh

# Bootloader & kernel
cp target/i686-unknown-uefi/release/biboot.efi disk/EFI/BOOT/BOOTX64.EFI
cp biboot/config/* disk/loader/
cp target/i686-unknown-linux-gnu/release/kernel disk/vmkernel

# Initrd
cargo r -p mkinitrd create initrd disk/initramfs