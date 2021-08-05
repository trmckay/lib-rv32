/// Contains reference `Memory` struct.
mod memory;

/// Contains referende `RegisterFile` struct.
mod register_file;

pub use memory::*;
pub use register_file::*;

use memory::Memory;
use register_file::RegisterFile;

/// Reference implementation of an MCU. Contains a PC,
/// register file, and memory.
#[derive(Clone)]
pub struct Mcu {
    pub pc: u32,
    pub mem: Memory,
    pub rf: RegisterFile,
}

impl Mcu {
    /// Construct an MCU with the provided memory size.
    pub fn new(size: usize) -> Self {
        Mcu {
            pc: 0,
            mem: Memory::new(size),
            rf: RegisterFile::new(),
        }
    }
}
