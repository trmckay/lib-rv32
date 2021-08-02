#![allow(dead_code)]

mod bits;
mod error;
mod exec;
mod traits;

pub mod decode;
pub mod mcu;

pub use error::RiscvError;
pub use exec::exec_one;
pub use traits::{Memory, RegisterFile};
