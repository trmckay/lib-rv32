#![allow(dead_code)]

/// Make assertions about an MCU>
mod assertions;
/// Useful bitwise operations.
mod bits;
/// Enumeration for errors thrown by an MCU.
mod error;
/// Execution and decoding logic.
mod exec;
/// Generic utility functions.
mod util;

/// Decoding macros.
pub mod decode;
/// Reference implementation of an MCU.
pub mod mcu;
/// Traits to be implementation by other implementations of
/// an MCU.
pub mod traits;

pub use assertions::Assertions;
pub use error::RiscvError;
pub use exec::exec_one;

/// Array to match register numbers to their common names.
pub static REG_NAMES: &[&str] = &[
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];
