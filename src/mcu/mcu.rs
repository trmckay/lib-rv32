use super::memory::Memory;
use super::register_file::RegisterFile;

pub struct Mcu {
    pub pc: u32,
    pub mem: Memory,
    pub rf: RegisterFile,
}

impl Mcu {
    pub fn new(size: usize) -> Self {
        Mcu {
            pc: 0,
            mem: Memory::new(size),
            rf: RegisterFile::new(),
        }
    }
}
