/// Parse an integer from a string slice. If it leads with `0x` or `0X`, it
/// will be parsed as base 16, otherwise it will be parsed as base 10.
#[macro_export]
macro_rules! parse_int {
    ($t:ty,$s:expr) => {{
        if $s.len() < 2 && !$s.to_ascii_lowercase().starts_with("0x") {
            $s.parse()
        } else {
            <$t>::from_str_radix(&$s[2..], 16)
        }
    }};
}
