pub use lib_rv32_isa::traits::RegisterFile as RegisterFileTrait;
use lib_rv32_isa::{RiscvError, common::constants::*};
use log::info;

/// Heap allocated implementation of a register file.
#[derive(Default, Clone)]
pub struct RegisterFile {
    registers: Vec<u32>,
}

impl RegisterFile {
    pub fn new() -> Self {
        RegisterFile {
            registers: vec![0; 31],
        }
    }
}

impl RegisterFileTrait for RegisterFile {
    fn write(&mut self, num: u8, data: u32) -> Result<(), RiscvError> {
        if num > 31 {
            return Err(RiscvError::RegisterOutOfRangeError(num));
        } else if num >= 1 {
            self.registers[num as usize - 1] = data;
        }

        info!(
            "{} <- 0x{:x} ({}); ",
            REG_NAMES[num as usize], data, data as i32
        );

        Ok(())
    }

    fn read(&self, num: u8) -> Result<u32, RiscvError> {
        if num == 0 {
            Ok(0)
        } else if num > 31 {
            Err(RiscvError::RegisterOutOfRangeError(num))
        } else {
            Ok(self.registers[num as usize - 1])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let mut rf = RegisterFile::new();
        rf.write(0, 17).unwrap();
        assert_eq!(0, rf.read(0).unwrap());
    }

    #[test]
    fn test_read_write() {
        let mut rf = RegisterFile::new();
        for i in 0..128 {
            let d = i << 16;
            for n in 0..32 {
                rf.write(n, d).unwrap();
                assert_eq!(if n == 0 { 0 } else { d }, rf.read(n).unwrap());
            }
        }
    }

    #[test]
    fn test_out_of_range() {
        assert_eq!(
            Err(RiscvError::RegisterOutOfRangeError(32)),
            RegisterFile::new().read(32)
        )
    }
}
