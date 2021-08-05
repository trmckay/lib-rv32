# lib-rv32

Rust library for emulating 32-bit RISC-V

![build](https://github.com/trmckay/lib-rv32i/actions/workflows/build.yml/badge.svg)
![tests](https://github.com/trmckay/lib-rv32i/actions/workflows/test.yml/badge.svg)

---

## Libray

This library can execute instructions against any memory and register file that implements
the required primitives in the traits `lib_rv32::traits::{Memory, RegisterFile}`. This is to
encourage usage with whatever frontend you desire.

However, reference implementations are provided in `lib_rv32::mcu`. The library provides
functions to read from the memory, registers, and step a single instruction. Since, the
user decides when to call these functions, these will probably fit most use-cases.

### Example

`my_app.rs`:
```rust
use std::path::Path;

use lib_rv32::mcu::*;
use lib_rv32::exec_one;

fn main() {
    let mut mcu: Mcu = Mcu::new(1024 * 64);
    
    mcu.mem
        .program_from_file(&Path::from("./prog.bin"))
        .expect("Could not program MCU.");

    loop {
        exec_one(&mut mcu.pc, &mut mcu.mem, &mut mcu.rf).unwrap();
    }
}
```

---
## CLI

### Usage

The primary use of the CLI is tracing execution of RISC-V programs and making assertions
about their behavior.

```
USAGE:
    lrv-cli [FLAGS] [OPTIONS] <binary>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Enable verbose logging

OPTIONS:
    -a, --assertions <ASSERTIONS>    A JSON formatted set of assertions.
    -m, --mem <MEM_SIZE>             Set the size of the MCU memory (default 64 KB).
    -s, --stop <STOP_PC>             Set the program counter at which to stop emulation.

ARGS:
    <binary>    RISC-V binary to execute
```

### Example

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

---

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

## TODO

- [ ] Base/integer ISA (i)
    - [x] Basic support
    - [ ] CSR/interrupt instructions
- [ ] Multiply (m)
- [ ] Atomics (a)
- [ ] Compressed (c)
