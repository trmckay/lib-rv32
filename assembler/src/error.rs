/// Enumeration of possible errors when assembling a program.
#[derive(Debug, PartialEq)]
pub enum AssemblerError {
    InvalidOperationError(String),
    NoSuchLabelError(String),
    NoSuchRegisterError(String),
    WrongOperandTypeError(String),
    TooManyTokensError,
    TooFewTokensError,
    ImmediateTooLargeError(String),
    InvalidImmediateError(String),
    IOError,
}
