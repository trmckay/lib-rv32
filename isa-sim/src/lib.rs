/// Decoding macros.
pub mod decode;
/// Enumeration for errors thrown by an MCU.
mod error;
/// Execution and decoding logic.
mod exec;

/// Traits to be implementation by other implementations of
/// an MCU.
pub mod traits;

pub use error::{AssemblerError, RiscvError};
pub use exec::exec_one;

pub use lib_rv32_common as common;

#[cfg(test)]
mod test;
