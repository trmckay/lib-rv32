/// Returns a bitmask for the n'th bit.
#[macro_export]
macro_rules! bit {
    ($n:expr) => {
        ((0b1 as u32) << $n)
    };
}

/// Macro to help with bit level access to integers. Example
/// attempts to mimic Verilog syntax.
///
/// Example:
///
/// ```
/// # use lib_rv32::bit_slice;
/// bit_slice!(0b1101, 3, 2) == 0b11;
/// bit_slice!(0b1101, 1) == 0b0;
/// ```
#[macro_export]
macro_rules! bit_slice {
    ($n:expr, $i:expr) => {
        ($n & (0b1 << $i)) >> $i
    };

    ($n:expr, $msb:expr, $lsb:expr) => {
        ($n & (((0b1 << ($msb - $lsb + 1)) - 1) << $lsb)) >> $lsb
    };
}

/// Concatenate the bits of integers.
///
/// Example:
///
/// ```
/// # use lib_rv32::bit_concat;
/// bit_concat!(
///     (0b111, 3),
///     (0b01, 2)
/// ) == 0b11101;
/// ```
#[macro_export]
macro_rules! bit_concat {
    ($($x:expr),*) => {{
        let mut i = 0;
        let mut t = 0;
        for n in [$($x),*].iter().rev() {
            t += n.0 << i;
            i += n.1;
        }
        t
    }}
}

/// Extend a bit (useful for sign extension).
///
/// Example:
///
/// ```
/// # use lib_rv32::bit_extend;
/// bit_extend!(0b1, 8) == 0b1111_1111;
/// ```
#[macro_export]
macro_rules! bit_extend {
    ($n:expr, $r:expr) => {
        match $n {
            0 => 0,
            _ => (0..$r).map(|i| 1 << i).sum(),
        }
    };
}

/// Like `bit_slice`, but outputs the result and its
/// size in a tuple.
#[macro_export]
macro_rules! sized_bit_slice {
    ($n: expr, $i:expr) => {
        (bit_slice!($n, $i), 1)
    };

    ($n: expr, $msb:expr, $lsb:expr) => {
        (bit_slice!($n, $msb, $lsb), $msb - $lsb + 1)
    };
}

/// Like `bit_extend`, but outputs the result and its
/// size in a tuple.
#[macro_export]
macro_rules! sized_bit_extend {
    ($n: expr, $r:expr) => {
        (bit_extend!($n, $r), $r)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn bit_slice() {
        let x = 0b1011;

        assert_eq!(0b1, bit_slice!(x, 3));
        assert_eq!(0b0, bit_slice!(x, 2));
        assert_eq!(0b1, bit_slice!(x, 1));
        assert_eq!(0b1, bit_slice!(x, 0));

        assert_eq!(0b10, bit_slice!(x, 3, 2));
        assert_eq!(0b101, bit_slice!(x, 3, 1));
        assert_eq!(0b1011, bit_slice!(x, 3, 0));
        assert_eq!(0b011, bit_slice!(x, 2, 0));
        assert_eq!(0b11, bit_slice!(x, 1, 0));
    }

    #[test]
    fn bit_concat() {
        assert_eq!(0b1101, bit_concat!((0b11, 2), (0b01, 2)));
    }

    #[test]
    fn bit_extend() {
        assert_eq!(0b1111, bit_extend!(1, 4));
        assert_eq!(0b0, bit_extend!(0, 32));
    }
}
