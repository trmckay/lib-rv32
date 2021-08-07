#[macro_export]
macro_rules! encode_rd {
    ($n:expr) => {
        (($n as u32) << 7)
    };
}

#[macro_export]
macro_rules! encode_rs1 {
    ($n:expr) => {
        (($n as u32) << 15)
    };
}

#[macro_export]
macro_rules! encode_rs2 {
    ($n:expr) => {
        (($n as u32) << 20)
    };
}

#[macro_export]
macro_rules! encode_func3 {
    ($n: expr) => {
        (($n as u32) << 12)
    };
}

#[macro_export]
macro_rules! encode_func7 {
    ($n: expr) => {
        (($n as u32) << 25)
    };
}

/// Take a `u32` and encode it as an I-type immediate. Evaluates to a
/// `Result<u32, AssemblerError>` where the `u32` is a 32-bit
/// bitmask of the immediate.
#[macro_export]
macro_rules! encode_i_imm {
    ($n:expr) => {{
        let n_bits = ($n & 0xFFF) as u32;
        let mut res: u32 = 0;
        res |= (n_bits as u32) << 20;
        let sign_bit = if (($n as u32) & crate::bit!(31)) != 0 {
            1
        } else {
            0
        };
        res |= sign_bit << 31;
        res
    }};
}

#[macro_export]
macro_rules! encode_j_imm {
    ($n:expr) => {
        (((($n as u32) & 0b10000000_00000000_00000000_00000000) << (31 - 31))
            | ((($n as u32) & 0b00000000_00001111_11110000_00000000) << (12 - 12))
            | ((($n as u32) & 0b00000000_00010000_00000000_00000000) << (20 - 20))
            | ((($n as u32) & 0b00000000_00000000_00000111_11100000) << (25 - 25))
            | ((($n as u32) & 0b00000000_00000000_00000000_00011110) << (21 - 1)))
    };
}

#[macro_export]
macro_rules! encode_u_imm {
    ($n:expr) => {
        (($n as u32) << 12)
    };
}

#[macro_export]
macro_rules! encode_s_imm {
    ($n:expr) => {
        (((($n as u32) & 0b111111100000) << 25) | ((($n as u32) & 0b000000011111) << 7))
    };
}

#[macro_export]
macro_rules! encode_b_imm {
    ($n:expr) => {
        $n as u32
    };
}
