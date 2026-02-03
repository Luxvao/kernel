limine_branch := "v7.x"

alias b := build_kernel

# Set up the base image
setup: mkimage mount_image && unmount_image
    mkdir -p disk/limine disk/EFI/BOOT
    wget -O disk/EFI/BOOT/BOOTX64.EFI https://github.com/limine-bootloader/limine/raw/{{limine_branch}}-binary/BOOTX64.EFI

# Makes a new image
mkimage:
    qemu-img create -f raw disk.img 64M
    mkfs.fat -F32 disk.img

# Mounts the image
mount_image:
    sudo mount -o loop,uid=$(id -u),gid=$(id -g) disk.img disk --mkdir

# Unmount the image
unmount_image:
    sudo umount disk

# Builds the kernel
build_kernel:
    cargo b -Zbuild-std=core,alloc --release -p kernel

# Builds the kernel in debug mode
build_kernel_debug:
    cargo b -Zbuild-std=core,alloc -p kernel

# Sets up the disk fs
disk: build_kernel mount_image && unmount_image
    cp ./boot/limine/limine.cfg disk/limine/limine.cfg
    cp ./target/release/kernel disk/kernel

# Sets up the disk with debug mode
disk_debug: build_kernel_debug mount_image && unmount_image
    cp ./boot/limine/limine.cfg disk/limine/limine.cfg
    cp ./target/debug/kernel disk/kernel

# Launches qemu
qemu: disk_debug
    qemu-system-x86_64 -d int -D qemu.log -serial file:kernel_serial.log -enable-kvm -drive if=pflash,format=raw,readonly=on,file=./boot/ovmf/OVMF_CODE.fd -drive if=pflash,format=raw,readonly=on,file=./boot/ovmf/OVMF_VARS.fd -drive format=raw,file=disk.img ${QEMU_ARGS:-}

# Cleans everything
clean:
    rm -r disk
    cargo clean
