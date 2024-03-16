#! /bin/sh

# Build bootloader
cargo build --release -p biboot --target i686-unknown-uefi

# Build kernel
cargo build --release -p kernel --target i686-unknown-linux-gnu
