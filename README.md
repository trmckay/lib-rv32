# lib-rv32

![build](https://github.com/trmckay/lib-rv32i/actions/workflows/build.yml/badge.svg)
![tests](https://github.com/trmckay/lib-rv32i/actions/workflows/test.yml/badge.svg)

## Overview

lib-rv32 is a collection of Rust libraries for emulating, learning, and assembling 32-bit RISC-V
integer ISAs.

- [lib-rv32-isa](https://crates.io/crates/lib-rv32-isa): library for ISA simulation
- [lib-rv32-mcu](https://crates.io/crates/lib-rv32-mcu): reference implemenation of an MCU used in conjunction with lib_rv32_isa
- [lib-rv32-asm](https://crates.io/crates/lib-rv32-asm): library for assembling RISC-V programs
- [lib-rv32-cli](https://crates.io/crates/lib-rv32-cli): CLI tool exposing the libraries
- [lib-rv32-wasm]: An webapp using the library's WASM bindings.

---

## Libraries

### ISA simulator

This library can execute instructions against any memory and register file that implements
the required primitives in the traits `lib_rv32_common::traits::{Memory, RegisterFile}`. This is to
encourage usage with whatever frontend you desire.

However, reference implementations are provided in `lib_rv32_mcu::*`. The library provides
functions to read from the memory, registers, and step a single instruction. Since, the
user decides when to call these functions, these will probably fit most use-cases.

### MCU

The MCU crate provides an implemenation of `Memory` and `RegisterFile` for use with the ISA
simulator. With this, one can fully emulate an embedded RISC-V core.

### Assembler

This crate can be used to assemble simple RISC-V assembly programs. The main functions offered
by this library are:

- `assemble_ir`: assemble an instruction `&str` to a `u32`
- `assemble_program`: assemble a program `&str` to a `Vec<u32>`
- `assemble_program_buf`: assemble a `BufRead` to a `Vec<u32>`


## CLI

### Emulator

The primary use of the emulator is tracing execution of RISC-V programs and making assertions
about their behavior. It currently only supports simple binary memory images
(not ELF binaries).

Enter assertions into a JSON file (note: all numbers are strings to allow for hex or decimal radices).

`assert.json`:
```json
{
    "registers": {
        "x0": "0x0",
        "a0": "20"
    },
    "memory": {
        "0x0000": "0x00010117"
    }
}
```

Then run:
```
lrv-cli -v ./prog.bin -s 24 -a assert.json
```

This will execute `prog.bin`, stop at the PC value 0x24, and then make the assertions from `assert.json`.

The program will trace the execution instruction-by-instruction:
```
[0000]  00010117  |  auipc  sp, 0x10           |  sp <- 0x10000 (65536);
[0004]  fe010113  |  addi   sp, sp, -32        |  sp <- 0xffe0 (65504);
[0008]  00400513  |  addi   a0, zero, 4        |  a0 <- 0x4 (4);
[000c]  00500593  |  addi   a1, zero, 5        |  a1 <- 0x5 (5);
[0010]  00000097  |  auipc  ra, 0x0            |  ra <- 0x10 (16);
[0014]  018080e7  |  jalr   ra, (24)ra         |  ra <- 0x18 (24); pc <- 0x28;
...
```

When complete, it will summarize results:
```
...
[001c]  f0028293  |  addi   t0, t0, -256       |  t0 <- 0xf00 (3840);
[0020]  00a2a023  |  sw     a0, 0(t0)          |  (word *)0x00000f00 <- 0x14 (20);

Reached stop-PC.

a0 == 20
*0x00000000 == 65815
```

### Assembler

The CLI also exposes the assembler via the command line. You can assemble the file
`program.s` to `program.bin` using

`lrv-cli -cv program.s -o program.bin`

---

## Testing

This project has a very flexible testing system.

Unit-tests are provided wherever appropriate.

Additionally, to test the whole system, test programs can be added to `mcu/tests/programs`.
A test is simply a directory containing `.c` and `.s` source files and a `test_case.json`
consisting of assertions about the state of the MCU after the program is complete.

During testing, Cargo will for each test:

1. Compile it for RISC-V
2. Spin up a new MCU
3. Program it with the generated binary
4. Run the test program for some number of cycles
5. Make assertions
6. Report succes or failure

If a test fails, it will describe the error that caused the crash or the assertion that failed
and print an object dump of the compiled test binary:

```
...
[001c]  f0028293  |  addi   t0, t0, -256       |  t0 <- 0xf00 (3840);
[0020]  00a2a023  |  sw     a0, 0(t0)          |  (word *)0x00000f00 <- 0x14 (20);
Stopping because the stop PC 0x24 was reached.


Failed test: tests/programs/mul@0x00000024: Register assertion failed: (x10=0x00000014) != 0x00000018.

prog.elf:     file format elf32-littleriscv


Disassembly of section .text.init:

00000000 <start>:
   0:   00010117                auipc   sp,0x10
   4:   fe010113                addi    sp,sp,-32 # ffe0 <__global_pointer$+0xf75c>
   8:   00400513                li      a0,4
   c:   00500593                li      a1,5
...
```

Tests are run in CI, but can be run locally provided your system has `riscv(32|64)-unknown-elf-gcc`.
