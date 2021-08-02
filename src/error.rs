/// Enum that encapsulates the various different ways execution can fail.
/// Some errors are caused by other errors and reference them.
#[derive(Debug, PartialEq)]
pub enum RiscvError {
    InvalidOpcodeError(u32, u8),
    InvalidFunc3Error(u32, u8),
    InvalidFunc7Error(u32, u8),
    RegisterOutOfRangeError(u8),
    MemoryOutOfBoundsError(u32),
    MemoryAlignmentError(u32),
}
