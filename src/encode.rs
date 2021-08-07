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
