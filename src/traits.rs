use crate::RiscvError;

/// Trait to be implemented by a RISC-V register file. Should support
/// reads to registers 0-31 and writes to registers 1-31.
pub trait RegisterFile {
    /// Read a value from the register numbered `num`. Returns a `Result` containing
    /// an error, or the `u32` data contained.
    fn read(&self, num: u8) -> Result<u32, RiscvError>;

    /// Write a value `data` to the register numbered `num`. Returns a `Result` containing
    /// an error if one occured, otherwise returns an empty `Result`.
    fn write(&mut self, num: u8, data: u32) -> Result<(), RiscvError>;
}

pub trait Memory {
    /// This should have the same behavior as `read_word`, with the distinction that
    /// it does not generate logs or count as an access for the purpose of performance
    /// counters.
    fn fetch(&self, pc: u32) -> Result<u32, RiscvError>;

    /// Read a 32-bit word from the address `addr`. Returns a `Result` containing
    /// an error, or the `u32` data contained.
    fn read_word(&self, addr: u32) -> Result<u32, RiscvError>;

    /// Read a 16-bit half-word from the address `addr`. Returns a `Result` containing
    /// an error, or the `u32` data contained.
    fn read_half_word(&self, addr: u32) -> Result<u32, RiscvError>;

    /// Read a byte from the address `addr`. Returns a `Result` containing
    /// an error, or the `u32` data contained.
    fn read_byte(&self, addr: u32) -> Result<u32, RiscvError>;

    /// Write a 32-bit word `data` to the address `addr`. Returns a `Result` containing
    /// an error, otherwise returns an empty `Result`.
    ///
    /// This makes no guarantees about endianness, only that `read_word` returns the same
    /// data after a `write_word`.
    fn write_word(&mut self, addr: u32, data: u32) -> Result<(), RiscvError>;

    /// Write 16-bit half-word `data` to the address `addr`. Returns a `Result` containing
    /// an error, otherwise returns an empty `Result`.
    ///
    /// This makes no guarantees about endianness, only that `read_half_word` returns the same
    /// data after a `write_half_word`.
    fn write_half_word(&mut self, addr: u32, data: u32) -> Result<(), RiscvError>;

    /// Write byte `data` to the address `addr`. Returns a `Result` containing
    /// an error, otherwise returns an empty `Result`.
    ///
    /// This makes no guarantees about endianness, only that `read_byte` returns the same
    /// data after a `write_byte`.
    fn write_byte(&mut self, addr: u32, data: u32) -> Result<(), RiscvError>;
}
