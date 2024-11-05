#!/bin/sh
OUT_DIR="./bin"
OUT_IMG="${OUT_DIR}/galos.img"

# Determine grub target arch from args
if [ -z "$1" ]; then
    GRUB_ARCH="i386-pc"
else
    GRUB_ARCH="$1"
fi

# Build kernel
cargo build --release

if [ -d "${OUT_DIR}" ]; then
    rm -rf "${OUT_DIR}"
fi

printf "\x1b[92mBuilding disk image\x1b[97m\n"

# Create output directory
mkdir -p "${OUT_DIR}"
mkdir -p "${OUT_DIR}/mnt/"

# Create disk image
dd if=/dev/zero of=${OUT_IMG} bs=1M count=64

# Create a partition table and partition with `parted`
parted -s ${OUT_IMG} mklabel msdos
parted -s ${OUT_IMG} mkpart primary fat32 1MiB 100%
parted -s ${OUT_IMG} set 1 boot on

# Set up a loop device with partition support
LOOP_DEVICE=$(sudo losetup -Pf --show ${OUT_IMG})

# Create the filesystem
sudo mkfs.vfat "${LOOP_DEVICE}p1"

# Mount the partition
sudo mount "${LOOP_DEVICE}p1" ${OUT_DIR}/mnt

# Create boot dir
sudo mkdir -p ${OUT_DIR}/mnt/boot
sudo mkdir -p ${OUT_DIR}/mnt/EFI/
sudo mkdir -p ${OUT_DIR}/mnt/EFI/BOOT

sudo cp ./target/i386-unknown-elf/release/galkernel ${OUT_DIR}/mnt/boot/kernel-7.elf
sudo cp -r ./grub ${OUT_DIR}/mnt/boot/grub

# Install grub to disk
printf "\x1b[92mInstalling grub to image\x1b[97m\n"
sudo grub-install \
    --target=${GRUB_ARCH} \
    --boot-directory=${OUT_DIR}/mnt/boot \
    --modules="multiboot" \
    --root-directory=${OUT_DIR}/mnt \
    --no-floppy \
    ${LOOP_DEVICE} \
    --removable

# Unmount disk
sudo umount ${OUT_DIR}/mnt
sudo losetup -d "${LOOP_DEVICE}"

printf "\x1b[92mDisk image successfully created at ${OUT_IMG}\x1b[97m\n"