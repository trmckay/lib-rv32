/// Contains reference `Memory` struct.
mod memory;

/// Contains referende `RegisterFile` struct.
mod register_file;

#[cfg(test)]
mod test_runner;

pub use memory::*;
pub use register_file::*;

pub use lib_rv32_isa as isa;
pub use lib_rv32_isa::common as common;

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

#[cfg(test)]
mod tests {
    use super::*;
    use lib_rv32_isa::{exec_one, common::instructions};

    const MEM_SIZE: u32 = 0x10000;

    #[test]
    fn test_addi_x5_x5_1() {
        let mut mcu = Mcu::new(MEM_SIZE as usize);
        let bytes = instructions::ADDI_X5_X5_1.to_le_bytes();
        mcu.mem.program_le_bytes(&bytes).unwrap();
        exec_one(&mut mcu.pc, &mut mcu.mem, &mut mcu.rf).unwrap();

        for i in 0..32 {
            assert_eq!(
                match i {
                    5 => 1,
                    _ => 0,
                },
                mcu.rf.read(i).unwrap()
            );
        }

        for i in 1..(MEM_SIZE / 4) {
            assert_eq!(0, mcu.mem.read_word(i * 4).unwrap());
        }

        assert_eq!(4, mcu.pc);
    }

    #[test]
    fn test_addi_x5_x6_neg_1() {
        let mut mcu = Mcu::new(MEM_SIZE as usize);
        let bytes = instructions::ADDI_X5_X6_NEG_1.to_le_bytes();
        mcu.mem.program_le_bytes(&bytes).unwrap();
        exec_one(&mut mcu.pc, &mut mcu.mem, &mut mcu.rf).unwrap();

        for i in 0..32 {
            assert_eq!(
                match i {
                    5 => -1,
                    _ => 0,
                },
                mcu.rf.read(i).unwrap() as i32
            );
        }

        for i in 1..(MEM_SIZE / 4) {
            assert_eq!(0, mcu.mem.read_word(i * 4).unwrap());
        }

        assert_eq!(4, mcu.pc);
    }
}
