/// Parse an integer from a string slice. If it leads with `0x` or `0X`, it
/// will be parsed as base 16, otherwise it will be parsed as base 10.
#[macro_export]
macro_rules! parse_int {
    ($t:ty,$s:expr) => {{
        if !$s.to_ascii_lowercase().starts_with("0x") {
            $s.parse()
        } else {
            <$t>::from_str_radix(&$s[2..], 16)
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::parse_int;

    #[test]
    fn test_parse_int() {
        assert_eq!(17, parse_int!(u32, "17").unwrap());
        assert_eq!(17, parse_int!(u32, "017").unwrap());
        assert_eq!(17, parse_int!(i32, "17").unwrap());

        assert_eq!(-21, parse_int!(i32, "-21").unwrap());

        assert_eq!(0x16, parse_int!(u32, "0x16").unwrap());
        assert_eq!(0x16, parse_int!(u32, "0x0016").unwrap());
    }
}
