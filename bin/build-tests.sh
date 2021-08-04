#!/bin/bash

set -e

cd $(git rev-parse --show-toplevel)

deps_32=(riscv32-unknown-elf-gcc riscv32-unknown-elf-objdump riscv32-unknown-elf-objcopy)
deps_64=(riscv64-unknown-elf-gcc riscv64-unknown-elf-objdump riscv64-unknown-elf-objcopy)

if ! command -v ${deps_32[@]} || command -v ${deps_64[@]}; then
    echo "Missing a RISC-V toolchain."
    exit 1
fi

for dir in `find ./tests/programs -type d -not -path ./tests/programs`; do
    (cd $dir && make -f ../Makefile)
done
