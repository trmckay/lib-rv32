/// Enum that encapsulates the various different ways execution can fail.
/// Some errors are caused by other errors and reference them.
#[derive(Debug, PartialEq)]
pub enum RiscvError {
    InvalidOpcodeError,
    InvalidFunctionError,
    RegisterOutOfRangeError,
    MemoryOutOfBoundsError,
    MemoryAlignmentError,
}
