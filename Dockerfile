# This Dockerfile aims to emulate CI for local debugging.
# It's also useful if you want to test/develop but do not
# have a toolchain installed locally.

from ubuntu:latest

RUN apt update

RUN \
    DEBIAN_FRONTEND="noninteractive" \
    apt install -y \
    git cargo make gcc-riscv64-unknown-elf binutils-riscv64-unknown-elf

RUN mkdir -p /repo
COPY . /repo

WORKDIR /repo
ENTRYPOINT ["make", "test"]
