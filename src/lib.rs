#![allow(overflowing_literals)]
#![allow(arithmetic_overflow)]
#![allow(dead_code)]

mod bits;
mod err;
mod exec;
mod traits;

pub mod decode;
pub mod mcu;

pub use err::RiscvError;
pub use exec::exec_one;
pub use traits::{Memory, RegisterFile};