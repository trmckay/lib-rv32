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

## Testing

This project has a very flexible testing system.

Unit-tests are provided wherever appropriate.

Additionally, to test the whole system, test programs can be added to `tests/programs`.
A test is simply a directory containing `.c` and `.s` source files and a `test_case.json`
consisting of assertions about the state of the MCU after the program is complete.

During testing, Cargo will for each test:

1. Compile it for RISC-V
2. Spin up a new MCU
3. Program it with the generated binary
4. Run the test program for some number of cycles
5. Make assertions
6. Report succes or failure

Tests are run in CI, but can be run locally provided your system has `riscv(32|64)-unknown-elf-gcc`.

## TODO

- [ ] Base/integer ISA (i)
    - [x] Basic support
    - [ ] CSR/interrupt instructions
- [ ] Multiply (m)
- [ ] Atomics (a)
- [ ] Compressed (c)
