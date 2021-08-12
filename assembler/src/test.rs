#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_tokenize() {
        let tokens: Vec<String> = tokenize!("addi t0, t1, 17");
        assert_eq!(
            vec![
                "addi".to_string(),
                "t0".to_string(),
                "t1".to_string(),
                "17".to_string()
            ],
            tokens
        );
    }

    #[test]
    fn test_tokenize_with_offsets() {
        let tokens: Vec<String> = tokenize!("lw t0, 17(s0)");
        assert_eq!(
            vec![
                "lw".to_string(),
                "t0".to_string(),
                "17".to_string(),
                "s0".to_string(),
            ],
            tokens
        );
        let tokens: Vec<String> = tokenize!("lw x5, 0(x5)");
        assert_eq!(
            vec![
                "lw".to_string(),
                "x5".to_string(),
                "0".to_string(),
                "x5".to_string(),
            ],
            tokens
        );
    }

    #[test]
    fn test_tokenize_many_commas() {
        let tokens: Vec<String> = tokenize!("lw,,, t0,,,,, 17,,,(s0),,,,,,");
        assert_eq!(
            vec![
                "lw".to_string(),
                "t0".to_string(),
                "17".to_string(),
                "s0".to_string(),
            ],
            tokens
        );
    }

    #[test]
    fn test_tokenize_many_spaces() {
        let tokens: Vec<String> = tokenize!("lw    t0      17   (s0)      ");
        assert_eq!(
            vec![
                "lw".to_string(),
                "t0".to_string(),
                "17".to_string(),
                "s0".to_string(),
            ],
            tokens
        );
    }

    #[test]
    fn test_tokenize_label() {
        let tokens: Vec<String> = tokenize!("label: addi t0, t1, 12");
        assert_eq!(
            vec![
                "label:".to_string(),
                "addi".to_string(),
                "t0".to_string(),
                "t1".to_string(),
                "12".to_string(),
            ],
            tokens
        );
    }

    #[test]
    fn test_parse_imm() {
        let mut labels: HashMap<String, u32> = HashMap::new();
        labels.insert("loop".to_string(), 0);
        let pc = 4;

        assert_eq!(-4, parse_imm("loop", &labels, pc).unwrap() as i32);
        assert_eq!(-24, parse_imm("-24", &labels, pc).unwrap() as i32);
        assert_eq!(16, parse_imm("16", &labels, pc).unwrap());
    }
}
