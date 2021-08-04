pub use crate::{bit_concat, bit_extend, bit_slice, sized_bit_extend, sized_bit_slice};

pub const OPCODE_LUI: u8 = 0b0110111;
pub const OPCODE_AUIPC: u8 = 0b0010111;
pub const OPCODE_JAL: u8 = 0b1101111;
pub const OPCODE_JALR: u8 = 0b1100111;
pub const OPCODE_BRANCH: u8 = 0b1100011;
pub const OPCODE_LOAD: u8 = 0b0000011;
pub const OPCODE_STORE: u8 = 0b0100011;
pub const OPCODE_ARITHMETIC_IMM: u8 = 0b0010011;
pub const OPCODE_ARITHMETIC: u8 = 0b0110011;

pub const FUNC3_BEQ: u8 = 0b000;
pub const FUNC3_BNE: u8 = 0b001;
pub const FUNC3_BLT: u8 = 0b100;
pub const FUNC3_BGE: u8 = 0b101;
pub const FUNC3_BLTU: u8 = 0b110;
pub const FUNC3_BGEU: u8 = 0b111;
pub const FUNC3_LB: u8 = 0b000;
pub const FUNC3_LH: u8 = 0b001;
pub const FUNC3_LW: u8 = 0b010;
pub const FUNC3_LBU: u8 = 0b100;
pub const FUNC3_LHU: u8 = 0b101;
pub const FUNC3_SB: u8 = 0b000;
pub const FUNC3_SH: u8 = 0b001;
pub const FUNC3_SW: u8 = 0b010;
pub const FUNC3_ADD_SUB: u8 = 0b000;
pub const FUNC3_SLL: u8 = 0b001;
pub const FUNC3_SLT: u8 = 0b010;
pub const FUNC3_SLTU: u8 = 0b011;
pub const FUNC3_XOR: u8 = 0b100;
pub const FUNC3_SRA_SRL: u8 = 0b101;
pub const FUNC3_OR: u8 = 0b110;
pub const FUNC3_AND: u8 = 0b111;

pub const FUNC7_ADD: u8 = 0b0000000;
pub const FUNC7_SUB: u8 = 0b0100000;
pub const FUNC7_SRA: u8 = 0b0000000;
pub const FUNC7_SRL: u8 = 0b0100000;

#[macro_export]
macro_rules! j_imm {
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

#[macro_export]
macro_rules! u_imm {
    ($ir:expr) => {
        bit_concat!(sized_bit_slice!($ir, 31, 12), sized_bit_extend!(0, 12)) as u32
    };
}

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

#[macro_export]
macro_rules! i_imm {
    ($ir:expr) => {
        bit_concat!(
            sized_bit_extend!(bit_slice!($ir, 31), 20),
            sized_bit_slice!($ir, 31, 20)
        ) as u32
    };
}

#[macro_export]
macro_rules! s_imm {
    ($ir:expr) => {
        bit_concat!(
            sized_bit_extend!(bit_slice!($ir, 31), 20),
            sized_bit_slice!($ir, 31, 25),
            sized_bit_slice!($ir, 11, 7)
        ) as u32
    };
}

#[macro_export]
macro_rules! func3 {
    ($ir:expr) => {
        bit_slice!($ir, 14, 12) as u8
    };
}

#[macro_export]
macro_rules! func7 {
    ($ir:expr) => {
        bit_slice!($ir, 31, 25) as u8
    };
}

#[macro_export]
macro_rules! rd {
    ($ir:expr) => {
        bit_slice!($ir, 11, 7) as u8
    };
}

#[macro_export]
macro_rules! rs1 {
    ($ir:expr) => {
        bit_slice!($ir, 19, 15) as u8
    };
}

#[macro_export]
macro_rules! rs2 {
    ($ir:expr) => {
        bit_slice!($ir, 24, 20) as u8
    };
}

#[macro_export]
macro_rules! opcode {
    ($ir:expr) => {
        bit_slice!($ir, 6, 0) as u8
    };
}
