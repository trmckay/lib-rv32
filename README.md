# lib-rv32
Rust library for emulating 32-bit RISC-V

![build](https://github.com/trmckay/lib-rv32i/actions/workflows/build.yml/badge.svg)
![tests](https://github.com/trmckay/lib-rv32i/actions/workflows/test.yml/badge.svg)


## Usage

Use as a library or as a CLI emulator.

### Libray

This library can execute instructions against any memory and register file that implements
the required primitives in the traits `lib_rv32::{Memory, RegisterFile}`. This is to
encourage usage with whatever frontend you desire.

However, reference implementations are provided in `lib_rv32::mcu`. The library provides
functions to read from the memory, registers, and step a single instruction. Since, the
user decides when to call these functions, these will probably fit most use-cases.

### CLI

Work in progress.

## TODO

- [ ] Base/integer ISA (i)
    - [x] Basic support
    - [ ] CSR/interrupt instructions
- [ ] Multiply (m)
- [ ] Atomics (a)
- [ ] Compressed (c)
