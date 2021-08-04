use crate::{bit_slice, RiscvError};
use log::info;
/// Heap allocated implementation of memory.
#[derive(Clone)]
pub struct Memory {
    pub size: usize,
    mem: Vec<u8>,
}

impl Memory {
    /// Allocate a memory with the given size.
    pub fn new(size: usize) -> Self {
        assert!(size % 4 == 0);
        assert!(size > 0);

        Memory {
            size,
            mem: vec![0; size],
        }
    }

    /// Read a little-endian number.
    fn read(&self, base: usize, size: usize, log: bool) -> Result<u32, RiscvError> {
        // Check if read falls on a word, half-word, or byte boundary.
        if base % size != 0 {
            return Err(RiscvError::MemoryAlignmentError(base as u32));
        // Check that the read is within bounds.
        } else if base >= self.size as usize {
            return Err(RiscvError::MemoryOutOfBoundsError(base as u32));
        }

        let data = self.mem[base..base + size]
            .iter()
            .enumerate()
            .map(|(i, b)| ((*b as u32) << (i * 8)) as u32)
            .sum();

        if log {
            match size {
                1 => info!("(byte *)0x{:08x} = 0x{:x} ({}); ", base, data, data as i32),
                2 => info!(
                    "(half-word *)0x{:08x} = 0x{:x} ({}); ",
                    base, data, data as i32
                ),
                4 => info!("(word *)0x{:08x} = 0x{:x} ({}); ", base, data, data as i32),
                _ => (),
            }
        }

        Ok(data)
    }

    /// Write a little-endian number.
    fn write(&mut self, base: usize, data: u32, size: usize, log: bool) -> Result<(), RiscvError> {
        if log {
            match size {
                1 => info!("(byte *)0x{:08x} <- 0x{:x} ({}); ", base, data, data as i32),
                2 => info!(
                    "(half-word *)0x{:08x} <- 0x{:x} ({}); ",
                    base, data, data as i32
                ),
                4 => info!("(word *)0x{:08x} <- 0x{:x} ({}); ", base, data, data as i32),
                _ => (),
            }
        }

        // Check if read falls on a word, half-word, or byte boundary.
        if base % size != 0 {
            return Err(RiscvError::MemoryAlignmentError(base as u32));
        // Check that the read is within bounds.
        } else if base >= self.size as usize {
            return Err(RiscvError::MemoryOutOfBoundsError(base as u32));
        }

        for (i, b) in self.mem[base..base + size].iter_mut().enumerate() {
            *b = bit_slice!(data, 8 * (i + 1), 8 * i) as u8;
        }

        Ok(())
    }

    pub fn program_be_bytes(&mut self, bytes: &[u8]) -> Result<(), RiscvError> {
        for (word_addr, chunk) in bytes.chunks(4).enumerate() {
            for (byte_offset, byte) in chunk.iter().rev().enumerate() {
                if let Err(why) = self.write(word_addr * 4 + byte_offset, *byte as u32, 1, false) {
                    return Err(why);
                }
            }
        }
        Ok(())
    }

    pub fn program_le_bytes(&mut self, bytes: &[u8]) -> Result<(), RiscvError> {
        for (word_addr, chunk) in bytes.chunks(4).enumerate() {
            for (byte_offset, byte) in chunk.iter().enumerate() {
                if let Err(why) = self.write(word_addr * 4 + byte_offset, *byte as u32, 1, false) {
                    return Err(why);
                }
            }
        }
        Ok(())
    }

    pub fn program_words(&mut self, words: &[u32]) -> Result<(), RiscvError> {
        for (word_addr, word) in words.iter().enumerate() {
            if let Err(why) = self.write(word_addr * 4, *word, 4, false) {
                return Err(why);
            }
        }
        Ok(())
    }
}

// Implement the trait that allows us to execute instructions on this memory.
impl super::MemoryTrait for Memory {
    fn fetch(&self, pc: u32) -> Result<u32, RiscvError> {
        self.read(pc as usize, 4, false)
    }

    fn read_word(&self, addr: u32) -> Result<u32, RiscvError> {
        self.read(addr as usize, 4, true)
    }

    fn read_half_word(&self, addr: u32) -> Result<u32, RiscvError> {
        match self.read(addr as usize, 2, true) {
            Ok(d) => Ok(d),
            Err(why) => Err(why),
        }
    }
    fn read_byte(&self, addr: u32) -> Result<u32, RiscvError> {
        match self.read(addr as usize, 1, true) {
            Ok(d) => Ok(d),
            Err(why) => Err(why),
        }
    }

    fn write_word(&mut self, addr: u32, data: u32) -> Result<(), RiscvError> {
        self.write(addr as usize, data, 4, true)
    }

    fn write_half_word(&mut self, addr: u32, data: u32) -> Result<(), RiscvError> {
        self.write(addr as usize, data as u32, 2, true)
    }

    fn write_byte(&mut self, addr: u32, data: u32) -> Result<(), RiscvError> {
        self.write(addr as usize, data as u32, 1, true)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Memory as MemoryTrait;

    #[test]
    #[should_panic]
    fn create_misaligned() {
        let _ = Memory::new(3);
    }

    #[test]
    #[should_panic]
    fn create_zero() {
        let _ = Memory::new(0);
    }

    #[test]
    fn out_of_bounds() {
        let mem = Memory::new(1024);
        match mem.read_byte(1028) {
            Err(why) => assert_eq!(why, RiscvError::MemoryOutOfBoundsError(1028)),
            _ => panic!(),
        };
    }

    #[test]
    fn misaligned() {
        let mut mem = Memory::new(1024);

        match mem.read_half_word(3) {
            Err(why) => assert_eq!(why, RiscvError::MemoryAlignmentError(3)),
            _ => panic!(),
        };
        match mem.read_word(2) {
            Err(why) => assert_eq!(why, RiscvError::MemoryAlignmentError(2)),
            _ => panic!(),
        };
        match mem.write_half_word(3, 0) {
            Err(why) => assert_eq!(why, RiscvError::MemoryAlignmentError(3)),
            _ => panic!(),
        };
        match mem.write_word(2, 0) {
            Err(why) => assert_eq!(why, RiscvError::MemoryAlignmentError(2)),
            _ => panic!(),
        };
    }

    #[test]
    fn create() {
        let mem = Memory::new(1024);
        assert_eq!(1024, mem.size);
    }

    #[test]
    fn byte() {
        let mut mem = Memory::new(1024);

        for data in 0..0xFF {
            for addr in 0..16 {
                mem.write_byte(addr, data).unwrap();
                assert_eq!(data, mem.read_byte(addr).unwrap());
            }
        }
    }

    #[test]
    fn half_word_write() {
        const ADDR: u32 = 0x02;
        let mut mem = Memory::new(1024);

        mem.write_half_word(ADDR, 0x1712).unwrap();

        // Is it little-endian?
        assert_eq!(mem.mem[ADDR as usize], 0x12);
        assert_eq!(mem.mem[(ADDR + 1) as usize], 0x17);
    }

    #[test]
    fn half_word_read() {
        const ADDR: u32 = 0x02;
        let mut mem = Memory::new(1024);

        // mem[ADDR] = 0x1712;
        mem.mem[ADDR as usize] = 0x12;
        mem.mem[(ADDR + 1) as usize] = 0x17;

        assert_eq!(0x1712, mem.read_half_word(ADDR).unwrap());
    }

    #[test]
    fn half_word_read_write() {
        const ADDR: u32 = 0x02;
        let mut mem = Memory::new(1024);
        for data in 0..0xFFFF {
            mem.write_half_word(ADDR, data).unwrap();
            assert_eq!(data, mem.read_half_word(ADDR).unwrap());
        }
    }

    #[test]
    fn word_write() {
        const ADDR: u32 = 0x04;
        let mut mem = Memory::new(1024);

        mem.write_word(ADDR, 0x76821712).unwrap();

        // Is it little-endian?
        assert_eq!(mem.mem[ADDR as usize], 0x12);
        assert_eq!(mem.mem[(ADDR + 1) as usize], 0x17);
        assert_eq!(mem.mem[(ADDR + 2) as usize], 0x82);
        assert_eq!(mem.mem[(ADDR + 3) as usize], 0x76);
    }

    #[test]
    fn word_read() {
        const ADDR: u32 = 0x04;
        let mut mem = Memory::new(1024);

        // mem[ADDR] = 0x1712;
        mem.mem[ADDR as usize] = 0x12;
        mem.mem[(ADDR + 1) as usize] = 0x17;
        mem.mem[(ADDR + 2) as usize] = 0x82;
        mem.mem[(ADDR + 3) as usize] = 0x76;

        assert_eq!(0x76821712, mem.read_word(ADDR).unwrap());
    }

    #[test]
    fn word_read_write() {
        const ADDR: u32 = 0x04;
        let mut mem = Memory::new(1024);
        for data in 0xFE000000..0xFE100000 {
            mem.write_word(ADDR, data).unwrap();
            assert_eq!(data, mem.read_word(ADDR).unwrap());
        }
    }

    #[test]
    fn program_big_endian() {
        const NUM: u32 = 0x12345678;
        const BE_BYTES: [u8; 4] = [0x12, 0x34, 0x56, 0x78];

        let mut mem = Memory::new(1024);
        mem.program_be_bytes(&BE_BYTES).unwrap();

        assert_eq!(NUM, mem.read_word(0).unwrap());
    }

    #[test]
    fn program_little_endian() {
        const NUM: u32 = 0x12345678;
        const LE_BYTES: [u8; 4] = [0x78, 0x56, 0x34, 0x12];

        let mut mem = Memory::new(1024);
        mem.program_le_bytes(&LE_BYTES).unwrap();

        assert_eq!(NUM, mem.read_word(0).unwrap());
    }
}
