#! /bin/sh

mkdir disk disk/EFI disk/EFI/BOOT disk/loader disk/loader/entries

cargo r -p mkinitrd -- setup initrd