#! /bin/sh

# Bootloader & kernel
mv target/i686-unknown-uefi/release/biboot.efi disk/efi/boot/IA32BOOT.EFI
mv target/i686-unknown-linux-gnu/release/kernel disk/vmkernel

# Initrd
