pub use lib_rv32_common::{bit_concat, bit_extend, bit_slice, sized_bit_extend, sized_bit_slice};

/// Decode the J-type immediate from a `u32` formatted instruction.
#[macro_export]
macro_rules! decode_j_imm {
    ($ir:expr) => {
        bit_concat!(
            sized_bit_extend!(bit_slice!($ir, 31), 12),
            sized_bit_slice!($ir, 19, 12),
            sized_bit_slice!($ir, 20),
            sized_bit_slice!($ir, 30, 21),
            sized_bit_extend!(0, 1)
        ) as u32
    };
}

/// Decode the U-type immediate from a `u32` formatted instruction.
#[macro_export]
macro_rules! decode_u_imm {
    ($ir:expr) => {
        bit_concat!(sized_bit_slice!($ir, 31, 12), sized_bit_extend!(0, 12)) as u32
    };
}

/// Decode the B-type immediate from a `u32` formatted instruction.
#[macro_export]
macro_rules! b_imm {
    ($ir:expr) => {
        bit_concat!(
            sized_bit_extend!(bit_slice!($ir, 31), 20),
            sized_bit_slice!($ir, 7),
            sized_bit_slice!($ir, 30, 25),
            sized_bit_slice!($ir, 11, 8),
            sized_bit_extend!(0, 1)
        ) as u32
    };
}

/// Decode the I-type immediate from a `u32` formatted instruction.
#[macro_export]
macro_rules! decode_i_imm {
    ($ir:expr) => {
        bit_concat!(
            sized_bit_extend!(bit_slice!($ir, 31), 20),
            sized_bit_slice!($ir, 31, 20)
        ) as u32
    };
}

/// Decode the S-type immediate from a `u32` formatted instruction.
#[macro_export]
macro_rules! decode_s_imm {
    ($ir:expr) => {
        bit_concat!(
            sized_bit_extend!(bit_slice!($ir, 31), 20),
            sized_bit_slice!($ir, 31, 25),
            sized_bit_slice!($ir, 11, 7)
        ) as u32
    };
}

/// Decode the FUNC3 field from a `u32` formatted instruction.
#[macro_export]
macro_rules! decode_func3 {
    ($ir:expr) => {
        bit_slice!($ir, 14, 12) as u8
    };
}

/// Decode the FUNC7 field from a `u32` formatted instruction.
#[macro_export]
macro_rules! decode_func7 {
    ($ir:expr) => {
        bit_slice!($ir, 31, 25) as u8
    };
}

/// Decode the destination register field from a `u32` formatted instruction.
#[macro_export]
macro_rules! decode_rd {
    ($ir:expr) => {
        bit_slice!($ir, 11, 7) as u8
    };
}

/// Decode the first operand register field from a `u32` formatted instruction.
#[macro_export]
macro_rules! decode_rs1 {
    ($ir:expr) => {
        bit_slice!($ir, 19, 15) as u8
    };
}

/// Decode the second operand register field from a `u32` formatted instruction.
#[macro_export]
macro_rules! decode_rs2 {
    ($ir:expr) => {
        bit_slice!($ir, 24, 20) as u8
    };
}

/// Decode the opcode field from a `u32` formatted instruction.
#[macro_export]
macro_rules! decode_opcode {
    ($ir:expr) => {
        bit_slice!($ir, 6, 0) as u8
    };
}
