pub fn parse_int(input: &str) -> Result<u32, std::num::ParseIntError> {
    if input.len() < 2 {
        return u32::from_str_radix(input, 10);
    }
    match &input[0..2] {
        "0x" | "0X" => u32::from_str_radix(&input[2..], 16),
        _ => u32::from_str_radix(input, 10),
    }
}
