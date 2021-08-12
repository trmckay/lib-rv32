from ubuntu:latest

RUN apt update
RUN apt install cargo gcc-riscv64-unknown-elf binutils-riscv64-unknown-elf

RUN mkdir -p /repo
COPY ./assembler repo
COPY ./cli repo
COPY ./common repo
COPY ./mcu repo
COPY ./Cargo.toml repo

WORKDIR /repo
CMD "cargo test -- --nocapture"
