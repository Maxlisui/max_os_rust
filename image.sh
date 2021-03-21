#!/bin/bash

PWD=$(pwd)
IMAGE="image.hdd"
KERNEL_ELF="target/x86_64-unknown-none/debug/max_os_rust.elf"

rm $IMAGE
dd if=/dev/zero bs=1M count=0 seek=64 of=$IMAGE
parted -s $IMAGE mklabel gpt
parted -s $IMAGE mkpart primary 2048s 100%
echfs-utils -g -p0 $IMAGE quick-format 512
echfs-utils -g -p0 $IMAGE import limine.cfg limine.cfg
echfs-utils -g -p0 $IMAGE import $KERNEL_ELF max_os_rust.elf
limine-install $IMAGE