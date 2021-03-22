#!/bin/bash

bash image.sh
qemu-system-x86_64 image.hdd -no-reboot -s -serial stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04