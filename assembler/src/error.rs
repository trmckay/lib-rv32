/// Enumeration of possible errors when assembling a program.
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
    IOError,
}
