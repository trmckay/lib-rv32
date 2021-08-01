use crate::RiscvError;

pub trait RegisterFile {
    fn read(&self, num: u8) -> Result<u32, RiscvError>;
    fn write(&mut self, num: u8, data: u32) -> Result<(), RiscvError>;
}

pub trait Memory {
    fn read_word(&self, addr: u32) -> Result<u32, RiscvError>;
    fn read_half_word(&self, addr: u32) -> Result<u32, RiscvError>;
    fn read_byte(&self, addr: u32) -> Result<u32, RiscvError>;
    fn write_word(&mut self, addr: u32, data: u32) -> Result<(), RiscvError>;
    fn write_half_word(&mut self, addr: u32, data: u32) -> Result<(), RiscvError>;
    fn write_byte(&mut self, addr: u32, data: u32) -> Result<(), RiscvError>;
}
