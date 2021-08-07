/// Enum that encapsulates the various different ways execution can fail.
/// Some errors are caused by other errors and reference them.
///
/// Instruction format errors contain `(instruction: u32, bad_field: u8)`.
///
/// Memory errors contain `(address: u32)`.
///
/// Register file errors contain `(reg_num: u8)`.
#[derive(Debug, PartialEq)]
pub enum RiscvError {
    InvalidOpcodeError(u32, u8),
    InvalidFunc3Error(u32, u8),
    InvalidFunc7Error(u32, u8),
    RegisterOutOfRangeError(u8),
    MemoryOutOfBoundsError(u32),
    MemoryAlignmentError(u32),
}

#[derive(Debug, PartialEq)]
pub enum AssemblerError {
    InvalidOperationError,
    NoSuchLabelError,
    NoSuchRegisterError,
    WrongOperandTypeError,
    TooManyTokensError,
    TooFewTokensError,
    ImmediateTooLargeError,
    InvalidImmediateError,
}
