#!/bin/bash

set -e
cargo build
# qemu-system-x86_64  \
    # -enable-kvm \
    # -m 128 \
    # -nographic \
    # -bios /usr/share/edk2-ovmf/x64/OVMF_CODE.fd \
    # -device driver=e1000,netdev=n0 \
    # -netdev user,id=n0,tftp=target/x86_64-unknown-uefi/debug,bootfile=lazarus.efi

qemu-system-x86_64  \
    -enable-kvm \
    -m 128 \
    -bios /usr/share/edk2-ovmf/x64/OVMF_CODE.fd \
    -device driver=e1000,netdev=n0 \
    -netdev user,id=n0,tftp=target/x86_64-unknown-uefi/debug,bootfile=lazarus.efi
