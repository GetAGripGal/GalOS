#!/bin/sh
USAGE="usage: buildiso.sh <target_arch>"

# Read arguments
if [ -z "$1" ]; then 
    printf "\x1b[91mNo target architecture provided\x1b[97m\n"
    echo ${USAGE}
    exit
fi

TARGET_ARCH="${1}"

OUT_DIR="./bin"
OUT_IMG="${OUT_DIR}/galos-${TARGET_ARCH}.iso"

# Build kernel
cargo build --release --target "targets/${TARGET_ARCH}/arch.json" -Zbuild-std=core,compiler_builtins -Zbuild-std-features=compiler-builtins-mem

# Remove old output directory if it exists
if [ -d "${OUT_DIR}" ]; then
    rm -rf "${OUT_DIR}"
fi

printf "\x1b[92mBuilding ISO image\x1b[97m\n"

# Create output directory and necessary subdirectories
mkdir -p "${OUT_DIR}/iso/boot/grub"

# Copy the kernel binary
cp ./target/arch/release/galkernel "${OUT_DIR}/iso/boot/kernel-7.elf"

# Copy GRUB configuration
cp -r ./grub/* "${OUT_DIR}/iso/boot/grub/"

# Install GRUB to create a bootable ISO
printf "\x1b[92mInstalling grub to ISO\x1b[97m\n"
grub-mkrescue -o "${OUT_IMG}" "${OUT_DIR}/iso" --compress=xz

printf "\x1b[92mISO image successfully created at ${OUT_IMG}\x1b[97m\n"
