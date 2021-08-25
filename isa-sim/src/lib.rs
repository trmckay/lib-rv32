/// Decoding macros.
pub mod decode;
/// Enumeration for errors thrown by an MCU.
mod error;
/// Execution and decoding logic.
mod exec;

/// Traits to be implementation by other implementations of
/// an MCU.
pub mod traits;

#[cfg(test)]
mod test;

/// Re-export common library.
pub use lib_rv32_common as common;

pub use error::RiscvError;
pub use exec::exec_one;
